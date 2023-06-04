#![allow(clippy::result_large_err)]
// TODO: Remove this ^^^

use std::{fs, path::Path};

mod ast;
mod error;
mod integration;
mod macros;

use chrono::Utc;
use error::ParseResult;
use pest::Parser;
use pest_derive::Parser;

use crate::{
    ast::{item::Item, Parse},
    error::unexpected,
};

#[derive(Parser)]
#[grammar = "perchance.pest"]
struct PerchanceParser();

fn setup_logger() -> ParseResult<()> {
    let _ = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                Utc::now().to_rfc3339(),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply();
    // Eat the set_logger error, we dont care if we're trying to re-init the logger.
    Ok(())
}

pub fn parse_file<P: AsRef<Path>>(path: P) -> ParseResult<Vec<Item>> {
    setup_logger()?;
    let unparsed_file = fs::read_to_string(path)?;

    parse_string(unparsed_file)
}

pub fn parse_string<S: ToString>(content: S) -> ParseResult<Vec<Item>> {
    let content = content.to_string();
    let mut file = PerchanceParser::parse(Rule::file, &content)?;

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
