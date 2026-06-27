# List available recipes
default:
    @just --list

# Build the compiler
build:
    nix develop -c cargo build

# Run the test suite
test:
    nix develop -c cargo test

# Compile and run a Vampa program, reporting its result
run file:
    nix develop -c cargo run --quiet --bin vamc {{ file }}
    nix develop -c sh -c 'lli {{ without_extension(file) }}.o; echo "Program exited with $?"'

# Format every file in the tree
fmt:
    nix fmt

# Build the package and run all flake checks
check:
    nix flake check
