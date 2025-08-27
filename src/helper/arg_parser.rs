use clap::{Parser};
use crate::model::{argument::Argument};

pub fn arg_parser() -> Argument {
    let args = Argument::parse();
    return args;
}
