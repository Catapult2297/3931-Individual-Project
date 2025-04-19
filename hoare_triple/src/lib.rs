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
use first_order::Formula;
use std::fmt;

//use crate::first_order::Formula;

/// Represents a Hoare triple, which is a formalism used in computer science to reason about the correctness
/// of computer programs.
///
/// A `Triple` consists of a precondition, a command (or program statement), and a postcondition.
/// It is typically expressed in the form `{ P } C { Q }`, where:
/// - `P` is the precondition that must hold true before executing the command `C`.
/// - `C` is the command or program statement being executed.
/// - `Q` is the postcondition that must hold true after executing the command, assuming the precondition was true.
///
/// # Fields
/// * `precondition` - A `Formula` representing the condition before executing the command.
/// * `command` - A `String` representing the command or program statement to be executed.
/// * `postcondition` - A `Formula` representing the condition after executing the command,
///   given that the precondition was true.
///
/// # Example
/// ```
/// use first_order::Formula;
/// use hoare_triple::Triple;
///
/// let precondition: Formula = Formula::new("P");
/// let command: String = "x≔x+1".to_string();
/// let postcondition = Formula::new("P");
///
/// let triple: Triple = Triple {
///     precondition,
///     command,
///     postcondition,
/// };
/// ```
#[derive(Debug, PartialEq)]
pub struct Triple {
    /// A `Formula` representing the precondition before executing the command.
    pub precondition: Formula,
    /// A `String` representing the command or program statement to be executed.
    pub command: String,
    /// A `Formula` representing the postcondition after executing the command.
    pub postcondition: Formula,
}

