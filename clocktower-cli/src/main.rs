use chrono::{NaiveDate, NaiveDateTime};
use clap::{Parser, Subcommand};
use clocktower_core::*;
use tabled::{settings::Style, Table, Tabled};

pub fn format_duration(duration: &chrono::Duration) -> String {
    let seconds = duration.num_seconds();
    let minutes = seconds / 60;
    let hours = minutes / 60;

    if seconds < 0 {
        format!("-{}h {:02}m", -hours, -minutes % 60)
    } else {
        format!("{}h {:02}m", hours, minutes % 60)
    }
}

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

fn display_datetime_option(o: &Option<NaiveDateTime>) -> String {
    match o {
        Some(s) => display_datetime(s),
        None => format!("--:--"),
    }
}

fn display_duration_option(o: &Option<chrono::Duration>) -> String {
    match o {
        Some(s) => format_duration(s),
        None => format!("--:--"),
    }
}

fn display_weekday(s: &NaiveDate) -> String {
    format!("{}", s.format("%a, %v"))
}

fn display_datetime(s: &NaiveDateTime) -> String {
    format!("{}", s.format("%H:%Mh"))
}

#[derive(Tabled)]
pub struct EntryTable {
    #[tabled(display_with = "display_weekday")]
    pub day: NaiveDate,
    #[tabled(display_with = "display_datetime")]
    pub start_time: NaiveDateTime,
    #[tabled(display_with = "display_datetime_option")]
    pub end_time: Option<NaiveDateTime>,
    #[tabled(display_with = "display_duration_option")]
    pub duration: Option<chrono::Duration>,
    pub hadbreak: bool,
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
    Summary,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let conn = &mut establish_connection()?;

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

            let mut entries = get_all_entries(conn)?
                .iter()
                .map(|entry| EntryTable {
                    day: entry.day,
                    start_time: entry.start_time,
                    end_time: entry.end_time,
                    duration: entry.end_time.map(|e| e - entry.start_time),
                    hadbreak: entry.hadbreak,
                })
                .collect::<Vec<_>>();

            entries.sort_by_key(|entry| entry.day);

            entries = entries.into_iter().rev().take(5).rev().collect();

            println!("{}", Table::new(entries).with(Style::modern()).to_string());

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
