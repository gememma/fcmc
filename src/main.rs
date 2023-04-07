#![feature(box_patterns, box_syntax)]
extern crate lalrpop_util;

use crate::fcmc::{FcmcProgramState, FcmcTerm};
use crate::lambdaterm::LambdaTerm;
use crate::pam::PState;
use crate::parser::FcmcTermParser;
use clap::Parser;
use lalrpop_util::lalrpop_mod;
use std::io;

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
    println!("Input a term: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let parser = FcmcTermParser::new();
    let parsed: FcmcTerm = parser.parse(&input.trim()).expect("");
    println!("{}", parsed);
    FcmcProgramState::run(parsed);
}

pub fn run_example() {
    println!("{}", FcmcTerm::term1());
    FcmcProgramState::run(FcmcTerm::term1());
    println!("");
}

pub fn print_help() {
    println!("To see an example of a term, choose option 2 in the menu.");
    println!("When you run a term, it will print out the term and then run it.");
    println!("Any time a term is forked, the term on the new thread will be printed.");
    println!("The output of the term will be printed as a list of locations and their contents.");
    println!("");
}

fn main() {
    loop {
        println!(
            "Welcome to the FCMC abstract machine.\n\
        1. Run an FCMC term\n\
        2. Run an example term\n\
        3. Help\n\
        4. Exit\n\
        [Please choose an option] "
        );
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        match &*input.trim() {
            "1" => run_parser(),
            "2" => run_example(),
            "3" => print_help(),
            "4" => break,
            _ => {
                println!("Invalid choice. Input a number from 1 to 4.")
            }
        }
    }
}
