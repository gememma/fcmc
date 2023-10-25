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

    /// Return a [`LambdaTerm`] for the Church encoding of the given [`bool`]
    pub fn new_bool(bool: bool) -> Self {
        let x = if bool { "a" } else { "b" };
        LambdaTerm::Lambda {
            arg: "a".to_string(),
            body: box LambdaTerm::Lambda {
                arg: "b".to_string(),
                body: box LambdaTerm::new_var(x),
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

    /// Return a single [`Var`] not in `used`
    pub fn fresh_from_used(used: HashSet<Var>) -> Var {
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

    /// Return a single [`Var`] not returned by calling [`get_used_names()`][Self::get_used_names()]
    pub fn get_fresh_name(&self) -> Var {
        let used = self.get_used_names();
        LambdaTerm::fresh_from_used(used)
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

    /// Rename a [`Var`] in self and return the new term
    pub fn renamed(&self, old: &Var, new: &Var) -> Self {
        let mut t = self.clone();
        t.rename(old, new);
        t
    }

    /// Perform a substitution and return the new term
    pub fn substitute(&self, old: &Var, new: &LambdaTerm) -> Self {
        match self {
            LambdaTerm::Variable { name } => {
                if name == old {
                    new.clone()
                } else {
                    self.clone()
                }
            }
            LambdaTerm::Lambda { arg, body } => {
                if arg == old {
                    self.clone()
                } else {
                    let mut a = &self.get_used_names() | &new.get_used_names();
                    a.insert(old.clone());
                    let z = LambdaTerm::fresh_from_used(a);
                    LambdaTerm::Lambda {
                        arg: z.clone(),
                        body: Box::new(body.renamed(arg, &z).substitute(old, new)),
                    }
                }
            }
            LambdaTerm::Apply { t1, t2 } => {
                let n1 = t1.substitute(old, new);
                let n2 = t2.substitute(old, new);
                LambdaTerm::Apply {
                    t1: Box::new(n1),
                    t2: Box::new(n2),
                }
            }
        }
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
    use super::LambdaTerm;
    use super::*;

    #[test]
    fn produces_used_names() {
        let term = LambdaTerm::example5();
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
        let term = LambdaTerm::example5();
        assert_eq!(term.get_fresh_name(), 'c'.to_string());
    }

    #[test]
    fn renames() {
        let mut term = LambdaTerm::example5();
        term.rename(&'b'.to_string(), &'z'.to_string());
        assert_eq!(term, LambdaTerm::example6());
    }

    #[test]
    fn substitutes() {
        let zero = LambdaTerm::new_num(0);
        let term = LambdaTerm::example5().substitute(&'b'.to_string(), &zero);
        assert_eq!(term, *LambdaTerm::example7());
    }
}
