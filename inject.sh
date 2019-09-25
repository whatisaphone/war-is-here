#!/usr/bin/env bash

set -euo pipefail

cargo build
vendor/Injector.exe -n darksiders1.exe -i ../target/debug/marionette.dll
