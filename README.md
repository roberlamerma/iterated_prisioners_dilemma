# Iterated Prisoner's Dilemma in Rust

## About The Project

This project is a personal exploration driven by two main goals:

*   To delve deeper into the fascinating **Iterated [Prisoner's Dilemma](https://en.wikipedia.org/wiki/Prisoner%27s_dilemma) (IPD)** problem and some related Game Theory concepts such as Nash equilibria.
*   To learn **Rust** "by doing". **Disclaimer:** I received a lot of help from the AI's mentioned below. 

## Built With

*   [Rust](https://www.rust-lang.org/) (Programming Language)
*   [rand](https://crates.io/crates/rand) (For strategies involving randomness)
*   [clap](https://crates.io/crates/clap) (For command line parsing)
*   [chrono](https://crates.io/crates/chrono) (For manipulating date/times)
*   **std** For everything else!

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
    mkdir <YOUR_PROJECT_DIR>
    cd <YOUR_PROJECT_DIR>
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

You basically need to provide the 2 competing strategies (`--strategy1` and `--strategy2`), and how many times they will "play" against each other (`--iterations`)

### Using `cargo run`

```bash
cargo run -- --verbose --iterations 100 --strategy1 Random --strategy2 TitForTat
```

### Using the compiled release executable

```bash
# After building with `cargo build --release`
./target/debug/ipd-simulator --verbose --iterations 100 --strategy1 Random --strategy2 TitForTat
```
### List all available strategies

As of Today, the easiest way to do this is to check the `create_strategy` function, in the `main.rs` file.

*(Note: I'm planning to add a command line option for this. It will also provide a short description for each one)*

```
Random
TitForTat
SuspiciousTitForTat
TitForTwoTats
Gradual
AlwaysDefect
AlwaysCooperate
Defect
```

### Others 
- Check the `--help` for all the possibilities.)
- Simulation results, including scores and move histories can be saved to a CSV file:
`./target/debug/ipd-simulator --verbose --iterations 100 --strategy1 Random --strategy2 TitForTat --raw-scores-folder <SOME_FOLDER>`

## Roadmap

-   [x] Implement initial version that can perform the simulation with 2 given strategies. This version includes a minimal set of strategies.
-   [x] Save simulation results as CSV files.
-   [ ] Add pre-built executables to this README, in order to avoid building the project.
-   [ ] Make a better/smarter registration of the strategies.
-   [ ] List all strategies, and provide a description
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
    *   This fascinating Veritasium video sparked further interest: [What Game Theory Reveals About Life, The Universe, and Everything](https://www.youtube.com/watch?v=mScpHTIi-kM)
