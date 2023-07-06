use clap::Parser;
use humantime::Duration;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Human-readable timer duration, e.g. "20 minutes" or "2h 30m"
    #[arg(short, long, default_value = "20 minutes")]
    pub work: Duration,
    #[arg(short, long, default_value = "5 minutes")]
    pub rest: Duration,
}
