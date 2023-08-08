use crate::{
    builder::du_builder::DuBuilder,
    cl_parser::parser::{CliOption, CliParser},
};

pub mod builder;
pub mod cl_parser;
pub mod core;

fn main() {
    println!("Hello, world!");
    let p = CliParser::parse();
    DuBuilder::build_and_run(p)
}
