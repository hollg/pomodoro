# Pomodoro

A simple command line [pomodoro](https://en.wikipedia.org/wiki/Pomodoro_Technique) timer.

## Installation

1. Clone the repository
2. `cd` into the repository and run `cargo install --path .`

## Usage

Run `tomato` to start a pomodoro session with 20 minute work sessions and 5 minute breaks. Use the `--work` and `--rest` flags to change the duration of the work and rest sessions respectively, e.g. `pomodoro --work 25m --rest 10m`. Any [`humantime`](https://docs.rs/humantime/latest/humantime/) compatible duration can be used.

## Example

![pomodoro](https://github.com/hollg/pomodoro/assets/21319237/2b904546-cdc3-49bf-b9f6-d763229c73b1)
