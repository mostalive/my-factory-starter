//! Step-level accounting for the factory's Gherkin suite.
//!
//! cucumber-rs stops a scenario at its first non-passing step, so the console
//! output never says how many steps are still *open* after that step. This
//! writer counts them from the parsed feature, prints a per-example table, and
//! (on a full run) records the run in `factory/stats/runs.jsonl` and rewrites
//! `factory/stats/steps.md`.
//!
//! Open steps are steps that have not passed yet: they are either failing,
//! undefined, or never reached. In terms of work they all count the same.
//!
//! The median number of open steps per example is the rolling-wave
//! signal: it should go down as the iteration progresses.

use std::{
    collections::{BTreeMap, HashMap},
    fmt::Write as _,
    fs,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use cucumber::{
    Event, Writer,
    cli,
    event::{
        Cucumber, Feature as FeatureEvent, Hook as HookEvent,
        Rule as RuleEvent, Scenario as ScenarioEvent, Step as StepEvent,
    },
    gherkin, parser, writer,
};

use super::FactoryWorld;

/// Key identifying one example. Scenario Outline rows are expanded into
/// scenarios with their own line, so this stays unique.
type CaseKey = (String, usize, String);

/// Outcome of a single step that actually ran.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Outcome {
    Passed,
    Failed,
    Skipped,
}

/// One example of one feature, with every step it is supposed to run.
#[derive(Debug)]
struct Case {
    feature: String,
    example: String,
    step_texts: Vec<String>,
    outcomes: Vec<Outcome>,
}

impl Case {
    fn total(&self) -> usize {
        self.step_texts.len()
    }

    fn passed(&self) -> usize {
        self.outcomes
            .iter()
            .filter(|o| **o == Outcome::Passed)
            .count()
    }

    fn open(&self) -> usize {
        self.total() - self.passed()
    }

    /// The first step that did not pass: the failing or undefined one, or the
    /// step that would have run next had the scenario not stopped.
    fn blocker(&self) -> Option<&str> {
        let index = self
            .outcomes
            .iter()
            .position(|o| *o != Outcome::Passed)
            .unwrap_or(self.outcomes.len());
        self.step_texts.get(index).map(String::as_str)
    }
}

/// [`Writer`] collecting per-example step counts, then reporting them.
#[derive(Debug, Default)]
pub struct StepStats {
    cases: BTreeMap<CaseKey, Case>,
    current: Option<CaseKey>,
    passed_steps: usize,
    skipped_steps: usize,
    failed_steps: usize,
    parsing_errors: usize,
    hook_errors: usize,
}

impl StepStats {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    fn feature(
        &mut self,
        feature: &gherkin::Feature,
        event: FeatureEvent<FactoryWorld>,
    ) {
        match event {
            FeatureEvent::Scenario(scenario, event) => {
                self.scenario(feature, &scenario, event.event);
            }
            FeatureEvent::Rule(_, RuleEvent::Scenario(scenario, event)) => {
                self.scenario(feature, &scenario, event.event);
            }
            _ => {}
        }
    }

    fn scenario(
        &mut self,
        feature: &gherkin::Feature,
        scenario: &gherkin::Scenario,
        event: ScenarioEvent<FactoryWorld>,
    ) {
        match event {
            ScenarioEvent::Started => {
                let file = feature
                    .path
                    .as_ref()
                    .and_then(|path| path.file_name())
                    .map_or_else(
                        || "<unknown>".to_owned(),
                        |name| name.to_string_lossy().into_owned(),
                    );
                let key =
                    (file.clone(), scenario.position.line, scenario.name.clone());
                let step_texts = feature
                    .background
                    .iter()
                    .flat_map(|background| background.steps.iter())
                    .chain(scenario.steps.iter())
                    .map(step_text)
                    .collect();

                self.current = Some(key.clone());
                self.cases.insert(
                    key,
                    Case {
                        feature: file,
                        example: example_label(scenario),
                        step_texts,
                        outcomes: Vec::new(),
                    },
                );
            }
            ScenarioEvent::Background(_, event)
            | ScenarioEvent::Step(_, event) => self.step(event),
            ScenarioEvent::Hook(_, HookEvent::Failed(..)) => {
                self.hook_errors += 1;
            }
            ScenarioEvent::Finished => self.current = None,
            _ => {}
        }
    }

