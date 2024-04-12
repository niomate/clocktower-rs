use chrono::{NaiveDate, NaiveDateTime};
use clap::{Parser, Subcommand};
use clocktower_core::*;

fn parse_date(date_string: &str) -> anyhow::Result<NaiveDate> {
    parse_datetime(date_string).map(|date| date.date())
}

fn parse_datetime(date_string: &str) -> anyhow::Result<NaiveDateTime> {
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
    StartAt {
        #[arg(value_parser = parse_datetime)]
        start_time: NaiveDateTime,
    },
    EndAt {
        #[arg(value_parser = parse_datetime)]
        end_time: NaiveDateTime,
        #[clap(short = 'b', long)]
        hadbreak: bool,
    },
    UpdateStart {
        #[arg(value_parser = parse_date)]
        date: NaiveDate,
        #[arg(value_parser = parse_datetime)]
        start_time: NaiveDateTime,
    },
    UpdateEnd {
        #[arg(value_parser = parse_date)]
        date: NaiveDate,
        #[arg(value_parser = parse_datetime)]
        end_time: NaiveDateTime,
        #[clap(short = 'b', long)]
        hadbreak: bool,
    },
    Insert {
        #[arg(value_parser = parse_date)]
        date: NaiveDate,
        #[arg(value_parser = parse_datetime)]
        start_time: NaiveDateTime,
        #[arg(value_parser = parse_datetime)]
        end_time: NaiveDateTime,
        #[clap(short = 'b', long)]
        hadbreak: bool,
    },
    DeleteEntry {
        #[arg(value_parser = parse_date)]
        date: NaiveDate,
    },
    SetBreak {
        #[arg(value_parser = parse_date)]
        date: NaiveDate,
    },
    DeleteAll,
    Summary
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let conn = &mut establish_connection();

    let today = chrono::Local::now().date_naive();

    match args.command {
        Commands::Start => insert_worktime_entry(conn, None, None, None, false),
        Commands::End { hadbreak } => set_workday_finished_now(conn, today, hadbreak),
        Commands::StartAt { start_time } => {
            insert_worktime_entry(conn, Some(start_time.date()), Some(start_time), None, false)
        }
        Commands::EndAt { end_time, hadbreak } => {
            set_workday_finished(conn, end_time.date(), end_time, hadbreak)
        }
        Commands::DeleteAll => delete_all_entries(conn),
        Commands::Summary => {
            let worktime_summary = sum_worktimes(conn)?;
            let overtime = worktime_summary.overtime();
            print_entries(conn)?;
            println!(
                "Total time worked: {}",
                format_duration(&worktime_summary.total_duration)
            );
            println!("Overtime: {}", format_duration(&overtime));

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
        } => insert_worktime_entry(conn, Some(date), Some(start_time), Some(end_time), hadbreak),
        Commands::SetBreak { date } => set_break(conn, date),
    }?;

    Ok(())
}
