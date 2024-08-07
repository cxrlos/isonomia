#!/bin/bash

# format_project.sh

# Set the color variables
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo "Starting project formatting..."

# Run cargo fmt to format all files
cargo fmt

# Check if formatting was successful
if cargo fmt -- --check; then
    echo -e "${GREEN}Formatting successful! All files are correctly formatted.${NC}"
    exit 0
else
    echo -e "${RED}Formatting check failed. Please review the changes made by cargo fmt.${NC}"
    exit 1
fi
