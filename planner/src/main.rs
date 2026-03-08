mod model;
mod simulator;
mod cli;

use anyhow::Result;
use clap::Parser;

use cli::{Cli, Command, InfoTopic, OutputFormat};
use model::GameData;
use simulator::{BuildPlanInput, simulate};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Simulate { data, plan, format } => {
            let game_data = GameData::load(&data)?;
            let build_plan = BuildPlanInput::load(&plan)?;
            let result = simulate(&game_data, &build_plan)?;

            match format {
                OutputFormat::Table => cli::print_build_table(&result, &game_data),
                OutputFormat::Json => cli::print_build_json(&result),
            }
        }

        Command::Info { data, topic } => {
            let game_data = GameData::load(&data)?;
            match topic {
                InfoTopic::Skills => cli::print_skills(&game_data),
                InfoTopic::Perks => cli::print_perks(&game_data),
                InfoTopic::Traits => cli::print_traits(&game_data),
                InfoTopic::Implants => cli::print_implants(&game_data),
                InfoTopic::Leveling => cli::print_leveling(&game_data),
            }
        }

        Command::Init { data, output } => {
            let game_data = GameData::load(&data)?;
            cli::generate_template(&game_data, &output)?;
        }
    }

    Ok(())
}
