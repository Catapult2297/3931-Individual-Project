use first_order::Formula;
use hoare_triple::{Triple, composition_rule, consequence_rule, while_rule};
use proof_line::ProofLine;

fn main() {
    let mut proof: Vec<ProofLine> = vec![];
    proof.push(ProofLine::Triple(Triple::new(
        "∧ ∧ ∧ ∧ = 0 prevFib = 1 currentFib = 1 index ∨ < 1 N = 1 N = N n",
        "",
        "postcondition",
    )))
}
