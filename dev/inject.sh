#!/usr/bin/env bash

set -euo pipefail

nc localhost 53508 <<<shutdown || true
cargo run
