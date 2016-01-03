use std::thread;

struct Philosopher {
    name: String,
}

impl Philosopher {
    fn new(name: &str) -> Philosopher {
        Philosopher {
            name: name.to_string(),
        }
    }

    fn eat(&self) {
        println!("{} is eating.", self.name);
        thread::sleep_ms(1000);
        println!("{} is done eating.", self.name);
    }
}

fn main() {
    let philosophers = vec![
        Philosopher::new("Jacques Rancière"),
        Philosopher::new("Gilles Deleuze"),
        Philosopher::new("Georgio Agamben"),
        Philosopher::new("Hannah Arendt"),
        Philosopher::new("Michel Foucault"),
    ];

    for philosopher in &philosophers {
        philosopher.eat();
    }
}
