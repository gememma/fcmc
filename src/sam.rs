use crate::lambdaterm::Var;

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

#[derive(Clone, Debug, PartialEq)]
pub struct SClosure {
    term: SLambdaTerm,
    env: Vec<(Var, SClosure)>,
}

impl SClosure {
    pub fn new(term: SLambdaTerm, env: Vec<(Var, SClosure)>) -> Self {
        SClosure { term, env }
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

    fn final_(&self) -> bool {
        match self.closure.term {
            SLambdaTerm::Skip => true,
            _ => false,
        }
    }
}
