#![feature(box_patterns, box_syntax)]
extern crate lalrpop_util;

use crate::fcmc::{FcmcProgramState, FcmcTerm};
use crate::kam::{Closure, State};
use crate::lambdaterm::LambdaTerm;
use crate::pam::PState;
use crate::parser::FcmcTermParser;
use crate::sam::{SLambdaTerm, SState};
use crate::LambdaTerm::{Apply, Lambda};
use clap::Parser;
use lalrpop_util::lalrpop_mod;

pub mod examples;
pub mod fcmc;
pub mod fmc;
pub mod kam;
pub mod lambdaterm;
pub mod pam;
pub mod sam;

lalrpop_mod!(pub parser);

#[derive(Parser)]
struct Args {
    /// Raw term to run
    input: String,
}

pub fn run_parser() {
    let input = Args::parse().input;
    let parser = FcmcTermParser::new();
    let output: FcmcTerm = parser.parse(&input).expect("");
    println!("{}", output);
    FcmcProgramState::run(output);
}

fn main() {
    run_parser();
}
