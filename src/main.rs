mod filesystem;
mod index;
mod manager;
mod parser;
mod query;
mod record;
mod util;

use clap::Parser;

fn main() {
    let config = manager::RunConfig::parse();

    let manager = manager::DatabaseManager::new(config);

    manager.run();
}
