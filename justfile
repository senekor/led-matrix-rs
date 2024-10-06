_default:
    just --list --unsorted

deploy-documentation:
    ./dev/scripts/deploy_documentation.sh

# checks compilation for both targets and examples
check:
    cargo clippy
    cargo clippy --target thumbv6m-none-eabi
    cargo clippy --examples
    cargo doc --quiet

# promp for an example to run in the emulator
example:
    cargo run --example $(echo examples/* | xargs -n1 | xargs -I _ basename _ | cut -d. --fields 1 | fzf)
