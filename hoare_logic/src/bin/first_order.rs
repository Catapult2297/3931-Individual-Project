#[allow(dead_code)]
use std::fmt;

#[derive(Debug, Clone)]
pub enum Formula {
    Atom(String),                                // Atomic propositions
    Negation(Box<Formula>),                      // Negation
    Conjunction(Box<Formula>, Box<Formula>),     // AND
    Disjunction(Box<Formula>, Box<Formula>),     // OR
    Implication(Box<Formula>, Box<Formula>),     // IMPLIES
    Equivalence(Box<Formula>, Box<Formula>),     // Equivalence
    UniversalQuantifier(String, Box<Formula>),   // ∀
    ExistentialQuantifier(String, Box<Formula>), // ∃
}

impl Formula {
    pub fn new<T: Into<String>>(input: T) -> Self {
        let input_str: String = input.into();
        let tokens = input_str
            .split_whitespace()
            .map(String::from)
            .collect::<Vec<_>>();
        let mut parser = Parser::new(&tokens);
        parser.parse()
    }
}

impl fmt::Display for Formula {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Formula::Atom(ref s) => write!(f, "{}", s),
            Formula::Negation(ref formula) => write!(f, "¬({formula})"),
            Formula::Conjunction(ref lhs, ref rhs) => write!(f, "({lhs}∧{rhs})"),
            Formula::Disjunction(ref lhs, ref rhs) => write!(f, "({lhs}∨{rhs})"),
            Formula::Implication(ref lhs, ref rhs) => write!(f, "({lhs}→{rhs})"),
            Formula::Equivalence(ref lhs, ref rhs) => write!(f, "({lhs}={rhs})"),
            Formula::UniversalQuantifier(ref variable, ref formula) => {
                write!(f, "∀{variable}({formula})")
            }
            Formula::ExistentialQuantifier(ref variable, ref formula) => {
                write!(f, "∃{variable}({formula})")
            }
        }
    }
}

struct Parser<'a> {
    tokens: &'a [String],
    current: usize,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a [String]) -> Self {
        Parser { tokens, current: 0 }
    }

    fn parse(&mut self) -> Formula {
        self.parse_formula()
    }

    fn parse_formula(&mut self) -> Formula {
        let token: &String = &self.tokens[self.current];
        self.current += 1;

        match token.as_str() {
            "¬" => {
                let inner: Formula = self.parse_formula();
                Formula::Negation(Box::new(inner))
            }
            "∧" => {
                let left: Formula = self.parse_formula();
                let right: Formula = self.parse_formula();
                Formula::Conjunction(Box::new(left), Box::new(right))
            }
            "∨" => {
                let left: Formula = self.parse_formula();
                let right: Formula = self.parse_formula();
                Formula::Disjunction(Box::new(left), Box::new(right))
            }
            "→" => {
                let left: Formula = self.parse_formula();
                let right: Formula = self.parse_formula();
                Formula::Implication(Box::new(left), Box::new(right))
            }
            "∀" => {
                let var: String = self.tokens[self.current].clone();
                self.current += 1;
                let inner: Formula = self.parse_formula();
                Formula::UniversalQuantifier(var, Box::new(inner))
            }
            "∃" => {
                let var: String = self.tokens[self.current].clone();
                self.current += 1;
                let inner: Formula = self.parse_formula();
                Formula::ExistentialQuantifier(var, Box::new(inner))
            }
            "=" => {
                let left: Formula = self.parse_formula();
                let right: Formula = self.parse_formula();
                Formula::Equivalence(Box::new(left), Box::new(right))
            }
            _ => Formula::Atom(token.clone()), // Atomic proposition
        }
    }
}

fn main() {
    let formulae: [Formula; 8] = [
        Formula::new("x"),
        Formula::new("¬ x"),
        Formula::new("∧ x y"),
        Formula::new("∨ x y"),
        Formula::new("→ x y"),
        Formula::new("= x y"),
        Formula::new("∀ x x"),
        Formula::new("∃ x x"),
    ];

    for formula in formulae {
        print!("{formula}");
        print!("            {:?}\n", formula);
    }
}
