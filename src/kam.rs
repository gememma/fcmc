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
                        LambdaTerm::Variable { name }
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
        State::new(Closure::closure2(), vec![])
    }

    pub fn state4() -> Self {
        State::new(
            Closure::closure4(),
            vec![Closure::new(
                LambdaTerm::new_var("z"),
                vec![(
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
            )],
        )
    }

    pub fn run(term: LambdaTerm) -> LambdaTerm {
        fn start(t: LambdaTerm) -> State {
            State::new(Closure::new(t, vec![]), vec![])
        }
        fn step(s: &mut State) {
            if final_(s) {
                return;
            }
            match s.closure.term.clone() {
                LambdaTerm::Variable { name } => {
                    let env_last = s.closure.env.pop().expect("");
                    if name.clone() == env_last.0 {
                        s.closure = env_last.1;
                    }
                }
                LambdaTerm::Lambda { arg, body } => {
                    let stack_last = s.stack.pop().expect("");
                    s.closure.term = *body;
                    s.closure.env.push((arg.clone(), stack_last.clone()))
                }
                LambdaTerm::Apply { t1, t2 } => {
                    s.closure.term = *t1;
                    s.stack.push(Closure::new(*t2, s.closure.env.clone()))
                }
            }
        }
        fn final_(s: &State) -> bool {
            match s.closure.term {
                LambdaTerm::Variable { .. } => s.closure.env.is_empty(),
                LambdaTerm::Lambda { .. } => s.stack.is_empty(),
                _ => false,
            }
        }
        fn readback(st: &mut State) -> LambdaTerm {
            let cl = st.closure.clone();
            let mut t = cl.retrieve_term();
            while !st.stack.is_empty() {
                let c = st.stack.pop().expect("");
                t = LambdaTerm::Apply {
                    t1: box t,
                    t2: box c.retrieve_term(),
                }
            }
            t
        }
        let mut s = start(term);
        while !final_(&s) {
            println!("{}", s);
            step(&mut s);
        }
        let ans = readback(&mut s.clone());
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
