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
<summary>15.8 us + 40.1 us</summary>

| Part | Min       | Avg       | Max       |
| ---- | --------- | --------- | --------- |
| 1    | 32.554 us | 32.666 us | 32.777 us |
| 2    | 75.914 us | 76.251 us | 76.728 us |

#### fxhash used

| Part | Min       | Avg       | Max       | Note                    |
| ---- | --------- | --------- | --------- | ----------------------- |
| 1    | 15.760 us | 15.806 us | 15.878 us | Using fxhash (a6b434af) |
| 2    | 39.888 us | 40.126 us | 40.463 us | Using fxhash (a6b434af) |

Note: About times 2 faster

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
<summary>112.8 ns + 140.4 us </summary>

| Part | Min       | Avg       | Max       |
| ---- | --------- | --------- | --------- |
| 1    | 112.59 ns | 113.15 ns | 113.83 ns |
| 2    | 184.55 us | 185.67 us | 187.03 us |

#### fxhash used

| Part | Min       | Avg       | Max       | Note                    |
| ---- | --------- | --------- | --------- | ----------------------- |
| 1    | 112.64 ns | 112.84 ns | 113.10 ns | Using fxhash (a6b434af) |
| 2    | 139.82 us | 140.35 us | 140.98 us | Using fxhash (a6b434af) |

Note: About times 1.2 times faster in part 2. No change in part 1.

</details>

### Day 7

<details>
<summary>109.6 us + 3.3 us</summary>

| Part | Min       | Avg       | Max       |
| ---- | --------- | --------- | --------- |
| 1    | 1.8931 ms | 1.8978 ms | 1.9033 ms |
| 2    | 4.3457 us | 4.3570 us | 4.3707 us |

Note: The implementation of Part 1 was not optimized meaningfully. It could be optimized by either passing along a "contains cache" or recursing up from children.

#### fxhash used

| Part | Min       | Avg       | Max       | Note                    |
| ---- | --------- | --------- | --------- | ----------------------- |
| 1    | 820.38 us | 822.48 us | 825.24 us | Using fxhash (a6b434af) |
| 2    | 3.2765 us | 3.2784 us | 3.2803 us | Using fxhash (a6b434af) |

Note: About 2 times faster in part 1 and 1.5 times in part 2

#### Part 1 parallelized

| Part | Min       | Avg       | Max       | Note                             |
| ---- | --------- | --------- | --------- | -------------------------------- |
| 1    | 108.17 us | 109.59 us | 111.11 us | Palallized with rayon (e3b8945e) |

Note: More than 5 times faster

</details>

### Day 8

<details>
<summary>3.2 us + 84.9 us</summary>

| Part | Min       | Avg       | Max       | Note                        |
| ---- | --------- | --------- | --------- | --------------------------- |
| 1    | 7.4319 us | 7.4430 us | 7.4536 us |                             |
| 2    | 1.0777 ms | 1.0795 ms | 1.0813 ms | Initial solution (4afbe335) |

#### Part 2 parallelized

| Part | Min       | Avg       | Max       | Note                             |
| ---- | --------- | --------- | --------- | -------------------------------- |
| 2    | 169.12 us | 172.90 us | 177.49 us | Palallized with rayon (1d7c887c) |

Note: More than 5 times faster

#### fxhash used

| Part | Min       | Avg       | Max       | Note                    |
| ---- | --------- | --------- | --------- | ----------------------- |
| 1    | 3.1714 us | 3.1867 us | 3.2112 us | Using fxhash (b224c911) |
| 2    | 83.605 us | 84.886 us | 86.271 us | Using fxhash (b224c911) |

Note: More than 2 times faster

</details>

### Day 9

<details>
<summary>47.1 us + 73.9 us</summary>

| Part | Min       | Avg       | Max       |
| ---- | --------- | --------- | --------- |
| 1    | 46.882 us | 47.091 us | 47.381 us |
| 2    | 73.148 us | 73.863 us | 74.691 us |

</details>
