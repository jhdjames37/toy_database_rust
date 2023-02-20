mod builder;
mod record_selector;
mod runner;
mod select_validator;
mod types;

pub use builder::QueryBuilder;
pub use runner::{QueryResult, QueryRunner};

pub enum Header {
    // table, column
    Selector(Vec<(String, String, bool)>),
    // table, column, op, deletable
    Aggregator(Vec<(String, String, String, bool)>),
}
