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
            stack: vec![LambdaTerm::new_num(1), LambdaTerm::new_num(0)],
        }
    }
}

impl fmt::Display for PState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.stack.is_empty() {
            write!(f, "({},[])", self.term)
        } else {
            write!(f, "({}, [", self.term)?;
            for t in &self.stack {
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
            stack: vec![LambdaTerm::new_num(1), LambdaTerm::new_num(0)],
        }
    }

    #[test]
    fn prints() {
        let state = state1();
        assert_eq!(
            state.to_string(),
            "(\\x. \\y. x, [\\f. \\x. f x, \\f. \\x. x, *])"
        );
    }
}
