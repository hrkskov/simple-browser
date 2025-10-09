[working-directory('simple_browser')]
@run:
    cargo run --target

[working-directory('simple_browser')]
@build:
    cargo build --release

[working-directory('simple_browser')]
@test:
    cargo test

[working-directory('simple_browser')]
@lint:
    cargo clippy -- -D warnings

[working-directory('simple_browser/sb_core')]
@test-core:
    cargo test

[working-directory('simple_browser/sb_core')]
@lint-core:
    cargo clippy -- -D warnings

[working-directory('simple_browser/net/wasabi')]
@lint-net:
    cargo clippy -- -D warnings
