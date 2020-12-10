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
**Part 1**
```
time:   [4.9735 us 5.0120 us 5.0561 us]
change: [+0.1240% +1.2758% +2.4500%] (p = 0.03 < 0.05)
Change within noise threshold.
```

**Part 2**
```
time:   [987.66 us 1.0014 ms 1.0195 ms]
change: [+0.8968% +2.9449% +5.1670%] (p = 0.01 < 0.05)
Change within noise threshold.
```
</details>

### Day 2

<details>
**Part 1**
```
time:   [60.475 us 60.732 us 60.992 us]
change: [-1.4374% -1.0619% -0.6705%] (p = 0.00 < 0.05)
Change within noise threshold.
```

**Part 2**
```
time:   [80.131 us 80.889 us 81.735 us]
change: [-4.3173% -3.5318% -2.7307%] (p = 0.00 < 0.05)
Performance has improved.
```
</details>

### Day 3

<details>
**Part 1**
```
time:   [1.2403 us 1.2423 us 1.2450 us]
change: [-0.0955% +0.2115% +0.5370%] (p = 0.18 > 0.05)
No change in performance detected.
```

**Part 2**
```
time:   [5.6236 us 5.6277 us 5.6325 us]
change: [-0.0757% +0.1160% +0.2977%] (p = 0.23 > 0.05)
No change in performance detected.
```
</details>

### Day 4

<details>
**Part 1**
```
time:   [33.950 us 34.142 us 34.391 us]
change: [+2.9915% +3.8458% +4.9158%] (p = 0.00 < 0.05)
Performance has regressed.
```

**Part 2**
```
time:   [77.007 us 77.584 us 78.244 us]
change: [+0.1740% +0.9562% +1.7658%] (p = 0.02 < 0.05)
Change within noise threshold.
```
</details>
