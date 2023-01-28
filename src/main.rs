#![feature(box_patterns, box_syntax)]

use crate::lamdaterm::LambdaTerm;
use crate::LambdaTerm::{Apply, Lambda};

pub mod lamdaterm;
pub mod pam;

fn main() {
    let example = box LambdaTerm::new_var("a");
    let example2 = box Lambda {
        arg: "x".to_string(),
        body: example,
    };
    let example3 = box Lambda {
        arg: "y".to_string(),
        body: Box::new(LambdaTerm::new_var("y")),
    };
    let example4 = box Apply {
        t1: example2,
        t2: example3,
    };
    println!("{}", example4);

    let example5 = box Lambda {
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
    };
    println!("{}", example5);

    // Church numeral 5
    println!("\n{}", LambdaTerm::new_num(5));
    // loop function
    let loopy_sub = Lambda {
        arg: "x".to_string(),
        body: box Apply {
            t1: box LambdaTerm::new_var("x"),
            t2: box LambdaTerm::new_var("x"),
        },
    };
    let loopy = box Apply {
        t1: box loopy_sub.clone(),
        t2: box loopy_sub.clone(),
    };
    println!("\n{}", loopy);
}
