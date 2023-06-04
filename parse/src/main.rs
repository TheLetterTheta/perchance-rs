use std::fs;

mod ast;
mod error;
mod macros;

use chrono::Utc;
use lazy_static::lazy_static;
use pest::Parser;
use pest_derive::Parser;
use regex::Regex;

use crate::ast::{item::Item, Parse};

lazy_static! {
    pub static ref RE_STARTSWITH_WHITESPACE: Regex = Regex::new(r#"^\s+"#).unwrap();
}

pub const NEWLINE_CHR: &'static str = "\n";

#[derive(Parser)]
#[grammar = "perchance.pest"]
struct PerchanceParser();

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
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
        .apply()?;
    Ok(())
}
fn main() {
    setup_logger().unwrap();
    let unparsed_file = fs::read_to_string("samples/pack.pc").expect("cannot read file");

    let mut file = match PerchanceParser::parse(Rule::file, &unparsed_file) {
        Err(e) => {
            eprintln!("{}", e);
            panic!("{:?}", e.variant)
        }
        Ok(f) => f,
    };
    let file = file.next().unwrap(); // get and unwrap the `file` rule; never fails

    for line in file.into_inner() {
        match line.as_rule() {
            Rule::section => {
                let item = Item::parse(line).unwrap();
                println!("item={:#?}", item)
            }
            _ => println!("{:#?}: {:?}", line.as_rule(), line.into_inner()),
        }
    }
}
