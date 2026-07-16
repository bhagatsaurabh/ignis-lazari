#!/bin/sh
set -eu

REPO="bhagatsaurabh/ignis-lazari"
INSTALL_DIR="/opt/igl-activator"
BIN_PATH="$INSTALL_DIR/igl-activator"
SERVICE_PATH="/etc/systemd/system/igl-activator.service"

if [ "$(id -u)" -ne 0 ]; then
  echo "This installer needs root (systemd unit + /opt install). Try: curl -fsSL <url> | sudo sh" >&2
  exit 1
fi

ARCH=$(uname -m)
case "$ARCH" in
  x86_64) TARGET="x86_64-unknown-linux-gnu" ;;
  aarch64|arm64) TARGET="aarch64-unknown-linux-gnu" ;;
  *)
    echo "Unsupported architecture: $ARCH" >&2
    exit 1
    ;;
esac

if ! command -v systemctl >/dev/null 2>&1; then
  echo "systemd not found. This installer currently only supports systemd-based distros." >&2
  exit 1
fi

echo "==> Fetching latest release for $TARGET"
LATEST_URL=$(curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" \
  | grep "browser_download_url.*$TARGET" \
  | cut -d '"' -f 4)

if [ -z "$LATEST_URL" ]; then
  echo "Could not find a release asset for $TARGET" >&2
  exit 1
fi

TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT

echo "==> Downloading $LATEST_URL"
curl -fsSL "$LATEST_URL" -o "$TMP_DIR/igl-activator.tar.gz"
tar -xzf "$TMP_DIR/igl-activator.tar.gz" -C "$TMP_DIR"

echo "==> Installing to $INSTALL_DIR"
id -u igl-activator >/dev/null 2>&1 || useradd --system --home "$INSTALL_DIR" --shell /usr/sbin/nologin igl-activator

mkdir -p "$INSTALL_DIR/config/instances" "$INSTALL_DIR/.oci"
install -m 755 "$TMP_DIR/igl-activator" "$BIN_PATH"

if [ ! -f "$INSTALL_DIR/config/config.yaml" ]; then
  cat > "$INSTALL_DIR/config/config.yaml" <<'EOF'
server:
  host: 0.0.0.0
  port: 8080

instances: []
EOF
  echo "==> Wrote a starter config at $INSTALL_DIR/config/config.yaml — edit this before starting the service."
fi

chown -R igl-activator:igl-activator "$INSTALL_DIR"

echo "==> Installing systemd unit"
cat > "$SERVICE_PATH" <<EOF
[Unit]
Description=Ignis Lazari
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
User=igl-activator
Group=igl-activator
WorkingDirectory=$INSTALL_DIR
Environment=ACTIVATOR_CONFIG_DIR=$INSTALL_DIR/config
Environment=OCI_CONFIG_FILE=$INSTALL_DIR/.oci/config
Environment=RUST_LOG=info
ExecStart=$BIN_PATH
Restart=on-failure
RestartSec=5

NoNewPrivileges=true
ProtectSystem=strict
ReadWritePaths=$INSTALL_DIR
ProtectHome=true
PrivateTmp=true

[Install]
WantedBy=multi-user.target
EOF

systemctl daemon-reload
systemctl enable igl-activator

echo ""
echo "==> Installed. Before starting:"
echo "    1. Edit $INSTALL_DIR/config/config.yaml and add your instance(s)"
echo "    2. Add per-instance provider configs under $INSTALL_DIR/config/instances/"
echo "    3. Configure and authenticate provider CLIs (for e.g. Azure CLI if using Azure instances in previous step)"
echo "    Then run: sudo systemctl start activator"
echo "    Check status with: sudo systemctl status igl-activator"
echo "    Follow logs with: journalctl -u igl-activator -f"
