# bashx

Run bash scripts from GitHub repositories.

[![crates.io](https://img.shields.io/crates/v/bashx.svg)](https://crates.io/crates/bashx)

## Install

```bash
# From source
git clone https://github.com/DeepanshuMishraa/bashx
cd bashx
cargo build --release

# Install globally
cargo install  bashx

# Add to PATH (if not already configured)
export PATH="$HOME/.cargo/bin:$PATH"
```

## Quick Start

```bash
# Download scripts from a repo
bashx get https://github.com/username/scripts

# List available scripts
bashx list

# Run a script (prompts for confirmation)
bashx run backup

# Clean cache
bashx clean
```

## Commands

### `bashx get <url>`
Clones a GitHub repository to `~/.bashx/cache/`

### `bashx list`
Shows all `.sh` files in your cache

### `bashx run <name>`
Finds and executes a script by name (without `.sh` extension)
- Prompts for security confirmation
- Sets executable permissions
- Runs in script's directory

### `bashx clean`
Removes all cached scripts

## Security

⚠️ Scripts have full system access. Only run scripts from trusted sources.

## License

MIT

