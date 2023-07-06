use clap::Parser;
use humantime::Duration;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Length of the work period in human-readable format, e.g. "20min", "1hr 30min" etc
    ///
    /// Defaults to 20 minutes
    #[arg(short, long, default_value = "20 minutes")]
    pub work: Duration,
    /// Length of the rest period in human-readable format, e.g. "20min", "1hr 30min" etc
    ///
    /// Defaults to 5 minutes
    #[arg(short, long, default_value = "5 minutes")]
    pub rest: Duration,
}
