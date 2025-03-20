//! Main example function that implement the example in the paper
#![allow(dead_code)]
#![warn(missing_docs)]

//use backtrace::{Backtrace, BacktraceFrame, BacktraceSymbol};
use first_order::Formula;
use hoare_triple::{composition_rule, consequence_rule, while_rule, Triple};
use proof_line::ProofLine;
fn main() {
    let mut proof: Vec<ProofLine> = vec![];
    //line 0
    proof.push(ProofLine::Formula(Formula::new("→ ⊤ = x x+y*0")));
    //line 1
    proof.push(ProofLine::Triple(Triple::new(
        "= x x+y*0",
        "r≔x",
        "= x r+y*0",
    )));
    //line 2
    proof.push(ProofLine::Triple(Triple::new(
        "= x r+y*0",
        "q≔0",
        "= x r+y*q",
    )));
    //line 3
    proof.push(ProofLine::new_triple_from_rule(consequence_rule(
        proof[0].get_formula(),
        proof[1].get_triple(),
        &Formula::new("→ = x r+y*0 = x r+y*0"),
    )));
    //line 4
    proof.push(ProofLine::new_triple_from_rule(composition_rule(
        &proof[3].get_triple(),
        &proof[2].get_triple(),
    )));
    //line 5
    proof.push(ProofLine::Formula(Formula::new(
        "→ ∧ = x r+y*q ∨ < y r = y r = x (r-y)+y*(1+q)",
    )));
    //line 6
    proof.push(ProofLine::Triple(Triple::new(
        "= x (r-y)+y*(1+q)",
        "r≔r-y",
        "= x r+y*(1+q)",
    )));
    //line 7
    proof.push(ProofLine::Triple(Triple::new(
        "= x r+y*(1+q)",
        "q≔1+q",
        "= x r+y*q",
    )));
    //line 8
    proof.push(ProofLine::new_triple_from_rule(composition_rule(
        &proof[6].get_triple(),
        &proof[7].get_triple(),
    )));
    //line 9
    proof.push(ProofLine::new_triple_from_rule(consequence_rule(
        &proof[5].get_formula(),
        &proof[8].get_triple(),
        &Formula::new("→ = x r+y*q = x r+y*q"),
    )));
    //line 10
    proof.push(ProofLine::Formula(Formula::new(
        "→ ∧ = x r+y*q ¬ ∨ < y r = y r ∧ ¬ ∨ < y r = y r = x r+y*q",
    )));
    //line 11
    proof.push(ProofLine::new_triple_from_rule(while_rule(
        &proof[9].get_triple(),
    )));
    //line 12
    proof.push(ProofLine::new_triple_from_rule(composition_rule(
        &proof[4].get_triple(),
        &proof[11].get_triple(),
    )));

    //output
    for (line_number, line) in proof.iter().enumerate() {
        println!("{line_number:<4}   {line}");
    }
}
