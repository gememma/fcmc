#![feature(box_patterns, box_syntax)]
extern crate lalrpop_util;

use crate::fcmc::LambdaTermParser;
use crate::kam::{Closure, State};
use crate::lambdaterm::LambdaTerm;
use crate::pam::PState;
use crate::sam::{SLambdaTerm, SState};
use crate::LambdaTerm::{Apply, Lambda};
use clap::Parser;
use lalrpop_util::lalrpop_mod;

pub mod examples;
pub mod kam;
pub mod lambdaterm;
pub mod pam;
pub mod sam;

lalrpop_mod!(pub fcmc);

#[allow(dead_code)]
fn run_misc_examples() {
    let example = box LambdaTerm::new_var("a");
    let example2 = box Lambda {
        arg: "x".to_string(),
        body: example,
    };
    let example3 = box Lambda {
        arg: "y".to_string(),
        body: box LambdaTerm::new_var("y"),
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
    println!("{}\n", loopy);

    let example8 = Apply {
        t1: box Apply {
            t1: box Lambda {
                arg: "x".to_string(),
                body: box Lambda {
                    arg: "y".to_string(),
                    body: box LambdaTerm::new_var("x"),
                },
            },
            t2: box LambdaTerm::new_bool(true),
        },
        t2: box LambdaTerm::new_bool(false),
    };

    println!("{}", State::state4());

    State::run(Closure::closure2().retrieve_term());

    State::run(example8);

    let term2 = LambdaTerm::Apply {
        t1: box LambdaTerm::Apply {
            t1: box LambdaTerm::Lambda {
                arg: "b".to_string(),
                body: box LambdaTerm::Apply {
                    t1: box LambdaTerm::Lambda {
                        arg: "a".to_string(),
                        body: box LambdaTerm::Lambda {
                            arg: "x".to_string(),
                            body: box LambdaTerm::Apply {
                                t1: box LambdaTerm::Apply {
                                    t1: box LambdaTerm::Lambda {
                                        arg: "y".to_string(),
                                        body: box LambdaTerm::new_var("a"),
                                    },
                                    t2: box LambdaTerm::new_var("x"),
                                },
                                t2: box LambdaTerm::new_var("b"),
                            },
                        },
                    },
                    t2: box LambdaTerm::new_bool(true),
                },
            },
            t2: box LambdaTerm::Lambda {
                arg: "z".to_string(),
                body: box LambdaTerm::new_var("z"),
            },
        },
        t2: box LambdaTerm::new_bool(false),
    };

    println!("{}", PState::p_run(term2));

    // sequential calculus
    println!("{}", SLambdaTerm::term1());
    println!("{}", SLambdaTerm::term2());
    println!("{}", SLambdaTerm::term3());
    SState::run(SLambdaTerm::term3());
}

#[derive(Parser)]
struct Args {
    /// Raw term to run
    input: String,
}

pub fn run_parser() {
    let input = Args::parse().input;
    // let input = r#"(\b. (\a. \x. (\y. a) x b) (\a. \b. a)) (\z. z) (\a. \b. b)"#;
    let parser = LambdaTermParser::new();
    let output: LambdaTerm = parser.parse(&input).expect("");
    println!("{}", output);
    assert_eq!(output, LambdaTerm::term2());
    State::run(output);
}

fn main() {
    // run_misc_examples();
    run_parser();
}
