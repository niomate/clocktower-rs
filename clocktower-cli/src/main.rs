use chrono::{NaiveDate, NaiveDateTime};
use clap::{Parser, Subcommand};
use clocktower_core::*;

fn parse_date(date_string: String) -> anyhow::Result<NaiveDate> {
    parse_datetime(date_string).map(|date| date.date())
}

fn parse_datetime(date_string: String) -> anyhow::Result<NaiveDateTime> {
    chrono_english::parse_date_string(
        &date_string,
        chrono::Local::now(),
        chrono_english::Dialect::Uk,
    )
    .map(|date| date.naive_local())
    .map_err(|err| err.into())
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Start,
    End {
        #[clap(short = 'b', long)]
        hadbreak: bool,
    },
    UpdateStart {
        #[arg(value_parser = |arg0: &str| parse_date(arg0.to_string()))]
        date: NaiveDate,
        #[arg(value_parser = |arg0: &str| parse_datetime(arg0.to_string()))]
        start_time: NaiveDateTime,
    },
    UpdateEnd {
        #[arg(value_parser = |arg0: &str| parse_date(arg0.to_string()))]
        date: NaiveDate,
        #[arg(value_parser = |arg0: &str| parse_datetime(arg0.to_string()))]
        end_time: NaiveDateTime,
        #[clap(short = 'b', long)]
        hadbreak: bool,
    },
    Insert {
        #[arg(value_parser = |arg0: &str| parse_date(arg0.to_string()))]
        date: NaiveDate,
        #[arg(value_parser = |arg0: &str| parse_datetime(arg0.to_string()))]
        start_time: NaiveDateTime,
        #[arg(value_parser = |arg0: &str| parse_datetime(arg0.to_string()))]
        end_time: NaiveDateTime,
        #[clap(short = 'b', long)]
        hadbreak: bool,
    },
    DeleteEntry {
        #[arg(value_parser = |arg0: &str| parse_date(arg0.to_string()))]
        date: NaiveDate,
    },
    SetBreak {
        #[arg(value_parser = |arg0: &str| parse_date(arg0.to_string()))]
        date: NaiveDate,
    },
    Overtime,
    DeleteAll,
    Print,
    // TODO: Pass a filter here (e.g. time worked per month)
    // TODO: Compute max possible worktime by counting how many days where summed up
    TotalWorktime,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let conn = &mut establish_connection();

    let today = chrono::Local::now().date_naive();

    let _success = match args.command {
        Commands::Start => create_worktime_entry(conn, None, None, None, false),
        Commands::End { hadbreak } => set_workday_finished_now(conn, today, hadbreak),
        Commands::DeleteAll => delete_all_entries(conn),
        Commands::Print => print_entries(conn).map(|_| true),
        Commands::TotalWorktime => {
            let total_duration = sum_worktimes(conn)?;
            let seconds = total_duration.num_seconds();
            let minutes = seconds / 60;
            let hours = minutes / 60;

            println!(
                "Total worktime: {}h {}m {}s",
                hours % 60,
                minutes % 60,
                seconds % 60
            );
            Ok(true)
        }
        Commands::Overtime => {
            let total_duration = overtime(conn)?;
            let mut negative = false;
            let mut seconds = total_duration.num_seconds();
            if seconds < 0 {
                seconds *= -1;
                negative = true;
            }
            let minutes = seconds / 60;
            let hours = minutes / 60;

            println!(
                "Total overtime: {}{}h {}m {}s",
                if negative { "-" } else { "" },
                hours % 60,
                minutes % 60,
                seconds % 60
            );
            Ok(true)
        }
        Commands::UpdateEnd {
            date,
            end_time,
            hadbreak,
        } => set_workday_finished(conn, date, end_time, hadbreak),
        Commands::UpdateStart { date, start_time } => update_start_time(conn, date, start_time),
        Commands::DeleteEntry { date } => delete_entry(conn, date),
        Commands::Insert {
            date,
            start_time,
            end_time,
            hadbreak,
        } => create_worktime_entry(conn, Some(date), Some(start_time), Some(end_time), hadbreak),
        Commands::SetBreak { date } => set_break(conn, date),
    }?;

    Ok(())
}
