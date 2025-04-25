# TEVM CLI

A Rust CLI tool that runs Claude Code with specified prompts and automatically skips permissions.

## Features

- Wrapper for Claude Code CLI
- Automatically applies `--dangerously-skip-permissions` flag
- Docker image with Tevm monorepo pre-cloned and built
- Passes through all Claude Code arguments

## Usage

### Local Usage

Build and run the CLI:

```bash
cargo build --release
./target/release/tevm-cli --prompt "Your prompt here"
```

Additional Claude Code arguments can be passed after `--`:

```bash
./target/release/tevm-cli --prompt "Your prompt" -- --help
```

### Docker Usage

Build the Docker image:

```bash
docker build -t tevm-cli .
```

Run the CLI in Docker:

```bash
docker run tevm-cli --prompt "Your prompt here"
```

## Docker Image Contents

The Docker image includes:

- Ubuntu 24.04 base
- Node.js 20.x
- Claude Code CLI installed globally
- Tevm monorepo cloned and built
- Our Rust CLI tool pre-compiled and ready to use

## Development

To modify the CLI:

1. Update the Rust code in `src/main.rs`
2. Rebuild using `cargo build --release`
3. For Docker, rebuild the image with `docker build -t tevm-cli .`

## License

See the LICENSE file for details.