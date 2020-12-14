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

So far, these are the timings of solving the puzzles on my Ryzen 3700X CPU:

### Day 1

<details>
<summary>113.9 ns + 181.7 us</summary>

| Part | Min       | Avg       | Max       |
| ---- | --------- | --------- | --------- |
| 1    | 113.37 ns | 113.91 ns | 114.62 ns |
| 2    | 181.18 us | 181.72 us | 182.32 us |

</details>

### Day 2

<details>
<summary>114.5 ns + 178.9 us</summary>

| Part | Min       | Avg       | Max       |
| ---- | --------- | --------- | --------- |
| 1    | 114.01 ns | 114.46 ns | 114.94 ns |
| 2    | 178.42 us | 178.91 us | 179.51 us |

</details>

### Day 3

<details>
<summary>111.9 ns + 182.9 us</summary>

| Part | Min       | Avg       | Max       |
| ---- | --------- | --------- | --------- |
| 1    | 111.68 ns | 111.93 ns | 112.24 ns |
| 2    | 182.51 us | 182.91 us | 183.34 us |

</details>

### Day 4

<details>
<summary>113.8 ns + 181.4 us</summary>

| Part | Min       | Avg       | Max       |
| ---- | --------- | --------- | --------- |
| 1    | 113.51 ns | 113.76 ns | 114.07 ns |
| 2    | 180.90 us | 181.38 us | 182.24 us |

</details>

### Day 5

<details>
<summary>114.8 ns + 181.0 us</summary>

| Part | Min       | Avg       | Max       |
| ---- | --------- | --------- | --------- |
| 1    | 114.51 ns | 114.82 ns | 115.13 ns |
| 2    | 180.62 us | 180.99 us | 181.44 us |

</details>

### Day 6

<details>
<summary>113.1 ns + 180.4 us </summary>

| Part | Min       | Avg       | Max       |
| ---- | --------- | --------- | --------- |
| 1    | 112.82 ns | 113.08 ns | 113.39 ns |
| 2    | 179.83 us | 180.44 us | 181.10 us |

</details>

### Day 7

<details>
<summary>1.9ms + 4.4us</summary>

| Part | Min       | Avg       | Max       |
| ---- | --------- | --------- | --------- |
| 1    | 1.8931 ms | 1.8978 ms | 1.9033 ms |
| 2    | 4.3457 us | 4.3570 us | 4.3707 us |

Note: The implementation of Part 1 was not optimized meaningfully. It could be optimized by either passing along a "contains cache" or recursing up from children.

</details>

### Day 8

<details>
<summary>7.4us + 1.1ms</summary>

| Part | Min       | Avg       | Max       |
| ---- | --------- | --------- | --------- |
| 1    | 7.4319 us | 7.4430 us | 7.4536 us |
| 2    | 1.0777 ms | 1.0795 ms | 1.0813 ms |

</details>
