# Billion CSV

`billion-csv-rs` is a Rust project designed to generate a CSV file with one billion records. Each record contains randomly generated data including name, age, birth date, height, weight, and gender.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Dependencies](#dependencies)
- [License](#license)

## Installation

To build and run this project, you need to have [Rust](https://www.rust-lang.org/) installed on your machine. Clone the repository and navigate to the project directory:

```sh
git clone https://github.com/gabrielalmir/billion-csv-rs.git
cd billion-csv
```

## Usage

To generate the CSV file, run the following command:

```sh
cargo run
```

This will create a file named `data.csv` in the project directory with one billion records.

## Dependencies

The project relies on several crates, as specified in `Cargo.toml`:

- `chrono` for date and time handling
- `csv` for CSV file writing
- `fake` for generating random data
- `rand` for random number generation
