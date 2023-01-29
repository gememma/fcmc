use crate::lamdaterm::LambdaTerm;
use std::fmt;
use std::fmt::Formatter;

#[derive(Clone, Debug, PartialEq)]
pub struct PState {
    term: LambdaTerm,
    stack: Vec<LambdaTerm>,
}

impl PState {
    pub fn state1() -> PState {
        PState {
            term: LambdaTerm::Lambda {
                arg: "x".to_string(),
                body: box LambdaTerm::Lambda {
                    arg: "y".to_string(),
                    body: box LambdaTerm::new_var("x"),
                },
            },
            stack: vec![LambdaTerm::new_bool(false), LambdaTerm::new_bool(true)],
        }
    }

    pub fn p_start(n: LambdaTerm) -> PState {
        PState {
            term: n,
            stack: vec![],
        }
    }

    /// Perform a transition step in-place
    pub fn p_step(&mut self) {
        if self.p_final() {
            return;
        }
        match &self.term {
            LambdaTerm::Lambda { arg, body } => {
                self.term = body.substitute(
                    arg,
                    &self.stack.pop().expect("Error when performing transition"),
                );
            }
            LambdaTerm::Apply { t1, t2 } => {
                let term = *t2.clone();
                self.term = *t1.clone();
                self.stack.push(term);
            }
            LambdaTerm::Variable { .. } => {
                unreachable!()
            }
        }
    }

    pub fn p_final(&self) -> bool {
        match self.term {
            LambdaTerm::Variable { .. } => true,
            LambdaTerm::Lambda { .. } => self.stack.is_empty(),
            _ => false,
        }
    }

    pub fn p_run(term: LambdaTerm) {
        fn p_readback(st: &mut PState) -> LambdaTerm {
            let mut t = st.term.clone();
            while !st.stack.is_empty() {
                t = LambdaTerm::Apply {
                    t1: box t,
                    t2: box st.stack.pop().expect(""),
                }
            }
            t
        }
        let mut s = PState::p_start(term);
        while !s.p_final() {
            println!("{}", s);
            s.p_step();
        }
        println!("{}", p_readback(&mut s));
    }
}

impl fmt::Display for PState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.stack.is_empty() {
            write!(f, "({},[*])", self.term)
        } else {
            write!(f, "({}, [", self.term)?;
            for t in self.stack.iter().rev() {
                write!(f, "{}, ", t)?;
            }
            write!(f, "*])")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{LambdaTerm, PState};

    fn state1() -> PState {
        PState {
            term: LambdaTerm::Lambda {
                arg: "x".to_string(),
                body: box LambdaTerm::Lambda {
                    arg: "y".to_string(),
                    body: box LambdaTerm::new_var("x"),
                },
            },
            stack: vec![LambdaTerm::new_bool(false), LambdaTerm::new_bool(true)],
        }
    }

    #[test]
    fn prints() {
        let state = state1();
        assert_eq!(
            state.to_string(),
            "(\\x. \\y. x, [\\a. \\b. a, \\a. \\b. b, *])"
        );
    }

    // TODO: finish PAM tests
}