    fn step(&mut self, event: StepEvent<FactoryWorld>) {
        let Some(key) = self.current.clone() else {
            return;
        };
        let Some(case) = self.cases.get_mut(&key) else {
            return;
        };

        match event {
            StepEvent::Passed(..) => {
                self.passed_steps += 1;
                case.outcomes.push(Outcome::Passed);
            }
            StepEvent::Skipped => {
                self.skipped_steps += 1;
                case.outcomes.push(Outcome::Skipped);
            }
            StepEvent::Failed(..) => {
                self.failed_steps += 1;
                case.outcomes.push(Outcome::Failed);
            }
            StepEvent::Started => {}
        }
    }

    fn finish(&mut self) {
        if self.cases.is_empty() {
            return;
        }

        let report = Report::new(&self.cases);
        let dir = stats_dir();
        let runs = dir.join("runs.jsonl");
        let filtered = filtered_run();
        let previous = if filtered { None } else { last_median(&runs) };

        report.print(previous);

        if filtered {
            println!(
                "\n  partial run (name/tag filter, fail-fast or @real-agent): \
                 not recorded, so the trend stays comparable"
            );
            return;
        }

        let at = now_utc();
        let iteration = iteration();

        if let Err(e) = fs::create_dir_all(&dir) {
            eprintln!("steps: cannot create {}: {e}", dir.display());
            return;
        }
        if let Err(e) = append(&runs, &report.json(&at, &iteration)) {
            eprintln!("steps: cannot append {}: {e}", runs.display());
        }
        if let Err(e) =
            fs::write(dir.join("steps.md"), report.markdown(&at, &iteration, previous))
        {
            eprintln!("steps: cannot write steps.md: {e}");
        }
    }
}

impl Writer<FactoryWorld> for StepStats {
    type Cli = cli::Empty;

    async fn handle_event(
        &mut self,
        event: parser::Result<Event<Cucumber<FactoryWorld>>>,
        _: &Self::Cli,
    ) {
        match event.map(Event::into_inner) {
            Ok(Cucumber::Feature(feature, event)) => {
                self.feature(&feature, event);
            }
            Ok(Cucumber::Finished) => self.finish(),
            Err(_) => self.parsing_errors += 1,
            _ => {}
        }
    }
}

impl writer::Normalized for StepStats {}

impl writer::Stats<FactoryWorld> for StepStats {
    fn passed_steps(&self) -> usize {
        self.passed_steps
    }

    fn skipped_steps(&self) -> usize {
        self.skipped_steps
    }

    fn failed_steps(&self) -> usize {
        self.failed_steps
    }

    fn retried_steps(&self) -> usize {
        0
    }

    fn parsing_errors(&self) -> usize {
        self.parsing_errors
    }

    fn hook_errors(&self) -> usize {
        self.hook_errors
    }
}

/// Totals for one feature file.
#[derive(Debug, Default)]
struct FeatureTotals {
    examples: usize,
    examples_open: usize,
    steps: usize,
    passed: usize,
}

impl FeatureTotals {
    fn open(&self) -> usize {
        self.steps - self.passed
    }
}

/// One row of the per-example table.
struct Row<'a> {
    feature: &'a str,
    example: &'a str,
    steps: usize,
    passed: usize,
    open: usize,
}

/// Snapshot of a whole run.
struct Report<'a> {
    rows: Vec<Row<'a>>,
    features: BTreeMap<&'a str, FeatureTotals>,
    blockers: Vec<(&'a str, usize)>,
    median_open: f64,
    examples: usize,
    examples_open: usize,
    steps: usize,
    passed: usize,
}

