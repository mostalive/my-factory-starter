#!/bin/sh
set -eu

if [ ! -f .factory/plan.md ]; then
    mkdir -p .factory
    cat > .factory/plan.md <<'EOF'
- [ ] alpha
- [ ] beta
EOF

    printf '{"complete":false}\n'
    exit 0
fi


for task in alpha beta; do
    if grep -qF -- "- [ ] $task" .factory/plan.md; then
        printf 'work for %s\n' "$task" > "$task.txt"

        sed "s/- \[ \] $task/- [x] $task/" \
            .factory/plan.md > .factory/plan.tmp
        mv .factory/plan.tmp .factory/plan.md

        printf '{"complete":false,"task":"%s"}\n' "$task"
        exit 0
    fi
done

printf '{"complete":true}\n'
