# List available commands
list:
  just --list

# Run tests (sample data in AoC)
test:
  cargo test

# Run the 2024 AoC
run:
  cargo run

# Compile for release, time initial execution execute, then run a second timed execution
time:
  cargo build --release
  time ./target/release/aoc2024
  time ./target/release/aoc2024

# Remove all built artefacts
clean:
  rm -rf target
