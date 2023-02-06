use crate::lambdaterm::Var;
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

    pub fn retrieve_term(mut self) -> LambdaTerm {
        match self.term {
            LambdaTerm::Variable { name } => {
                if self.env.is_empty() {
                    LambdaTerm::Variable { name }
                } else {
                    let env_last = self.env.pop().expect("");
                    if &env_last.0 == &name {
                        env_last.1.retrieve_term()
                    } else {
                        self.term = LambdaTerm::Variable { name };
                        self.retrieve_term()
                    }
                }
            }
            LambdaTerm::Lambda { arg, body } => {
                let mut e = self.env;
                e.push((arg.clone(), Closure::new(LambdaTerm::new_var(&arg), vec![])));
                let c = Closure::new(*body, e);
                LambdaTerm::Lambda {
                    arg,
                    body: box c.retrieve_term(),
                }
            }
            LambdaTerm::Apply { t1, t2 } => LambdaTerm::Apply {
                t1: box Closure::new(*t1, self.env.clone()).retrieve_term(),
                t2: box Closure::new(*t2, self.env).retrieve_term(),
            },
        }
    }
}

impl fmt::Display for Closure {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}, ", self.term)?;
        write!(f, "[")?;
        let len = self.env.len();
        for (i, e) in self.env.iter().rev().enumerate() {
            write!(f, "(\"{}\", {})", e.0, e.1)?;
            if i < len - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
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

    fn start(t: LambdaTerm) -> State {
        State::new(Closure::new(t, vec![]), vec![])
    }

    fn step(&mut self) {
        if self.final_() {
            return;
        }
        match self.closure.term.clone() {
            LambdaTerm::Variable { name } => {
                let env_last = self.closure.env.pop().expect("");
                if name.clone() == env_last.0 {
                    self.closure = env_last.1;
                }
            }
            LambdaTerm::Lambda { arg, body } => {
                let stack_last = self.stack.pop().expect("");
                self.closure.term = *body;
                self.closure.env.push((arg.clone(), stack_last.clone()))
            }
            LambdaTerm::Apply { t1, t2 } => {
                self.closure.term = *t1;
                self.stack.push(Closure::new(*t2, self.closure.env.clone()))
            }
        }
    }

    fn final_(&self) -> bool {
        match self.closure.term {
            LambdaTerm::Variable { .. } => self.closure.env.is_empty(),
            LambdaTerm::Lambda { .. } => self.stack.is_empty(),
            _ => false,
        }
    }

    fn readback(&mut self) -> LambdaTerm {
        let cl = self.closure.clone();
        let mut t = cl.retrieve_term();
        while !self.stack.is_empty() {
            let c = self.stack.pop().expect("");
            t = LambdaTerm::Apply {
                t1: box t,
                t2: box c.retrieve_term(),
            }
        }
        t
    }

    pub fn run(term: LambdaTerm) -> LambdaTerm {
        let mut s = State::start(term);
        while !s.final_() {
            println!("{}", s);
            s.step();
        }
        println!("{}", s);
        let ans = s.clone().readback();
        println!("{}", ans);
        ans
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, ", self.closure)?;
        if self.stack.is_empty() {
            write!(f, "[])")
        } else {
            write!(f, "(")?;
            let len = self.stack.len();
            for (i, t) in self.stack.iter().rev().enumerate() {
                write!(f, "{}", t)?;
                if i < len - 1 {
                    write!(f, ", ")?;
                }
            }
            write!(f, "))")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::kam::{Closure, State};
    use crate::lambdaterm::LambdaTerm;

    #[test]
    fn prints_state() {
        let s = State::start(LambdaTerm::term1());
        assert_eq!(
            s.to_string(),
            "(((\\x. \\y. x) (\\a. \\b. a)) (\\a. \\b. b), [], [])"
        );
    }

    #[test]
    fn prints_closure() {
        let c = Closure::new(
            LambdaTerm::Lambda {
                arg: "b".to_string(),
                body: box LambdaTerm::new_var("a"),
            },
            vec![
                (
                    "b".to_string(),
                    Closure::new(
                        LambdaTerm::Lambda {
                            arg: "z".to_string(),
                            body: box LambdaTerm::new_var("z"),
                        },
                        vec![],
                    ),
                ),
                (
                    "a".to_string(),
                    Closure::new(
                        LambdaTerm::new_var("b"),
                        vec![(
                            "x".to_string(),
                            Closure::new(
                                LambdaTerm::Lambda {
                                    arg: "a".to_string(),
                                    body: box LambdaTerm::Lambda {
                                        arg: "b".to_string(),
                                        body: box LambdaTerm::new_var("b"),
                                    },
                                },
                                vec![],
                            ),
                        )],
                    ),
                ),
            ],
        );
        assert_eq!(
            c.to_string(),
            "\\b. a, [(\"a\", b, [(\"x\", \\a. \\b. b, [])]), (\"b\", \\z. z, [])]"
        )
    }

    #[test]
    fn term_into_state() {
        let s = State::start(LambdaTerm::term1());
        let ans = State::new(Closure::new(LambdaTerm::term1(), vec![]), vec![]);
        assert_eq!(s, ans);
    }

    #[test]
    fn detect_end_state() {
        let s = State::start(LambdaTerm::term1());
        let s2 = State::new(
            Closure::new(
                LambdaTerm::Lambda {
                    arg: "y".to_string(),
                    body: box LambdaTerm::new_var("a"),
                },
                vec![],
            ),
            vec![],
        );
        let s3 = State::new(Closure::new(LambdaTerm::new_var("a"), vec![]), vec![]);
        let s4 = State::new(
            Closure::new(
                LambdaTerm::Lambda {
                    arg: "y".to_string(),
                    body: box LambdaTerm::new_var("a"),
                },
                vec![(
                    "x".to_string(),
                    Closure::new(
                        LambdaTerm::Lambda {
                            arg: "y".to_string(),
                            body: box LambdaTerm::new_var("a"),
                        },
                        vec![],
                    ),
                )],
            ),
            vec![],
        );
        let s5 = State::new(
            Closure::new(
                LambdaTerm::Lambda {
                    arg: "y".to_string(),
                    body: box LambdaTerm::new_var("a"),
                },
                vec![],
            ),
            vec![s4.closure.clone()],
        );
        let s6 = State::new(
            Closure::new(
                LambdaTerm::Lambda {
                    arg: "y".to_string(),
                    body: box LambdaTerm::new_var("a"),
                },
                vec![(
                    "x".to_string(),
                    Closure::new(
                        LambdaTerm::Lambda {
                            arg: "y".to_string(),
                            body: box LambdaTerm::new_var("a"),
                        },
                        vec![],
                    ),
                )],
            ),
            vec![s4.closure.clone()],
        );

        assert_eq!(s.final_(), false);
        assert_eq!(s2.final_(), true);
        assert_eq!(s3.final_(), true);
        assert_eq!(s4.final_(), true);
        assert_eq!(s5.final_(), false);
        assert_eq!(s6.final_(), false);
    }

    #[test]
    fn closure_to_term() {
        let s = State::state2();
        let ans = LambdaTerm::Apply {
            t1: box LambdaTerm::Lambda {
                arg: "x".to_string(),
                body: box LambdaTerm::new_var("x"),
            },
            t2: box LambdaTerm::Lambda {
                arg: "z".to_string(),
                body: box LambdaTerm::new_var("z"),
            },
        };
        assert_eq!(s.closure.retrieve_term(), ans);

        let s2 = State::state4();
        let ans2 = LambdaTerm::Lambda {
            arg: "y".to_string(),
            body: box LambdaTerm::new_var("x"),
        };
        assert_eq!(s2.closure.retrieve_term(), ans2);

        // \b. a, [("a", b, [("x", \a. \b. b, []), ("a", \a. \b. a, [("b", \z. z, [])]), ("b", \z. z, [])]), ("b", \z. z, [])]
        let s3 = State::new(
            Closure::new(
                LambdaTerm::Lambda {
                    arg: "b".to_string(),
                    body: box LambdaTerm::new_var("a"),
                },
                vec![
                    (
                        "a".to_string(),
                        Closure::new(
                            LambdaTerm::new_var("b"),
                            vec![
                                (
                                    "x".to_string(),
                                    Closure::new(
                                        LambdaTerm::Lambda {
                                            arg: "a".to_string(),
                                            body: box LambdaTerm::Lambda {
                                                arg: "b".to_string(),
                                                body: box LambdaTerm::new_var("b"),
                                            },
                                        },
                                        vec![],
                                    ),
                                ),
                                (
                                    "a".to_string(),
                                    Closure::new(
                                        LambdaTerm::Lambda {
                                            arg: "a".to_string(),
                                            body: box LambdaTerm::Lambda {
                                                arg: "b".to_string(),
                                                body: box LambdaTerm::new_var("a"),
                                            },
                                        },
                                        vec![(
                                            "b".to_string(),
                                            Closure::new(
                                                LambdaTerm::Lambda {
                                                    arg: "z".to_string(),
                                                    body: box LambdaTerm::new_var("z"),
                                                },
                                                vec![],
                                            ),
                                        )],
                                    ),
                                ),
                                (
                                    "b".to_string(),
                                    Closure::new(
                                        LambdaTerm::Lambda {
                                            arg: "z".to_string(),
                                            body: box LambdaTerm::new_var("z"),
                                        },
                                        vec![],
                                    ),
                                ),
                            ],
                        ),
                    ),
                    (
                        "b".to_string(),
                        Closure::new(
                            LambdaTerm::Lambda {
                                arg: "z".to_string(),
                                body: box LambdaTerm::new_var("z"),
                            },
                            vec![],
                        ),
                    ),
                ],
            ),
            vec![],
        );
        let ans3 = LambdaTerm::Lambda {
            arg: "b".to_string(),
            body: box LambdaTerm::Lambda {
                arg: "z".to_string(),
                body: box LambdaTerm::new_var("z"),
            },
        };
        assert_eq!(s3.closure.retrieve_term(), ans3);
    }

    #[test]
    fn application_step() {
        let mut step0 = State::new(
            Closure::new(
                LambdaTerm::Apply {
                    t1: box LambdaTerm::Lambda {
                        arg: "x".to_string(),
                        body: box LambdaTerm::new_var("x"),
                    },
                    t2: box LambdaTerm::Lambda {
                        arg: "z".to_string(),
                        body: box LambdaTerm::new_var("z"),
                    },
                },
                vec![],
            ),
            vec![],
        );
        step0.step();
        let step1 = State::new(
            Closure::new(
                LambdaTerm::Lambda {
                    arg: "x".to_string(),
                    body: box LambdaTerm::new_var("x"),
                },
                vec![],
            ),
            vec![Closure::new(
                LambdaTerm::Lambda {
                    arg: "z".to_string(),
                    body: box LambdaTerm::new_var("z"),
                },
                vec![],
            )],
        );
        assert_eq!(step0, step1);
    }

    #[test]
    fn lambda_step() {
        let mut step0 = State::new(
            Closure::new(
                LambdaTerm::Lambda {
                    arg: "a".to_string(),
                    body: box LambdaTerm::Lambda {
                        arg: "b".to_string(),
                        body: box LambdaTerm::new_var("a"),
                    },
                },
                vec![(
                    "b".to_string(),
                    Closure::new(
                        LambdaTerm::Lambda {
                            arg: "z".to_string(),
                            body: box LambdaTerm::new_var("z"),
                        },
                        vec![],
                    ),
                )],
            ),
            vec![Closure::new(
                LambdaTerm::new_var("b"),
                vec![(
                    "x".to_string(),
                    Closure::new(
                        LambdaTerm::Lambda {
                            arg: "a".to_string(),
                            body: box LambdaTerm::Lambda {
                                arg: "b".to_string(),
                                body: box LambdaTerm::new_var("b"),
                            },
                        },
                        vec![],
                    ),
                )],
            )],
        );
        step0.step();
        let step1 = State::new(
            Closure::new(
                LambdaTerm::Lambda {
                    arg: "b".to_string(),
                    body: box LambdaTerm::new_var("a"),
                },
                vec![
                    (
                        "b".to_string(),
                        Closure::new(
                            LambdaTerm::Lambda {
                                arg: "z".to_string(),
                                body: box LambdaTerm::new_var("z"),
                            },
                            vec![],
                        ),
                    ),
                    (
                        "a".to_string(),
                        Closure::new(
                            LambdaTerm::new_var("b"),
                            vec![(
                                "x".to_string(),
                                Closure::new(
                                    LambdaTerm::Lambda {
                                        arg: "a".to_string(),
                                        body: box LambdaTerm::Lambda {
                                            arg: "b".to_string(),
                                            body: box LambdaTerm::new_var("b"),
                                        },
                                    },
                                    vec![],
                                ),
                            )],
                        ),
                    ),
                ],
            ),
            vec![],
        );
        assert_eq!(step0, step1);
    }

    #[test]
    fn run_term1() {
        let s = State::run(LambdaTerm::term1());
        assert_eq!(LambdaTerm::new_bool(true).to_string(), s.to_string());
    }

    #[test]
    fn run_term2() {
        let s = State::run(LambdaTerm::term2());
        let s2 = LambdaTerm::Lambda {
            arg: "b".to_string(),
            body: box LambdaTerm::Lambda {
                arg: "z".to_string(),
                body: box LambdaTerm::new_var("z"),
            },
        };
        assert_eq!(s, s2);
    }
}
