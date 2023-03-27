use crate::lambdaterm::Var;
use crossbeam_channel::{unbounded, Receiver, Sender};
use std::collections::{HashMap, HashSet};
use std::fmt::Formatter;
use std::{fmt, thread};

#[derive(Clone, Debug, PartialEq)]
pub enum FcmcTerm {
    Skip,
    Variable {
        name: Var,
    },
    Pop {
        location_id: Var,
        arg: Var,
        next: Box<FcmcTerm>,
    },
    Push {
        term: Box<FcmcTerm>,
        location_id: Var,
        next: Box<FcmcTerm>,
    },
    Seq {
        term: Box<FcmcTerm>,
        next: Box<FcmcTerm>,
    },
    Fork {
        forked: Box<FcmcTerm>,
        cont: Box<FcmcTerm>,
    },
}

impl FcmcTerm {
    /// Turn a [`&str`] into a [`Variable`][FcmcTerm::Variable]
    pub fn new_variable(name: &str) -> Self {
        FcmcTerm::Variable {
            name: name.to_string(),
        }
    }

    /// Turn a [`&str`] and a [`FcmcTerm`] into a [`Pop`][FcmcTerm::Pop]
    pub fn new_pop(location: Var, arg: &str, next: FcmcTerm) -> Self {
        FcmcTerm::Pop {
            location_id: location,
            arg: arg.to_string(),
            next: box next,
        }
    }

    /// Turn two [`FcmcTerm`]s into a [`Push`][FcmcTerm::Push]
    pub fn new_push(term: FcmcTerm, location: Var, next: FcmcTerm) -> Self {
        FcmcTerm::Push {
            term: box term,
            location_id: location,
            next: box next,
        }
    }

    /// Turn two [`FcmcTerm`]s into a [`Seq`][FcmcTerm::Seq]
    pub fn new_seq(term: FcmcTerm, next: FcmcTerm) -> Self {
        FcmcTerm::Seq {
            term: box term,
            next: box next,
        }
    }

    /// Turn two [`FcmcTerm`]s into a [`Fork`][FcmcTerm::Fork]
    pub fn new_fork(forked: FcmcTerm, cont: FcmcTerm) -> Self {
        FcmcTerm::Fork {
            forked: box forked,
            cont: box cont,
        }
    }

    /// Return a list of location names accessed in a [FcmcTerm]
    pub fn location_scan(&self) -> HashSet<Var> {
        fn traverse(term: &FcmcTerm, mut list: HashSet<Var>) -> HashSet<Var> {
            match term {
                FcmcTerm::Skip => return list,
                FcmcTerm::Variable { .. } => return list,
                FcmcTerm::Pop {
                    location_id, next, ..
                } => {
                    list.insert(location_id.clone());
                    traverse(next, list)
                }
                FcmcTerm::Push {
                    term,
                    location_id,
                    next,
                } => {
                    list.insert(location_id.clone());
                    traverse(term, traverse(next, list))
                }
                FcmcTerm::Seq { term, next } => traverse(term, traverse(next, list)),
                FcmcTerm::Fork { forked, cont } => traverse(forked, traverse(cont, list)),
            }
        }
        let mut list = HashSet::new();
        traverse(self, list)
    }
}

