use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Date of the workday to be modified
    date: String,

    #[arg(short, long)]
    start_time: Option<String>,

    #[arg(short, long)]
    end_time: Option<String>,

    #[arg(short, long)]
    hadbreak: bool
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let date = chrono_english::parse_date_string(
        &args.date,
        chrono::Local::now(),
        chrono_english::Dialect::Uk,
    );
    println!("{:?}", date?.naive_local());
    Ok(())
}
