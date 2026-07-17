# Ignis Lazari

Ever wonder how much money you're burning on a VM that's idle 23 hours a day?

Most hobby or personal projects get a handful of visitors — but if you're using VMs behind them running 24/7, then its billed at full price whether anyone shows up or not. With Ignis Lazari your "heavy" VM stays _off_ by default, and wakes up on demand the moment someone actually visits your site.

> You only pay for compute when compute is doing something.

It's a lightweight, provider-agnostic "activator" service — a tiny always-on process (perfectly suitable for a nano/micro VM instance) that starts your real infrastructure on request. Ofcourse, this comes at a cost of slightly reduced first visit experience after your VM is stopped.

> **"Ignis Lazari"?** — the spark/fire that raises "something" from the dead :)

## Why

Personal projects rarely have real traffic (a game server, a self-hosted app, a demo environment). Most cloud providers don't offer true "scale to zero" for full VMs the way they do for serverless functions — so the VM either runs all the time, or you manually start/stop it, which defeats the purpose of having a public project.

Ignis Lazari is a small always-on process that sits in front of your heavier VM(s). It runs on the cheapest possible instance (or as a tiny container) and exposes a public HTTP API that your hobby site's frontend can call to:

1. Check whether the real VM is currently running.
2. Trigger a start & wait if it's off.

## How It Works

Every provider integration shells out to that provider's own official CLI tool, using whatever authentication method the CLI is already configured with on the host (API signing keys, instance principal, etc.). This keeps the core binary free of provider-specific auth/signing code and makes adding new providers a matter of wrapping a CLI, not implementing a new SDK integration from scratch.

- **Origin-restricted public API:** Since the API is designed to be called directly from browser, each instance has its own CORS allow-list — no blanket "allow all" and no heavyweight auth system, appropriate for demo-scale projects.

- **Per-instance configuration:** Each managed VM has its own provider, credentials location, and allowed browser origins.

- **Deployment:** Run as a single static binary via a one-line installer (systemd), or as a container image you extend with your provider's CLI.

- **Small footprint.** Written in Rust, ships as a static binary — designed to comfortably run on the smallest/cheapest VM tier a provider offers.

## Project Reference

The project is a Cargo workspace split into focused crates:

| Crate                   | Responsibility                                                                                           |
| ----------------------- | -------------------------------------------------------------------------------------------------------- |
| `bin/activator`         | Entry point. Initializes logging and starts the app.                                                     |
| `crates/activator-core` | HTTP server (axum), instance registry, bootstrapping, CORS middleware.                                   |
| `crates/plugin-api`     | The `Provider` / `ProviderFactory` traits every provider implements. Zero provider-specific code.        |
| `crates/config`         | Loads and validates `config.yaml`.                                                                       |
| `crates/process-exec`   | Generic async subprocess runner (spawn, timeout, JSON output parsing) shared by all CLI-based providers. |
| `crates/mock-provider`  | A fake provider for local testing without touching real infrastructure.                                  |

