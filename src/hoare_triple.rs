//! # Hoare Logic implementation in Rust
//!
//! This module provides an implementation of Hoare Logic Triple using an struct `Triple`.
//! It supports the following Hoare Logic axioms and rules:
//! - Empty Statement Axiom
//! - Assignment Axiom
//! - Rule of Composition
//! - Condition Rule
//! - Consequence Rule
//! - While Rule
#![warn(missing_docs)]
#[allow(dead_code)]
use std::fmt;
//mod first_order;
use first_order::Formula;

//use crate::first_order::Formula;

/// A struct for storing the three parts of a Hoare Triple
pub struct Triple {
    precondition: Formula,
    command: String,
    postcondition: Formula,
}

impl Triple {
    /// Creates a new `Triple` from three string input.
    ///
    /// # Arguments
    /// * `precondition` - A `String` or `&str` that represents the logical formula in prefix notation. Every atomic propositions, logical connectives, and logical quantifiers must be separated using a whitespace.
    /// * `command` - A `String` or `&str` that represents the command of the Triple.
    /// * `precondition` - A `String` or `&str` that represents the logical formula in prefix notation. Every atomic propositions, logical connectives, and logical quantifiers must be separated using a whitespace.
    ///
    /// # Returns
    /// A `Triple` instance representing the parsed input.
    ///
    /// # Example
    /// ```
    /// let triple1: Triple = Triple::new("= ⊤ ⊤", "x≔5", "= x 5");
    /// println!("{triple1}"); // Output: {(⊤=⊤)} x≔5 {(x=5)}
    /// ```
    pub fn new<T: Into<String>>(precondition: T, command: T, postcondition: T) -> Triple {
        Triple {
            precondition: Formula::new(precondition),
            command: command.into(),
            postcondition: Formula::new(postcondition),
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
/// Creates a new `Triple` using the Rule of Composition [1].
///
/// # Arguments
/// * `left` - The `Triple` executed first.
/// * `right` - The `Triple` executed after `left`.
///
/// # Returns
/// A `Triple` instance with the Rule of Composition applied on `left` and `right`.
///
/// # Example
/// ```
/// let triple1: Triple = Triple::new("= ⊤ ⊤", "x≔5", "= x 5");
/// let triple2: Triple = Triple::new("= x 5", "y≔x+1", "= y 6");
/// let triple3: Triple = composition_rule(&triple1, &triple2);
/// println!("{triple3}"); // Output: {(⊤=⊤)} x≔5;y≔x+1 {(y=6)}
/// ```
/// [1]: https://en.wikipedia.org/wiki/Hoare_logic#Rule_of_composition
pub fn composition_rule(left: &Triple, right: &Triple) -> Triple {
    if left.postcondition.to_string() != right.precondition.to_string() {
        panic!("The input triples do not have matching midcondition\nleft postcondition: {:?}, right precondition: {:?}",left.postcondition.to_string(), right.precondition.to_string())
    }
    Triple::new(
        left.precondition.to_prefix_notation(),
        format!("{}{}{}", left.command, "; ", right.command),
        right.postcondition.to_prefix_notation(),
    )
}

/// Creates a new `Triple` using the Condition Rule [2].
///
/// # Arguments
/// * `left` - The `Triple` with the unnegated condition. The unnegated condition must be the first value of the conjunction formula.
/// * `right` - The `Triple` with the negated condition. The negated condition must be the first value of the conjunction formula.
///
/// # Returns
/// A `Triple` instance with the Condition Rule applied on `left` and `right`.
///
/// # Example
/// ```
/// let triple1: Triple = Triple::new("= ⊤ ⊤", "x≔5", "= x 5");
/// let triple2: Triple = Triple::new("= x 5", "y≔x+1", "= y 6");
/// let triple3: Triple = composition_rule(&triple1, &triple2);
/// println!("{triple3}"); // Output: {(⊤=⊤)} x≔5;y≔x+1 {(y=6)}
/// ```
/// [2]: https://en.wikipedia.org/wiki/Hoare_logic#Conditional_rule
pub fn condition_rule(left: &Triple, right: &Triple) -> Triple {
    let negated_condition = &mut right.precondition.get_info()[1];
    negated_condition.replace_range(..3, "");
    if left.precondition.get_info()[0] != "Conjunction"
        || right.precondition.get_info()[0] != "Conjunction"
    {
        panic!("The input triples do not have `Conjunction` formulae as precondition")
    } else if left.precondition.get_info()[1] != *negated_condition {
        panic!(
            "The input triples do not matched negated {:?} and unnegated {:?} conditions",
            left.precondition.get_info()[1],
            negated_condition
        )
    } else if left.postcondition.to_string() != right.postcondition.to_string() {
        panic!(
            "The input triples do not have identical postconditions\nleft: {:?}, right: {:?}",
            left.postcondition.to_string(),
            right.postcondition.to_string()
        )
    }
    Triple::new(
        format!("{}", left.precondition.get_info()[2]),
        format!(
            "if {} then {} else {} endif",
            left.precondition.get_info()[1],
            left.command,
            right.command,
        ),
        left.postcondition.to_prefix_notation(),
    )
}

/// Creates a new `Triple` using the Consequence Rule [3].
///
/// # Arguments
/// * `left` - The `Formula` that strengthen/weaken the precondition.
/// * `middle` - The `Triple` which the Consequence Rule is applied on.
/// * `right` - The `Formula` that strengthen/weaken the postcondition.
///
/// # Returns
/// A `Triple` instance with the Consequence Rule applied according to `left` and `right`.
///
/// # Example
/// ```
/// println!("Hello, World!");
/// ```
/// [3]: https://en.wikipedia.org/wiki/Hoare_logic#Consequence_rule
pub fn consequence_rule(left: &Formula, middle: &Triple, right: &Formula) -> Triple {
    if left.get_info()[2] != middle.precondition.to_prefix_notation() {
        panic!(
            "The left `Formula` {:?} does not match middle `Triple` {:?}",
            left.get_info()[2],
            middle.precondition.to_prefix_notation()
        );
    } else if right.get_info()[1] != middle.postcondition.to_prefix_notation() {
        panic!(
            "The right `Formula` {:?} does not match middle `Triple` {:?}",
            right.get_info()[1],
            middle.postcondition.to_prefix_notation()
        );
    }
    Triple::new(
        format!("{}", left.get_info()[1]),
        format!("{}", middle.command),
        format!("{}", right.get_info()[2]),
    )
}
/// Creates a new `Triple` using the While Rule [4].
///
/// # Arguments
/// * `input` - The `Triple` contains the loop invariant and loop condition.
///
/// # Returns
/// A `Triple` instance with the While Rule applied to the `input`.
///
/// # Example
/// ```
/// println!("Hello, World!");
/// ```
/// [4]: https://en.wikipedia.org/wiki/Hoare_logic#While_rule
pub fn while_rule(input: &Triple) -> Triple {
    if input.precondition.get_info()[1] != input.postcondition.to_prefix_notation() {
        panic!(
            "The loop invariant is not preserved\nprecondition: {}, postcondition: {}",
            input.precondition.to_string(),
            input.postcondition.to_string()
        )
    }
    Triple::new(
        input.postcondition.to_prefix_notation(),
        format!(
            "while {} do {} done",
            Formula::new(&input.precondition.get_info()[2]).to_string(),
            input.command
        ),
        format!(
            "∧ ¬ {} {}",
            input.precondition.get_info()[2],
            input.postcondition.to_prefix_notation()
        ),
    )
}

fn main() {
    let comp1: Triple = Triple::new("P", "S", "Q");
    let comp2: Triple = Triple::new("Q", "T", "R");
    let cond1: Triple = Triple::new("∧ B P", "S", "Q");
    let cond2: Triple = Triple::new("∧ ¬ B P", "T", "Q");
    let whil: Triple = Triple::new("∧ B P", "S", "P");
    println!("{}", composition_rule(&comp1, &comp2));
    println!("{}", condition_rule(&cond1, &cond2));
    println!(
        "{}",
        consequence_rule(
            &Formula::new("→ P1 P2"),
            &Triple::new("P2", "S", "Q2"),
            &Formula::new("→ Q2 Q1")
        )
    );
    println!("{}", while_rule(&whil));
}
