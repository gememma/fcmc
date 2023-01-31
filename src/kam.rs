use crate::lamdaterm::Var;
use crate::LambdaTerm;
use std::fmt;
use std::fmt::Formatter;

#[derive(Clone, Debug, PartialEq)]
pub struct Closure {
    term: LambdaTerm,
    env: Vec<(Var, Closure)>,
}

impl Closure {
    pub fn new(term: LambdaTerm, env: Vec<(Var, Closure)>) -> Self {
        Closure { term, env }
    }

    pub fn closure2() -> Self {
        Closure {
            term: LambdaTerm::Apply {
                t1: box LambdaTerm::Lambda {
                    arg: "x".to_string(),
                    body: box LambdaTerm::new_var("x"),
                },
                t2: box LambdaTerm::new_var("y"),
            },
            env: vec![(
                "y".to_string(),
                Closure {
                    term: LambdaTerm::Lambda {
                        arg: "z".to_string(),
                        body: box LambdaTerm::new_var("z"),
                    },
                    env: vec![],
                },
            )],
        }
    }

    pub fn closure4() -> Self {
        Closure {
            term: LambdaTerm::Lambda {
                arg: "y".to_string(),
                body: box LambdaTerm::new_var("x"),
            },
            env: vec![],
        }
    }
}

impl fmt::Display for Closure {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}, ", self.term)?;
        if self.env.is_empty() {
            write!(f, "[]")
        } else {
            write!(
                f,
                "[(\"{}\", {})]",
                self.env.last().expect("").0,
                self.env.last().expect("").1
            )
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct State {
    closure: Closure,
    stack: Vec<Closure>,
}

impl State {
    pub fn new(closure: Closure, stack: Vec<Closure>) -> Self {
        State { closure, stack }
    }

    pub fn state2() -> Self {
        State {
            closure: Closure::closure2(),
            stack: vec![],
        }
    }

    pub fn state4() -> Self {
        State {
            closure: Closure::closure4(),
            stack: vec![Closure {
                term: LambdaTerm::new_var("z"),
                env: vec![(
                    "z".to_string(),
                    Closure {
                        term: LambdaTerm::Lambda {
                            arg: "a".to_string(),
                            body: box LambdaTerm::new_var("b"),
                        },
                        env: vec![(
                            "b".to_string(),
                            Closure {
                                term: LambdaTerm::new_var("b"),
                                env: vec![],
                            },
                        )],
                    },
                )],
            }],
        }
    }

    pub fn run(term: LambdaTerm) {
        fn start(t: LambdaTerm) -> State {
            State {
                closure: Closure {
                    term: t,
                    env: vec![],
                },
                stack: vec![],
            }
        }
        fn step() {
            todo!()
        }
        fn final_(s: State) -> bool {
            match s.closure.term {
                LambdaTerm::Variable { .. } => s.closure.env.is_empty(),
                LambdaTerm::Lambda { .. } => true,
                _ => false,
            }
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, ", self.closure)?;
        if self.stack.is_empty() {
            write!(f, "[])")
        } else {
            write!(f, "(")?;
            for t in self.stack.iter().rev() {
                write!(f, "{}, ", t)?;
            }
            write!(f, ")")
        }
    }
}
