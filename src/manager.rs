use crate::filesystem::LRUBufPageManager as BufPageManager;
use crate::parser::SQLLexer;
use crate::parser::SQLParser;
use crate::query::QueryBuilder;
use crate::query::{QueryResult, QueryRunner};
use crate::record::column::DataType;
use crate::record::column::TableMeta;
use crate::record::data::DataValue;
use crate::record::database::DatabaseTable;

use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::tree::ParseTreeVisitorCompat;
use antlr_rust::BailErrorStrategy;
use antlr_rust::InputStream;
use clap::Parser;
use comfy_table::Table;

use std::cell::RefCell;
use std::io;
use std::io::Write;
use std::rc::Rc;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(version)]
pub struct RunConfig {
    #[arg(long = "path")]
    root_path: String,

    #[arg(long = "cache", default_value_t = 10000)]
    cache_size: usize,
}

pub struct DatabaseManager {
    buf_page: Rc<RefCell<BufPageManager>>,
    db_table: Rc<RefCell<DatabaseTable>>,
    query_runner: Rc<RefCell<QueryRunner>>,
}

mod syntax_error {
    use antlr_rust::tree::{ErrorNode, ParseTreeVisitorCompat};

    use crate::parser::{SQLParserContextType, SQLVisitorCompat};

    pub struct ErrorVisitor(pub bool);
    impl ParseTreeVisitorCompat<'_> for ErrorVisitor {
        type Node = SQLParserContextType;
        type Return = bool;
        fn temp_result(&mut self) -> &mut Self::Return {
            &mut self.0
        }

        fn visit_error_node(&mut self, _node: &ErrorNode<'_, Self::Node>) -> Self::Return {
            true
        }

        fn aggregate_results(&self, aggregate: Self::Return, next: Self::Return) -> Self::Return {
            aggregate || next
        }
    }
    impl SQLVisitorCompat<'_> for ErrorVisitor {}
}

impl DatabaseManager {
    pub fn new(config: RunConfig) -> DatabaseManager {
        let buf_page = Rc::new(RefCell::new(BufPageManager::create_lru(config.cache_size)));
        let db_table = Rc::new(RefCell::new(
            DatabaseTable::new(&config.root_path, &buf_page)
                .expect("Error when creating database!"),
        ));
        let query_runner = Rc::new(RefCell::new(QueryRunner::new(
            &db_table,
            &buf_page,
            &config.root_path,
        )));

        DatabaseManager {
            buf_page,
            db_table,
            query_runner,
        }
    }

    pub fn run(&self) {
        let is_terminal = true;
        if is_terminal {
            println!("Database system. Initialized.")
        }

        'main: loop {
            // TODO: more robust input handling
            // such as, for forget `;'
            if is_terminal {
                if let Some(db) = self.query_runner.borrow().get_db() {
                    print!("({}) ", db);
                }
                print!(">>> ");
                io::stdout().flush().unwrap();
            }
            let mut guess = String::new();

            if io::stdin().read_line(&mut guess).is_err() {
                break;
            }

            // EOF, exit
            if guess.is_empty() {
                break;
            }

            while !guess.trim_end().ends_with(';') {
                if is_terminal {
                    if let Some(db) = self.query_runner.borrow().get_db() {
                        let s = format!("({}) ", db);
                        for _ in 0..s.len() {
                            print!(" ",);
                        }
                    }
                    print!("--> ");
                    io::stdout().flush().unwrap();
                }

                if io::stdin().read_line(&mut guess).is_err() {
                    break 'main;
                }
            }

            // parse input
            let lexer = SQLLexer::new(InputStream::new(&guess[..]));
            let mut parser =
                SQLParser::with_strategy(CommonTokenStream::new(lexer), BailErrorStrategy::new());

            if let Ok(root) = parser.program() {
                if syntax_error::ErrorVisitor(false).visit(&*root) {
                    println!("Error on parsing SQL");
                    continue;
                }

                let mut query_builder = QueryBuilder::new();

                let res = query_builder.visit(&*root);

                let start = Instant::now();
                for item in res {
                    match self.query_runner.borrow_mut().run_cmd(item) {
                        Ok(res) => match res {
                            QueryResult::Nop(x) | QueryResult::PartialSuccess(x, None) => {
                                println!("{} line(s) affected", x);
                            }
                            QueryResult::PartialSuccess(x, Some(e)) => {
                                println!("Error occured: {}", e);
                                println!("{} line(s) affected", x);
                            }
                            QueryResult::Result(header, content) => {
                                self.print(header, content);
                            }
                            QueryResult::MetaResult(meta) => {
                                self.print_meta(meta);
                            }
                        },
                        Err(err) => {
                            println!("Error occured: {}", err)
                        }
                    }
                }

                println!("Elapsed: {} seconds", start.elapsed().as_secs_f64());
            } else {
                println!("Error on parsing SQL");
                continue;
            };
        }
    }

    fn print(&self, header: Vec<String>, content: Vec<Vec<DataValue>>) {
        // Naiive printing functions

        // println!("{}", header.join("\t"));
        let mut data: Vec<Vec<String>> = vec![];
        for i in &content {
            let mut row: Vec<String> = vec![];
            for j in i {
                row.push(print_data(j));
            }
            data.push(row);
        }

        self.print_console(&header, &data);
        println!("{} line(s) in total", content.len());
    }

    fn print_meta(&self, meta: TableMeta) {
        let header = vec![
            "Field".to_string(),
            "Type".to_string(),
            "Null".to_string(),
            "Default".to_string(),
        ];
        let mut data: Vec<Vec<String>> = vec![];
        for i in &meta.field {
            data.push(vec![
                format!("{}", i.name),
                match i.data_type {
                    DataType::Int => "INT".to_string(),
                    DataType::Float => "FLOAT".to_string(),
                    DataType::VarStr(x) => format!("VARCHAR({})", x),
                },
                format!("{}", i.is_nullable),
                print_data(&i.default_value),
            ]);
        }
        self.print_console(&header, &data);
        if meta.field.iter().any(|x| x.is_primary) {
            println!(
                "PRIMARY KEY ({})",
                meta.field
                    .iter()
                    .filter(|x| x.is_primary)
                    .map(|x| x.name.clone())
                    .collect::<Vec<_>>()
                    .join(",")
            )
        }
        for i in meta.foreign_key {
            println!(
                "FOREIGN KEY {} ({}) REFERECES {}({})",
                i.name,
                i.from.join(","),
                i.table,
                i.to.join(",")
            )
        }
        for i in meta.index {
            println!("INDEX ({})", i.col.join(","),)
        }
    }

    fn print_console(&self, header: &Vec<String>, data: &Vec<Vec<String>>) {
        let mut table = Table::new();
        table.set_header(header);
        table.load_preset("||--+==+|    --++++");
        data.iter().for_each(|row| {
            table.add_row(row);
        });
        println!("{table}")
    }
}
fn print_data(data: &DataValue) -> String {
    match data {
        DataValue::Float(j) => format!("{}", j),
        DataValue::Int(j) => format!("{}", j),
        DataValue::VarStr(j) => format!("{}", j),
        DataValue::Null => "NULL".to_string(),
    }
}
