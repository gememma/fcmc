use crate::lambdaterm::Var;
use clap::error::ErrorKind::NoEquals;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use std::ops::Index;

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

    pub fn retrieve_term(mut self) -> FmcTerm {
        match self.term {
            FmcTerm::Skip => self.term,
            FmcTerm::Variable { name } => {
                if self.env.is_empty() {
                    FmcTerm::Variable { name }
                } else {
                    let env_last = self.env.pop().unwrap();
                    if &env_last.0 == &name {
                        env_last.1.retrieve_term()
                    } else {
                        self.term = FmcTerm::Variable { name };
                        self.retrieve_term()
                    }
                }
            }
            FmcTerm::Pop {
                location_id,
                arg,
                next,
            } => {
                let mut e = self.env;
                e.push((
                    arg.clone(),
                    FmcClosure::new(FmcTerm::new_variable(&arg), vec![]),
                ));
                let c = FmcClosure::new(*next, e);
                FmcTerm::Pop {
                    location_id,
                    arg,
                    next: box c.retrieve_term(),
                }
            }
            FmcTerm::Push {
                term,
                location_id,
                next,
            } => FmcTerm::Push {
                term: box FmcClosure::new(*term, self.env.clone()).retrieve_term(),
                location_id,
                next: box FmcClosure::new(*next, self.env).retrieve_term(),
            },
            FmcTerm::Seq { term, next } => FmcTerm::Seq {
                term: box FmcClosure::new(*term, self.env.clone()).retrieve_term(),
                next: box FmcClosure::new(*next, self.env).retrieve_term(),
            },
        }
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

    fn start(t: FmcTerm) -> Self {
        FmcState::new(FmcClosure::new(t, vec![]), HashMap::new(), vec![])
    }

    fn step(&mut self) -> Result<(), String> {
        match self.closure.term.clone() {
            FmcTerm::Skip => {
                if !self.final_() {
                    self.closure = self.continuation.pop().unwrap();
                }
            }
            FmcTerm::Variable { name } => {
                let env_last = self.closure.env.pop().unwrap();
                if name.clone() == env_last.0 {
                    self.closure = env_last.1;
                }
            }
            FmcTerm::Pop {
                location_id,
                arg,
                next,
            } => {
                let mut location = self
                    .memory
                    .get_mut(&location_id)
                    .ok_or("Specified location doesn't exist".to_string())?;
                let stack_last = location.pop().ok_or(
                    "Term cannot be executed. Pop action encountered but location is empty."
                        .to_string(),
                )?;
                self.closure.term = *next;
                self.closure.env.push((arg.clone(), stack_last.clone()));
            }
            FmcTerm::Push {
                term,
                location_id,
                next,
            } => {
                self.closure.term = *next;
                self.memory
                    .entry(location_id)
                    .or_default()
                    .push(FmcClosure::new(*term, self.closure.env.clone()));
            }
            FmcTerm::Seq { term, next } => {
                self.continuation
                    .push(FmcClosure::new(*next, self.closure.env.clone()));
                self.closure.term = *term;
            }
        }
        Ok(())
    }

    fn readback(&mut self) -> Vec<(Var, FmcTerm)> {
        let mut res = vec![];
        for (name, location) in self.memory.iter() {
            for c in location {
                res.push((name.clone(), c.clone().retrieve_term()));
            }
        }
        res
    }

    fn final_(&self) -> bool {
        match self.closure.term {
            FmcTerm::Skip => self.continuation.is_empty(),
            _ => false,
        }
    }

    pub fn run(term: FmcTerm) -> Vec<(Var, FmcTerm)> {
        let mut s = FmcState::start(term);
        while !s.final_() {
            println!("{}", s);
            match s.step() {
                Ok(_) => {}
                Err(e) => {
                    println!("Error during step: \"{}\"", e);
                    return vec![];
                }
            }
        }
        println!("{}", s);
        let ans = s.clone().readback();
        let len = ans.len();
        for (i, (n, t)) in ans.iter().rev().enumerate() {
            print!("{}: {}", n, t);
            if i < len - 1 {
                println!(", ");
            }
        }
        ans
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
