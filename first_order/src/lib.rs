//! # First Order Logic Formulae implementation in Rust
//!
//! This module provides an implementation of logical formulae using an enum `Formula`.
//! It supports the following logical operations:
//! - Negation ¬
//! - Conjunction ∧
//! - Disjunction ∨
//! - Implication →
//! - Equivalence =
//! - Less Than <
//! - Universal Quantifier ∀
//! - Existential Quantifier ∃
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
/// An enum representing different types of logical formulae.
///
/// A `Formula` is defined as follows:
/// - `⊥` is a formula.
/// - If `R` is an `n`-place relation symbol and `a,b,...,m` are terms, then `R(a,b,...,m)` is a formula.
/// - If `φ` and `ψ` are formulae and `x` is a variable, then the following are formulae:
///     * `¬ φ`
///     * `∧ φ ψ`
///     * `∨ φ ψ`
///     * `→ φ ψ`
///     * `= φ ψ`
///     * `< φ ψ`
///     * `∀ x φ`
///     * `∃ x φ`
pub enum Formula {
    /// A `Term` is define as follows
    /// - Every variable is a term.
    /// - Every constant symbol is a term
    /// - if `f` is an arity `m` function symbol and `a,b,...,m` are terms then `f(a,b,...,m)` is a term.
    /// <div class="warning">
    /// Do not use whitespace to separate a term. The program will not build a parse tree of a term. Separate a term with whitespace will cause the program to treat the parts as different terms.
    /// </div>
    ///
    /// While a term is distinct from a formula, it is necessary to include term in the `Formula` enum to facilitate the construction of a formula parse tree.
    Term(String),
    /// A `Negation` `Formula` takes a form `¬ φ` where `φ` is a formula.
    Negation(Box<Formula>),
    /// A `Conjunction` `Formula` takes a form `∧ φ ψ` where `φ` and `ψ` are formulae.
    Conjunction(Box<Formula>, Box<Formula>),
    /// A `Disjunction` `Formula` takes a form `∨ φ ψ` where `φ` and `ψ` are formulae.      
    Disjunction(Box<Formula>, Box<Formula>),
    /// A `Implication` `Formula` takes a form `→ φ ψ` where `φ` and `ψ` are formulae.  
    Implication(Box<Formula>, Box<Formula>),
    /// A `Equivalence` `Formula` takes a form `= φ ψ` where `φ` and `ψ` are formulae.
    Equivalence(Box<Formula>, Box<Formula>),
    /// A `LessThan` `Formula` takes a form `< φ ψ` where `φ` and `ψ` are formulae.
    LessThan(Box<Formula>, Box<Formula>),
    /// A `UniversalQuantifier` `Formula` takes a form `∀ x φ` where `φ` is a formula and `x` is a variable.
    UniversalQuantifier(String, Box<Formula>),
    /// A `ExistentialQuantifier` `Formula` takes a form `∃ x φ` where `φ` is a formula and `x` is a variable.
    ExistentialQuantifier(String, Box<Formula>),
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
    /// use first_order::Formula;
    /// // Create a formula using the new function
    /// let test_formula: Formula = Formula::new("∧ ∀ x → P(x) ∧ Q(x) ∃ y ∨ R(y) S(y) = ¬ T(x) < U V");
    /// assert_eq!(
    ///     format!("{test_formula}"),
    ///     "(∀x((P(x)→(Q(x)∧∃y((R(y)∨S(y))))))∧((¬T(x))=(U<V)))"
    /// );
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_infix_notation())
    }
}

