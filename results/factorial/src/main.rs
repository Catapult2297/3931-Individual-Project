use first_order::Formula;
use hoare_triple::{Triple, composition_rule, consequence_rule, while_rule};
//use hoare_triple::{condition_rule, Triple};
use proof_line::ProofLine;

fn main() {
    let mut proof: Vec<ProofLine> = vec![];

    proof.push(ProofLine::Triple(Triple::new(
        "∧ = (result*count)*fact(count-1) fact(x) ∨ < 0 (count-1) = 0 (count-1)",
        "result≔result*count",
        "∧ = result*fact(count-1) fact(x) ∨ < 0 (count-1) = 0 (count-1)",
    )));
    proof.push(ProofLine::Triple(Triple::new(
        "∧ = result*fact(count-1) fact(x) ∨ < 0 (count-1) = 0 (count-1)",
        "count≔count-1",
        "∧ = result*fact(count) fact(x) ∨ < 0 count = 0 count",
    )));

    proof.push(ProofLine::new_triple_from_rule(composition_rule(
        &proof[0].get_triple(),
        &proof[1].get_triple(),
    )));
    proof.push(ProofLine::Formula(Formula::new("→ ∧ ∧ = result*fact(count) fact(x) ∨ < 0 count = 0 count ¬ = 0 count ∧ = (result*count)*fact(count-1) fact(x) ∨ < 0 (count-1) = 0 (count-1)")));

    proof.push(ProofLine::new_triple_from_rule(consequence_rule(
        &proof[3].get_formula(),
        &proof[2].get_triple(),
        &Formula::new(format!(
            "→ {} {}",
            &proof[2].get_triple().postcondition.to_prefix_notation(),
            &proof[2].get_triple().postcondition.to_prefix_notation()
        )),
    )));

    proof.push(ProofLine::new_triple_from_rule(while_rule(
        proof[4].get_triple(),
    )));

    proof.push(ProofLine::Formula(Formula::new("→ ∧ ∧ = count x ∨ < 0 count = 0 count = result 1 ∧ = result*fact(count) fact(x) ∨ < 0 count = 0 count")));
    proof.push(ProofLine::Formula(Formula::new(
        "→ ∧ ¬ ¬ = 0 count ∧ = result*fact(count) fact(x) ∨ < 0 count = 0 count = result fact(x)",
    )));

    proof.push(ProofLine::new_triple_from_rule(consequence_rule(
        &proof[6].get_formula(),
        &proof[5].get_triple(),
        &proof[7].get_formula(),
    )));

    for (line_number, line) in proof.iter().enumerate() {
        println!("{line_number} {line}\n");
    }
}
