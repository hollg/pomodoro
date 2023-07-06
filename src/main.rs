mod cli;
mod state;
mod timer;

use clap::Parser;
use cli::Args;
use state::State;
use std::process::Command;
use timer::run_timer;

fn main() {
    let Args { rest, work, vox } = Args::parse();
    let mut state = State::Work;

    loop {
        let time = match state {
            State::Work => work.as_secs(),
            State::Rest => rest.as_secs(),
        };

        if vox {
            Command::new("say")
                .arg(format!("Time to start {}", state))
                .spawn()
                .expect("Failed to speak");
        }

        run_timer(time, &state);
        state.toggle();
    }
}
