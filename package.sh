#!/bin/bash
set -e

echo "Building release binary..."
cargo build --release

echo "Creating package structure..."
rm -rf build/debian
mkdir -p build/debian/usr/bin
mkdir -p build/debian/usr/share/applications
mkdir -p build/debian/usr/share/icons/hicolor/512x512/apps
mkdir -p build/debian/DEBIAN

echo "Copying files..."
cp target/release/rust-finger build/debian/usr/bin/
# Ensure executable permissions
chmod +x build/debian/usr/bin/rust-finger

cp assets/rust-finger.desktop build/debian/usr/share/applications/
cp assets/rust-finger.png build/debian/usr/share/icons/hicolor/512x512/apps/

echo "Creating control file..."
cat > build/debian/DEBIAN/control << EOL
Package: rust-finger
Version: 0.1.0
Section: utils
Priority: optional
Architecture: amd64
Maintainer: Kamidesu <kamidesu@example.com>
Description: Rust Finger Monitor
 A keyboard and mouse dominance visualization tool.
EOL

echo "Building .deb package..."
dpkg-deb --build build/debian rust-finger_0.1.0_amd64.deb

echo "Done! Package saved as rust-finger_0.1.0_amd64.deb"
