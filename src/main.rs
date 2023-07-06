mod cli;
mod state;
mod timer;

use clap::Parser;
use cli::Args;
use state::State;
use timer::run_timer;

fn main() {
    let Args { rest, work } = Args::parse();
    let mut state = State::Work;

    loop {
        let time = match state {
            State::Work => work.as_secs(),
            State::Rest => rest.as_secs(),
        };

        run_timer(time, &state);
        state.toggle();
    }
}
