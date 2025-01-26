mod first_order;
mod hoare_triple;
//use first_order::Formula;
use hoare_triple::Triple;

fn main() {
    let mut proof: Vec<Triple> = vec![];
    proof.push(Triple::new("∧ = a ϕ = b ψ", "q≔0", "∧ ∧ = a ϕ = b ψ = q 0"));
    proof.push(Triple::new(
        "∧ ∧ = a ϕ = b ψ = q 0",
        "r≔0",
        "∧ ∧ ∧ = a ϕ = b ψ = q 0 = r 0",
    ));
    proof.push(Triple::new(
        "∧ ∧ ∧ = a ϕ = b ψ = q 0 = r 0",
        "b≔b",
        "∧ ∧ ∧ = a ϕ = b ψ = q 0 = r 0",
    ));
    proof.push(Triple::new(
        "∧ ∧ ∧ = a ϕ = b ψ = q 0 = r 0",
        "b≔b",
        "∧ ∧ ∧ = a ϕ = b ψ = q 0 = r 0",
    ));
    proof.push(hoare_triple::while_rule(&proof[2]));

    for i in proof {
        println!("{i}");
    }
}
