mod cli;
mod state;
mod timer;

use clap::Parser;
use cli::Args;
use state::State;
use timer::run_timer;

fn main() {
    let args = Args::parse();

    let mut state = State::Work;

    let work_seconds = args.work.as_secs();
    let rest_seconds = args.rest.as_secs();

    loop {
        let time = match state {
            State::Work => work_seconds,
            State::Rest => rest_seconds,
        };

        run_timer(time, &state);
        state.toggle();
    }
}
