mod sqllexer;
mod sqllistener;
pub mod sqlparser;
pub mod sqlvisitor;

pub use sqllexer::SQLLexer;
pub use sqlparser::SQLParser;
pub use sqlparser::SQLParserContextType;
pub use sqlvisitor::SQLVisitor;
pub use sqlvisitor::SQLVisitorCompat;

#[cfg(test)]
mod tests {
    use crate::parser::sqlparser::SQLParserContextType;
    use crate::parser::sqlvisitor::SQLVisitorCompat;
    use antlr_rust::common_token_stream::CommonTokenStream;
    use antlr_rust::tree::ParseTree;
    use antlr_rust::tree::ParseTreeVisitorCompat;
    use antlr_rust::tree::TerminalNode;
    use antlr_rust::InputStream;

    use super::sqllexer::SQLLexer;
    use super::sqlparser::SQLParser;

    #[test]
    fn test_parser_working() {
        let lexer = SQLLexer::new(InputStream::new("SELECT * FROM student;"));
        let mut parser = SQLParser::new(CommonTokenStream::new(lexer));
        let root = parser.program().unwrap();

        struct TestVisitor(String);

        println!("{}", root.to_string_tree(&*parser));

        // Base trait
        // Seems like *VisitorCompat is the trait with Node type and
        // return type, while *Visitor is not.
        impl ParseTreeVisitorCompat<'_> for TestVisitor {
            type Node = SQLParserContextType;
            type Return = String;

            fn temp_result(&mut self) -> &mut Self::Return {
                &mut self.0
            }
            fn visit_terminal(&mut self, node: &TerminalNode<'_, Self::Node>) -> Self::Return {
                node.symbol.to_string() + "\n"
            }
            fn aggregate_results(
                &self,
                aggregate: Self::Return,
                next: Self::Return,
            ) -> Self::Return {
                aggregate + &next
            }
        }

        impl SQLVisitorCompat<'_> for TestVisitor {}

        let result = TestVisitor(String::new()).visit(&*root);
        println!("{}", result);
    }
}
