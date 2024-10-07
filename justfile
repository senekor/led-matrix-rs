_default:
    just --list --unsorted

deploy-documentation:
    ./dev/scripts/deploy_documentation.sh

# checks compilation for both targets and examples
check:
    cargo clippy
    cargo clippy --features tui
    cargo clippy --target thumbv6m-none-eabi
    cargo clippy --examples
    cargo doc --quiet

# promp for an example to run in the emulator
example:
    cargo run --example $(echo examples/* | xargs -n1 | xargs -I _ basename _ | cut -d. --fields 1 | fzf)

open-docs:
    cargo doc --open

# publish to git.buenzli.dev, add empty arg to disable dry run
publish *cargo-args="--dry-run":
    cargo publish --package led-matrix-core {{ cargo-args }}
    cargo publish --package led-matrix-bsp {{ cargo-args }}
    cargo publish --package led-matrix-tui {{ cargo-args }}
    cargo publish --package led-matrix-gui {{ cargo-args }}
    cargo publish --package led-matrix {{ cargo-args }}
