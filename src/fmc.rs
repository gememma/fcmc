use crate::lambdaterm::Var;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;

#[derive(Clone, Debug, PartialEq)]
pub enum FmcTerm {
    Skip,
    Variable {
        name: Var,
    },
    Pop {
        location_id: Var,
        arg: Var,
        next: Box<FmcTerm>,
    },
    Push {
        term: Box<FmcTerm>,
        location_id: Var,
        next: Box<FmcTerm>,
    },
    Seq {
        term: Box<FmcTerm>,
        next: Box<FmcTerm>,
    },
}

impl FmcTerm {
    /// Turn a [`&str`] into a [`Variable`][FmcTerm::Variable]
    pub fn new_variable(name: &str) -> Self {
        FmcTerm::Variable {
            name: name.to_string(),
        }
    }

    /// Turn a [`&str`] and a [`FmcTerm`] into a [`Pop`][FmcTerm::Pop]
    pub fn new_pop(location: Var, arg: &str, next: FmcTerm) -> Self {
        FmcTerm::Pop {
            location_id: location,
            arg: arg.to_string(),
            next: box next,
        }
    }

    /// Turn two [`FmcTerm`]s into a [`Push`][FmcTerm::Push]
    pub fn new_push(term: FmcTerm, location: Var, next: FmcTerm) -> Self {
        FmcTerm::Push {
            term: box term,
            location_id: location,
            next: box next,
        }
    }

    /// Turn two [`FmcTerm`]s into a [`Seq`][FmcTerm::Seq]
    pub fn new_seq(term: FmcTerm, next: FmcTerm) -> Self {
        FmcTerm::Seq {
            term: box term,
            next: box next,
        }
    }
}

impl fmt::Display for FmcTerm {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FmcTerm::Skip => write!(f, "*"),
            FmcTerm::Variable { name } => write!(f, "{}", name),
            FmcTerm::Pop {
                location_id,
                arg,
                next,
            } => match **next {
                FmcTerm::Skip => write!(f, "{}<{}>", location_id, arg),
                _ => write!(f, "{}<{}>.{}", location_id, arg, next),
            },
            FmcTerm::Push {
                term,
                location_id,
                next,
            } => match **next {
                FmcTerm::Skip => write!(f, "[{}]{}", term, location_id),
                _ => write!(f, "[{}]{}.{}", term, location_id, next),
            },
            FmcTerm::Seq { term, next } => match **next {
                FmcTerm::Skip => write!(f, "{}", term),
                _ => write!(f, "{};{}", term, next),
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FmcClosure {
    term: FmcTerm,
    env: Vec<(Var, FmcClosure)>,
}

impl FmcClosure {
    pub fn new(term: FmcTerm, env: Vec<(Var, FmcClosure)>) -> Self {
        FmcClosure { term, env }
    }
}

impl fmt::Display for FmcClosure {
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

#[derive(Clone, Debug)]
pub struct FmcState {
    closure: FmcClosure,
    memory: HashMap<Var, Vec<FmcClosure>>,
    continuation: Vec<FmcClosure>,
}

impl FmcState {
    pub fn new(
        closure: FmcClosure,
        memory: HashMap<Var, Vec<FmcClosure>>,
        continuation: Vec<FmcClosure>,
    ) -> Self {
        FmcState {
            closure,
            memory,
            continuation,
        }
    }

    pub fn start(t: FmcTerm) -> Self {
        FmcState::new(FmcClosure::new(t, vec![]), HashMap::new(), vec![])
    }
}

impl fmt::Display for FmcState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // closure
        write!(f, "({}, ", self.closure)?;
        // memory
        if self.memory.is_empty() {
            write!(f, "[], ")?;
        } else {
            for (id, location) in &self.memory {
                if location.is_empty() {
                    write!(f, "{}[], ", id)?;
                } else {
                    write!(f, "{}[", id)?;
                    let len = location.len();
                    for (i, t) in location.iter().rev().enumerate() {
                        write!(f, "({})", t)?;
                        if i < len - 1 {
                            write!(f, ", ")?;
                        }
                    }
                    write!(f, "], ")?;
                }
            }
        }
        // continuation stack
        if self.continuation.is_empty() {
            write!(f, "[])")
        } else {
            write!(f, "[")?;
            let len = self.continuation.len();
            for (i, t) in self.continuation.iter().rev().enumerate() {
                write!(f, "({})", t)?;
                if i < len - 1 {
                    write!(f, ", ")?;
                }
            }
            write!(f, "])")
        }
    }
}
