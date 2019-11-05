fn main() {
    println!("Hello, world!");
    let mut state = State::On;
    state = flip(state);
    match state{
        State::On => (println!("Something bad happened")),
        State::Off => (println!("All is well"))
    }
}

fn flip(state: State) -> State
{
    match state{
        State::On => (return State::Off),
        State::Off => (return State::On)
    };
}

enum State
{
    On,
    Off 
}