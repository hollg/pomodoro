pub enum State {
    Work,
    Rest,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = match self {
            State::Work => "Working",
            State::Rest => "Resting",
        };
        write!(f, "{}", state)
    }
}

impl State {
    pub fn toggle(&mut self) {
        match self {
            State::Work => *self = State::Rest,
            State::Rest => *self = State::Work,
        }
    }
}
