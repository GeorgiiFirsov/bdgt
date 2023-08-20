#!/bin/sh

echo "[i] Running configuration"
echo "[i] Installing dependencies..."

# Install necessary dependencies
apt-get install -y   \
    libgpg-error-dev \
    libgpgme-dev     

echo "[+] Dependencies installed successfully"
echo "[i] Configuring system..."

# Configure gpg (git looks for gpg at /usr/local/bin/gpg)
ln $(which gpg) /usr/local/bin/gpg

echo "[+] System configured successfully"
