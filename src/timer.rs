use indicatif::{ProgressBar, ProgressStyle};

use crate::state::State;

pub fn run_timer(seconds: u64, state: &State) {
    let progress_bar = ProgressBar::new(seconds)
        .with_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {percent}% {msg}",
                )
                .unwrap()
                .progress_chars("#>-"),
        )
        .with_message(format!("{} for {} minutes ", state, seconds / 60));

    let mut seconds_left = seconds;

    update_progress_bar_message(&progress_bar, seconds_left, &state);

    for second in 0..seconds {
        progress_bar.set_position(second);

        seconds_left = seconds - second;

        update_progress_bar_message(&progress_bar, seconds_left, &state);

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn update_progress_bar_message(progress_bar: &ProgressBar, seconds_left: u64, state: &State) {
    let mins_left = seconds_left / 60;
    if mins_left >= 1 {
        progress_bar.set_message(format!("{} for {} minutes", state, mins_left));
    } else {
        progress_bar.set_message(format!("{} for {} seconds", state, seconds_left))
    }
}
