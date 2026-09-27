#!/usr/bin/env bash
set -e

export HTTP_PROXY=http://127.0.0.1:10808
export HTTPS_PROXY=http://127.0.0.1:10808

cargo run --release