impl<'a> Report<'a> {
    fn new(cases: &'a BTreeMap<CaseKey, Case>) -> Self {
        let mut rows = Vec::with_capacity(cases.len());
        let mut features: BTreeMap<&str, FeatureTotals> = BTreeMap::new();
        let mut blockers: HashMap<&str, usize> = HashMap::new();

        for case in cases.values() {
            let (steps, passed, open) =
                (case.total(), case.passed(), case.open());

            rows.push(Row {
                feature: &case.feature,
                example: &case.example,
                steps,
                passed,
                open,
            });

            let totals = features.entry(&case.feature).or_default();
            totals.examples += 1;
            totals.examples_open += usize::from(open > 0);
            totals.steps += steps;
            totals.passed += passed;

            if let Some(blocker) = case.blocker() {
                *blockers.entry(blocker).or_default() += 1;
            }
        }

        let mut blockers: Vec<_> = blockers.into_iter().collect();
        blockers.sort_by(|(a_step, a_count), (b_step, b_count)| {
            b_count.cmp(a_count).then_with(|| a_step.cmp(b_step))
        });

        let mut open: Vec<usize> = rows.iter().map(|row| row.open).collect();
        open.sort_unstable();
        let median_open = median(&open);

        let examples = rows.len();
        let examples_open = rows.iter().filter(|row| row.open > 0).count();
        let steps = rows.iter().map(|row| row.steps).sum();
        let passed = rows.iter().map(|row| row.passed).sum();

        Self {
            rows,
            features,
            blockers,
            median_open,
            examples,
            examples_open,
            steps,
            passed,
        }
    }

    fn print(&self, previous: Option<f64>) {
        let features: Vec<String> = self
            .rows
            .iter()
            .map(|row| fit(row.feature, 24))
            .collect();
        let examples: Vec<String> = self
            .rows
            .iter()
            .map(|row| fit(row.example, 70))
            .collect();
        let widths = (
            features.iter().map(|f| f.chars().count()).max().unwrap_or(7).max(7),
            examples.iter().map(|e| e.chars().count()).max().unwrap_or(8),
        );

        println!("\n[Steps]");
        println!(
            "{:<fw$}  {:<ew$}  {:>5}  {:>6}  {:>4}",
            "Feature",
            "Example",
            "Steps",
            "Passed",
            "Open",
            fw = widths.0,
            ew = widths.1,
        );
        for (row, feature, example) in
            self.rows.iter().zip(features).zip(examples).map(|((row, f), e)| (row, f, e))
        {
            println!(
                "{:<fw$}  {:<ew$}  {:>5}  {:>6}  {:>4}",
                feature,
                example,
                row.steps,
                row.passed,
                row.open,
                fw = widths.0,
                ew = widths.1,
            );
        }

        println!(
            "\n{} examples ({} done, {} open)",
            self.examples,
            self.examples - self.examples_open,
            self.examples_open,
        );
        println!(
            "{} steps ({} passed, {} open)",
            self.steps,
            self.passed,
            self.steps - self.passed,
        );
        println!(
            "median open steps per example: {:.1}{}",
            self.median_open,
            delta(previous, self.median_open),
        );

        if !self.blockers.is_empty() {
            println!("\nopen steps by blocker:");
            for (step, count) in self.blockers.iter().take(5) {
                println!("  {count:>3}  {}", fit(step, 80));
            }
        }
    }

