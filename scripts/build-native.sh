#!/bin/sh

# Get absolute directory of the script
AB_PATH=$(readlink -f "$0")
AB_DIR=$(dirname "$AB_PATH")

# CD to ic-agent-backend
cd "$AB_DIR/../ic-agent-backend"

# Run rust build command
# cargo rustc -- --crate-type=cdylib