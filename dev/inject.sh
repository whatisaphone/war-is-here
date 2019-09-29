#!/usr/bin/env bash

set -euo pipefail

nc -u localhost 12345 -w 1 <<<shutdown
cargo build
vendor/Injector.exe -n darksiders1.exe -i ../target/debug/marionette.dll
