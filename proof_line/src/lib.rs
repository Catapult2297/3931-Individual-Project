//! # Interface for the `first_order` and `hoare_triple` libraries
//!
//! This module provides an interface for working with logical formulae and Hoare Logic through the `first_order`
//! and `hoare_triple` crates. It defines the `ProofLine` enum, which serves as a versatile container for
//! representing elements of a proof, allowing users to manipulate proofs more easily.
//!
//! The `ProofLine` enum can hold either a logical formula from the `first_order` crate or a Hoare triple from
//! the `hoare_triple` crate. This design enables users to create a vector of `ProofLine` instances, facilitating
//! the construction and manipulation of proofs in a structured manner.
//!
//! # Usage
//! Users can create a vector of `ProofLine` instances to represent a sequence of proof steps, making it easier
//! to manage and manipulate logical arguments and Hoare triples in their programs.
use backtrace::{Backtrace, BacktraceFrame, BacktraceSymbol};
use first_order::Formula;
use hoare_triple::Triple;
use std::fmt;

/// An enum that holds either a Formula or a Triple.
/// This enum is designed to facilitate the manipulation of proofs by allowing users to store
/// different types of proof elements in a single collection. Users can refer to the `first_order`
/// crate for the definition of `Formula` and the `hoare_triple` crate for the definition of `Triple`.
#[derive(Debug, PartialEq)]
pub enum ProofLine {
    /// A `Formula` type in `first_order`,
    Formula(Formula),
    /// A `Triple` type in `hoare_triple`
    Triple(Triple),
}

/// Returns a string representation of the current trace location.
///
/// This function is used internally to provide context in panic messages.
fn trace() -> String {
    let level: usize = 1;
    let (trace, current_file, current_line) = (Backtrace::new(), file!(), line!());
    let frames: &[BacktraceFrame] = trace.frames();

    let symbol = frames
        .iter()
        .flat_map(BacktraceFrame::symbols)
        .skip_while(|s| {
            s.filename()
                .map(|p| !p.ends_with(current_file))
                .unwrap_or(true)
                || s.lineno() != Some(current_line)
        })
        .nth(1 + level as usize)
        .cloned();
    format!(
        "{:?}:{}",
        symbol.as_ref().and_then(BacktraceSymbol::filename).unwrap(),
        symbol.as_ref().and_then(BacktraceSymbol::lineno).unwrap()
    )
}

impl ProofLine {
    /// An interface for creating a `ProofLine` from applying a rule on a `Triple`.
    ///
    /// # Arguments
    /// * `result` - A `result` type from applying a rule from `hoare_triple` crate, which can either be:
    ///   - `Ok(Triple)`: A successful application of the rule, resulting in a `Triple`.
    ///   - `Err(String)`: An error message indicating the failure of the rule application.
    ///
    /// # Panics
    /// The function will panic if the `Result` is an `Err` type. The panic message will include the error
    /// message from the rule and point to the location in the code where the error occurred.
    ///
    /// # Example
    /// ```
    /// use hoare_triple::{Triple, composition_rule};
    /// use proof_line::ProofLine;
    ///
    /// let triple1: Triple = Triple::new("= x+1 43", "y≔x+1", "= y 43");
    /// let triple2: Triple = Triple::new("= y 43", "z≔y", "= z 43");
    /// let test_proofline = ProofLine::new_triple_from_rule(composition_rule(&triple1, &triple2));
    /// let result = ProofLine::Triple(Triple::new("= x+1 43", "y≔x+1;z≔y", "= z 43"));
    /// assert_eq!(test_proofline, result)
    /// ```
    pub fn new_triple_from_rule(result: Result<Triple, String>) -> Self {
        match result {
            Ok(triple) => Self::Triple(triple),
            Err(err) => {
                panic!("Error at {}.\n{err}", trace())
            }
        }
    }
    /// A function to return a reference of a `Formula` from a `ProofLine::Formula` instance.
    ///
    /// # Panics
    /// The function will panic if it is called on type `ProofLine::Triple`. The panic message will
    /// include an error message and point to the location in the code where the error occurred.
    ///
    /// # Example
    /// ```
    /// use first_order::Formula;
    /// use proof_line::ProofLine;
    ///
    /// let test_proofline: ProofLine = ProofLine::Formula(Formula::new(
    ///     "∧ ∀ x → P(x) ∧ Q(x) ∃ y ∨ R(y) S(y) = ¬ T(x) < U V",
    /// ));
    /// let result: Formula = Formula::new("∧ ∀ x → P(x) ∧ Q(x) ∃ y ∨ R(y) S(y) = ¬ T(x) < U V");
    /// assert_eq!(*test_proofline.get_formula(), result); // Compare dereferenced Formula
    /// ```
    pub fn get_formula(&self) -> &Formula {
        match self {
            ProofLine::Formula(formula) => &formula,
            _ => panic!(
                "Error at {}.\nAttempt to access Formula from a non-Formula ProofLine",
                trace()
            ),
        }
    }
    /// A function to return a reference of a `Triple` from a `ProofLine::Triple` instance.
    ///
    /// # Panics
    /// The function will panic if it is called on type `ProofLine::TriFormulaple`. The panic message will
    /// include an error message and point to the location in the code where the error occurred.
    ///
    /// # Example
    /// ```
    /// use hoare_triple::Triple;
    /// use proof_line::ProofLine;
    ///
    /// let test_proofline: ProofLine = ProofLine::Triple(Triple::new("= y 43", "z≔y", "= z 43"));
    /// let result: Triple = Triple::new("= y 43", "z≔y", "= z 43");
    /// assert_eq!(*test_proofline.get_triple(), result); // Compare dereferenced Triple
    /// ```
    pub fn get_triple(&self) -> &Triple {
        match self {
            ProofLine::Triple(triple) => &triple,
            _ => panic!(
                "Error at {}.\nAttempt to access Triple from a non-Triple ProofLine",
                trace()
            ),
        }
    }
}

impl fmt::Display for ProofLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProofLine::Formula(formula) => write!(f, "{}", formula),
            ProofLine::Triple(triple) => write!(f, "{}", triple),
        }
    }
}
