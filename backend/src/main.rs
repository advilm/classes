use clap::Parser;
use classes::setup_database::*;

/// This doc string acts as a help message when the user runs '--help'
#[derive(Parser)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser)]
enum SubCommand {
    FetchJson(FetchJson),
    JsonToPostgres(JsonToPostgres),
}

/// Fetch JSON from UCR API
#[derive(Parser)]
struct FetchJson {}

/// Inserts data from JSON to postgres
#[derive(Parser)]
struct JsonToPostgres {
    #[clap(short, long)]
    file: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string("config.toml").expect("Failed to read config.toml");
    let config: classes::models::Config =
        toml::from_str(&contents).expect("Failed to parse config.toml contents");

    let opts = Opts::parse();

    match opts.subcmd {
        SubCommand::FetchJson(_f) => {
            fetch_json().await?;
        }
        SubCommand::JsonToPostgres(f) => {
            json_to_postgres(f.file, &config).await?;
        }
    }

    Ok(())
}
