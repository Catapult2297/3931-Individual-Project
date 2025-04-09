use first_order::Formula;
use hoare_triple::{composition_rule, while_rule, Triple};
use proof_line::ProofLine;

///Reference Code
///
///```
///fn dec_to_bin(input: u32) -> u32 {
///    let mut input: u32 = input;
///    let mut power: u32 = 0;
///    let mut output: u32 = 0;
///    while input != 0 {
///        let remainder = input % 2;
///        input = input / 2;
///        output += remainder * (10 as u32).pow(power);
///        power += 1;
///    }
///    output
///}
/// ```
///
fn main() {
    let mut proof: Vec<ProofLine> = vec![];
    //variable assignment
    //proof[0] define remainder, relabel as r
    proof.push(ProofLine::Triple(Triple::new(
        "∧ ∨ = mod(i,2) 0 = mod(i,2) 1 ¬ = i 0",
        "r≔mod(i,2)",
        "∧ ∨ = mod(i,2) 0 = mod(i,2) 1 ¬ = i 0",
    )));
    //proof[1] define input, relabel as i
    proof.push(ProofLine::Triple(Triple::new(
        "∧ ∨ = mod(i,2) 0 = mod(i,2) 1 ¬ = i 0",
        "i≔i/2",
        "∧ ∨ = mod(i,2) 0 = mod(i,2) 1 ¬ = i 0",
    )));
    //proof[2] define output, relabel as 0
    proof.push(ProofLine::Triple(Triple::new(
        "∧ ∨ = mod(i,2) 0 = mod(i,2) 1 ¬ = i 0",
        "o≔o+r*10^(p)",
        "∧ ∨ = mod(i,2) 0 = mod(i,2) 1 ¬ = i 0",
    )));
    //proof[3] define power, relabel as p
    proof.push(ProofLine::Triple(Triple::new(
        "∧ ∨ = mod(i,2) 0 = mod(i,2) 1 ¬ = i 0",
        "p≔p+1",
        "∨ = mod(i,2) 0 = mod(i,2) 1",
    )));
    //proof[4] combine proof[0] and proof[1] using composition_rule
    proof.push(ProofLine::new_triple_from_rule(composition_rule(
        proof[0].get_triple(),
        proof[1].get_triple(),
    )));
    //proof[5] combine proof[4] and proof[2] using composition_rule
    proof.push(ProofLine::new_triple_from_rule(composition_rule(
        proof[4].get_triple(),
        proof[2].get_triple(),
    )));
    //proof[6] combine proof[5] and proof[3] using composition_rule
    proof.push(ProofLine::new_triple_from_rule(composition_rule(
        proof[5].get_triple(),
        proof[3].get_triple(),
    )));
    //proof[7] apply while_rule on proof[6]
    proof.push(ProofLine::new_triple_from_rule(while_rule(
        proof[6].get_triple(),
    )));
    for line in proof {
        println! {"{line}"};
    }
}
