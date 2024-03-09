use color_eyre::Result;
use color_eyre::Report as Error;
use clap::Parser;
use generator::parser::StrategyParser;

fn main() -> Result<(), Error> {
    let strategy_parser = StrategyParser::parse();

    strategy_parser.generate()?;

    Ok(())
}
