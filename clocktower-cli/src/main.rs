use clap::{Parser, Subcommand};
use clocktower_core::{create_worktime_entry, establish_connection, set_workday_finished};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    StartNow,
    EndNow {
        #[clap(short = 'b', long)]
        hadbreak: bool,
    },
    UpdateStartTime {
        date: Option<String>,
        start_time: String,
    },
}

fn parse_date<T: ToString>(date_string: T) -> anyhow::Result<chrono::NaiveDateTime> {
    chrono_english::parse_date_string(
        &date_string.to_string(),
        chrono::Local::now(),
        chrono_english::Dialect::Uk,
    )
    .map(|date| date.naive_local())
    .map_err(|err| err.into())
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let conn = &mut establish_connection();

    let today = chrono::Local::now().date_naive();

    let _success = match &args.command {
        Commands::StartNow => create_worktime_entry(conn, None, None, None, false),
        Commands::EndNow { hadbreak } => set_workday_finished(conn, today, *hadbreak),
        _ => Err(anyhow::anyhow!("Invalid command")),
    }?;

    Ok(())
}
