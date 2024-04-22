# Ruff Plugin

[![ci](https://github.com/fluentci-io/ruff-plugin/actions/workflows/ci.yml/badge.svg)](https://github.com/fluentci-io/ruff-plugin/actions/workflows/ci.yml)

This plugin sets up your CI/CD pipeline with a specific version of [ruff](https://github.com/astral-sh/ruff).

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm ruff setup
```

## Functions

| Name   | Description                                |
| ------ | ------------------------------------------ |
| setup  | Installs a specific version of ruff.       |
| check  | Run Ruff on the given files or directories |
| clean  | Clear any caches in the current directory and any subdirectories |
| format | Run the Ruff formatter on the given files or directories |

## Code Usage

Add `fluentci-pdk` crate to your `Cargo.toml`:

```toml
[dependencies]
fluentci-pdk = "0.1.9"
```

Use the following code to call the plugin:

```rust
use fluentci_pdk::dag;

// ...

dag().call("https://pkg.fluentci.io/ruff@v0.1.0?wasm=1", "setup", vec!["latest"])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: ruff
    args: |
      setup
    working-directory: example
- name: Show ruff version
  run: |
    type ruff
    ruff --version
```
