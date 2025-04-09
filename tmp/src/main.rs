use first_order::Formula;
fn main() {
    /*
    let test_formula: Formula = Formula::new("∧ ∀ x → P(x) ∧ Q(x) ∃ y ∨ R(y) S(y) = ¬ T(x) < U V");
    let result: Formula = Formula::Conjunction(
        Box::new(Formula::UniversalQuantifier(
            "x".to_string(),
            Box::new(Formula::Implication(
                Box::new(Formula::Term("P(x)".to_string())),
                Box::new(Formula::Conjunction(
                    Box::new(Formula::Term("Q(x)".to_string())),
                    Box::new(Formula::ExistentialQuantifier(
                        "y".to_string(),
                        Box::new(Formula::Disjunction(
                            Box::new(Formula::Term("R(y)".to_string())),
                            Box::new(Formula::Term("S(y)".to_string())),
                        )),
                    )),
                )),
            )),
        )),
        Box::new(Formula::Equivalence(
            Box::new(Formula::Negation(Box::new(Formula::Term(
                "T(x)".to_string(),
            )))),
            Box::new(Formula::LessThan(
                Box::new(Formula::Term("U".to_string())),
                Box::new(Formula::Term("V".to_string())),
            )),
        )),
    );
    assert_eq!(test_formula, result);
    */

    let test_formula: Formula = Formula::new("∧ ∀ x → P(x) ∧ Q(x) ∃ y ∨ R(y) S(y) = ¬ T(x) < U V");
    println!("{:?}", test_formula.get_info());
    assert_eq!(
        test_formula.get_info(),
        [
            "Conjunction",
            "∀ x → P(x) ∧ Q(x) ∃ y ∨ R(y) S(y)",
            "= ¬ T(x) < U V"
        ]
    );
}
