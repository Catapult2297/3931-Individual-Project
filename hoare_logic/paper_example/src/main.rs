//! Main example function that implement the example in the paper
#![allow(dead_code)]
#![warn(missing_docs)]

use backtrace::{Backtrace, BacktraceFrame, BacktraceSymbol};
use first_order::Formula;
use hoare_triple::{composition_rule, consequence_rule, while_rule, Triple};
use std::fmt;

enum ProofLine {
    Formula(Formula),
    Triple(Triple),
}

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
    fn new_triple_from_rule(result: Result<Triple, String>) -> Self {
        match result {
            Ok(triple) => Self::Triple(triple),
            Err(err) => {
                //let backtrace = Backtrace::new();
                panic!("Error at {}.\n{err}", trace())
            }
        }
    }
    fn get_formula(&self) -> &Formula {
        match self {
            ProofLine::Formula(formula) => &formula,
            _ => panic!(
                "Error at {}.\nAttempt to access Formula from a non-Formula ProofLine",
                trace()
            ),
        }
    }

    fn get_triple(&self) -> &Triple {
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
