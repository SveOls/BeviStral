#!/usr/bin/env sh
# Install system build dependencies for Bevy 0.19 on Debian/Ubuntu.
# Bevy's default `wayland` feature runs `pkg-config` for wayland-client at
# build time, so the Wayland/X11 dev libraries must be present to compile.
# Idempotent: safe to run in every fresh sandbox/session.
set -eu

sudo apt-get update -qq

sudo apt-get install -y --no-install-recommends \
    pkg-config \
    libwayland-dev \
    libxkbcommon-dev \
    libx11-dev \
    libxcursor-dev \
    libxrandr-dev \
    libxi-dev \
    libxinerama-dev \
    libudev-dev \
    libasound2-dev

echo "System dependencies installed."
