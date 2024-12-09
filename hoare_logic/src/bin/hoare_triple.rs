#[allow(dead_code)]
use std::fmt;
mod first_order;

pub struct Triple {
    precondition: first_order::Formula,
    command: String,
    postcondition: first_order::Formula,
}

impl Triple {
    pub fn new<T: Into<String>>(precondition: T, command: T, postcondition: T) -> Triple {
        Triple {
            precondition: first_order::Formula::new(precondition),
            command: command.into(),
            postcondition: first_order::Formula::new(postcondition),
        }
    }
}

impl fmt::Display for Triple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{}}} {} {{{}}}",
            self.precondition, self.command, self.postcondition
        )
    }
}

fn main() {
    let test: Triple = Triple {
        precondition: first_order::Formula::new("⊤"),
        command: String::from("a := 5"),
        postcondition: first_order::Formula::new("= a 5"),
    };

    let test2: Triple = Triple::new("⊤", "skip", "⊤");
    println!("{test}");
    println!("{test2}")
}
