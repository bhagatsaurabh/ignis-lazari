# Ignis Lazari

Ever wonder how much money you're burning on a VM that's idle 23 hours a day?

For Most hobby or personal projects traffic is sporadic at best, but if you're using instances behind them running 24/7, then its billed at full price whether anyone shows up or not. With Ignis Lazari your "heavy" VM stays _off_ by default, and wakes up on demand the moment someone actually visits your site.

> You only pay for compute when compute is doing something.

A lightweight, provider-agnostic activator: tiny service designed to run continuously on an inexpensive nano or micro instance, starting your real infrastructure only when it's needed. The trade-off is a slightly slower first visit whenever the VM has been stopped.

> **"Ignis Lazari"?** The spark/fire that raises "something" from the dead :)

## Why

Wwhether it's a self-hosted app, a game-server or a demo environment, personal projects typically see very little traffic. Yet unlike serverless platforms, most cloud providers can't truly scale VMs to zero. Your options are to leave the VM running around the clock or manually start and stop it, making public deployments like these unnecessarily expensive or inconvenient.

Ignis Lazari is a small always-on process that sits in front of your heavier VM(s). It runs on the cheapest possible instance (or as a tiny container) and exposes a public HTTP API that your hobby site's frontend can call to:

1. Check whether the real instance is currently running.
2. Trigger a start & wait if it's off.

## How It Works

Every provider integration shells out to that provider's own official CLI tool, using whatever authentication method the CLI is already configured with on the host (API signing keys, instance principal, etc.). This keeps the core binary free of provider-specific auth/signing code and makes adding new providers a matter of wrapping a CLI, not implementing a new SDK integration from scratch.

- **Origin-restricted public API:** The API is built for direct browser access, so each instance defines its own CORS allow-list instead of relying on a blanket "allow all" policy. It also avoids a heavyweight authentication system, striking a balance that's well suited to demo-scale deployments.

- **Per-instance configuration:** Each managed VM has its own provider, credentials location, and allowed browser origins.

- **Deployment:** Run as a single static binary via a one-line installer (systemd), or as a container image you extend with your provider's CLI.

- **Small footprint.** Built in Rust and distributed as a single static binary, it's lightweight enough to run comfortably on even the smallest/cheapest VM tier offered by most cloud providers.

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

Supporting a new cloud provider is as simple as adding a crate that implements `Provider` and `ProviderFactory`; `activator-core` itself remains unchanged. See [Adding a New Provider](#adding-a-new-provider).

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

The published image is intentionally minimal, containing only the `activator` binary and no provider-specific tooling. Simply extend it with the CLI required for your chosen cloud provider.

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

| Field                              | Description                                                                           |
| ---------------------------------- | ------------------------------------------------------------------------------------- |
| `server.host` / `server.port`      | Address the HTTP API binds to.                                                        |
| `instances[].id`                   | Unique identifier, used in API URLs (`/v1/instances/<id>/...`).                       |
| `instances[].provider`             | Which provider handles this instance.                                                 |
| `instances[].allowed_origins`      | Browser origins permitted to call this instance's endpoints.                          |
| `instances[].provider_config.path` | Path to the provider-specific config file, resolved relative to the config directory. |

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

No credentials are stored in this configuration file. Authentication is delegated to the cloud provider's CLI and uses the credentials already configured on the host. (see [Supported Providers](#supported-providers)).

## API Reference

[Spec](./docs/openapi.yaml)

## Supported Providers

### OCI (Oracle Cloud Infrastructure)

Install and authenticate the CLI on the host running Ignis Lazari (or in your extended container image) _before_ starting the service.

For an unattended 24/7 deployment, the recommended auth method is an **API signing key** (for e.g. `oci setup config` for Oracle Cloud), as it doesn't expire like session-token auth. If Ignis Lazari is running on an OCI instance, `Instance Principals` are also an excellent choice, requiring only the appropriate dynamic group and IAM policy, no key management needed.

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

Providers aren't required to use a CLI, `Provider` is simply an async trait. If a cloud provider offers a mature Rust SDK, you can implement it directly and bypass `process-exec` altogether.

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
