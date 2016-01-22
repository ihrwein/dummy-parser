#[macro_use]
extern crate syslog_ng_common;
#[macro_use]
extern crate log;

#[derive(Clone)]
pub struct DummyParser;

#[derive(Clone)]
pub struct DummyParserBuilder;

use syslog_ng_common::{LogMessage, Parser, ParserBuilder, OptionError};

impl ParserBuilder for DummyParserBuilder {
    type Parser = DummyParser;
    fn new() -> Self {
        DummyParserBuilder
    }
    fn option(&mut self, name: String, value: String) {
        debug!("Setting option: {}={}", name, value);
    }
    fn build(self) -> Result<Self::Parser, OptionError> {
        debug!("Building Rust parser");
        Ok(DummyParser)
    }
}

impl Parser for DummyParser {
    fn parse(&mut self, _: &mut LogMessage, input: &str) -> bool {
        debug!("Processing input in Rust Parser: {}", input);
        true
    }
}

parser_plugin!(DummyParserBuilder);
