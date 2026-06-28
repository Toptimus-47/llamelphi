#!/bin/bash
# MAGI 2026: Linux Build & .deb Packaging Script
# Optimized for Ryzen 5800U / MX450

set -e

VERSION="1.0.0"
PACKAGE_NAME="magi-llamelphi"
ARCH="amd64"
STAGE_DIR="pkg_stage/${PACKAGE_NAME}_${VERSION}_${ARCH}"

echo ">>> [1/4] Building Rust Backend (Release)..."
cd magi_core
cargo build --release
cd ..

echo ">>> [2/4] Building Flutter Frontend (Linux)..."
cd magi_gui
# flutter build linux --release
# Note: Ensure flutter is installed on the laptop.
cd ..

echo ">>> [3/4] Structuring Debian Package..."
mkdir -p "${STAGE_DIR}/DEBIAN"
mkdir -p "${STAGE_DIR}/usr/bin"
mkdir -p "${STAGE_DIR}/usr/share/magi/prompts"
mkdir -p "${STAGE_DIR}/usr/share/applications"

# Create Control file
cat <<EOT > "${STAGE_DIR}/DEBIAN/control"
Package: ${PACKAGE_NAME}
Version: ${VERSION}
Section: utils
Priority: optional
Architecture: ${ARCH}
Depends: libc6, libssl-dev, libgtk-3-0
Maintainer: Toptimus-47 <toptimus47@github.com>
Description: MAGI Multi-Agent Orchestration System
 High-fidelity research platform with adversarial consensus.
 Optimized for Ryzen 5800U and MX450.
EOT

# Copy Binaries
cp magi_core/target/release/magi_server "${STAGE_DIR}/usr/bin/magi-server"
cp -r prompts/* "${STAGE_DIR}/usr/share/magi/prompts/"

# Create Desktop Entry
cat <<EOT > "${STAGE_DIR}/usr/share/applications/magi.desktop"
[Desktop Entry]
Name=MAGI llamelphi
Exec=/usr/bin/magi-server
Icon=/usr/share/magi/icon.png
Type=Application
Categories=Development;Science;
EOT

echo ">>> [4/4] Finalizing .deb Package..."
dpkg-deb --build "pkg_stage/${PACKAGE_NAME}_${VERSION}_${ARCH}"
echo ">>> SUCCESS: .deb package created in pkg_stage/"
