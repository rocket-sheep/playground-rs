## Overview

This repository contains Rust code samples from [The Rocket Sheep](https://rocketsheep.org/) blog.

Each article corresponds to a tag in the repository with the same date.

## Usage

Cargo is your friend, and should work as usual to build, test and run the code. (At the moment, the main executable does nothing.)

### Benchmarks

Benchmarks use the [Criterion](https://docs.rs/criterion/latest/criterion/) module. To run them, use something like:

```sh
cargo bench --bench benchmark "${benchmark_name}"
```

Tip: You can pass additional Criterion options after a double dash (`--`), for example:

```sh
cargo bench --bench benchmark "${benchmark_name}" -- --warm-up-time=0.5 --measurement-time=1 --sample-size=100 --quiet
```
