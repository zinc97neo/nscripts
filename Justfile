clean:
    rm -rf commands

build: clean
    cargo build --release
    mkdir commands
    cp target/release/wmbc commands/
