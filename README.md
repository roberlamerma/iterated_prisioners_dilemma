# Iterated Prisoner's Dilemma in Rust

## About The Project

This project is a personal exploration driven by two main goals:

*   To delve deeper into the fascinating **Iterated Prisoner's Dilemma (IPD)** problem, Game Theory concepts, and Nash equilibria.
*   To learn the **Rust** programming language by applying it to this interesting problem domain.

This is primarily a learning exercise and a way to satisfy personal curiosity about game theory simulations.

## Built With

*   [Rust](https://www.rust-lang.org/) (Programming Language)
*   [rand](https://crates.io/crates/rand) (For strategies involving randomness)
*   [csv](https://crates.io/crates/csv) (For saving simulation results)
    *   *(Add any other significant libraries used, e.g., `clap` for command-line parsing)*

## Getting Started

Follow these steps to get a local copy up and running.

### Prerequisites

Ensure you have the Rust toolchain installed. If not, follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install). This typically involves installing `rustup`.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Follow the on-screen instructions
```

### Build

1.  Clone the repository:
    ```bash
    mkdir <your_project_directory>
    cd <your_project_directory>
    git clone https://github.com/roberlamerma/iterated_prisioners_dilemma.git
    ```
2.  Build the project:
    ```bash
    cargo build
    ```
3.  For optimized performance, especially for longer simulations, use the release profile:
    ```bash
    cargo build --release
    ```

## Usage

**Example using `cargo run`:**

```bash
# Replace <strategy1>, <strategy2>, and <num_iterations> with actual values supported by the program
# e.g., TitForTat, AlwaysCooperate, AlwaysDefect, Random, etc.
cargo run -- --verbose --iterations 100 --strategy1 random --strategy2 titfortat
```

**Example using the compiled release executable:**

```bash
# After building with `cargo build --release`
./target/debug/ipd-simulator --verbose --iterations 100 --strategy1 random --strategy2 titfortat
```

*(Note: Check the `--help` for all the possibilities.)*

Simulation results, including scores and potentially move histories, are saved to a CSV file in the specified output location:
`./target/debug/ipd-simulator --verbose --iterations 100 --strategy1 random --strategy2 titfortat --raw-scores-folder SOME_FOLDER`

## Roadmap

-   [x] Implement initial version that can perform the simulation with 2 given strategies. This version includes a minimal set of strategies.
-   [x] Save simulation results as CSV files.
-   [ ] Add pre-built executables to this README, in order to avoid building the project.
-   [ ] Make a better/smarter registration of the strategies.
-   [ ] Implement all remaining known strategies (e.g., Pavlov, Grim Trigger variants, etc.).
-   [ ] Implement an "all against all" tournament option.
-   [ ] Run the "all against all" simulations concurrently (multi-threaded).

## Acknowledgments

*   **AI Assistants:**
    *   Google AI Studio
    *   ChatGPT
    *   DeepSeek
    *   GitHub Copilot
*   **Inspiration:**
    *   This fascinating Veritasium video which sparked further interest: [What Game Theory Reveals About Life, The Universe, and Everything](https://www.youtube.com/watch?v=mScpHTIi-kM)
