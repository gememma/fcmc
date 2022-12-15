use std::collections::HashSet;
use std::fmt;
use std::fmt::Formatter;

#[derive(Clone, Debug, PartialEq)]
pub enum LambdaTerm {
    Variable {
        name: Var,
    },
    Lambda {
        arg: Var,
        body: Box<LambdaTerm>,
    },
    Apply {
        t1: Box<LambdaTerm>,
        t2: Box<LambdaTerm>,
    },
}

pub type Var = String;

impl LambdaTerm {
    /// Turn a [`&str`] into a [`Variable`][LambdaTerm::Variable]
    pub fn new_var(name: &str) -> Self {
        LambdaTerm::Variable {
            name: name.to_string(),
        }
    }

    /// Return a [`LambdaTerm`] for the Church encoding of the given [`usize`]
    pub fn new_num(numeral: usize) -> Self {
        fn new_num_sub(numeral: usize) -> LambdaTerm {
            if numeral == 0 {
                LambdaTerm::new_var("x")
            } else {
                LambdaTerm::Apply {
                    t1: box LambdaTerm::new_var("f"),
                    t2: box new_num_sub(numeral - 1),
                }
            }
        }
        LambdaTerm::Lambda {
            arg: "f".to_string(),
            body: box LambdaTerm::Lambda {
                arg: "x".to_string(),
                body: box new_num_sub(numeral),
            },
        }
    }

    /// Return all [`Var`]s that occur in self
    pub fn get_used_names(&self) -> HashSet<Var> {
        match self {
            LambdaTerm::Variable { name } => [name.to_string()].into(),
            LambdaTerm::Lambda { arg, body } => {
                let mut names = body.get_used_names();
                names.insert(arg.to_string());
                names
            }
            LambdaTerm::Apply { t1, t2 } => &t1.get_used_names() | &t2.get_used_names(),
        }
    }

    /// Return a single [`Var`] not returned by calling [`get_used_names()`][Self::get_used_names()]
    pub fn get_fresh_name(&self) -> Var {
        let used = self.get_used_names();
        for c in 'a'..='z' {
            let var = c.to_string();
            if !used.contains(&var) {
                return var;
            }
        }
        for (c, i) in ('a'..='z').into_iter().cycle().zip(1..) {
            let var = format!("{}{}", c, i);
            if !used.contains(&var) {
                return var;
            }
        }
        unreachable!()
    }

    /// Rename a [`Var`] in self in-place
    pub fn rename(&mut self, old: &Var, new: &Var) {
        match self {
            LambdaTerm::Variable { name } => {
                if name == old {
                    *name = new.clone();
                }
            }
            LambdaTerm::Lambda { arg, body } => {
                if arg != old {
                    body.rename(old, new);
                }
            }
            LambdaTerm::Apply { t1, t2 } => {
                t1.rename(old, new);
                t2.rename(old, new);
            }
        }
    }

    pub fn substitute(&self, _old: &Var, _new: &Var) -> Self {
        todo!()
    }
}

impl fmt::Display for LambdaTerm {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            LambdaTerm::Variable { name } => {
                write!(f, "{}", name)
            }
            LambdaTerm::Lambda { arg, body } => {
                write!(f, "\\{}. {}", arg, body)
            }
            LambdaTerm::Apply {
                t1: t1 @ box LambdaTerm::Variable { .. },
                t2: t2 @ box LambdaTerm::Variable { .. },
            }
            | LambdaTerm::Apply {
                t1: t1 @ box LambdaTerm::Apply { .. },
                t2: t2 @ box LambdaTerm::Variable { .. },
            } => {
                write!(f, "{} {}", t1, t2)
            }
            LambdaTerm::Apply {
                t1: t1 @ box LambdaTerm::Variable { .. },
                t2,
            } => {
                write!(f, "{} ({})", t1, t2)
            }
            LambdaTerm::Apply {
                t1,
                t2: t2 @ box LambdaTerm::Variable { .. },
            } => {
                write!(f, "({}) {}", t1, t2)
            }
            LambdaTerm::Apply { t1, t2 } => {
                write!(f, "({}) ({})", t1, t2)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LambdaTerm::{Apply, Lambda};
    use super::*;

    fn example5() -> Box<LambdaTerm> {
        box Lambda {
            arg: "a".to_string(),
            body: box Lambda {
                arg: "x".to_string(),
                body: box Apply {
                    t1: box Apply {
                        t1: box Lambda {
                            arg: "y".to_string(),
                            body: box LambdaTerm::new_var("a"),
                        },
                        t2: box LambdaTerm::new_var("x"),
                    },
                    t2: box LambdaTerm::new_var("b"),
                },
            },
        }
    }

    fn example6() -> Box<LambdaTerm> {
        box Lambda {
            arg: "a".to_string(),
            body: box Lambda {
                arg: "x".to_string(),
                body: box Apply {
                    t1: box Apply {
                        t1: box Lambda {
                            arg: "y".to_string(),
                            body: box LambdaTerm::new_var("a"),
                        },
                        t2: box LambdaTerm::new_var("x"),
                    },
                    t2: box LambdaTerm::new_var("z"),
                },
            },
        }
    }

    #[test]
    fn produces_used_names() {
        let term = example5();
        assert_eq!(
            term.get_used_names(),
            ["x", "b", "y", "a"]
                .into_iter()
                .map(|ea| ea.to_string())
                .collect::<HashSet<Var>>()
        );
    }

    #[test]
    fn produces_fresh_name() {
        let term = example5();
        assert_eq!(term.get_fresh_name(), 'c'.to_string());
    }

    #[test]
    fn renames() {
        let mut term = example5();
        term.rename(&'b'.to_string(), &'z'.to_string());
        assert_eq!(term, example6());
    }
}
