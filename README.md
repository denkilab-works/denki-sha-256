# Denki SHA-256

## Preprocessing
Initial operations that we need to do:
- Padding -> Goal: ensuring that the padded message is a 512-bit multiple
- Parsing -> The message and its padding must be parsed into N 512-bit blocks, each containing 16 32-bit words
> This step is simply achieved with slice operations
- Initial hash value -> 8 constant 32-bit words derived from the square roots of the first 8 primes

## Hash computation
For each 512-bit block:
1. Message schedule -> Expands the 16 block words into 64 words using rotation/shift mixing functions (σ₀, σ₁)
2. Working variables init -> Copies the current 8-word hash state into variables a–h
3. Compression -> 64 rounds, each combining:
   - Choose (Ch) -> Selects bits from f or g based on e
   - Majority (Maj) -> Returns the majority bit among a, b, c
   - Rotation mixing (Σ₀, Σ₁) -> Bitwise rotation applied to a and e
   - Round constant (K) -> One of 64 constants derived from the cube roots of the first 64 primes
   - Scheduled word (W) -> The corresponding word from the message schedule
4. Hash update -> Adds the compressed working variables back into the hash state (mod 2³²)

## Output
- A 256-bit digest (the concatenation of the final 8 hash words)

## Building / Developing

```sh
cargo run # compiles the code and then runs it
cargo test # Runs all the tests
cargo check # Checks if the code compiles, but does not compile it (really fast)
cargo add --dev crate_name # Adds a dev dependency
cargo build --release # Builds the release executable (and updates the Cargo.lock if not in sync)

cargo fmt # Manually formats the code
```
