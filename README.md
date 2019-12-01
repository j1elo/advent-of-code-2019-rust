# Advent of Code 2019

Problems from [Advent of Code 2019](https://adventofcode.com/2019), written in [Rust](https://www.rust-lang.org/).



## Day preparation

To prepare a sub-project for each new day, run:

```sh
./new-day.sh <DayNumber>
```

For example, running `./new-day.sh 2` initializes all files in a directory named `day-02`.

Now add `"day-NN"` (with quotes) to the `members` array in [Cargo.toml](Cargo.toml),
copy the problem input from the web into `day-NN/input.txt`,
and (optionally) copy the problem description into `day-NN/README.md`.



## Run

`rustc` and `cargo` must be installed; if they are not, check the language's [Install Rust](https://www.rust-lang.org/tools/install) page.

To build and run the code for one of the days, run:

```sh
./run-day.sh <DayNumber>
```