Adding a new cloud provider means adding a new crate that implements `Provider` and `ProviderFactory` — nothing in `activator-core` needs to change. See [Adding a New Provider](#adding-a-new-provider).

## Quick Start

```bash
# 1. Build
cargo build --release

# 2. (Optional) Edit config files with your real instance details, given you have the specific provider CLI installed and authenticated

# 3. Run
./target/release/activator --config-dir ./example
```

Then check it live:

```bash
curl http://localhost:8080/v1/instances/website/status
```

> The example is configured with a mock provider

## Installation

### Direct Executable (systemd)

For a Linux VM you plan to run this on 24/7, the installer downloads a prebuilt static binary, installs it, creates a dedicated system user, and registers it as a systemd service that starts on boot.

```bash
curl -fsSL https://raw.githubusercontent.com/bhagatsaurabh/ignis-lazari/main/scripts/install.sh | sudo sh
```

This will:

1. Detect your architecture (`x86_64`, `aarch64`/`arm64`, or `armv7`) and download the matching release binary.
2. Install it to `/opt/igl-activator/igl-activator`.
3. Create a system user `igl-activator` with no login shell.
4. Write a starter `config/config.yaml` (empty instance list) if one doesn't already exist.
5. Install and enable (but not start) a systemd unit at `/etc/systemd/system/igl-activator.service`.

After installing, finish setup manually:

```bash
# 1. Edit the config
sudo nano /opt/igl-activator/config/config.yaml

# 2. Add any per-instance provider config files
sudo nano /opt/igl-activator/config/instances/website.yaml

# 3. Install your provider's CLI (e.g. the OCI CLI) and make sure it's
#    authenticated for the `igl-activator` system user

# 4. Start it
sudo systemctl start igl-activator

# Check status / logs
sudo systemctl status igl-activator
journalctl -u igl-activator -f
```

### Container Image

The published image is a minimal base — it contains the `activator` binary and nothing provider-specific. You extend it with whatever CLI your chosen provider needs.

```dockerfile
FROM ghcr.io/bhagatsaurabh/ignis-lazari:latest

USER root
# Example: install the OCI CLI
RUN apk add --no-cache python3 py3-pip \
    && pip3 install --no-cache-dir --break-system-packages oci-cli
USER igl-activator
```

Build and run:

```bash
docker build -t my-activator .

docker run -d \
  -p 8080:8080 \
  -v /path/to/config:/home/igl-activator/config:ro \
  --name igl-activator \
  my-activator
```

Mount your provider's credentials directory (e.g. `~/.oci` for OCI) and your `config/` directory (containing `config.yaml` and `instances/`) as read-only volumes. The container reads its config directory location from `ACTIVATOR_CONFIG_DIR`, already set to `/home/igl-activator/config` in the base image.

## Configuration

All configurations live in a single directory. Its location is resolved in this order:

1. `--config-dir <path>` command-line flag
2. `ACTIVATOR_CONFIG_DIR` environment variable
3. Defaults to `./config`

Expected directory structure:

```
config/
  config.yaml
  instances/
    instance-1.yaml
    instance-2.yaml
```

### `config.yaml`

```yaml
server:
  host: 0.0.0.0
  port: 8080

instances:
  - id: website
    provider: oracle-oci
    allowed_origins:
      - https://mywebsite.com
    provider_config:
      type: file
      path: instances/website.yaml
```

| Field                              | Description                                                                                       |
| ---------------------------------- | ------------------------------------------------------------------------------------------------- |
| `server.host` / `server.port`      | Address the HTTP API binds to.                                                                    |
| `instances[].id`                   | Unique identifier, used in API URLs (`/v1/instances/<id>/...`).                                   |
| `instances[].provider`             | Which provider handles this instance.                                                             |
| `instances[].allowed_origins`      | Browser origins permitted to call this instance's endpoints (see [CORS](#cors--origin-handling)). |
| `instances[].provider_config.path` | Path to the provider-specific config file, resolved relative to the config directory.             |

### Per-Instance Provider Config

Each provider defines its own config file shape. For OCI:

```yaml
instance_id: ocid...
```

Optional fields:

```yaml
instance_id: ocid1.instance.oc1.ap-mumbai-1.xxxxxxxxxxxxxxxxxxxxxxxxxxxxx
profile: DEFAULT # OCI CLI profile name, defaults to the CLI's own default
cli_binary: oci # override if the CLI isn't named/located as "oci" on PATH
```

No credentials are stored in this file — authentication is entirely delegated to however the provider's CLI is already configured on the host (see [Supported Providers](#supported-providers)).

## API Reference

[Spec](./docs/openapi.yaml)

## Supported Providers

### OCI (Oracle Cloud Infrastructure)

Install and authenticate the CLI on the host running Ignis Lazari (or in your extended container image) _before_ starting the service.

Recommended auth method for a 24/7 unattended service: **API signing key** (`oci setup config`), since it doesn't expire the way session-token auth does. Instance principal auth is also a good option if Ignis Lazari itself runs on an OCI instance with the appropriate dynamic group/policy — it requires no key management at all.

Verify the CLI works standalone before pointing Ignis Lazari at it:

```bash
oci compute instance get --instance-id <your-instance-ocid>
```
## Contributing

Contributions and provider implementations are always welcome!

### Adding a New Provider

1. Create a new crate, e.g. `crates/your-provider`.
2. Implement `plugin_api::Provider` (the `start` / `stop` / `status` trait) — for a CLI-based provider, use `process_exec::CliRunner` to shell out, following the pattern in `crates/oci-provider`.
3. Implement `plugin_api::ProviderFactory` to construct your provider from a config file path.
4. Define your provider's own config struct/YAML shape — it's private to your crate, `config.yaml` only ever passes it a file path.
5. Register your factory in `crates/activator-core/src/bootstrapper.rs`'s `build_factory_registry`.
6. Add your crate as a workspace member and as a dependency of `activator-core`.

No provider is required to use a CLI — `Provider` is just an async trait. If a provider ever has a mature Rust SDK you'd rather use directly, implement it the same way, just skip `process-exec` entirely.

### Building From Source

```bash
cargo build --release -p activator
```

To build a specific target (e.g. for cross-compiling to ARM):

```bash
rustup target add aarch64-unknown-linux-musl
cargo build --release -p activator --target aarch64-unknown-linux-musl
```

## License

[MIT](./LICENSE)
