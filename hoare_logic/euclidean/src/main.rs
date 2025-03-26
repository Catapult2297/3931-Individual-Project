use first_order::Formula;
use hoare_triple::{composition_rule, while_rule, Triple};
use proof_line::ProofLine;
/// Reference Code:
///```
/// fn gcd(a: u32, b: u32) -> u32 {
///     let mut b: u32 = b;
///     let mut a: u32 = a;
///     let mut temp: u32;
///     while b != 0 {
///         temp = b;
///         b = a % b;
///         a = temp;
///     }
///     a
/// }
/// ```

fn gcd(a: u32, b: u32) -> u32 {
    let mut b: u32 = b;
    let mut a: u32 = a;
    let mut temp: u32;
    while b != 0 {
        temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() {
    let mut proof: Vec<ProofLine> = vec![];

    proof.push(ProofLine::Triple(Triple::new(
        "∧ = gcd(a,b) gcd(a,mod(a,b)) ¬ = b 0",
        "temp≔b",
        "∧ = gcd(a,b) gcd(a,mod(a,b)) ¬ = b 0",
    )));
    proof.push(ProofLine::Triple(Triple::new(
        "∧ = gcd(a,b) gcd(a,mod(a,b)) ¬ = b 0",
        "b≔mod(a,b)",
        "∧ = gcd(a,b) gcd(a,mod(a,b)) ¬ = b 0",
    )));
    proof.push(ProofLine::Triple(Triple::new(
        "∧ = gcd(a,b) gcd(a,mod(a,b)) ¬ = b 0",
        "a≔temp",
        "= gcd(a,b) gcd(a,mod(a,b))",
    )));
    proof.push(ProofLine::new_triple_from_rule(composition_rule(
        proof[0].get_triple(),
        proof[1].get_triple(),
    )));
    proof.push(ProofLine::new_triple_from_rule(composition_rule(
        proof[3].get_triple(),
        proof[2].get_triple(),
    )));
    proof.push(ProofLine::new_triple_from_rule(while_rule(
        proof[4].get_triple(),
    )));
    for line in proof {
        println!("{}", line);
    }
}
