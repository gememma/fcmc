use std::fmt;
use std::fmt::Formatter;

#[derive(Clone, Debug, PartialEq)]
pub enum LambdaTerm {
    Variable {
        name: Var,
    },
    Lambda {
        arg: Var,
        body: Box<LambdaTerm>,
    },
    Apply {
        t1: Box<LambdaTerm>,
        t2: Box<LambdaTerm>,
    },
}

pub type Var = String;

impl LambdaTerm {
    pub fn new_var(name: &str) -> Self {
        LambdaTerm::Variable {
            name: name.to_string(),
        }
    }
    pub fn new_num(numeral: usize) -> Self {
        fn new_num_sub(numeral: usize) -> LambdaTerm {
            if numeral == 0 {
                LambdaTerm::new_var("x")
            } else {
                LambdaTerm::Apply {
                    t1: box LambdaTerm::new_var("f"),
                    t2: box new_num_sub(numeral - 1),
                }
            }
        }
        LambdaTerm::Lambda {
            arg: "f".to_string(),
            body: box LambdaTerm::Lambda {
                arg: "x".to_string(),
                body: box new_num_sub(numeral),
            },
        }
    }
}

impl fmt::Display for LambdaTerm {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            LambdaTerm::Variable { name } => {
                write!(f, "{}", name)
            }
            LambdaTerm::Lambda { arg, body } => {
                write!(f, "\\{}. {}", arg, body)
            }
            LambdaTerm::Apply {
                t1: t1 @ box LambdaTerm::Variable { .. },
                t2: t2 @ box LambdaTerm::Variable { .. },
            }
            | LambdaTerm::Apply {
                t1: t1 @ box LambdaTerm::Apply { .. },
                t2: t2 @ box LambdaTerm::Variable { .. },
            } => {
                write!(f, "{} {}", t1, t2)
            }
            LambdaTerm::Apply {
                t1: t1 @ box LambdaTerm::Variable { .. },
                t2,
            } => {
                write!(f, "{} ({})", t1, t2)
            }
            LambdaTerm::Apply {
                t1,
                t2: t2 @ box LambdaTerm::Variable { .. },
            } => {
                write!(f, "({}) {}", t1, t2)
            }
            LambdaTerm::Apply { t1, t2 } => {
                write!(f, "({}) ({})", t1, t2)
            }
        }
    }
}
