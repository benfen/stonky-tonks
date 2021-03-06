mod calculate_balances;
mod fetch_prices;

use clap::Clap;
use std::fmt;

#[derive(Debug, Clone)]
struct CliError {
    reason: String,
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.reason)
    }
}

#[derive(Clap, Debug)]
#[clap(version = "0.1.0", author = "Ben F. <ben@fenwick.info>")]
struct Opts {
    #[clap(subcommand)]
    task: SubCommand,
}

#[derive(Clap, Debug)]
enum SubCommand {
    CalculateBalances(CalculateBalances),
    FetchPrices(FetchPrices),
}

#[derive(Clap, Debug)]
struct CalculateBalances;

#[derive(Clap, Debug)]
struct FetchPrices;

#[tokio::main]
async fn main() -> Result<(), CliError> {
    let opts: Opts = Opts::parse();

    match opts.task {
        SubCommand::CalculateBalances(_) => match calculate_balances::calculate_balances() {
            Ok(()) => Ok(()),
            Err(err) => Err(CliError { reason: err }),
        },
        SubCommand::FetchPrices(_) => match fetch_prices::fetch_prices().await {
            Ok(_) => Ok(()),
            Err(e) => Err(CliError {
                reason: e.to_string(),
            }),
        },
    }
}
