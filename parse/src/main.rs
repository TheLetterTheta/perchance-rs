use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

mod ast;
mod error;
mod macros;

use chrono::Utc;
use error::{ParseError, ParseResult};
use fern::InitError;
use lazy_static::lazy_static;
use pest::Parser;
use pest_derive::Parser;
use regex::Regex;

use crate::{
    ast::{item::Item, Parse},
    error::unexpected,
};

lazy_static! {
    pub static ref RE_STARTSWITH_WHITESPACE: Regex = Regex::new(r#"^\s+"#).unwrap();
}

pub const NEWLINE_CHR: &'static str = "\n";

#[derive(Parser)]
#[grammar = "perchance.pest"]
struct PerchanceParser();

fn setup_logger() -> ParseResult<()> {
    match fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                Utc::now().to_rfc3339(),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Trace)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()
    {
        Ok(_) => Ok(()),
        Err(_) => {
            // Eat the set_logger error, we dont care if we're trying to re-init the logger.
            Ok(())
        }
    }
}
fn main() {
    let items = parse_file("samples/complex.pc").expect("failed");
    println!("{:#?}", items);
}

fn parse_file<P: AsRef<Path>>(path: P) -> ParseResult<Vec<Item>> {
    setup_logger()?;
    let unparsed_file = fs::read_to_string(path)?;

    let mut file = match PerchanceParser::parse(Rule::file, &unparsed_file) {
        Err(e) => {
            eprintln!("{}", e);
            panic!("{:?}", e.variant)
        }
        Ok(f) => f,
    };

    let file = file.next().unwrap(); // get and unwrap the `file` rule; never fails

    let items = file
        .into_inner()
        .filter_map(|line| match line.as_rule() {
            Rule::section => Some(Item::parse(line)),
            Rule::EOI => None,
            rule => Some(unexpected("parse-file", rule)),
        })
        .collect::<ParseResult<Vec<_>>>()?;

    Ok(items)
}