    fn json(&self, at: &str, iteration: &str) -> String {
        let mut json = String::new();
        let _ = write!(
            json,
            "{{\"at\":\"{at}\",\"iteration\":\"{}\",\"filtered\":false,\
             \"examples\":{},\"examples_open\":{},\"steps\":{},\
             \"steps_passed\":{},\"steps_open\":{},\"median_open_steps\":{:.1},",
            escape(iteration),
            self.examples,
            self.examples_open,
            self.steps,
            self.passed,
            self.steps - self.passed,
            self.median_open,
        );

        json.push_str("\"blockers\":[");
        for (i, (step, count)) in self.blockers.iter().enumerate() {
            if i > 0 {
                json.push(',');
            }
            let _ = write!(
                json,
                "{{\"step\":\"{}\",\"examples\":{count}}}",
                escape(step),
            );
        }

        json.push_str("],\"by_feature\":[");
        for (i, (file, totals)) in self.features.iter().enumerate() {
            if i > 0 {
                json.push(',');
            }
            let _ = write!(
                json,
                "{{\"file\":\"{}\",\"examples\":{},\"examples_open\":{},\
                 \"steps\":{},\"steps_passed\":{},\"steps_open\":{}}}",
                escape(file),
                totals.examples,
                totals.examples_open,
                totals.steps,
                totals.passed,
                totals.open(),
            );
        }
        json.push_str("]}\n");
        json
    }

    fn markdown(
        &self,
        at: &str,
        iteration: &str,
        previous: Option<f64>,
    ) -> String {
        let mut md = String::new();
        md.push_str("# Steps\n\n");
        md.push_str(
            "Generated by `factory/tests/support/stats.rs` after every full \
             run. Commit it to keep the history.\n\n",
        );
        let _ = writeln!(md, "Run at `{at}` · iteration `{iteration}`\n");
        let _ = writeln!(
            md,
            "- {} examples: {} done, {} open",
            self.examples,
            self.examples - self.examples_open,
            self.examples_open,
        );
        let _ = writeln!(
            md,
            "- {} steps: {} passed, {} open",
            self.steps,
            self.passed,
            self.steps - self.passed,
        );
        let _ = writeln!(
            md,
            "- Median open steps per example: **{:.1}**{}\n",
            self.median_open,
            delta(previous, self.median_open),
        );

        md.push_str("| Feature | Example | Steps | Passed | Open |\n");
        md.push_str("| --- | --- | ---: | ---: | ---: |\n");
        for row in &self.rows {
            let _ = writeln!(
                md,
                "| {} | {} | {} | {} | {} |",
                cell(row.feature),
                cell(row.example),
                row.steps,
                row.passed,
                row.open,
            );
        }

        md.push_str("\n## Per feature\n\n");
        md.push_str(
            "| Feature | Examples | Examples open | Steps | Passed | Open |\n",
        );
        md.push_str("| --- | ---: | ---: | ---: | ---: | ---: |\n");
        for (file, totals) in &self.features {
            let _ = writeln!(
                md,
                "| {} | {} | {} | {} | {} | {} |",
                cell(file),
                totals.examples,
                totals.examples_open,
                totals.steps,
                totals.passed,
                totals.open(),
            );
        }

        if !self.blockers.is_empty() {
            md.push_str("\n## Open steps by blocker\n\n");
            md.push_str("| Blocked on | Examples |\n| --- | ---: |\n");
            for (step, count) in &self.blockers {
                let _ = writeln!(md, "| {} | {} |", cell(step), count);
            }
        }

        md
    }
}

/// Scenario Outline rows share the outline's name, so add the row's values to
/// tell the examples apart.
fn example_label(scenario: &gherkin::Scenario) -> String {
    let rows = scenario
        .examples
        .first()
        .and_then(|examples| examples.table.as_ref())
        .map_or(&[][..], |table| table.rows.as_slice());

    match rows {
        [header, values, ..] if !header.is_empty() => {
            let pairs: Vec<String> = header
                .iter()
                .zip(values)
                .map(|(name, value)| format!("{name}={value}"))
                .collect();
            format!("{} ({})", scenario.name, pairs.join(", "))
        }
        _ => scenario.name.clone(),
    }
}

