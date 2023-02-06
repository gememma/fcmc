use crate::lambdaterm::LambdaTerm;
use std::fmt;
use std::fmt::Formatter;

#[derive(Clone, Debug, PartialEq)]
pub struct PState {
    term: LambdaTerm,
    stack: Vec<LambdaTerm>,
}

impl PState {
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

    pub fn p_run(term: LambdaTerm) -> LambdaTerm {
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

    fn state2() -> PState {
        PState {
            term: LambdaTerm::Lambda {
                arg: "x".to_string(),
                body: box LambdaTerm::Lambda {
                    arg: "y".to_string(),
                    body: box LambdaTerm::new_var("x"),
                },
            },
            stack: vec![],
        }
    }

    fn state3() -> PState {
        PState {
            term: LambdaTerm::new_var("y"),
            stack: vec![LambdaTerm::Lambda {
                arg: "x".to_string(),
                body: box LambdaTerm::Lambda {
                    arg: "y".to_string(),
                    body: box LambdaTerm::new_var("x"),
                },
            }],
        }
    }

    fn term1() -> LambdaTerm {
        LambdaTerm::Apply {
            t1: box LambdaTerm::Apply {
                t1: box LambdaTerm::Lambda {
                    arg: "x".to_string(),
                    body: box LambdaTerm::Lambda {
                        arg: "y".to_string(),
                        body: box LambdaTerm::new_var("x"),
                    },
                },
                t2: box LambdaTerm::new_bool(true),
            },
            t2: box LambdaTerm::new_bool(false),
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

    #[test]
    fn term_into_state() {
        let created = PState::p_start(term1());
        assert_eq!(
            created.to_string(),
            "(((\\x. \\y. x) (\\a. \\b. a)) (\\a. \\b. b),[*])"
        );
    }

    #[test]
    fn detect_end_state() {
        assert_eq!(state1().p_final(), false);
        assert_eq!(PState::p_start(term1()).p_final(), false);
        assert_eq!(state2().p_final(), true);
        assert_eq!(state3().p_final(), true);
    }

    #[test]
    fn run_pam() {
        let ans = PState::p_run(term1());
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
