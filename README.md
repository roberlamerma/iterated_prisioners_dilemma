# Iterated Prisoner's Dilemma in Rust

## 1.- About The Project

This project is a personal exploration driven by two main goals:

*   To delve deeper into the fascinating **Iterated [Prisoner's Dilemma](https://en.wikipedia.org/wiki/Prisoner%27s_dilemma) (IPD)** problem and some related Game Theory concepts such as Nash equilibria.
*   To learn **Rust** "by doing". **Disclaimer:** I received a lot of help from the AI's mentioned further below. 

## 2.- Built With

*   [Rust](https://www.rust-lang.org/) (Programming Language)
*   [rand](https://crates.io/crates/rand) (library used on strategies involving randomness)
*   [clap](https://crates.io/crates/clap) (library for command line parsing)
*   [chrono](https://crates.io/crates/chrono) (library for or manipulating date/times)
*   **std** Rust's superb standard library, used for everything else!

## 3.- Getting Started
* If you just want to play with the simulator, download the [latest version](https://github.com/roberlamerma/iterated_prisioners_dilemma/releases/tag/v0.3.0), unzip it and jump to the [3.2.- Usage](https://github.com/roberlamerma/iterated_prisioners_dilemma?tab=readme-ov-file#32--usage) section.
* On the other hand, if you want to build the project yourself, keep reading...

### 3.1.- Build it
#### Prerequisites

Ensure you have the Rust toolchain installed. If not, follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install). This typically involves installing `rustup`.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Follow the on-screen instructions
```

#### Build

1.  Clone the repository:
    ```bash
    git clone https://github.com/roberlamerma/iterated_prisioners_dilemma.git
    cd iterated_prisioners_dilemma
    ```
2.  Build the project:
    ```bash
    cargo build
    ```
3.  For optimized performance, especially for longer simulations, use the release profile:
    ```bash
    cargo build --release
    ```

### 3.2.- Usage

You basically need to provide the program with 2 competing strategies (`--strategy1` and `--strategy2`), and how many times they will "play" against each other (`--iterations`)

#### Using the compiled release executable

```bash
./ipd-simulator --verbose --iterations 100 --strategy1 Random --strategy2 TitForTat
```

#### List all available strategies

```bash
./ipd-simulator --list-strategies
```

#### Misc 
- Check the `--help` for all the possibilities.
- Simulation results, including scores and move histories can be saved to a CSV file:
`./ipd-simulator --verbose --iterations 100 --strategy1 Random --strategy2 TitForTat --raw-scores-folder <SOME_FOLDER>`

## 4.- Roadmap

-   [x] Implement initial version that can perform the simulation with 2 given strategies. This version includes a minimal set of strategies.
-   [x] Save simulation results as CSV files.
-   [X] Add pre-built executables to this README (and spare some users from having to build the project).
-   [X] Make a better/smarter registration of the strategies.
-   [X] List all strategies with a command line option (and also provide a description for each strategy)
-   [ ] Implement all remaining known strategies (e.g., Pavlov, Grim Trigger variants, etc.).
-   [ ] Implement an "all against all" tournament option.
-   [ ] Run the "all against all" simulations concurrently (multi-threaded).

## 5.- Acknowledgments

*   **AI Assistants** (Google AI Studio, ChatGPT, DeepSeek, GitHub Copilot)
*   **Inspiration:**
    *   This fascinating Veritasium video re-sparked my interest in a somehow dormant topic (Game Theory): [What Game Theory Reveals About Life, The Universe, and Everything](https://www.youtube.com/watch?v=mScpHTIi-kM)
