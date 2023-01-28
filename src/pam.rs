use crate::lamdaterm::LambdaTerm;

pub struct PState {
    term: LambdaTerm,
    stack: Vec<LambdaTerm>,
}

#[derive(Clone, Debug, PartialEq)]
impl PState {
    pub fn state1() -> PState {
        PState {
            term: LambdaTerm::Lambda {
                arg: "x".to_string(),
                body: box LambdaTerm::Lambda {
                    arg: "y".to_string(),
                    body: box LambdaTerm::new_var("x"),
                },
            },
            stack: vec![LambdaTerm::new_num(1), LambdaTerm::new_num(0)],
        }
    }
}

#[cfg(test)]
mod tests {
    todo!();
}