impl Formula {
    /// Creates a new `Formula` from a string input.
    ///
    /// # Arguments
    /// * `input` - A `String` or `&str` that represents the logical formula in prefix notation. Every terms, logical connectives, and logical quantifiers must be separated using a whitespace.
    ///
    /// # Returns
    /// A `Formula` instance representing the parsed logical formula.
    ///
    /// # Panics
    /// This function will panic if the input string is malformed or cannot be parsed into a valid `Formula`.
    ///
    /// # Example
    /// ```
    /// use first_order::Formula;
    ///
    /// // Create a formula using the new function
    /// let test_formula: Formula = Formula::new("∧ ∀ x → P(x) ∧ Q(x) ∃ y ∨ R(y) S(y) = ¬ T(x) < U V");
    ///
    /// // Expected result constructed manually for comparison
    /// let result: Formula = Formula::Conjunction(
    ///     Box::new(Formula::UniversalQuantifier(
    ///         "x".to_string(),
    ///         Box::new(Formula::Implication(
    ///             Box::new(Formula::Term("P(x)".to_string())),
    ///             Box::new(Formula::Conjunction(
    ///                 Box::new(Formula::Term("Q(x)".to_string())),
    ///                 Box::new(Formula::ExistentialQuantifier(
    ///                     "y".to_string(),
    ///                     Box::new(Formula::Disjunction(
    ///                         Box::new(Formula::Term("R(y)".to_string())),
    ///                         Box::new(Formula::Term("S(y)".to_string())),
    ///                     )),
    ///                 )),
    ///             )),
    ///         )),
    ///     )),
    ///     Box::new(Formula::Equivalence(
    ///         Box::new(Formula::Negation(Box::new(Formula::Term(
    ///             "T(x)".to_string(),
    ///         )))),
    ///         Box::new(Formula::LessThan(
    ///             Box::new(Formula::Term("U".to_string())),
    ///             Box::new(Formula::Term("V".to_string())),
    ///         )),
    ///     )),
    /// );
    /// assert_eq!(test_formula, result);
    /// ```
    pub fn new<T: Into<String>>(input: T) -> Self {
        let input_str: String = input.into();
        let tokens: Vec<String> = input_str
            .split_whitespace()
            .map(String::from)
            .collect::<Vec<_>>();
        let mut parser: Parser<'_> = Parser::new(&tokens);
        match parser.parse() {
            Ok(formula) => formula,
            Err(_) => panic!("The input {:?} is malformed.", input_str),
        }
    }
    /// Converts the formula itself prefix notation.
    ///
    /// This method returns a string representation of the formula in prefix notation,
    /// which is a way of writing logical expressions where the operator precedes its operands.
    ///
    /// # Returns
    /// A `String` representing the formula in prefix notation
    ///
    /// # Example
    /// ```
    /// use first_order::Formula;
    ///     let test_formula: Formula = Formula::new("∧ ∀ x → P(x) ∧ Q(x) ∃ y ∨ R(y) S(y) = ¬ T(x) < U V");
    /// assert_eq!(
    ///     test_formula.to_prefix_notation(),
    ///     "∧ ∀ x → P(x) ∧ Q(x) ∃ y ∨ R(y) S(y) = ¬ T(x) < U V"
    /// );
    /// ```
    pub fn to_prefix_notation(&self) -> String {
        match self {
            Formula::Term(s) => format!("{s}"),
            Formula::Negation(formula) => format!("¬ {}", formula.to_prefix_notation()),
            Formula::Conjunction(lhs, rhs) => {
                format!(
                    "∧ {} {}",
                    lhs.to_prefix_notation(),
                    rhs.to_prefix_notation()
                )
            }
            Formula::Disjunction(lhs, rhs) => format!(
                "∨ {} {}",
                lhs.to_prefix_notation(),
                rhs.to_prefix_notation()
            ),
            Formula::Implication(lhs, rhs) => format!(
                "→ {} {}",
                lhs.to_prefix_notation(),
                rhs.to_prefix_notation()
            ),
            Formula::Equivalence(lhs, rhs) => format!(
                "= {} {}",
                lhs.to_prefix_notation(),
                rhs.to_prefix_notation()
            ),
            Formula::LessThan(lhs, rhs) => format!(
                "< {} {}",
                lhs.to_prefix_notation(),
                rhs.to_prefix_notation()
            ),
            Formula::UniversalQuantifier(variable, formula) => {
                format!("∀ {} {}", variable, formula.to_prefix_notation())
            }
            Formula::ExistentialQuantifier(variable, formula) => {
                format!("∃ {} {}", variable, formula.to_prefix_notation())
            }
        }
    }
    /// Converts the formula itself infix notation.
    ///
    /// This method returns a string representation of the formula in infix notation,
    /// which is a way of writing logical expressions where operators are placed between
    /// their operands. This format is commonly used in mathematical expressions and is
    /// more familiar to users.
    ///
    /// # Returns
    /// A `String` representing the formula in infix notation
    ///
    /// # Examples
    /// ```
    /// use first_order::Formula;
    /// let test_formula: Formula = Formula::new("∧ ∀ x → P(x) ∧ Q(x) ∃ y ∨ R(y) S(y) = ¬ T(x) < U V");
    /// assert_eq!(
    ///    format!("{test_formula}"),
    ///    "(∀x((P(x)→(Q(x)∧∃y((R(y)∨S(y))))))∧((¬T(x))=(U<V)))"
    /// );
    /// ```
    pub fn to_infix_notation(&self) -> String {
        match self {
            Formula::Term(s) => format!("{s}"),
            Formula::Negation(formula) => format!("(¬{})", formula.to_infix_notation()),
            Formula::Conjunction(lhs, rhs) => {
                format!("({}∧{})", lhs.to_infix_notation(), rhs.to_infix_notation())
            }
            Formula::Disjunction(lhs, rhs) => {
                format!("({}∨{})", lhs.to_infix_notation(), rhs.to_infix_notation())
            }
            Formula::Implication(lhs, rhs) => {
                format!("({}→{})", lhs.to_infix_notation(), rhs.to_infix_notation())
            }
            Formula::Equivalence(lhs, rhs) => {
                format!("({}={})", lhs.to_infix_notation(), rhs.to_infix_notation())
            }
            Formula::LessThan(lhs, rhs) => {
                format!("({}<{})", lhs.to_infix_notation(), rhs.to_infix_notation())
            }
            Formula::UniversalQuantifier(variable, formula) => {
                format!("∀{}({})", variable, formula.to_infix_notation())
            }
            Formula::ExistentialQuantifier(variable, formula) => {
                format!("∃{}({})", variable, formula.to_infix_notation())
            }
        }
    }
    /// Retrieves information about the formula in an array format
    ///
    /// # Returns
    /// Returns an array of three `String` values containing the following information:
    /// - Position 0: The type of the formula.
    /// - Position 1: The first argument of the formula.
    /// - Position 2: the second argument of the formula. If the formula is a term. The function will return an empty  `String`.
    ///
    /// # Examples
    /// ```
    /// use first_order::Formula;
    /// let test_formula: Formula = Formula::new("∧ ∀ x → P(x) ∧ Q(x) ∃ y ∨ R(y) S(y) = ¬ T(x) < U V");
    /// assert_eq!(
    ///    test_formula.get_info(),
    ///    [
    ///        "Conjunction",
    ///        "∀ x → P(x) ∧ Q(x) ∃ y ∨ R(y) S(y)",
    ///        "= ¬ T(x) < U V"
    ///    ]
    /// );
    /// ```
    pub fn get_info(&self) -> [String; 3] {
        match self {
            Formula::Term(s) => ["Term".to_string(), s.to_string(), "".to_string()],
            Formula::Negation(formula) => [
                "Negation".to_string(),
                formula.to_prefix_notation(),
                "".to_string(),
            ],
            Formula::Conjunction(lhs, rhs) => [
                "Conjunction".to_string(),
                lhs.to_prefix_notation(),
                rhs.to_prefix_notation(),
            ],
            Formula::Disjunction(lhs, rhs) => [
                "Disjunction".to_string(),
                lhs.to_prefix_notation(),
                rhs.to_prefix_notation(),
            ],
            Formula::Implication(lhs, rhs) => [
                "Implication".to_string(),
                lhs.to_prefix_notation(),
                rhs.to_prefix_notation(),
            ],
            Formula::Equivalence(lhs, rhs) => [
                "Equivalence".to_string(),
                lhs.to_prefix_notation(),
                rhs.to_prefix_notation(),
            ],
            Formula::LessThan(lhs, rhs) => [
                "LessThan".to_string(),
                lhs.to_prefix_notation(),
                rhs.to_prefix_notation(),
            ],
            Formula::UniversalQuantifier(variable, formula) => [
                "UniversalQuantifier".to_string(),
                variable.to_string(),
                formula.to_prefix_notation(),
            ],
            Formula::ExistentialQuantifier(variable, formula) => [
                "ExistentialQuantifier".to_string(),
                variable.to_string(),
                formula.to_prefix_notation(),
            ],
        }
    }
}

