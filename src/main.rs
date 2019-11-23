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
        State::On => State::Off,
        State::Off => State::On
    }
}

enum State
{
    On = 1,
    Off = 0 
}

pub struct Universe
{
    width: u32,
    height: u32,
    cells: Vec<State>,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let index = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[index] as u8;
            }
        }
    count
    }
}
