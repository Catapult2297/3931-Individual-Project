//! # First Order Logic Formulae implementation in Rust
//!
//! This module provides an implementation of logical formulaes using an enum `Formula`.
//! It supports the following logical operations:
//! - Atomic Proposition
//! - Negation ¬
//! - Conjunction ∧
//! - Disjunction ∨
//! - Implication →
//! - Equivalence =
//! - Less Than <
//! - Universal Quantifier ∀
//! - Existential Quantifier ∃

#[allow(dead_code)]
use std::fmt;

#[derive(Debug, Clone)]
/// An enum representing different types of logical formulae.
pub enum Formula {
    Atom(String),                                // ATOMIC PROPOSITION
    Negation(Box<Formula>),                      // NEGATION
    Conjunction(Box<Formula>, Box<Formula>),     // AND
    Disjunction(Box<Formula>, Box<Formula>),     // OR
    Implication(Box<Formula>, Box<Formula>),     // IMPLIES
    Equivalence(Box<Formula>, Box<Formula>),     // EQUIVALANCE
    LessThan(Box<Formula>, Box<Formula>),        // LESS THAN
    UniversalQuantifier(String, Box<Formula>),   // FOR ALL
    ExistentialQuantifier(String, Box<Formula>), // THERE EXIST
}

impl Formula {
    /// Creates a new `Formula` from a string input.
    ///
    /// # Arguments
    /// * `input` - A `String` or `&str` that represents the logical formula in prefix notation. Every atomic propositions, logical connectives, and logical quantifiers must be separated using a whitespace.
    ///
    /// # Returns
    /// A `Formula` instance representing the parsed logical formula.
    ///
    /// # Example
    /// ```
    /// let formula1: Formula = Formula::new("∃ a → b ∧ c a");
    ///
    /// ```
    pub fn new<T: Into<String>>(input: T) -> Self {
        let input_str: String = input.into();
        let tokens: Vec<String> = input_str
            .split_whitespace()
            .map(String::from)
            .collect::<Vec<_>>();
        let mut parser: Parser<'_> = Parser::new(&tokens);
        parser.parse()
    }
    /// Converts the formula itself prefix notation.
    ///
    /// # Returns
    /// A `String` representing the formuila in prefix notation
    ///
    /// # Example
    /// ```
    /// let formula1: Formula = Formula::new("∃ a → b ∧ c a");
    /// println!("{}", formula1.to_prefix_notation()); // Output: ∃ a → b ∧ c a
    /// ```
    fn to_prefix_notation(&self) -> String {
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
            Formula::LessThan(ref lhs, ref rhs) => format!(
                "< {} {}",
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
    /// Converts the formula itself infix notation.
    ///
    /// # Returns
    /// A `String` representing the formuila in infix notation
    ///
    /// # Examples
    /// ```
    ///let formula1: Formula = Formula::new("∃ a → b ∧ c a");
    ///println!("{}", formula1.to_prefix_notation()); // Output: (∃a)((b→(c∧a)))
    /// ```
    fn to_infix_notation(&self) -> String {
        match self {
            Formula::Atom(ref s) => format!("{s}"),
            Formula::Negation(ref formula) => format!("¬({})", formula.to_infix_notation()),
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
            Formula::LessThan(ref lhs, ref rhs) => {
                format!("({}<{})", lhs.to_infix_notation(), rhs.to_infix_notation())
            }
            Formula::UniversalQuantifier(ref variable, ref formula) => {
                format!("(∀{})({})", variable, formula.to_infix_notation())
            }
            Formula::ExistentialQuantifier(ref variable, ref formula) => {
                format!("(∃{})({})", variable, formula.to_infix_notation())
            }
        }
    }
    /// Retrieves information about the formula in an array format
    ///
    /// # Returns
    /// An array `[String; 3]` contains the following information:
    /// - Position 0: The type of the formula.
    /// - Position 1: The first argument of the formula.
    /// - Position 2: the second argument of the formula. If the formula is atomic. The function will return an empty "" `String`.
    ///
    /// # Examples
    /// ```
    ///let formula1: Formula = Formula::new("∃ a → b ∧ c a");
    ///println!("{:?}", formula1.get_info()); // Output: ["ExistentialQuantifier", "a", "→ b ∧ c a"]
    /// ```
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
            Formula::LessThan(ref lhs, ref rhs) => [
                "LessThan".to_string(),
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
    /// Formats the formula in infix notation for display.
    ///
    /// This implementation of the `Display` trait allows for easy printing of
    /// `Formula` instances. When a formula is used as a `print` macro's input, it will
    /// be represented in infix notation, which is more familiar and readable
    /// for users who are accustomed to standard mathematical expressions.
    ///
    /// # Example
    ///
    /// ```
    /// let formula1: Formula = Formula::new("∃ a → b ∧ c a")
    /// println!("{formula}"); // Output: (∃a)((b→(c∧a)))
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_infix_notation())
    }
}

/// A struct for parsing logical formulae from a sequence of tokens.
struct Parser<'a> {
    tokens: &'a [String], // A slice of tokens representing the logical formula.
    current: usize,       // The current index in the token slice.
}

impl<'a> Parser<'a> {
    /// Creates a new `Parser` instance with the given tokens.
    ///
    /// # Arguments
    ///
    /// * `tokens` - A slice of strings that represent the tokens of a logical formula.
    ///
    /// # Returns
    ///
    /// A new `Parser` instance initialized with the provided tokens and the current
    /// index set to zero.
    fn new(tokens: &'a [String]) -> Self {
        Parser { tokens, current: 0 }
    }
    /// Parses the tokens and constructs a `Formula`.
    ///
    /// This method is the entry point for parsing. It starts the parsing process
    /// and returns the resulting `Formula`.
    ///
    /// # Returns
    ///
    /// A `Formula` instance representing the parsed logical formula.
    fn parse(&mut self) -> Formula {
        self.parse_formula()
    }
    /// Parses a formula from the current token.
    ///
    /// This method processes the current token and constructs the corresponding
    /// `Formula`. It handles different types of logical operations and recursively
    /// parses sub-formulas as needed.
    ///
    /// # Returns
    ///
    /// A `Formula` instance representing the parsed formula.
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
            "<" => {
                let left: Formula = self.parse_formula();
                let right: Formula = self.parse_formula();
                Formula::LessThan(Box::new(left), Box::new(right))
            }
            _ => Formula::Atom(token.clone()), // Atomic proposition
        }
    }
}

fn main() {
    let formulae: [Formula; 10] = [
        Formula::new("x"),
        Formula::new("¬ x"),
        Formula::new("∧ x y"),
        Formula::new("∨ x y"),
        Formula::new("→ x y"),
        Formula::new("= x y"),
        Formula::new("∀ x x"),
        Formula::new("∃ x x"),
        Formula::new("∃ a → b ∧ c a"),
        Formula::new("→ ¬ ∨ ∧ a b c d"),
    ];

    println!(
        "{:<17} {:<90} {:<50}",
        "Formula", "Formula (Debug)", "Formula (Info Array)"
    );
    println!("{:-<17} {:-<90} {:-<50}", "", "", "");

    for formula in formulae {
        let array: [String; 3] = formula.get_info();
        println!(
            "{:<17} {:<90} {:<50}",
            format!("{}", formula),
            format!("{:?}", formula),
            format!("{:?}", array)
        );
    }
}
