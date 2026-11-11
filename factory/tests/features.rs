use cucumber::World as _;

#[derive(Debug, Default, cucumber::World)]
struct FactoryWorld {}

#[tokio::main]
async fn main() {
    FactoryWorld::cucumber()
        .fail_on_skipped()
        .filter_run_and_exit("spec/features", |feature, rule, scenario| {
            let real_agent = feature
                .tags
                .iter()
                .chain(rule.into_iter().flat_map(|rule| &rule.tags))
                .chain(&scenario.tags)
                .any(|tag| tag == "real-agent");
            real_agent == cfg!(feature = "real-agent")
        })
        .await;
}
