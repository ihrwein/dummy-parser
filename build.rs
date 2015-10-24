extern crate syslog_ng_build;

fn main() {
    let canonical_name = "dummy-parser";
    let description = "This is the DummyParser written in Rust";
    let parser_name = "dummy-rs";
    syslog_ng_build::create_module(canonical_name, description, Some(parser_name));
}
