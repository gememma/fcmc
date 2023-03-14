use crate::lambdaterm::Var;

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
}
