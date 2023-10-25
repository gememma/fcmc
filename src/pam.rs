use crate::lambdaterm::LambdaTerm;
use std::fmt;
use std::fmt::Formatter;

#[derive(Clone, Debug, PartialEq)]
pub struct PState {
    term: LambdaTerm,
    stack: Vec<LambdaTerm>,
}

impl PState {
    /// Create a new ['PState'] from a ['LambdaTerm'] and a stack: ['Vec<LambdaTerm>']
    pub fn new(term: LambdaTerm, stack: Vec<LambdaTerm>) -> Self {
        PState { term, stack }
    }

    /// Create a start ['PState'] from a term: ['SLambdaTerm'] and a stack: ['Vec<SClosure>']
    pub fn p_start(n: LambdaTerm) -> Self {
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

    /// Returns true if ['self'][PState] is a final state ie. computation is complete
    pub fn p_final(&self) -> bool {
        match self.term {
            LambdaTerm::Variable { .. } => true,
            LambdaTerm::Lambda { .. } => self.stack.is_empty(),
            _ => false,
        }
    }

    /// Run the given ['LambdaTerm'] on the partial abstract machine, printing each step and the output
    pub fn p_run(term: LambdaTerm) -> LambdaTerm {
        /// Given ['self'][PState], return the equivalent ['LambdaTerm']
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
        let ans = p_readback(&mut s.clone());
        println!("{}", ans);
        ans
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

    #[test]
    fn prints() {
        let state = PState::state1();
        assert_eq!(
            state.to_string(),
            "(\\x. \\y. x, [\\a. \\b. a, \\a. \\b. b, *])"
        );
    }

    #[test]
    fn term_into_state() {
        let created = PState::p_start(LambdaTerm::term1());
        assert_eq!(
            created.to_string(),
            "(((\\x. \\y. x) (\\a. \\b. a)) (\\a. \\b. b),[*])"
        );
    }

    #[test]
    fn detect_end_state() {
        assert_eq!(PState::state1().p_final(), false);
        assert_eq!(PState::p_start(LambdaTerm::term1()).p_final(), false);
        assert_eq!(PState::state2().p_final(), true);
        assert_eq!(PState::state3().p_final(), true);
    }

    #[test]
    fn run_pam() {
        let ans = PState::p_run(LambdaTerm::term1());
        assert_eq!(PState::p_start(ans.clone()).p_final(), true);
        assert_eq!(
            ans,
            LambdaTerm::Lambda {
                arg: "d".to_string(),
                body: box LambdaTerm::Lambda {
                    arg: "e".to_string(),
                    body: box LambdaTerm::new_var("d")
                }
            }
        );
    }
}
