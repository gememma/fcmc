use crate::lambdaterm::Var;
use std::fmt;
use std::fmt::Formatter;

/// The sequential lambda-calculus takes an altered version of the lambda-calculus as grammar
#[derive(Clone, Debug, PartialEq)]
pub enum SLambdaTerm {
    Skip,
    Variable {
        name: Var,
    },
    Pop {
        arg: Var,
        next: Box<SLambdaTerm>,
    },
    Push {
        term: Box<SLambdaTerm>,
        next: Box<SLambdaTerm>,
    },
    Seq {
        term: Box<SLambdaTerm>,
        next: Box<SLambdaTerm>,
    },
}

impl SLambdaTerm {
    /// Turn a [`&str`] into a [`Variable`][SLambdaTerm::Variable]
    pub fn new_variable(name: &str) -> Self {
        SLambdaTerm::Variable {
            name: name.to_string(),
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

    /// Turn two [`SLambdaTerm`]s into a [`Seq`][SLambdaTerm::Push]
    pub fn new_seq(term: SLambdaTerm, next: SLambdaTerm) -> Self {
        SLambdaTerm::Push {
            term: box term,
            next: box next,
        }
    }
}

impl fmt::Display for SLambdaTerm {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SLambdaTerm::Skip => write!(f, "*"),
            SLambdaTerm::Variable { name } => write!(f, "{}", name),
            SLambdaTerm::Pop { arg, next } => match **next {
                SLambdaTerm::Skip => write!(f, "<{}>", arg),
                _ => write!(f, "<{}>.{}", arg, next),
            },
            SLambdaTerm::Push { term, next } => match **next {
                SLambdaTerm::Skip => write!(f, "[{}]", term),
                _ => write!(f, "[{}].{}", term, next),
            },
            SLambdaTerm::Seq { term, next } => match **next {
                SLambdaTerm::Skip => write!(f, "{}", term),
                _ => write!(f, "{};{}", term, next),
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
    /// Create a new ['SClosure'] from a ['SLambdaTerm'] and a ['Vec<(Var, SClosure)>']
    pub fn new(term: SLambdaTerm, env: Vec<(Var, SClosure)>) -> Self {
        SClosure { term, env }
    }

    /// Given a ['SClosure'], perform substitutions from the env and return the final ['SLambdaTerm']
    pub fn retrieve_term(mut self) -> SLambdaTerm {
        match self.term {
            SLambdaTerm::Skip => self.term,
            SLambdaTerm::Variable { name } => {
                if self.env.is_empty() {
                    SLambdaTerm::Variable { name }
                } else {
                    let env_last = self.env.pop().unwrap();
                    if &env_last.0 == &name {
                        env_last.1.retrieve_term()
                    } else {
                        self.term = SLambdaTerm::Variable { name };
                        self.retrieve_term()
                    }
                }
            }
            SLambdaTerm::Pop { arg, next } => {
                let mut e = self.env;
                e.push((
                    arg.clone(),
                    SClosure::new(SLambdaTerm::new_variable(&arg), vec![]),
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
            SLambdaTerm::Seq { term, next } => SLambdaTerm::Seq {
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
    continuation: Vec<SClosure>,
}

impl SState {
    /// Create a new ['SState'] from a ['SClosure'], a stack: ['Vec<SClosure>'] and a continuation: ['Vec<(SClosure)>']
    pub fn new(closure: SClosure, stack: Vec<SClosure>, continuation: Vec<SClosure>) -> Self {
        SState {
            closure,
            stack,
            continuation,
        }
    }

    /// Create a start state: ['SState'] from a term: ['SLambdaTerm'] and a stack: ['Vec<SClosure>']
    fn start(t: SLambdaTerm, s: Vec<SClosure>) -> SState {
        SState::new(SClosure::new(t, vec![]), s, vec![])
    }

    /// Perform a single step in-place of the abstract machine by mutating ['self'][SState]
    /// returns an error if the term is not a valid program ie. cannot be run
    pub fn step(&mut self) -> Result<(), String> {
        match self.closure.term.clone() {
            SLambdaTerm::Skip => {
                if !self.final_() {
                    self.closure = self.continuation.pop().unwrap();
                }
            }
            SLambdaTerm::Variable { name } => {
                let env_last = self.closure.env.pop().unwrap();
                if name.clone() == env_last.0 {
                    self.closure = env_last.1;
                }
            }
            SLambdaTerm::Pop { arg, next } => {
                if self.stack.is_empty() {
                    return Err("Term cannot be executed. Pop action encountered but execution stack is empty.".to_string());
                } else {
                    let stack_last = self.stack.pop().unwrap();
                    self.closure.term = *next;
                    self.closure.env.push((arg.clone(), stack_last.clone()))
                }
            }
            SLambdaTerm::Push { term, next } => {
                self.closure.term = *next;
                self.stack
                    .push(SClosure::new(*term, self.closure.env.clone()))
            }
            SLambdaTerm::Seq { term, next } => {
                self.continuation
                    .push(SClosure::new(*next, self.closure.env.clone()));
                self.closure.term = *term;
            }
        }
        Ok(())
    }

    /// Given ['self'][SState], return the equivalent stack of ['SLambdaTerm']s
    fn readback(&mut self) -> Vec<SLambdaTerm> {
        let mut res = vec![];
        for c in self.stack.iter() {
            res.push(c.clone().retrieve_term());
        }
        res
    }

    /// Returns true if ['self'][SState] is a final state ie. computation is complete
    fn final_(&self) -> bool {
        match self.closure.term {
            SLambdaTerm::Skip => self.continuation.is_empty(),
            _ => false,
        }
    }

    /// Run the given ['SLambdaTerm'] on the sequential abstract machine, printing each step and the output
    pub fn run(term: SLambdaTerm) -> Vec<SLambdaTerm> {
        let mut s = SState::start(term, vec![]);
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
        for (i, t) in ans.iter().rev().enumerate() {
            print!("{}", t);
            if i < len - 1 {
                println!(", ");
            }
        }
        ans
    }
}

impl fmt::Display for SState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, ", self.closure)?;
        if self.stack.is_empty() {
            write!(f, "[], ")?;
        } else {
            write!(f, "[")?;
            let len = self.stack.len();
            for (i, t) in self.stack.iter().rev().enumerate() {
                write!(f, "({})", t)?;
                if i < len - 1 {
                    write!(f, ", ")?;
                }
            }
            write!(f, "], ")?;
        }
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

#[cfg(test)]
mod tests {
    use crate::sam::{SClosure, SLambdaTerm, SState};

    #[test]
    fn prints_seq_state() {
        let s = SState::state1();
        assert_eq!(s.to_string(), "([<x>.[x]].<f>.f;f;f, [(\"y\", [<x>.[x]].<f>.f;f;f, [])], [([<x>.[x]].<f>.f;f;f, [(\"y\", [<x>.[x]].<f>.f;f;f, [])]), (<x>.<y>, [(\"y\", <x>.[x].[x], [])])], [])");
    }

    #[test]
    fn prints_seq_closure() {
        let c = SClosure::closure1();
        assert_eq!(
            c.to_string(),
            "[<x>.[x]].<f>.f;f;f, [(\"y\", [<x>.[x]].<f>.f;f;f, [])]"
        );
    }

    #[test]
    fn term_into_state() {
        let s = SState::start(SLambdaTerm::term1(), vec![]);
        assert_eq!(s.to_string(), "(<x>.[x].[x], [], [], [])");
    }

    #[test]
    fn detect_end_state() {
        let s = SState::state1();
        let s2 = SState::state2();
        assert_eq!(s.final_(), false);
        assert_eq!(s2.final_(), true);
    }

    #[test]
    fn closure_to_term() {
        let ans = SState::state2().readback();
        assert_eq!(ans, vec![SLambdaTerm::term2(), SLambdaTerm::term3()])
    }

    #[test]
    fn run_term4() {
        let ans = SState::run(SLambdaTerm::term4());
        assert_eq!(
            ans,
            vec![
                SLambdaTerm::Variable {
                    name: "y".to_string()
                },
                SLambdaTerm::Variable {
                    name: "x".to_string()
                },
                SLambdaTerm::Variable {
                    name: "y".to_string()
                },
                SLambdaTerm::Variable {
                    name: "x".to_string()
                },
            ]
        );
    }
}
