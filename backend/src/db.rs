use std::option::Option;
use std::path::PathBuf;

// refer to
// https://github.com/lucperkins/rust-graphql-juniper-actix-diesel-postgres/blob/015cf2e116124f8553ee31263ff29ecc8a1bfa3f/src/db.rs
#[derive(Clone)]
pub struct Bayard<'a> {
    pub node_id: &'a str,
    pub host: &'a str,
    pub index_port: &'a str,
    pub schema: Option<PathBuf>,
    pub tokenizer: Option<PathBuf>,
}
