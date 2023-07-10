# Tomato

A simple command line [pomodoro](https://en.wikipedia.org/wiki/Pomodoro_Technique) timer.

## Installation

1. [Install Rust](https://www.rust-lang.org/tools/install) 
2. Clone this repository
3. `cd` into the repository and run `cargo install --path .`

## Usage

Run `tomato` to start a pomodoro session with 20 minute work sessions and 5 minute breaks. Use the `--work` and `--rest` flags to change the duration of the work and rest sessions respectively, e.g. `pomodoro --work 25m --rest 10m`. Any [`humantime`](https://docs.rs/humantime/latest/humantime/) compatible duration can be used.

## Example
![Kapture 2023-07-10 at 08 28 39](https://github.com/hollg/tomato/assets/21319237/171b338a-4288-4a34-a2ec-e90ee3683389)