/// Keeps the console table readable; the Markdown file keeps full names.
fn fit(text: &str, width: usize) -> String {
    if text.chars().count() <= width {
        text.to_owned()
    } else {
        let mut short: String = text.chars().take(width - 1).collect();
        short.push('…');
        short
    }
}

fn step_text(step: &gherkin::Step) -> String {
    format!("{} {}", step.keyword.trim(), step.value)
}

fn median(values: &[usize]) -> f64 {
    match values.len() {
        0 => 0.0,
        len if len % 2 == 1 => values[len / 2] as f64,
        len => (values[len / 2 - 1] + values[len / 2]) as f64 / 2.0,
    }
}

fn delta(previous: Option<f64>, current: f64) -> String {
    previous.map_or_else(String::new, |previous| {
        let change = current - previous;
        let word = if change < 0.0 {
            "down"
        } else if change > 0.0 {
            "up"
        } else {
            "unchanged"
        };
        if change == 0.0 {
            format!(" ({word} from the previous run)")
        } else {
            format!(" ({word} {:.1} from the previous run)", change.abs())
        }
    })
}

/// Steps counters, keyed by the file they belong to.
fn stats_dir() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("stats")
}

fn iteration() -> String {
    fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("ITERATION"))
        .map_or_else(|_| "unknown".to_owned(), |text| text.trim().to_owned())
}

/// Appends one JSON object per line. Uses `O_APPEND` via `append(true)`.
fn append(path: &Path, line: &str) -> std::io::Result<()> {
    use std::io::Write as _;
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    file.write_all(line.as_bytes())
}

fn last_median(path: &Path) -> Option<f64> {
    let text = fs::read_to_string(path).ok()?;
    text.lines()
        .rev()
        .filter(|line| line.contains("\"filtered\":false"))
        .find_map(|line| {
            let rest =
                line.split_once("\"median_open_steps\":")?.1;
            let number: String = rest
                .chars()
                .take_while(|c| c.is_ascii_digit() || *c == '.')
                .collect();
            number.parse().ok()
        })
}

/// A run with a name/tag filter or `--fail-fast` only executes part of the
/// suite, so it must not move the trend. Same for the `@real-agent` subset.
fn filtered_run() -> bool {
    if cfg!(feature = "real-agent") {
        return true;
    }
    std::env::args().skip(1).any(|arg| {
        matches!(
            arg.as_str(),
            "-n" | "--name"
                | "--scenario-name"
                | "-t"
                | "--tags"
                | "--fail-fast"
                | "--ff"
        ) || arg.starts_with("--name=")
            || arg.starts_with("--tags=")
            || arg.starts_with("-n")
    })
}

fn escape(text: &str) -> String {
    text.chars()
        .flat_map(|c| match c {
            '"' => "\\\"".chars().collect::<Vec<_>>(),
            '\\' => "\\\\".chars().collect(),
            '\n' => "\\n".chars().collect(),
            c => vec![c],
        })
        .collect()
}

/// Markdown table cells cannot contain a raw `|` or newline.
fn cell(text: &str) -> String {
    text.replace('|', "\\|").replace('\n', " ")
}

fn now_utc() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs());
    iso8601(seconds)
}

fn iso8601(seconds: u64) -> String {
    let days = (seconds / 86_400) as i64;
    let rest = seconds % 86_400;
    let (year, month, day) = civil_from_days(days);
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}Z",
        rest / 3_600,
        rest / 60 % 60,
        rest % 60,
    )
}

/// Howard Hinnant's `civil_from_days`, adapted for UTC.
fn civil_from_days(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let day_of_era = (z - era * 146_097) as u64;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524
            - day_of_era / 146_096)
            / 365;
    let year = year_of_era as i64 + era * 400;
    let day_of_year =
        day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let mp = (5 * day_of_year + 2) / 153;
    let day = (day_of_year - (153 * mp + 2) / 5 + 1) as u32;
    let month = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if month <= 2 { year + 1 } else { year }, month, day)
}
