#!/bin/bash

# format_project.sh

# Set the color variables
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo "Starting project formatting and linting..."

# Run cargo fmt to format all files
cargo fmt

# Check if formatting was successful
if ! cargo fmt -- --check; then
    echo -e "${RED}Formatting check failed. Please review the changes made by cargo fmt.${NC}"
    exit 1
fi

echo -e "${GREEN}Formatting successful! All files are correctly formatted.${NC}"

# Run Clippy
echo "Running Clippy..."
if ! cargo clippy -- -D warnings; then
    echo -e "${RED}Clippy check failed. Please address the warnings above.${NC}"
    exit 1
fi

echo -e "${GREEN}Clippy check passed successfully!${NC}"
echo -e "${GREEN}All checks passed. Your code is well-formatted and free of Clippy warnings.${NC}"
exit 0
