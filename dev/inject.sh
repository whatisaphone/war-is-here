#!/usr/bin/env bash

set -euo pipefail

nc localhost 43508 <<<shutdown || true
cargo run
