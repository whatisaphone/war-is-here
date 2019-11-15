#!/usr/bin/env bash

set -euo pipefail

nc localhost 53508 <<<shutdown || true
cargo build
vendor/Injector.exe -n darksiders1.exe -i ../target/debug/aether.dll
