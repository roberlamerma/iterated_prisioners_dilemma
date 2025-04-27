use iterated_prisoners_dilemma_lib::{Move, Strategy, calculate_payoffs};
use iterated_prisoners_dilemma_lib::strategies::{
    always_cooperate::AlwaysCooperateStrategy, 
    always_defect::AlwaysDefectStrategy,
    random::RandomStrategy, 
    tit_for_tat::TitForTatStrategy, 
    tit_for_two_tats::TitForTwoTatsStrategy,
    suspicious_tit_for_tat::SuspiciousTitForTatStrategy,
    gradual::GradualStrategy,
    tester::TesterStrategy
};
//use iterated_prisoners_dilemma_lib:: //strategies::{create_strategy_by_name, list_available_strategies};

use clap::Parser;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;
use std::time::Instant;
use chrono::Local;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "Simulates the Iterated Prisoner's Dilemma between two strategies.")]
struct Args {
    #[arg(long, help = "Number of iterations to run for the simulation")]
    iterations: u32,

    #[arg(long, help = "Name of the first strategy (e.g., 'Random' or 'Tit for Tat')")]
    strategy1: String,

    #[arg(long, help = "Name of the second strategy (e.g., 'Random' or 'Tit for Tat')")]
    strategy2: String,

    #[arg(long, help = "Folder to store the simulation results per iteration in CSV format. If not provided, no CSV is created")]
    raw_scores_folder: Option<String>,

    #[arg(long, help = "Verbose (console) simulation. Notice that this will increase the total sim time.")]
    verbose: bool
}

fn determine_winner(payoff1: i32, payoff2: i32, strategy1: &dyn Strategy, strategy2: &dyn Strategy) -> String {
    match payoff1.cmp(&payoff2) {
        std::cmp::Ordering::Greater => strategy1.to_string(),
        std::cmp::Ordering::Less    => strategy2.to_string(),
        std::cmp::Ordering::Equal   => "Tie".to_string(),
    }
}

fn create_strategy(name: &str) -> Result<Box<dyn Strategy>, String> {
    match name.to_lowercase().as_str() {
        "random" => Ok(Box::new(RandomStrategy)),
        "titfortat" => Ok(Box::new(TitForTatStrategy)),
        "tit for tat" => Ok(Box::new(TitForTatStrategy)),
        "tft" => Ok(Box::new(TitForTatStrategy)),
        "suspicioustitfortat" => Ok(Box::new(SuspiciousTitForTatStrategy)),
        "suspicious tit for tat" => Ok(Box::new(SuspiciousTitForTatStrategy)),
        "stft" => Ok(Box::new(SuspiciousTitForTatStrategy)),
        "titfortwotats" => Ok(Box::new(TitForTwoTatsStrategy)),
        "titfor2tats" => Ok(Box::new(TitForTwoTatsStrategy)),
        "tit for two tats" => Ok(Box::new(TitForTwoTatsStrategy)),
        "tit for 2 tats" => Ok(Box::new(TitForTwoTatsStrategy)),
        "tf2ts" => Ok(Box::new(TitForTwoTatsStrategy)),
        "gradual" => Ok(Box::new(GradualStrategy::new())),
        "always defect" => Ok(Box::new(AlwaysDefectStrategy)),
        "alwaysdefect" => Ok(Box::new(AlwaysDefectStrategy)),
        "alld" => Ok(Box::new(AlwaysDefectStrategy)),
        "always cooperate" => Ok(Box::new(AlwaysCooperateStrategy)),
        "alwayscooperate" => Ok(Box::new(AlwaysCooperateStrategy)),
        "allc" => Ok(Box::new(AlwaysCooperateStrategy)),
        "tester" => Ok(Box::new(TesterStrategy::new())),
        _ => Err(format!("Unknown strategy: {}", name)),
    }
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    if args.iterations == 0 {
        return Err("Iterations should be > 0".to_string());
    }

    let start = Instant::now();

    let mut strategy1 = create_strategy(&args.strategy1)?;
    let mut strategy2 = create_strategy(&args.strategy2)?;

    let mut history1: Vec<Move> = Vec::with_capacity(args.iterations as usize);
    let mut history2: Vec<Move> = Vec::with_capacity(args.iterations as usize);

    let mut total1 = 0;
    let mut total2 = 0;

    let mut csv_writer: Option<File> = match &args.raw_scores_folder {
        Some(folder) => {
            create_dir_all(folder)
                .map_err(|e| format!("Failed to create directory '{}': {}", folder, e))?;
    
            let datetime = Local::now().format("%Y%m%d-%H%M%S").to_string();
            let filename = format!(
                "{}_Iterated-Prisioners-Dilemma-Simulation_({} vs {}).csv",
                datetime,
                strategy1.to_string(), // Note: strategy names needed here
                strategy2.to_string()
            );
            let path = Path::new(folder).join(filename);
    
            let mut file = File::create(&path)
                .map_err(|e| format!("Failed to create file '{}': {}", path.display(), e))?;
    
            writeln!(file, "Iteration,Strategy 1 Name,Strategy 2 Name,Move 1,Move 2,Payoff 1,Payoff 2")
                .map_err(|e| format!("Failed to write header to CSV: {}", e))?;
    
            Some(file) // Return the file if all steps succeeded
        }
        None => None, // No folder provided, so no file
    };

    for iteration in 1..=args.iterations {
        let move1 = strategy1.next_move(&history1, &history2);
        let move2 = strategy2.next_move(&history2, &history1);

        // Calculate payoffs for this round
        let (payoff1, payoff2) = calculate_payoffs(move1, move2);
        total1 += payoff1;
        total2 += payoff2;

        // Determine the winner
        let winner = determine_winner(payoff1, payoff2, strategy1.as_ref(), &*strategy2);

        if args.verbose {
            println!(
                "{} - {:?} ; Payoffs: {:?} ; Winner: {}",
                iteration,
                (move1, move2),
                (payoff1, payoff2),
                winner
            );
        }

        // Write to CSV if a folder was provided
        if let Some(ref mut writer) = csv_writer {
            writeln!(
                writer,
                "{},{},{:?},{:?},{:?},{},{}",
                iteration,
                strategy1.to_string(),
                strategy2.to_string(),
                move1,
                move2,
                payoff1,
                payoff2
            ).unwrap();
        }

        history1.push(move1);
        history2.push(move2);
    }

    let duration = start.elapsed();
        println!("\nSimulation time: {:.2?}", duration);
        println!("Iterations: {}", args.iterations);
        println!("Strategy 1: {}", strategy1.to_string());
        println!("Strategy 2: {}", strategy2.to_string());
        println!("Strategy 1 cumulative score: {}", total1);
        println!("Strategy 2 cumulative score: {}", total2);

    Ok(())
}
