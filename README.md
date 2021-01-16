![Fuzz status](https://github.com/LimeEng/magpie-fuzz/workflows/Fuzz%20magpie%20v0.8.0/badge.svg)

# Fuzzing magpie

[Magpie](https://github.com/LimeEng/magpie) is an Othello library. This repository contains a number of fuzzing targets which can aid in bug hunting.

To run these targets, [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz) is needed.

## Github workflow

Included is a github workflow ([fuzz.yml](/.github/workflows/fuzz.yml)) which can be used to run these benchmarks for a limited time. Note that each job cannot run for longer than [6 hours](https://docs.github.com/en/free-pro-team@latest/actions/reference/usage-limits-billing-and-administration). Each job in the workflow is set to run for about 5 hours to avoid this limit.

## Targets

### Legal moves should be playable

```
cargo fuzz run legal_moves_should_place
```

Given a specific Othello board, generate all legal moves for black and check that each and every one can be placed on the board without errors.

### Illegal moves should not be playable

```
cargo fuzz run illegal_moves_should_not_place
```

Given a specific Othello board, generate all illegal moves for black and check that each and every one cannot be placed on the board without errors.