impl Triple {
    /// Creates a new `Triple` from three string input.
    ///
    /// # Arguments
    /// * `precondition` - A `String` or `&str` that represents the logical formula in prefix notation.
    ///   Every atomic propositions, logical connectives, and logical quantifiers must be separated using a whitespace.
    /// * `command` - A `String` or `&str` that represents the command of the Triple.
    /// * `postcondition` - A `String` or `&str` that represents the logical formula in prefix notation.
    ///   Similar to `precondition`, every atomic propositions, logical connectives, and logical quantifiers must be separated using a whitespace.
    ///
    /// # Type Requirements
    /// All arguments must be of the same type `T` (either `String` or `&str`). The function will convert them into `String` to ensure consistency. Ensure that the types match to avoid compilation errors.
    ///
    /// # Returns
    /// A `Triple` instance representing the parsed input.
    ///
    /// # Example
    /// ```
    /// use first_order::Formula;
    /// use hoare_triple::Triple;
    ///
    /// let test_triple: Triple = Triple::new(
    ///     "∧ ∀ x → P(x) ∧ Q(x) ∃ y ∨ R(y) S(y) = ¬ T(x) < U V",
    ///     "x≔z",
    ///     "∧ ∀ z → P(z) ∧ Q(z) ∃ y ∨ R(y) S(y) = ¬ T(z) < U V",
    /// );
    /// let result: Triple = Triple {
    ///     precondition: Formula::new("∧ ∀ x → P(x) ∧ Q(x) ∃ y ∨ R(y) S(y) = ¬ T(x) < U V"),
    ///     command: "x≔z".to_string(),
    ///     postcondition: Formula::new("∧ ∀ z → P(z) ∧ Q(z) ∃ y ∨ R(y) S(y) = ¬ T(z) < U V"),
    /// };
    /// assert_eq!(test_triple, result);
    /// ```
    /// # Note
    /// Ensure that the input strings are formatted correctly to avoid potential parsing errors.
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
/// This function applies the Rule of Composition to two `Triple` instances, `left` and `right`,
/// by combining them according to the Rule of Composition. The precondition of the `right` `Triple`
/// must match the postcondition of the `left` `Triple`, which is referred to as the midcondition.
///
/// # Arguments
/// * `left` - A reference to the `Triple` executed first.
/// * `right` - A reference to the `Triple` executed after `left`.
///
/// # Returns
/// A `Result` containing a `Triple` instance with the Rule of Composition applied on `left` and `right`,
/// or an error message if the midcondition does not match.
///
/// # Example
/// ```
/// use hoare_triple::{Triple, composition_rule};
///
/// let triple1: Triple = Triple::new("= x+1 43", "y≔x+1", "= y 43");
/// let triple2: Triple = Triple::new("= y 43", "z≔y", "= z 43");
/// let test_triple: Triple = composition_rule(&triple1, &triple2).unwrap();
/// let result: Triple = Triple::new("= x+1 43", "y≔x+1;z≔y", "= z 43");
/// assert_eq!(test_triple, result);
/// ```
/// [1]: https://en.wikipedia.org/wiki/Hoare_logic#Rule_of_composition
pub fn composition_rule(left: &Triple, right: &Triple) -> Result<Triple, String> {
    if left.postcondition.to_string() != right.precondition.to_string() {
        return Err(format!(
            "The input triples do not have matching midcondition\nleft postcondition: {:?}\n right precondition: {:?}",
            left.postcondition.to_prefix_notation(),
            right.precondition.to_prefix_notation()
        ));
    }
    Ok(Triple::new(
        left.precondition.to_prefix_notation(),
        format!("{}{}{}", left.command, ";", right.command),
        right.postcondition.to_prefix_notation(),
    ))
}

/// Creates a new `Triple` using the Condition Rule [2].
///
/// This function applies the Condition Rule to two `Triple` instances, `left` and `right`.
/// The `left` `Triple` must have an unnegated condition as the first value of its conjunction formula,
/// while the `right` `Triple` must have the corresponding negated condition as the first value of its conjunction formula.
///
/// # Arguments
/// * `left` - A reference to the `Triple` with the unnegated condition. The unnegated condition must be the first value of the conjunction formula.
/// * `right` - A reference to the `Triple` with the negated condition. The negated condition must be the first value of the conjunction formula.
///
/// # Returns
/// A `Result` containing a `Triple` instance with the Condition Rule applied on `left` and `right`,
/// or an error message if the input is malformed (e.g., if the preconditions are not of the expected type).
///
/// # Example
/// ```
/// use hoare_triple::{Triple, condition_rule};
///
/// let triple1: Triple = Triple::new("∧ B P", "S", "Q");
/// let triple2: Triple = Triple::new("∧ ¬ B P", "T", "Q");
/// let test_triple: Triple = condition_rule(&triple1, &triple2).unwrap();
/// let result = Triple::new("P", "if B then S else T endif", "Q");
/// assert_eq!(test_triple, result);
/// ```
/// [2]: https://en.wikipedia.org/wiki/Hoare_logic#Conditional_rule
pub fn condition_rule(left: &Triple, right: &Triple) -> Result<Triple, String> {
    let negated_condition = &mut right.precondition.get_info()[1];
    negated_condition.replace_range(..3, "");
    if left.precondition.get_info()[0] != "Conjunction"
        || right.precondition.get_info()[0] != "Conjunction"
    {
        return Err(
            "The input triples do not have `Conjunction` formulae as precondition".to_string(),
        );
    } else if left.precondition.get_info()[1] != *negated_condition {
        return Err(format!(
            "The input triples do not match\nnegated {:?}\nunnegated {:?} conditions",
            left.precondition.get_info()[1],
            negated_condition
        ));
    } else if left.postcondition != right.postcondition {
        return Err(format!(
            "The input triples do not have identical postconditions\nleft: {:?}\nright: {:?}",
            left.postcondition.to_prefix_notation(),
            right.postcondition.to_prefix_notation()
        ));
    }
    Ok(Triple::new(
        format!("{}", left.precondition.get_info()[2]),
        format!(
            "if {} then {} else {} endif",
            left.precondition.get_info()[1],
            left.command,
            right.command,
        ),
        left.postcondition.to_string(),
    ))
}

/// Creates a new `Triple` using the Consequence Rule [3].
///
/// This function applies the Consequence Rule to a `Triple` instances, `middle`, using the `left` and `right` `Formula`.
/// The `left` and `right` `Formula` must be type `Formula::Implication`, which strengthens or weakens the precondition and postcondition, respectively.
///
/// # Arguments
/// * `left` - A reference to the `Formula` that strengthens or weakens the precondition.
/// * `middle` - A reference to the `Triple` which the Consequence Rule is applied on.
/// * `right` - A reference to the `Formula` that strengthens or weakens the postcondition.
///
/// # Returns
/// A `Result` containing a `Triple` instance with the Consequence Rule applied on `middle` using the `left` and `right` `Formula`,
/// or a `String` error message if the input is malformed (e.g., if the `Formula` are not of the expected type).
///
/// # Example
/// ```
/// use first_order::Formula;
/// use hoare_triple::{Triple, consequence_rule};
///
/// let formula1: Formula = Formula::new("→ P1 P2");
/// let formula2: Formula = Formula::new("→ Q2 Q1");
/// let triple1: Triple = Triple::new("P2", "S", "Q2");
/// let test_triple: Triple = consequence_rule(&formula1, &triple1, &formula2).unwrap();
/// let result: Triple = Triple::new("P1", "S", "Q1");
/// assert_eq!(test_triple, result);
/// ```
/// [3]: https://en.wikipedia.org/wiki/Hoare_logic#Consequence_rule
pub fn consequence_rule(
    left: &Formula,
    middle: &Triple,
    right: &Formula,
) -> Result<Triple, String> {
    if left.get_info()[0] != "Implication" {
        return Err(format!(
            "The left `Formula` {:?} is not an Implication type Formula. Left type: {:?}",
            left.to_prefix_notation(),
            left.get_info()[0]
        ));
    } else if right.get_info()[0] != "Implication" {
        return Err(format!(
            "The right `Formula` {:?} is not an Implication type Formula. Right type: {:?}",
            right.to_prefix_notation(),
            right.get_info()[0]
        ));
    } else if left.get_info()[2] != middle.precondition.to_prefix_notation() {
        return Err(format!(
            "The left `Formula` {:?} does not match the precondition of the middle `Triple` {:?}",
            left.to_prefix_notation(),
            middle.precondition.to_prefix_notation()
        ));
    } else if right.get_info()[1] != middle.postcondition.to_prefix_notation() {
        return Err(format!(
            "The right `Formula` {:?} does not match the postcondition of the middle `Triple` {:?}",
            right.to_prefix_notation(),
            middle.postcondition.to_prefix_notation()
        ));
    }
    Ok(Triple::new(
        format!("{}", left.get_info()[1]),
        format!("{}", middle.command),
        format!("{}", right.get_info()[2]),
    ))
}

/// Creates a new `Triple` using the While Rule [4].
///
/// # Arguments
/// * `input` -  A reference to the `Triple` that contains the loop invariant and loop condition.
///
/// # Returns
/// A `Result` containing a `Triple` instance with the While Rule applied on `input`,
/// or a `String` error message if the input is malformed (e.g., if the loop invariant is not conserved).
///
/// # Example
/// ```
/// use hoare_triple::{Triple, while_rule};
///
/// let triple1: Triple = Triple::new("∧ P B", "S", "P");
/// let test_triple: Triple = while_rule(&triple1).unwrap();
/// let result: Triple = Triple::new("P", "while B do S done", "∧ ¬ B P");
/// assert_eq!(test_triple, result);
/// ```
/// [4]: https://en.wikipedia.org/wiki/Hoare_logic#While_rule
pub fn while_rule(input: &Triple) -> Result<Triple, String> {
    if input.precondition.get_info()[1] != input.postcondition.to_prefix_notation() {
        return Err(format!(
            "The loop invariant is not preserved\nprecondition (P∧B): {:?}, postcondition (P): {:?}",
            Formula::new(&input.precondition.get_info()[1]).to_prefix_notation(),
            input.postcondition.to_prefix_notation()
        ));
    }
    Ok(Triple::new(
        input.postcondition.to_prefix_notation(),
        format!(
            "while {} do {} done",
            Formula::new(&input.precondition.get_info()[2]),
            input.command
        ),
        format!(
            "∧ ¬ {} {}",
            input.precondition.get_info()[2],
            input.postcondition.to_prefix_notation()
        ),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use first_order::Formula;
    use std::panic;

    #[test]
    fn test_basic_valid_input() {
        let test_triple = Triple::new("A", "B", "C");
        let expected_triple = Triple {
            precondition: Formula::new("A"),
            command: "B".to_string(),
            postcondition: Formula::new("C"),
        };
        assert_eq!(test_triple, expected_triple);
    }

    #[test]
    fn test_empty_strings() {
        let result = panic::catch_unwind(|| {
            Triple::new("", "", "");
        });

        assert!(result.is_err());
    }

    #[test]
    fn test_whitespace_handling() {
        let result = panic::catch_unwind(|| {
            Triple::new("   ", "   ", "   ");
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_special_characters() {
        let test_triple = Triple::new("∧ ∀ x x y", "x≔z", "∧ ∀ z z y");
        let expected_triple = Triple {
            precondition: Formula::new("∧ ∀ x x y"),
            command: "x≔z".to_string(),
            postcondition: Formula::new("∧ ∀ z z y"),
        };
        assert_eq!(test_triple, expected_triple);
    }

    #[test]
    fn test_long_strings() {
        let long_precondition = "A".repeat(1000);
        let long_command = "B".repeat(1000);
        let long_postcondition = "C".repeat(1000);
        let test_triple = Triple::new(&long_precondition, &long_command, &long_postcondition);
        let expected_triple = Triple {
            precondition: Formula::new(&long_precondition),
            command: long_command.clone(),
            postcondition: Formula::new(&long_postcondition),
        };
        assert_eq!(test_triple, expected_triple);
    }

    #[test]
    fn test_identical_pre_post_conditions() {
        let test_triple = Triple::new("X", "Y", "X");
        let expected_triple = Triple {
            precondition: Formula::new("X"),
            command: "Y".to_string(),
            postcondition: Formula::new("X"),
        };
        assert_eq!(test_triple, expected_triple);
    }

    #[test]
    fn test_complex_formulas() {
        let test_triple = Triple::new(
            "∧ ∀ x → P(x) ∧ Q(x) ∃ y ∨ R(y) S(y) = ¬ T(x) < U V",
            "x≔z",
            "∧ ∀ z → P(z) ∧ Q(z) ∃ y ∨ R(y) S(y) = ¬ T(z) < U V",
        );
        let expected_triple = Triple {
            precondition: Formula::new("∧ ∀ x → P(x) ∧ Q(x) ∃ y ∨ R(y) S(y) = ¬ T(x) < U V"),
            command: "x≔z".to_string(),
            postcondition: Formula::new("∧ ∀ z → P(z) ∧ Q(z) ∃ y ∨ R(y) S(y) = ¬ T(z) < U V"),
        };
        assert_eq!(test_triple, expected_triple);
    }

    #[test]
    fn consequence_test() {
        let triple1: Triple = Triple::new("= x+1 43", "y≔x+1", "= y 43");
        let triple2: Triple = Triple::new("= y 43", "z≔y", "= z 43");
        let test_triple: Triple = composition_rule(&triple1, &triple2).unwrap();
        let result: Triple = Triple::new("= x+1 43", "y≔x+1;z≔y", "= z 43");
        assert_eq!(test_triple, result);
    }

    #[test]
    fn consequence_multiple_triples() {
        let triple1: Triple = Triple::new("= x+1 43", "y≔x+1", "= y 43");
        let triple2: Triple = Triple::new("= y 43", "z≔y", "= z 43");
        let triple3: Triple = Triple::new("= z 43", "w≔z", "= w 43");

        let intermediate: Triple = composition_rule(&triple1, &triple2).unwrap();
        let final_triple: Triple = composition_rule(&intermediate, &triple3).unwrap();

        let expected: Triple = Triple::new("= x+1 43", "y≔x+1;z≔y;w≔z", "= w 43");
        assert_eq!(final_triple, expected);
    }

    #[test]
    fn consequence_no_composition() {
        let triple1: Triple = Triple::new("= x+1 43", "y≔x+1", "= y 43");
        let triple2: Triple = Triple::new("= z 44", "w≔z", "= w 44");

        let result = composition_rule(&triple1, &triple2);
        assert!(result.is_err());
    }

    #[test]
    fn consequence_identity_triple() {
        let triple1: Triple = Triple::new("= x+1 43", "y≔x+1", "= y 43");
        let identity: Triple = Triple::new("= y 43", "y≔y", "= y 43");

        let test_triple: Triple = composition_rule(&triple1, &identity).unwrap();
        let expected: Triple = Triple::new("= x+1 43", "y≔x+1;y≔y", "= y 43");
        assert_eq!(test_triple, expected);
    }

    #[test]
    fn consequence_complex_formulas() {
        let triple1: Triple = Triple::new("= 2*x + 1 43", "y≔2*x+1", "= y 43");
        let triple2: Triple = Triple::new("= y 43", "z≔y", "= z 43");

        let test_triple: Triple = composition_rule(&triple1, &triple2).unwrap();
        let expected: Triple = Triple::new("= 2*x + 1 43", "y≔2*x+1;z≔y", "= z 43");
        assert_eq!(test_triple, expected);
    }

    #[test]
    fn test_condition_rule_valid() {
        let triple1 = Triple::new("∧ B P", "S", "Q");
        let triple2 = Triple::new("∧ ¬ B P", "T", "Q");
        let result = condition_rule(&triple1, &triple2).unwrap();
        let expected = Triple::new("P", "if B then S else T endif", "Q");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_condition_rule_invalid_conjunction() {
        let triple1 = Triple::new("P", "S", "Q"); // Not a conjunction
        let triple2 = Triple::new("∧ ¬ B P", "T", "Q");
        let result = condition_rule(&triple1, &triple2);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            "The input triples do not have `Conjunction` formulae as precondition".to_string()
        );
    }

    #[test]
    fn test_condition_rule_mismatched_conditions() {
        let triple1 = Triple::new("∧ B P", "S", "Q");
        let triple2 = Triple::new("∧ ¬ C P", "T", "Q"); // Mismatched negated condition
        let result = condition_rule(&triple1, &triple2);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            "The input triples do not match negated \"B\" and unnegated \"C\" conditions"
                .to_string()
        );
    }

    #[test]
    fn test_condition_rule_different_postconditions() {
        let triple1 = Triple::new("∧ B P", "S", "Q1");
        let triple2 = Triple::new("∧ ¬ B P", "T", "Q2"); // Different postconditions
        let result = condition_rule(&triple1, &triple2);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            "The input triples do not have identical postconditions\nleft: Q1, right: Q2"
                .to_string()
        );
    }

    #[test]
    fn test_condition_rule_empty_conditions() {
        let triple1 = Triple::new("∧ B P", "S", "Q");
        let triple2 = Triple::new("∧ ¬ B P", "", "Q"); // Empty command in right triple
        let result = condition_rule(&triple1, &triple2).unwrap();
        let expected = Triple::new("P", "if B then S else  endif", "Q");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_consequence_rule_valid() {
        let formula1 = Formula::new("→ P1 P2");
        let formula2 = Formula::new("→ Q2 Q1");
        let triple1 = Triple::new("P2", "S", "Q2");
        let result = consequence_rule(&formula1, &triple1, &formula2).unwrap();
        let expected = Triple::new("P1", "S", "Q1");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_consequence_rule_invalid_left_formula() {
        let formula1 = Formula::new("P1"); // Not an implication
        let formula2 = Formula::new("→ Q2 Q1");
        let triple1 = Triple::new("P2", "S", "Q2");
        let result = consequence_rule(&formula1, &triple1, &formula2);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            "The left `Formula` \"P1\" is not an Implication type Formula. Left type: \"Term\""
                .to_string()
        );
    }

    #[test]
    fn test_consequence_rule_invalid_right_formula() {
        let formula1 = Formula::new("→ P1 P2");
        let formula2 = Formula::new("Q1"); // Not an implication
        let triple1 = Triple::new("P2", "S", "Q2");
        let result = consequence_rule(&formula1, &triple1, &formula2);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            "The right `Formula` \"Q1\" is not an Implication type Formula. Right type: \"Term\""
                .to_string()
        );
    }

    #[test]
    fn test_consequence_rule_mismatched_precondition() {
        let formula1 = Formula::new("→ P1 P2");
        let formula2 = Formula::new("→ Q2 Q1");
        let triple1 = Triple::new("P3", "S", "Q2"); // Mismatched precondition
        let result = consequence_rule(&formula1, &triple1, &formula2);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "The left `Formula` \"→ P1 P2\" does not match the precondition of the middle `Triple` \"P3\"".to_string());
    }

    #[test]
    fn test_consequence_rule_mismatched_postcondition() {
        let formula1 = Formula::new("→ P1 P2");
        let formula2 = Formula::new("→ Q2 Q1");
        let triple1 = Triple::new("P2", "S", "Q3"); // Mismatched postcondition
        let result = consequence_rule(&formula1, &triple1, &formula2);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "The right `Formula` \"→ Q2 Q1\" does not match the postcondition of the middle `Triple` \"Q3\"".to_string());
    }

    #[test]
    fn test_while_rule_valid() {
        let triple1 = Triple::new("∧ P B", "S", "P");
        let result = while_rule(&triple1).unwrap();
        let expected = Triple::new("P", "while B do S done", "∧ ¬ B P");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_while_rule_invalid_invariant() {
        let triple1 = Triple::new("∧ P B", "S", "Q"); // Postcondition does not match invariant
        let result = while_rule(&triple1);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "The loop invariant is not preserved\nprecondition (P∧B): \"P\", postcondition (P): \"Q\"".to_string());
    }

    #[test]
    fn test_while_rule_empty_command() {
        let triple1 = Triple::new("∧ P B", "", "P"); // Empty command
        let result = while_rule(&triple1).unwrap();
        let expected = Triple::new("P", "while B do  done", "∧ ¬ B P");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_while_rule_different_invariant() {
        let triple1 = Triple::new("∧ P B", "S", "R"); // Different postcondition
        let result = while_rule(&triple1);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "The loop invariant is not preserved\nprecondition (P∧B): \"P\", postcondition (P): \"R\"".to_string());
    }

    #[test]
    fn test_while_rule_complex_invariant() {
        let triple1 = Triple::new("∧ ∧ A B C", "S", "∧ A B"); // Valid case with complex invariant
        let result = while_rule(&triple1).unwrap();
        let expected = Triple::new("∧ A B", "while C do S done", "∧ ¬ C ∧ A B");
        assert_eq!(result, expected);
    }
}