/// A struct for parsing logical formulae from a sequence of tokens.
#[derive(Debug)]
enum ParseError {
    MalformedInput,
}

struct Parser<'a> {
    tokens: &'a [String], // A slice of tokens representing the logical formula.
    current: usize,       // The current index in the token slice.
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a [String]) -> Self {
        Parser { tokens, current: 0 }
    }

    fn parse(&mut self) -> Result<Formula, ParseError> {
        self.parse_formula()
    }

    fn parse_formula(&mut self) -> Result<Formula, ParseError> {
        if self.current == self.tokens.len() {
            return Err(ParseError::MalformedInput);
        }

        let token: &String = &self.tokens[self.current];
        self.current += 1;

        match token.as_str() {
            "¬" => {
                let inner = self.parse_formula()?;
                Ok(Formula::Negation(Box::new(inner)))
            }
            "∧" => {
                let left = self.parse_formula()?;
                let right = self.parse_formula()?;
                Ok(Formula::Conjunction(Box::new(left), Box::new(right)))
            }
            "∨" => {
                let left = self.parse_formula()?;
                let right = self.parse_formula()?;
                Ok(Formula::Disjunction(Box::new(left), Box::new(right)))
            }
            "→" => {
                let left = self.parse_formula()?;
                let right = self.parse_formula()?;
                Ok(Formula::Implication(Box::new(left), Box::new(right)))
            }
            "∀" => {
                let var = self
                    .tokens
                    .get(self.current)
                    .ok_or(ParseError::MalformedInput)?
                    .clone();
                self.current += 1;
                let inner = self.parse_formula()?;
                Ok(Formula::UniversalQuantifier(var, Box::new(inner)))
            }
            "∃" => {
                let var = self
                    .tokens
                    .get(self.current)
                    .ok_or(ParseError::MalformedInput)?
                    .clone();
                self.current += 1;
                let inner = self.parse_formula()?;
                Ok(Formula::ExistentialQuantifier(var, Box::new(inner)))
            }
            "=" => {
                let left = self.parse_formula()?;
                let right = self.parse_formula()?;
                Ok(Formula::Equivalence(Box::new(left), Box::new(right)))
            }
            "<" => {
                let left = self.parse_formula()?;
                let right = self.parse_formula()?;
                Ok(Formula::LessThan(Box::new(left), Box::new(right)))
            }
            _ => Ok(Formula::Term(token.clone())), // Atomic proposition
        }
    }
}
