//! Fibonacci Algorithm
//! Reference Code
//! ```
//! fn fibonacci(input: u32) -> u32 {
//!     let mut preFib: u32 = 0;
//!     let mut curFib: u32 = 1;
//!     let mut index: u32 = 1;
//!     let N: u32 = input;
//!     while index < N {
//!         curFib = preFib + curFib;
//!         preFib = curFib - preFib;
//!         index += 1;
//!     }
//!     curFib
//! }
//! ```
use first_order::Formula;
use hoare_triple::{Triple, composition_rule, consequence_rule, while_rule};
use proof_line::ProofLine;

fn main() {
    let mut proof: Vec<ProofLine> = vec![];
    //proof[0] assignment
    proof.push(ProofLine::Triple(Triple::new(
        "∧ ∧ ∧ ∧ = preFib+curFib fib(index+1) = preFib+curFib-preFib fib(index) < 0 index+1 ∨ < index+1 N = index+1 N = N input",
        "curFib≔preFib+curFib",
        "∧ ∧ ∧ ∧ = curFib fib(index+1) = curFib-preFib fib(index) < 0 index+1 ∨ < index+1 N = index+1 N = N input",
    )));

    //proof[1] lemma 0
    proof.push(ProofLine::Formula(Formula::new(format!(
        "→ ∧ ∧ ∧ ∧ ∧ = curFib fib(index) = preFib fib(index-1) < 0 index ∨ < index N = index N = N input < index N {}",
        proof[0].get_triple().precondition.to_prefix_notation()
    ))));

    //proof[2] lemma 1
    proof.push(ProofLine::Formula(Formula::new(format!(
        "→ {} {}",
        proof[0].get_triple().postcondition.to_prefix_notation(),
        proof[0].get_triple().postcondition.to_prefix_notation()
    ))));

    //proof[3] consequence rule with proof[1] proof[0] proof[3]
    proof.push(ProofLine::new_triple_from_rule(consequence_rule(
        proof[1].get_formula(),
        proof[0].get_triple(),
        proof[2].get_formula(),
    )));

    //proof[4] assignment
    proof.push(ProofLine::Triple(Triple::new(
        "∧ ∧ ∧ ∧ = curFib fib(index+1) = curFib-preFib fib(index) < 0 index+1 ∨ < index+1 N = index+1 N = N input",
        "preFib≔curFib+preFib",
        "∧ ∧ ∧ ∧ = curFib fib(index+1) = preFib fib(index) < 0 index+1 ∨ < index+1 N = index+1 N = N input",
    )));

    //proof[5] lemma 2
    proof.push(ProofLine::Formula(Formula::new(format!(
        "→ {} {}",
        proof[4].get_triple().precondition.to_prefix_notation(),
        proof[4].get_triple().precondition.to_prefix_notation()
    ))));

    //proof[6] lemma 3
    proof.push(ProofLine::Formula(Formula::new(format!(
        "→ {} ∧ ∧ ∧ ∧ = curFib fib(index+1) = preFib fib(index+1-1) < 0 index+1 ∨ < index+1 N = index+1 N = N input",
        proof[4].get_triple().postcondition.to_prefix_notation()
    ))));

    //proof[7] consequence rule with proof[5](lemma 2) proof[4] proof[6](lemma 3)
    proof.push(ProofLine::new_triple_from_rule(consequence_rule(
        proof[5].get_formula(),
        proof[4].get_triple(),
        proof[6].get_formula(),
    )));

    //proof[8] assignment
    proof.push(ProofLine::Triple(Triple::new(
        "∧ ∧ ∧ ∧ = curFib fib(index+1) = preFib fib(index+1-1) < 0 index+1 ∨ < index+1 N = index+1 N = N input",
        "index≔index+1",
        "∧ ∧ ∧ ∧ = curFib fib(index) = preFib fib(index-1) < 0 index ∨ < index N = index N = N input",
    )));

    //proof[9] composition proof[3] proof[7]
    proof.push(ProofLine::new_triple_from_rule(composition_rule(
        proof[3].get_triple(),
        proof[7].get_triple(),
    )));

    //proof[10] composition proof[7] proof[8]
    proof.push(ProofLine::new_triple_from_rule(composition_rule(
        proof[9].get_triple(),
        proof[8].get_triple(),
    )));

    //proof[11] while proof[10]
    proof.push(ProofLine::new_triple_from_rule(while_rule(
        proof[10].get_triple(),
    )));

    //proof[12] lemma 4
    proof.push(ProofLine::Formula(Formula::new(format!(
        "→ ∧ ∧ ∧ ∧ = preFib 0 = curFib 1 = index 1 ∨ < 1 N = 1 N = N input {}",
        proof[11].get_triple().precondition.to_prefix_notation()
    ))));

    //proof[13] lemma 5
    proof.push(ProofLine::Formula(Formula::new(format!(
        "→ {} = curFib fib(input)",
        proof[11].get_triple().postcondition.to_prefix_notation()
    ))));

    //proof[14] consequence rule with proof[12](lemma 4) proof[11] proof[13](lemma 5)
    proof.push(ProofLine::new_triple_from_rule(consequence_rule(
        proof[12].get_formula(),
        proof[11].get_triple(),
        proof[13].get_formula(),
    )));

    for (line_num, line) in proof.iter().enumerate() {
        println!("{line_num} {line}")
    }
}
