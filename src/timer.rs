use indicatif::{ProgressBar, ProgressStyle};

use crate::state::State;
trait TimeTracker {
    fn track_time(&mut self, seconds_left: u64, state: &State);
    fn build(seconds: u64, state: &State) -> Self;
}

impl TimeTracker for ProgressBar {
    fn build(seconds: u64, state: &State) -> Self {
        ProgressBar::new(seconds)
        .with_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {percent}% {msg}",
                )
                .unwrap()
                .progress_chars("#>-"),
        )
        .with_message(format!("{} for {} minutes ", state, seconds / 60))
    }

    fn track_time(&mut self, seconds_left: u64, state: &State) {
        let mins_left = seconds_left / 60;

        if mins_left >= 1 {
            self.set_message(format!("{} for {} minutes", state, mins_left));
        } else {
            self.set_message(format!("{} for {} seconds", state, seconds_left))
        }
    }
}

pub fn run_timer(seconds: u64, state: &State) {
    let mut progress_bar = ProgressBar::build(seconds, state);
    let mut seconds_left = seconds;

    progress_bar.track_time(seconds_left, state);

    for second in 0..seconds {
        seconds_left = seconds - second;

        progress_bar.set_position(second);
        progress_bar.track_time(seconds_left, state);

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
