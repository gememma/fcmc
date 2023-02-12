use crate::lambdaterm::Var;
use std::fmt;
use std::fmt::Formatter;

/// The sequential lambda-calculus takes an altered version of the lambda-calculus as grammar
#[derive(Clone, Debug, PartialEq)]
pub enum SLambdaTerm {
    Skip,
    Variable {
        name: Var,
        next: Box<SLambdaTerm>,
    },
    Pop {
        arg: Var,
        next: Box<SLambdaTerm>,
    },
    Push {
        term: Box<SLambdaTerm>,
        next: Box<SLambdaTerm>,
    },
}

impl SLambdaTerm {
    /// Turn a [`&str`] and a [`SLambdaTerm`] into a [`Variable`][SLambdaTerm::Variable]
    pub fn new_variable(name: &str, next: Box<SLambdaTerm>) -> Self {
        SLambdaTerm::Variable {
            name: name.to_string(),
            next,
        }
    }

    /// Turn a [`&str`] and a [`SLambdaTerm`] into a [`Pop`][SLambdaTerm::Pop]
    pub fn new_pop(arg: &str, next: SLambdaTerm) -> Self {
        SLambdaTerm::Pop {
            arg: arg.to_string(),
            next: box next,
        }
    }

    /// Turn two [`SLambdaTerm`]s into a [`Push`][SLambdaTerm::Push]
    pub fn new_push(term: SLambdaTerm, next: SLambdaTerm) -> Self {
        SLambdaTerm::Push {
            term: box term,
            next: box next,
        }
    }
}

impl fmt::Display for SLambdaTerm {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SLambdaTerm::Skip => unreachable!(),
            SLambdaTerm::Variable { name, next } => match **next {
                SLambdaTerm::Skip => write!(f, "{}", name),
                _ => write!(f, "{}.{}", name, next),
            },
            SLambdaTerm::Pop { arg, next } => match **next {
                SLambdaTerm::Skip => write!(f, "<{}>", arg),
                _ => write!(f, "<{}>.{}", arg, next),
            },
            SLambdaTerm::Push { term, next } => match **next {
                SLambdaTerm::Skip => write!(f, "[{}]", term),
                _ => write!(f, "[{}].{}", term, next),
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SClosure {
    term: SLambdaTerm,
    env: Vec<(Var, SClosure)>,
}

impl SClosure {
    pub fn new(term: SLambdaTerm, env: Vec<(Var, SClosure)>) -> Self {
        SClosure { term, env }
    }

    pub fn retrieve_term(mut self) -> SLambdaTerm {
        match self.term {
            SLambdaTerm::Skip => self.term,
            SLambdaTerm::Variable { name, next } => {
                if self.env.is_empty() {
                    SLambdaTerm::Variable { name, next }
                } else {
                    let env_last = self.env.pop().expect("");
                    if &env_last.0 == &name {
                        env_last.1.retrieve_term()
                    } else {
                        self.term = SLambdaTerm::Variable { name, next };
                        self.retrieve_term()
                    }
                }
            }
            SLambdaTerm::Pop { arg, next } => {
                let mut e = self.env;
                e.push((
                    arg.clone(),
                    SClosure::new(SLambdaTerm::new_variable(&arg, next.clone()), vec![]),
                ));
                let c = SClosure::new(*next, e);
                SLambdaTerm::Pop {
                    arg,
                    next: box c.retrieve_term(),
                }
            }
            SLambdaTerm::Push { term, next } => SLambdaTerm::Push {
                term: box SClosure::new(*term, self.env.clone()).retrieve_term(),
                next: box SClosure::new(*next, self.env).retrieve_term(),
            },
        }
    }
}

impl fmt::Display for SClosure {
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
pub struct SState {
    closure: SClosure,
    stack: Vec<SClosure>,
}

impl SState {
    pub fn new(closure: SClosure, stack: Vec<SClosure>) -> Self {
        SState { closure, stack }
    }

    fn start(t: SLambdaTerm, s: Vec<SClosure>) -> SState {
        SState::new(SClosure::new(t, vec![]), s)
    }

    pub fn readback(&mut self) -> Vec<SLambdaTerm> {
        let mut res = vec![];
        for c in self.stack.iter() {
            res.push(c.clone().retrieve_term());
        }
        res
    }

    fn final_(&self) -> bool {
        match self.closure.term {
            SLambdaTerm::Skip => true,
            _ => false,
        }
    }
}

impl fmt::Display for SState {
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
