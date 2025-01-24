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
        let tokens: Vec<String> = input_str
            .split_whitespace()
            .map(String::from)
            .collect::<Vec<_>>();
        let mut parser: Parser<'_> = Parser::new(&tokens);
        parser.parse()
    }
    pub fn to_prefix_notation(&self) -> String {
        match self {
            Formula::Atom(ref s) => format!("{s}"),
            Formula::Negation(ref formula) => format!("¬ {}", formula.to_prefix_notation()),
            Formula::Conjunction(ref lhs, ref rhs) => {
                format!(
                    "∧ {} {}",
                    lhs.to_prefix_notation(),
                    rhs.to_prefix_notation()
                )
            }
            Formula::Disjunction(ref lhs, ref rhs) => format!(
                "∨ {} {}",
                lhs.to_prefix_notation(),
                rhs.to_prefix_notation()
            ),
            Formula::Implication(ref lhs, ref rhs) => format!(
                "→ {} {}",
                lhs.to_prefix_notation(),
                rhs.to_prefix_notation()
            ),
            Formula::Equivalence(ref lhs, ref rhs) => format!(
                "= {} {}",
                lhs.to_prefix_notation(),
                rhs.to_prefix_notation()
            ),
            Formula::UniversalQuantifier(ref variable, ref formula) => {
                format!("∀ {} {}", variable, formula.to_prefix_notation())
            }
            Formula::ExistentialQuantifier(ref variable, ref formula) => {
                format!("∃ {} {}", variable, formula.to_prefix_notation())
            }
        }
    }
    fn to_infix_notation(&self) -> String {
        match self {
            Formula::Atom(ref s) => format!("{s}"),
            Formula::Negation(ref formula) => format!("¬ {}", formula.to_prefix_notation()),
            Formula::Conjunction(ref lhs, ref rhs) => {
                format!("({}∧{})", lhs.to_infix_notation(), rhs.to_infix_notation())
            }
            Formula::Disjunction(ref lhs, ref rhs) => {
                format!("({}∨{})", lhs.to_infix_notation(), rhs.to_infix_notation())
            }
            Formula::Implication(ref lhs, ref rhs) => {
                format!("({}→{})", lhs.to_infix_notation(), rhs.to_infix_notation())
            }
            Formula::Equivalence(ref lhs, ref rhs) => {
                format!("({}={})", lhs.to_infix_notation(), rhs.to_infix_notation())
            }
            Formula::UniversalQuantifier(ref variable, ref formula) => {
                format!("∀{}({})", variable, formula.to_infix_notation())
            }
            Formula::ExistentialQuantifier(ref variable, ref formula) => {
                format!("∃{}({})", variable, formula.to_infix_notation())
            }
        }
    }
    pub fn get_info(&self) -> [String; 3] {
        match self {
            Formula::Atom(ref s) => ["Atom".to_string(), s.to_string(), "".to_string()],
            Formula::Negation(ref formula) => [
                "Negation".to_string(),
                formula.to_prefix_notation(),
                "".to_string(),
            ],
            Formula::Conjunction(ref lhs, ref rhs) => [
                "Conjunction".to_string(),
                lhs.to_prefix_notation(),
                rhs.to_prefix_notation(),
            ],
            Formula::Disjunction(ref lhs, ref rhs) => [
                "Disjunction".to_string(),
                lhs.to_prefix_notation(),
                rhs.to_prefix_notation(),
            ],
            Formula::Implication(ref lhs, ref rhs) => [
                "Implication".to_string(),
                lhs.to_prefix_notation(),
                rhs.to_prefix_notation(),
            ],
            Formula::Equivalence(ref lhs, ref rhs) => [
                "Equivalence".to_string(),
                lhs.to_prefix_notation(),
                rhs.to_prefix_notation(),
            ],
            Formula::UniversalQuantifier(ref variable, ref formula) => [
                "UniversalQuantifier".to_string(),
                variable.to_string(),
                formula.to_prefix_notation(),
            ],
            Formula::ExistentialQuantifier(ref variable, ref formula) => [
                "ExistentialQuantifier".to_string(),
                variable.to_string(),
                formula.to_prefix_notation(),
            ],
        }
    }
}

impl fmt::Display for Formula {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_infix_notation())
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
    let formulae: [Formula; 9] = [
        Formula::new("x"),
        Formula::new("¬ x"),
        Formula::new("∧ x y"),
        Formula::new("∨ x y"),
        Formula::new("→ x y"),
        Formula::new("= x y"),
        Formula::new("∀ x x"),
        Formula::new("∃ x x"),
        Formula::new("→ b ∧ c d"),
    ];

    println!(
        "{:<20} {:<60} {:<60}",
        "Formula", "Formula (Debug)", "Formula (Info Array)"
    );
    println!("{:-<20} {:-<60} {:-<60}", "", "", "");

    for formula in formulae {
        let array: [String; 3] = formula.get_info();
        println!(
            "{:<20} {:<60} {:<60}",
            format!("{}", formula),
            format!("{:?}", formula),
            format!("{:?}", array)
        );
    }
}
