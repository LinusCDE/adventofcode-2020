# AdventOfCode 2020

This year, I may do a couple of tasks from [AoC](https://adventofcode.com/2020/) in Rust.

I debated doing Go, but currently doing a REST api for work, I think Go will make me not appreciate Rust anymore. I want to stay ignorant for a few months more to keep motivation up.

This project is using more or less a Framework called [cargo-aoc](https://lib.rs/crates/cargo-aoc) which is an extension to Rust package manager "cargo".

## Installation

Install Rust. It's recommended to use [rustup](https://rustup.rs/) for that task. It can manage the Rust-Installation(s) fairly easy using the "rustup" command.

After installing rustup install Rust with it. Whether you choose stable or nightly probably doesn't matter. I use nightly out of habit for other projects. Command: `rustup install nightly`

After this you should have the command "cargo" and can install cargo-aoc: `cargo install cargo-aoc`. This will add the subcommand "aoc" to cargo.

Next, clone this repo and cd into it.

Now you can run any days solutions using `cargo aoc -d <DAY> (-p <1/2>)`. Benchmarks can be run using `cargo aoc -d <DAY> bench`.

## Benches

So far, this are the timings of solving the puzzles on my Ryzen 3700X CPU:

### Day 1

<details>
<summary>5.0 us + 1.0 ms</summary>

| Part | Min       | Avg       | Max       |
| ---- | --------- | --------- | --------- |
| 1    | 4.9735 us | 5.0120 us | 5.0561 us |
| 2    | 987.66 us | 1.0014 ms | 1.0195 ms |

</details>

### Day 2

<details>
<summary>60.7 us + 80.9 us</summary>

| Part | Min       | Avg       | Max       |
| ---- | --------- | --------- | --------- |
| 1    | 60.475 us | 60.732 us | 60.992 us |
| 2    | 80.131 us | 80.889 us | 81.735 us |

</details>

### Day 3

<details>
<summary>1.2 us + 5.6 us</summary>

| Part | Min       | Avg       | Max       |
| ---- | --------- | --------- | --------- |
| 1    | 1.2403 us | 1.2423 us | 1.2450 us |
| 2    | 5.6236 us | 5.6277 us | 5.6325 us |

</details>

### Day 4

<details>
<summary>34.1 us + 77.6 us</summary>

| Part | Min       | Avg       | Max       |
| ---- | --------- | --------- | --------- |
| 1    | 33.950 us | 34.142 us | 34.391 us |
| 2    | 77.007 us | 77.584 us | 78.244 us |

</details>

### Day 5

<details>
<summary>342.5 ns + 13.2 us us</summary>

| Part | Min       | Avg       | Max       |
| ---- | --------- | --------- | --------- |
| 1    | 340.58 ns | 342.48 ns | 345.07 ns |
| 2    | 13.152 us | 13.177 us | 13.208 us |

</details>

### Day 6

<details>
<summary>113.1 ns + 180.4 us </summary>

| Part | Min       | Avg       | Max       |
| ---- | --------- | --------- | --------- |
| 1    | 112.82 ns | 113.08 ns | 113.39 ns |
| 2    | 179.83 us | 180.44 us | 181.10 us |

</details>
