//! Main example function that implement the example in the paper
#![allow(dead_code)]
#![warn(missing_docs)]
//use first_order;
//use hoare_triple;
mod first_order;
mod hoare_triple;
use first_order::Formula;
use hoare_triple::{composition_rule, consequence_rule, while_rule, Triple};

fn main() {
    let mut proof: Vec<Triple> = vec![];
    let mut proof_line: Vec<String> = vec![];
    //line 1
    let lemma1: Formula = Formula::new("→ ⊤ = x x+y*0");
    //line 2; proof[0]
    proof.push(Triple::new("= x x+y*0", "r≔x", "= x r+y*0"));
    //line 3; proof[1]
    proof.push(Triple::new("= x r+y*0", "q≔0", "= x r+y*q"));
    //line 4; proof[2]
    proof.push(consequence_rule(
        &lemma1,
        &proof[0],
        &Formula::new("→ = x r+y*0 = x r+y*0"),
    ));
    //line 5; proof[3]
    proof.push(composition_rule(&proof[2], &proof[1]));
    //line 6
    let lemma2: Formula = Formula::new("→ ∧ = x r+y*q ∨ < y r = y r = x (r-y)+y*(1+q)");
    //line 7; proof[4]
    proof.push(Triple::new("= x (r-y)+y*(1+q)", "r≔r-y", "= x r+y*(1+q)"));
    //line 8; proof[5]
    proof.push(Triple::new("= x r+y*(1+q)", "q≔1+q", "= x r+y*q"));
    //line 9; proof[6]
    proof.push(composition_rule(&proof[4], &proof[5]));
    //line 10; proof[7]
    proof.push(consequence_rule(
        &lemma2,
        &proof[6],
        &Formula::new("→ = x r+y*q = x r+y*q"),
    ));
    //line 11
    let lemma3: Formula = Formula::new("→ ∧ = x r+y*q ¬ ∨ < y r = y r ∧ ¬ ∨ < y r = y r = x r+y*q");
    //line 12; proof[8]
    proof.push(while_rule(&proof[7]));
    //line 13; proof[9]
    proof.push(composition_rule(&proof[3], &proof[8]));

    proof_line.push(lemma1.to_string());
    for i in 0..4 {
        proof_line.push(proof[i].to_string());
    }
    proof_line.push(lemma2.to_string());
    for i in 4..8 {
        proof_line.push(proof[i].to_string());
    }
    proof_line.push(lemma3.to_string());
    for i in 9..10 {
        proof_line.push(proof[i].to_string());
    }

    for line in proof_line {
        println!("{line}")
    }
}
