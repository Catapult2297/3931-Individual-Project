use backtrace::{Backtrace, BacktraceFrame, BacktraceSymbol};
use first_order::Formula;
use hoare_triple::{composition_rule, consequence_rule, while_rule, Triple};
use std::fmt;

pub enum ProofLine {
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
    pub fn new_triple_from_rule(result: Result<Triple, String>) -> Self {
        match result {
            Ok(triple) => Self::Triple(triple),
            Err(err) => {
                //let backtrace = Backtrace::new();
                panic!("Error at {}.\n{err}", trace())
            }
        }
    }
    pub fn get_formula(&self) -> &Formula {
        match self {
            ProofLine::Formula(formula) => &formula,
            _ => panic!(
                "Error at {}.\nAttempt to access Formula from a non-Formula ProofLine",
                trace()
            ),
        }
    }

    pub fn get_triple(&self) -> &Triple {
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