impl fmt::Display for FcmcTerm {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FcmcTerm::Skip => write!(f, "*"),
            FcmcTerm::Variable { name } => write!(f, "{}", name),
            FcmcTerm::Pop {
                location_id,
                arg,
                next,
            } => match **next {
                FcmcTerm::Skip => write!(f, "{}<{}>", location_id, arg),
                _ => write!(f, "{}<{}>.{}", location_id, arg, next),
            },
            FcmcTerm::Push {
                term,
                location_id,
                next,
            } => match **next {
                FcmcTerm::Skip => write!(f, "[{}]{}", term, location_id),
                _ => write!(f, "[{}]{}.{}", term, location_id, next),
            },
            FcmcTerm::Seq { term, next } => match **next {
                FcmcTerm::Skip => write!(f, "{}", term),
                _ => write!(f, "{};{}", term, next),
            },
            FcmcTerm::Fork { forked, cont } => match **cont {
                FcmcTerm::Skip => write!(f, "{{{}}}", forked),
                _ => write!(f, "{{{}}}.{}", forked, cont),
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FcmcClosure {
    term: FcmcTerm,
    env: Vec<(Var, FcmcClosure)>,
}

impl FcmcClosure {
    pub fn new(term: FcmcTerm, env: Vec<(Var, FcmcClosure)>) -> Self {
        FcmcClosure { term, env }
    }

    pub fn retrieve_term(mut self) -> FcmcTerm {
        match self.term {
            FcmcTerm::Skip => self.term,
            FcmcTerm::Variable { name } => {
                if self.env.is_empty() {
                    FcmcTerm::new_variable(&name)
                } else {
                    let env_last = self.env.pop().unwrap();
                    if &env_last.0 == &name {
                        env_last.1.retrieve_term()
                    } else {
                        self.term = FcmcTerm::Variable { name };
                        self.retrieve_term()
                    }
                }
            }
            FcmcTerm::Pop {
                location_id,
                arg,
                next,
            } => {
                let mut e = self.env;
                e.push((
                    arg.clone(),
                    FcmcClosure::new(FcmcTerm::new_variable(&arg), vec![]),
                ));
                let c = FcmcClosure::new(*next, e);
                FcmcTerm::new_pop(location_id, &arg, c.retrieve_term())
            }
            FcmcTerm::Push {
                term,
                location_id,
                next,
            } => FcmcTerm::new_push(
                FcmcClosure::new(*term, self.env.clone()).retrieve_term(),
                location_id,
                FcmcClosure::new(*next, self.env).retrieve_term(),
            ),
            FcmcTerm::Seq { term, next } => FcmcTerm::new_seq(
                FcmcClosure::new(*term, self.env.clone()).retrieve_term(),
                FcmcClosure::new(*next, self.env).retrieve_term(),
            ),
            FcmcTerm::Fork { forked, cont } => FcmcTerm::new_fork(
                FcmcClosure::new(*forked, self.env.clone()).retrieve_term(),
                FcmcClosure::new(*cont, self.env).retrieve_term(),
            ),
        }
    }
}

impl fmt::Display for FcmcClosure {
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
pub struct Memory {
    bufs: HashMap<Var, (Sender<FcmcClosure>, Receiver<FcmcClosure>)>,
}

impl Memory {
    /// create a ['Memory'][Self] containing senders and receivers for each location
    /// locations in FCMC are channels
    pub fn new(locations: HashSet<Var>) -> Self {
        let mut bufs = HashMap::new();
        for name in locations.iter() {
            let (send, recv) = unbounded();
            bufs.insert(name.clone(), (send, recv));
        }
        Memory { bufs }
    }

    pub fn pop(&mut self, location: Var) -> FcmcClosure {
        self.bufs
            .get(&location)
            .expect("No location exists with specified name")
            .1
            .recv()
            .expect("Failed to pop from location")
    }

    fn pop_all(&self, location: Var) -> Vec<FcmcClosure> {
        self.bufs
            .get(&location)
            .expect("No location exists with specified name")
            .1
            .try_iter()
            .collect()
    }

    pub fn push(&mut self, location: Var, closure: FcmcClosure) {
        self.bufs
            .get(&location)
            // .expect("No location exists with specified name")
            .unwrap_or_else(|| panic!("No location exists with specified name: {location}"))
            .0
            .send(closure)
            .expect("Failed to push to location")
    }

    pub fn is_empty(&self, location: Var) -> bool {
        self.bufs
            .get(&location)
            .expect("No location exists with specified name")
            .1
            .is_empty()
    }

    fn readback(&mut self) -> Vec<(Var, FcmcTerm)> {
        let mut res = vec![];
        for name in self.bufs.keys() {
            let closures = self.pop_all(name.clone());
            for closure in closures {
                res.push((name.clone(), closure.retrieve_term()));
            }
        }
        res
    }
}

#[derive(Clone, Debug)]
pub struct FcmcThreadState {
    closure: FcmcClosure,
    continuation: Vec<FcmcClosure>,
    memory: Memory,
}

impl FcmcThreadState {
    pub fn new(closure: FcmcClosure, continuation: Vec<FcmcClosure>, memory: Memory) -> Self {
        FcmcThreadState {
            closure,
            continuation,
            memory,
        }
    }

    fn final_(&self) -> bool {
        match self.closure.term {
            FcmcTerm::Skip => self.continuation.is_empty(),
            _ => false,
        }
    }

    pub fn run_thread(&mut self) -> Result<(), String> {
        while !self.final_() {
            self.step()?;
        }
        Ok(())
    }

    fn step(&mut self) -> Result<(), String> {
        match self.closure.term.clone() {
            FcmcTerm::Skip => {
                if !self.final_() {
                    self.closure = self.continuation.pop().unwrap();
                }
            }
            FcmcTerm::Variable { name } => {
                let env_last = self.closure.env.pop().unwrap();
                if name.clone() == env_last.0 {
                    self.closure = env_last.1;
                }
            }
            FcmcTerm::Pop {
                location_id,
                arg,
                next,
            } => {
                let stack_last = self.memory.pop(location_id);
                self.closure.term = *next;
                self.closure.env.push((arg.clone(), stack_last.clone()));
            }
            FcmcTerm::Push {
                term,
                location_id,
                next,
            } => {
                self.closure.term = *next;
                self.memory.push(
                    location_id,
                    FcmcClosure::new(*term, self.closure.env.clone()),
                );
            }
            FcmcTerm::Seq { term, next } => {
                self.continuation
                    .push(FcmcClosure::new(*next, self.closure.env.clone()));
                self.closure.term = *term;
            }
            FcmcTerm::Fork { forked, cont } => {
                let mut new_thread = FcmcThreadState::new(
                    FcmcClosure::new(*forked, self.closure.env.clone()),
                    vec![],
                    self.memory.clone(),
                );
                thread::spawn(move || {
                    println!("New thread spawned: {}", new_thread.closure.term);
                    new_thread.run_thread();
                });
                self.closure.term = *cont;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct FcmcProgramState {
    main_thread: FcmcThreadState,
}

impl FcmcProgramState {
    pub fn new(closure: FcmcClosure, memory: Memory, continuation: Vec<FcmcClosure>) -> Self {
        FcmcProgramState {
            main_thread: FcmcThreadState::new(closure, continuation, memory.clone()),
        }
    }

    fn start(t: FcmcTerm, m: Memory) -> Self {
        FcmcProgramState::new(FcmcClosure::new(t, vec![]), m, vec![])
    }

    pub fn run(term: FcmcTerm) -> Vec<(Var, FcmcTerm)> {
        let locations = term.location_scan();
        let memory = Memory::new(locations);
        let mut state = FcmcProgramState::start(term, memory);
        state.main_thread.run_thread();

        // print final state of memory
        // clone state to prevent other threads continuing to edit memory
        let ans = state.main_thread.memory.clone().readback();
        let len = ans.len();
        for (i, (n, t)) in ans.iter().rev().enumerate() {
            print!("{}: {}", n, t);
            if i < len - 1 {
                println!(", ");
            }
        }
        println!();
        ans
    }
}

impl fmt::Display for FcmcProgramState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // closure
        write!(f, "({}, ", self.main_thread.closure)?;
        // memory
        if self.main_thread.memory.bufs.is_empty() {
            write!(f, "[], ")?;
        } else {
            for location in &self.main_thread.memory.bufs {
                if self.main_thread.memory.is_empty(location.0.clone()) {
                    write!(f, "{}[], ", location.0)?;
                } else {
                    write!(f, "{}[", location.0)?;
                    let len = location.1 .1.len();
                    for (i, t) in location.1 .1.try_iter().enumerate() {
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
        if self.main_thread.continuation.is_empty() {
            write!(f, "[])")
        } else {
            write!(f, "[")?;
            let len = self.main_thread.continuation.len();
            for (i, t) in self.main_thread.continuation.iter().rev().enumerate() {
                write!(f, "({})", t)?;
                if i < len - 1 {
                    write!(f, ", ")?;
                }
            }
            write!(f, "])")
        }
    }
}
