
enum State{
    Mario,
    SuperMario,
    FireMario,
    CapeMario
}

enum Transition{
    Flower,
    Mushroom,
    Feather
}

struct Player{
    state: State,
}

impl Player{
    fn new() -> Self {
        Self { state: State::Mario }
    }
    fn collect(&mut self, power: Transition){
        match (&self.state,power) {
            (_, _flower) => self.state = State::FireMario,
            (_, _mushroom) => self.state = State::SuperMario,
            (_, _feather) => self.state = State::CapeMario,
        }
    }
}

fn main() {
    println!("Hello, world!");
    let mut x = String::new();
    let _ = std::io::stdin().read_line(&mut x);
    println!("{}",x);
}
