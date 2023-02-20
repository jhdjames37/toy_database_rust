// Generated from SQL.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(clippy::all)]
use super::sqllistener::*;
use super::sqlvisitor::*;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::error_strategy::{DefaultErrorStrategy, ErrorStrategy};
use antlr_rust::errors::*;
use antlr_rust::int_stream::EOF;
use antlr_rust::parser::{BaseParser, Parser, ParserNodeType, ParserRecog};
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::parser_rule_context::{cast, cast_mut, BaseParserRuleContext, ParserRuleContext};
use antlr_rust::recognizer::{Actions, Recognizer};
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::token::{OwningToken, Token, TOKEN_EOF};
use antlr_rust::token_factory::{CommonTokenFactory, TokenAware, TokenFactory};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::tree::*;
use antlr_rust::vocabulary::{Vocabulary, VocabularyImpl};
use antlr_rust::PredictionContextCache;
use antlr_rust::TokenSource;

use antlr_rust::lazy_static;
use antlr_rust::{TidAble, TidExt};

use std::any::{Any, TypeId};
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::convert::TryFrom;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;

pub const T__0: isize = 1;
pub const T__1: isize = 2;
pub const T__2: isize = 3;
pub const T__3: isize = 4;
pub const T__4: isize = 5;
pub const T__5: isize = 6;
pub const T__6: isize = 7;
pub const T__7: isize = 8;
pub const T__8: isize = 9;
pub const T__9: isize = 10;
pub const T__10: isize = 11;
pub const T__11: isize = 12;
pub const T__12: isize = 13;
pub const T__13: isize = 14;
pub const T__14: isize = 15;
pub const T__15: isize = 16;
pub const T__16: isize = 17;
pub const T__17: isize = 18;
pub const T__18: isize = 19;
pub const T__19: isize = 20;
pub const T__20: isize = 21;
pub const T__21: isize = 22;
pub const T__22: isize = 23;
pub const T__23: isize = 24;
pub const T__24: isize = 25;
pub const T__25: isize = 26;
pub const T__26: isize = 27;
pub const T__27: isize = 28;
pub const T__28: isize = 29;
pub const T__29: isize = 30;
pub const T__30: isize = 31;
pub const T__31: isize = 32;
pub const T__32: isize = 33;
pub const T__33: isize = 34;
pub const T__34: isize = 35;
pub const T__35: isize = 36;
pub const T__36: isize = 37;
pub const T__37: isize = 38;
pub const T__38: isize = 39;
pub const T__39: isize = 40;
pub const T__40: isize = 41;
pub const T__41: isize = 42;
pub const T__42: isize = 43;
pub const T__43: isize = 44;
pub const T__44: isize = 45;
pub const T__45: isize = 46;
pub const T__46: isize = 47;
pub const T__47: isize = 48;
pub const T__48: isize = 49;
pub const T__49: isize = 50;
pub const T__50: isize = 51;
pub const EqualOrAssign: isize = 52;
pub const Less: isize = 53;
pub const LessEqual: isize = 54;
pub const Greater: isize = 55;
pub const GreaterEqual: isize = 56;
pub const NotEqual: isize = 57;
pub const Count: isize = 58;
pub const Average: isize = 59;
pub const Max: isize = 60;
pub const Min: isize = 61;
pub const Sum: isize = 62;
pub const Null: isize = 63;
pub const Identifier: isize = 64;
pub const Integer: isize = 65;
pub const String: isize = 66;
pub const Float: isize = 67;
pub const Whitespace: isize = 68;
pub const Annotation: isize = 69;
pub const RULE_program: usize = 0;
pub const RULE_statement: usize = 1;
pub const RULE_db_statement: usize = 2;
pub const RULE_io_statement: usize = 3;
pub const RULE_table_statement: usize = 4;
pub const RULE_select_table: usize = 5;
pub const RULE_alter_statement: usize = 6;
pub const RULE_field_list: usize = 7;
pub const RULE_field: usize = 8;
pub const RULE_type_: usize = 9;
pub const RULE_value_lists: usize = 10;
pub const RULE_value_list: usize = 11;
pub const RULE_value: usize = 12;
pub const RULE_where_and_clause: usize = 13;
pub const RULE_where_clause: usize = 14;
pub const RULE_column: usize = 15;
pub const RULE_expression: usize = 16;
pub const RULE_set_clause: usize = 17;
pub const RULE_selectors: usize = 18;
pub const RULE_selector: usize = 19;
pub const RULE_identifiers: usize = 20;
pub const RULE_operator_: usize = 21;
pub const RULE_aggregator: usize = 22;
pub const ruleNames: [&'static str; 23] = [
    "program",
    "statement",
    "db_statement",
    "io_statement",
    "table_statement",
    "select_table",
    "alter_statement",
    "field_list",
    "field",
    "type_",
    "value_lists",
    "value_list",
    "value",
    "where_and_clause",
    "where_clause",
    "column",
    "expression",
    "set_clause",
    "selectors",
    "selector",
    "identifiers",
    "operator_",
    "aggregator",
];

pub const _LITERAL_NAMES: [Option<&'static str>; 64] = [
    None,
    Some("';'"),
    Some("'CREATE'"),
    Some("'DATABASE'"),
    Some("'DROP'"),
    Some("'SHOW'"),
    Some("'DATABASES'"),
    Some("'USE'"),
    Some("'TABLES'"),
    Some("'INDEXES'"),
    Some("'LOAD'"),
    Some("'FROM'"),
    Some("'FILE'"),
    Some("'TO'"),
    Some("'TABLE'"),
    Some("'DUMP'"),
    Some("'('"),
    Some("')'"),
    Some("'DESC'"),
    Some("'INSERT'"),
    Some("'INTO'"),
    Some("'VALUES'"),
    Some("'DELETE'"),
    Some("'WHERE'"),
    Some("'UPDATE'"),
    Some("'SET'"),
    Some("'SELECT'"),
    Some("'GROUP'"),
    Some("'BY'"),
    Some("'LIMIT'"),
    Some("'OFFSET'"),
    Some("'ALTER'"),
    Some("'ADD'"),
    Some("'INDEX'"),
    Some("'PRIMARY'"),
    Some("'KEY'"),
    Some("'FOREIGN'"),
    Some("'CONSTRAINT'"),
    Some("'REFERENCES'"),
    Some("'UNIQUE'"),
    Some("','"),
    Some("'NOT'"),
    Some("'DEFAULT'"),
    Some("'INT'"),
    Some("'VARCHAR'"),
    Some("'FLOAT'"),
    Some("'AND'"),
    Some("'IS'"),
    Some("'IN'"),
    Some("'LIKE'"),
    Some("'.'"),
    Some("'*'"),
    Some("'='"),
    Some("'<'"),
    Some("'<='"),
    Some("'>'"),
    Some("'>='"),
    Some("'<>'"),
    Some("'COUNT'"),
    Some("'AVG'"),
    Some("'MAX'"),
    Some("'MIN'"),
    Some("'SUM'"),
    Some("'NULL'"),
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>; 70] = [
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some("EqualOrAssign"),
    Some("Less"),
    Some("LessEqual"),
    Some("Greater"),
    Some("GreaterEqual"),
    Some("NotEqual"),
    Some("Count"),
    Some("Average"),
    Some("Max"),
    Some("Min"),
    Some("Sum"),
    Some("Null"),
    Some("Identifier"),
    Some("Integer"),
    Some("String"),
    Some("Float"),
    Some("Whitespace"),
    Some("Annotation"),
];
lazy_static! {
    static ref _shared_context_cache: Arc<PredictionContextCache> =
        Arc::new(PredictionContextCache::new());
    static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(
        _LITERAL_NAMES.iter(),
        _SYMBOLIC_NAMES.iter(),
        None
    ));
}

type BaseParserType<'input, I> = BaseParser<
    'input,
    SQLParserExt<'input>,
    I,
    SQLParserContextType,
    dyn SQLListener<'input> + 'input,
>;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type SQLTreeWalker<'input, 'a> =
    ParseTreeWalker<'input, 'a, SQLParserContextType, dyn SQLListener<'input> + 'a>;

/// Parser for SQL grammar
pub struct SQLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    base: BaseParserType<'input, I>,
    interpreter: Arc<ParserATNSimulator>,
    _shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> SQLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn get_serialized_atn() -> &'static str {
        _serializedATN
    }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
        antlr_rust::recognizer::check_version("0", "3");
        let interpreter = Arc::new(ParserATNSimulator::new(
            _ATN.clone(),
            _decision_to_DFA.clone(),
            _shared_context_cache.clone(),
        ));
        Self {
            base: BaseParser::new_base_parser(
                input,
                Arc::clone(&interpreter),
                SQLParserExt {
                    _pd: Default::default(),
                },
            ),
            interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }
}

type DynStrategy<'input, I> = Box<dyn ErrorStrategy<'input, BaseParserType<'input, I>> + 'input>;

impl<'input, I> SQLParser<'input, I, DynStrategy<'input, I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self {
        Self::with_strategy(input, Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> SQLParser<'input, I, DefaultErrorStrategy<'input, SQLParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn new(input: I) -> Self {
        Self::with_strategy(input, DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for SQLParser
pub trait SQLParserContext<'input>:
    for<'x> Listenable<dyn SQLListener<'input> + 'x>
    + for<'x> Visitable<dyn SQLVisitor<'input> + 'x>
    + ParserRuleContext<'input, TF = LocalTokenFactory<'input>, Ctx = SQLParserContextType>
{
}

antlr_rust::coerce_from! { 'input : SQLParserContext<'input> }

impl<'input, 'x, T> VisitableDyn<T> for dyn SQLParserContext<'input> + 'input
where
    T: SQLVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn SQLVisitor<'input> + 'x))
    }
}

impl<'input> SQLParserContext<'input> for TerminalNode<'input, SQLParserContextType> {}
impl<'input> SQLParserContext<'input> for ErrorNode<'input, SQLParserContextType> {}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn SQLParserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn SQLListener<'input> + 'input }

pub struct SQLParserContextType;
antlr_rust::tid! {SQLParserContextType}

impl<'input> ParserNodeType<'input> for SQLParserContextType {
    type TF = LocalTokenFactory<'input>;
    type Type = dyn SQLParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for SQLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    type Target = BaseParserType<'input, I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for SQLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct SQLParserExt<'input> {
    _pd: PhantomData<&'input str>,
}

impl<'input> SQLParserExt<'input> {}
antlr_rust::tid! { SQLParserExt<'a> }

impl<'input> TokenAware<'input> for SQLParserExt<'input> {
    type TF = LocalTokenFactory<'input>;
}

impl<'input, I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>>
    ParserRecog<'input, BaseParserType<'input, I>> for SQLParserExt<'input>
{
}

impl<'input, I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>>
    Actions<'input, BaseParserType<'input, I>> for SQLParserExt<'input>
{
    fn get_grammar_file_name(&self) -> &str {
        "SQL.g4"
    }

    fn get_rule_names(&self) -> &[&str] {
        &ruleNames
    }

    fn get_vocabulary(&self) -> &dyn Vocabulary {
        &**VOCABULARY
    }
}
//------------------- program ----------------
pub type ProgramContextAll<'input> = ProgramContext<'input>;

pub type ProgramContext<'input> = BaseParserRuleContext<'input, ProgramContextExt<'input>>;

#[derive(Clone)]
pub struct ProgramContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLParserContext<'input> for ProgramContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for ProgramContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_program(self);
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.exit_program(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for ProgramContext<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        visitor.visit_program(self);
    }
}

impl<'input> CustomRuleContext<'input> for ProgramContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_program
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_program }
}
antlr_rust::tid! {ProgramContextExt<'a>}

impl<'input> ProgramContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ProgramContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ProgramContextExt { ph: PhantomData },
        ))
    }
}

pub trait ProgramContextAttrs<'input>:
    SQLParserContext<'input> + BorrowMut<ProgramContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(EOF, 0)
    }
    fn statement_all(&self) -> Vec<Rc<StatementContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn statement(&self, i: usize) -> Option<Rc<StatementContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> ProgramContextAttrs<'input> for ProgramContext<'input> {}

impl<'input, I, H> SQLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn program(&mut self) -> Result<Rc<ProgramContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ProgramContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_program);
        let mut _localctx: Rc<ProgramContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(49);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while (((_la) & !0x3f) == 0
                    && ((1usize << _la)
                        & ((1usize << T__1)
                            | (1usize << T__3)
                            | (1usize << T__4)
                            | (1usize << T__6)
                            | (1usize << T__9)
                            | (1usize << T__14)
                            | (1usize << T__17)
                            | (1usize << T__18)
                            | (1usize << T__21)
                            | (1usize << T__23)
                            | (1usize << T__25)
                            | (1usize << T__30)))
                        != 0)
                    || _la == Null
                    || _la == Annotation
                {
                    {
                        {
                            /*InvokeRule statement*/
                            recog.base.set_state(46);
                            recog.statement()?;
                        }
                    }
                    recog.base.set_state(51);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(52);
                recog.base.match_token(EOF, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- statement ----------------
pub type StatementContextAll<'input> = StatementContext<'input>;

pub type StatementContext<'input> = BaseParserRuleContext<'input, StatementContextExt<'input>>;

#[derive(Clone)]
pub struct StatementContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLParserContext<'input> for StatementContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for StatementContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_statement(self);
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.exit_statement(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for StatementContext<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        visitor.visit_statement(self);
    }
}

impl<'input> CustomRuleContext<'input> for StatementContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_statement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_statement }
}
antlr_rust::tid! {StatementContextExt<'a>}

impl<'input> StatementContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<StatementContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            StatementContextExt { ph: PhantomData },
        ))
    }
}

pub trait StatementContextAttrs<'input>:
    SQLParserContext<'input> + BorrowMut<StatementContextExt<'input>>
{
    fn db_statement(&self) -> Option<Rc<Db_statementContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn io_statement(&self) -> Option<Rc<Io_statementContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn table_statement(&self) -> Option<Rc<Table_statementContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn alter_statement(&self) -> Option<Rc<Alter_statementContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token Annotation
    /// Returns `None` if there is no child corresponding to token Annotation
    fn Annotation(&self) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Annotation, 0)
    }
    /// Retrieves first TerminalNode corresponding to token Null
    /// Returns `None` if there is no child corresponding to token Null
    fn Null(&self) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Null, 0)
    }
}

impl<'input> StatementContextAttrs<'input> for StatementContext<'input> {}

impl<'input, I, H> SQLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn statement(&mut self) -> Result<Rc<StatementContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = StatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_statement);
        let mut _localctx: Rc<StatementContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(70);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(1, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule db_statement*/
                        recog.base.set_state(54);
                        recog.db_statement()?;

                        recog.base.set_state(55);
                        recog.base.match_token(T__0, &mut recog.err_handler)?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule io_statement*/
                        recog.base.set_state(57);
                        recog.io_statement()?;

                        recog.base.set_state(58);
                        recog.base.match_token(T__0, &mut recog.err_handler)?;
                    }
                }
                3 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        /*InvokeRule table_statement*/
                        recog.base.set_state(60);
                        recog.table_statement()?;

                        recog.base.set_state(61);
                        recog.base.match_token(T__0, &mut recog.err_handler)?;
                    }
                }
                4 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 4);
                    recog.base.enter_outer_alt(None, 4);
                    {
                        /*InvokeRule alter_statement*/
                        recog.base.set_state(63);
                        recog.alter_statement()?;

                        recog.base.set_state(64);
                        recog.base.match_token(T__0, &mut recog.err_handler)?;
                    }
                }
                5 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 5);
                    recog.base.enter_outer_alt(None, 5);
                    {
                        recog.base.set_state(66);
                        recog.base.match_token(Annotation, &mut recog.err_handler)?;

                        recog.base.set_state(67);
                        recog.base.match_token(T__0, &mut recog.err_handler)?;
                    }
                }
                6 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 6);
                    recog.base.enter_outer_alt(None, 6);
                    {
                        recog.base.set_state(68);
                        recog.base.match_token(Null, &mut recog.err_handler)?;

                        recog.base.set_state(69);
                        recog.base.match_token(T__0, &mut recog.err_handler)?;
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- db_statement ----------------
#[derive(Debug)]
pub enum Db_statementContextAll<'input> {
    Show_dbsContext(Show_dbsContext<'input>),
    Drop_dbContext(Drop_dbContext<'input>),
    Show_tablesContext(Show_tablesContext<'input>),
    Create_dbContext(Create_dbContext<'input>),
    Use_dbContext(Use_dbContext<'input>),
    Show_indexesContext(Show_indexesContext<'input>),
    Error(Db_statementContext<'input>),
}
antlr_rust::tid! {Db_statementContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for Db_statementContextAll<'input> {}

impl<'input> SQLParserContext<'input> for Db_statementContextAll<'input> {}

impl<'input> Deref for Db_statementContextAll<'input> {
    type Target = dyn Db_statementContextAttrs<'input> + 'input;
    fn deref(&self) -> &Self::Target {
        use Db_statementContextAll::*;
        match self {
            Show_dbsContext(inner) => inner,
            Drop_dbContext(inner) => inner,
            Show_tablesContext(inner) => inner,
            Create_dbContext(inner) => inner,
            Use_dbContext(inner) => inner,
            Show_indexesContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}
impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for Db_statementContextAll<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        self.deref().accept(visitor)
    }
}
impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for Db_statementContextAll<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        self.deref().enter(listener)
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        self.deref().exit(listener)
    }
}

pub type Db_statementContext<'input> =
    BaseParserRuleContext<'input, Db_statementContextExt<'input>>;

#[derive(Clone)]
pub struct Db_statementContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLParserContext<'input> for Db_statementContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for Db_statementContext<'input> {}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for Db_statementContext<'input> {}

impl<'input> CustomRuleContext<'input> for Db_statementContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_db_statement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_db_statement }
}
antlr_rust::tid! {Db_statementContextExt<'a>}

impl<'input> Db_statementContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Db_statementContextAll<'input>> {
        Rc::new(Db_statementContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(
                parent,
                invoking_state,
                Db_statementContextExt { ph: PhantomData },
            ),
        ))
    }
}

pub trait Db_statementContextAttrs<'input>:
    SQLParserContext<'input> + BorrowMut<Db_statementContextExt<'input>>
{
}

impl<'input> Db_statementContextAttrs<'input> for Db_statementContext<'input> {}

pub type Show_dbsContext<'input> = BaseParserRuleContext<'input, Show_dbsContextExt<'input>>;

pub trait Show_dbsContextAttrs<'input>: SQLParserContext<'input> {}

impl<'input> Show_dbsContextAttrs<'input> for Show_dbsContext<'input> {}

pub struct Show_dbsContextExt<'input> {
    base: Db_statementContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {Show_dbsContextExt<'a>}

impl<'input> SQLParserContext<'input> for Show_dbsContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for Show_dbsContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_show_dbs(self);
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.exit_show_dbs(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for Show_dbsContext<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        visitor.visit_show_dbs(self);
    }
}

impl<'input> CustomRuleContext<'input> for Show_dbsContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_db_statement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_db_statement }
}

impl<'input> Borrow<Db_statementContextExt<'input>> for Show_dbsContext<'input> {
    fn borrow(&self) -> &Db_statementContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<Db_statementContextExt<'input>> for Show_dbsContext<'input> {
    fn borrow_mut(&mut self) -> &mut Db_statementContextExt<'input> {
        &mut self.base
    }
}

impl<'input> Db_statementContextAttrs<'input> for Show_dbsContext<'input> {}

impl<'input> Show_dbsContextExt<'input> {
    fn new(ctx: &dyn Db_statementContextAttrs<'input>) -> Rc<Db_statementContextAll<'input>> {
        Rc::new(Db_statementContextAll::Show_dbsContext(
            BaseParserRuleContext::copy_from(
                ctx,
                Show_dbsContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type Drop_dbContext<'input> = BaseParserRuleContext<'input, Drop_dbContextExt<'input>>;

pub trait Drop_dbContextAttrs<'input>: SQLParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token Identifier
    /// Returns `None` if there is no child corresponding to token Identifier
    fn Identifier(&self) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Identifier, 0)
    }
}

impl<'input> Drop_dbContextAttrs<'input> for Drop_dbContext<'input> {}

pub struct Drop_dbContextExt<'input> {
    base: Db_statementContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {Drop_dbContextExt<'a>}

impl<'input> SQLParserContext<'input> for Drop_dbContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for Drop_dbContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_drop_db(self);
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.exit_drop_db(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for Drop_dbContext<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        visitor.visit_drop_db(self);
    }
}

impl<'input> CustomRuleContext<'input> for Drop_dbContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_db_statement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_db_statement }
}

impl<'input> Borrow<Db_statementContextExt<'input>> for Drop_dbContext<'input> {
    fn borrow(&self) -> &Db_statementContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<Db_statementContextExt<'input>> for Drop_dbContext<'input> {
    fn borrow_mut(&mut self) -> &mut Db_statementContextExt<'input> {
        &mut self.base
    }
}

impl<'input> Db_statementContextAttrs<'input> for Drop_dbContext<'input> {}

impl<'input> Drop_dbContextExt<'input> {
    fn new(ctx: &dyn Db_statementContextAttrs<'input>) -> Rc<Db_statementContextAll<'input>> {
        Rc::new(Db_statementContextAll::Drop_dbContext(
            BaseParserRuleContext::copy_from(
                ctx,
                Drop_dbContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type Show_tablesContext<'input> = BaseParserRuleContext<'input, Show_tablesContextExt<'input>>;

pub trait Show_tablesContextAttrs<'input>: SQLParserContext<'input> {}

impl<'input> Show_tablesContextAttrs<'input> for Show_tablesContext<'input> {}

pub struct Show_tablesContextExt<'input> {
    base: Db_statementContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {Show_tablesContextExt<'a>}

impl<'input> SQLParserContext<'input> for Show_tablesContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for Show_tablesContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_show_tables(self);
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.exit_show_tables(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for Show_tablesContext<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        visitor.visit_show_tables(self);
    }
}

impl<'input> CustomRuleContext<'input> for Show_tablesContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_db_statement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_db_statement }
}

impl<'input> Borrow<Db_statementContextExt<'input>> for Show_tablesContext<'input> {
    fn borrow(&self) -> &Db_statementContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<Db_statementContextExt<'input>> for Show_tablesContext<'input> {
    fn borrow_mut(&mut self) -> &mut Db_statementContextExt<'input> {
        &mut self.base
    }
}

impl<'input> Db_statementContextAttrs<'input> for Show_tablesContext<'input> {}

impl<'input> Show_tablesContextExt<'input> {
    fn new(ctx: &dyn Db_statementContextAttrs<'input>) -> Rc<Db_statementContextAll<'input>> {
        Rc::new(Db_statementContextAll::Show_tablesContext(
            BaseParserRuleContext::copy_from(
                ctx,
                Show_tablesContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type Create_dbContext<'input> = BaseParserRuleContext<'input, Create_dbContextExt<'input>>;

pub trait Create_dbContextAttrs<'input>: SQLParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token Identifier
    /// Returns `None` if there is no child corresponding to token Identifier
    fn Identifier(&self) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Identifier, 0)
    }
}

impl<'input> Create_dbContextAttrs<'input> for Create_dbContext<'input> {}

pub struct Create_dbContextExt<'input> {
    base: Db_statementContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {Create_dbContextExt<'a>}

impl<'input> SQLParserContext<'input> for Create_dbContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for Create_dbContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_create_db(self);
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.exit_create_db(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for Create_dbContext<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        visitor.visit_create_db(self);
    }
}

impl<'input> CustomRuleContext<'input> for Create_dbContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_db_statement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_db_statement }
}

impl<'input> Borrow<Db_statementContextExt<'input>> for Create_dbContext<'input> {
    fn borrow(&self) -> &Db_statementContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<Db_statementContextExt<'input>> for Create_dbContext<'input> {
    fn borrow_mut(&mut self) -> &mut Db_statementContextExt<'input> {
        &mut self.base
    }
}

impl<'input> Db_statementContextAttrs<'input> for Create_dbContext<'input> {}

impl<'input> Create_dbContextExt<'input> {
    fn new(ctx: &dyn Db_statementContextAttrs<'input>) -> Rc<Db_statementContextAll<'input>> {
        Rc::new(Db_statementContextAll::Create_dbContext(
            BaseParserRuleContext::copy_from(
                ctx,
                Create_dbContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type Use_dbContext<'input> = BaseParserRuleContext<'input, Use_dbContextExt<'input>>;

pub trait Use_dbContextAttrs<'input>: SQLParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token Identifier
    /// Returns `None` if there is no child corresponding to token Identifier
    fn Identifier(&self) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Identifier, 0)
    }
}

impl<'input> Use_dbContextAttrs<'input> for Use_dbContext<'input> {}

pub struct Use_dbContextExt<'input> {
    base: Db_statementContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {Use_dbContextExt<'a>}

impl<'input> SQLParserContext<'input> for Use_dbContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for Use_dbContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_use_db(self);
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.exit_use_db(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for Use_dbContext<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        visitor.visit_use_db(self);
    }
}

impl<'input> CustomRuleContext<'input> for Use_dbContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_db_statement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_db_statement }
}

impl<'input> Borrow<Db_statementContextExt<'input>> for Use_dbContext<'input> {
    fn borrow(&self) -> &Db_statementContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<Db_statementContextExt<'input>> for Use_dbContext<'input> {
    fn borrow_mut(&mut self) -> &mut Db_statementContextExt<'input> {
        &mut self.base
    }
}

impl<'input> Db_statementContextAttrs<'input> for Use_dbContext<'input> {}

impl<'input> Use_dbContextExt<'input> {
    fn new(ctx: &dyn Db_statementContextAttrs<'input>) -> Rc<Db_statementContextAll<'input>> {
        Rc::new(Db_statementContextAll::Use_dbContext(
            BaseParserRuleContext::copy_from(
                ctx,
                Use_dbContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type Show_indexesContext<'input> =
    BaseParserRuleContext<'input, Show_indexesContextExt<'input>>;

pub trait Show_indexesContextAttrs<'input>: SQLParserContext<'input> {}

impl<'input> Show_indexesContextAttrs<'input> for Show_indexesContext<'input> {}

pub struct Show_indexesContextExt<'input> {
    base: Db_statementContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {Show_indexesContextExt<'a>}

impl<'input> SQLParserContext<'input> for Show_indexesContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for Show_indexesContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_show_indexes(self);
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.exit_show_indexes(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for Show_indexesContext<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        visitor.visit_show_indexes(self);
    }
}

impl<'input> CustomRuleContext<'input> for Show_indexesContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_db_statement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_db_statement }
}

impl<'input> Borrow<Db_statementContextExt<'input>> for Show_indexesContext<'input> {
    fn borrow(&self) -> &Db_statementContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<Db_statementContextExt<'input>> for Show_indexesContext<'input> {
    fn borrow_mut(&mut self) -> &mut Db_statementContextExt<'input> {
        &mut self.base
    }
}

impl<'input> Db_statementContextAttrs<'input> for Show_indexesContext<'input> {}

impl<'input> Show_indexesContextExt<'input> {
    fn new(ctx: &dyn Db_statementContextAttrs<'input>) -> Rc<Db_statementContextAll<'input>> {
        Rc::new(Db_statementContextAll::Show_indexesContext(
            BaseParserRuleContext::copy_from(
                ctx,
                Show_indexesContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

impl<'input, I, H> SQLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn db_statement(&mut self) -> Result<Rc<Db_statementContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Db_statementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 4, RULE_db_statement);
        let mut _localctx: Rc<Db_statementContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(86);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(2, &mut recog.base)? {
                1 => {
                    let tmp = Create_dbContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 1);
                    _localctx = tmp;
                    {
                        recog.base.set_state(72);
                        recog.base.match_token(T__1, &mut recog.err_handler)?;

                        recog.base.set_state(73);
                        recog.base.match_token(T__2, &mut recog.err_handler)?;

                        recog.base.set_state(74);
                        recog.base.match_token(Identifier, &mut recog.err_handler)?;
                    }
                }
                2 => {
                    let tmp = Drop_dbContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 2);
                    _localctx = tmp;
                    {
                        recog.base.set_state(75);
                        recog.base.match_token(T__3, &mut recog.err_handler)?;

                        recog.base.set_state(76);
                        recog.base.match_token(T__2, &mut recog.err_handler)?;

                        recog.base.set_state(77);
                        recog.base.match_token(Identifier, &mut recog.err_handler)?;
                    }
                }
                3 => {
                    let tmp = Show_dbsContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 3);
                    _localctx = tmp;
                    {
                        recog.base.set_state(78);
                        recog.base.match_token(T__4, &mut recog.err_handler)?;

                        recog.base.set_state(79);
                        recog.base.match_token(T__5, &mut recog.err_handler)?;
                    }
                }
                4 => {
                    let tmp = Use_dbContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 4);
                    _localctx = tmp;
                    {
                        recog.base.set_state(80);
                        recog.base.match_token(T__6, &mut recog.err_handler)?;

                        recog.base.set_state(81);
                        recog.base.match_token(Identifier, &mut recog.err_handler)?;
                    }
                }
                5 => {
                    let tmp = Show_tablesContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 5);
                    _localctx = tmp;
                    {
                        recog.base.set_state(82);
                        recog.base.match_token(T__4, &mut recog.err_handler)?;

                        recog.base.set_state(83);
                        recog.base.match_token(T__7, &mut recog.err_handler)?;
                    }
                }
                6 => {
                    let tmp = Show_indexesContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 6);
                    _localctx = tmp;
                    {
                        recog.base.set_state(84);
                        recog.base.match_token(T__4, &mut recog.err_handler)?;

                        recog.base.set_state(85);
                        recog.base.match_token(T__8, &mut recog.err_handler)?;
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- io_statement ----------------
#[derive(Debug)]
pub enum Io_statementContextAll<'input> {
    Dump_dataContext(Dump_dataContext<'input>),
    Load_dataContext(Load_dataContext<'input>),
    Error(Io_statementContext<'input>),
}
antlr_rust::tid! {Io_statementContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for Io_statementContextAll<'input> {}

impl<'input> SQLParserContext<'input> for Io_statementContextAll<'input> {}

impl<'input> Deref for Io_statementContextAll<'input> {
    type Target = dyn Io_statementContextAttrs<'input> + 'input;
    fn deref(&self) -> &Self::Target {
        use Io_statementContextAll::*;
        match self {
            Dump_dataContext(inner) => inner,
            Load_dataContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}
impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for Io_statementContextAll<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        self.deref().accept(visitor)
    }
}
impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for Io_statementContextAll<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        self.deref().enter(listener)
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        self.deref().exit(listener)
    }
}

pub type Io_statementContext<'input> =
    BaseParserRuleContext<'input, Io_statementContextExt<'input>>;

#[derive(Clone)]
pub struct Io_statementContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLParserContext<'input> for Io_statementContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for Io_statementContext<'input> {}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for Io_statementContext<'input> {}

impl<'input> CustomRuleContext<'input> for Io_statementContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_io_statement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_io_statement }
}
antlr_rust::tid! {Io_statementContextExt<'a>}

impl<'input> Io_statementContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Io_statementContextAll<'input>> {
        Rc::new(Io_statementContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(
                parent,
                invoking_state,
                Io_statementContextExt { ph: PhantomData },
            ),
        ))
    }
}

pub trait Io_statementContextAttrs<'input>:
    SQLParserContext<'input> + BorrowMut<Io_statementContextExt<'input>>
{
}

impl<'input> Io_statementContextAttrs<'input> for Io_statementContext<'input> {}

pub type Dump_dataContext<'input> = BaseParserRuleContext<'input, Dump_dataContextExt<'input>>;

pub trait Dump_dataContextAttrs<'input>: SQLParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token String
    /// Returns `None` if there is no child corresponding to token String
    fn String(&self) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(String, 0)
    }
    /// Retrieves first TerminalNode corresponding to token Identifier
    /// Returns `None` if there is no child corresponding to token Identifier
    fn Identifier(&self) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Identifier, 0)
    }
}

impl<'input> Dump_dataContextAttrs<'input> for Dump_dataContext<'input> {}

pub struct Dump_dataContextExt<'input> {
    base: Io_statementContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {Dump_dataContextExt<'a>}

impl<'input> SQLParserContext<'input> for Dump_dataContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for Dump_dataContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_dump_data(self);
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.exit_dump_data(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for Dump_dataContext<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        visitor.visit_dump_data(self);
    }
}

impl<'input> CustomRuleContext<'input> for Dump_dataContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_io_statement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_io_statement }
}

impl<'input> Borrow<Io_statementContextExt<'input>> for Dump_dataContext<'input> {
    fn borrow(&self) -> &Io_statementContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<Io_statementContextExt<'input>> for Dump_dataContext<'input> {
    fn borrow_mut(&mut self) -> &mut Io_statementContextExt<'input> {
        &mut self.base
    }
}

impl<'input> Io_statementContextAttrs<'input> for Dump_dataContext<'input> {}

impl<'input> Dump_dataContextExt<'input> {
    fn new(ctx: &dyn Io_statementContextAttrs<'input>) -> Rc<Io_statementContextAll<'input>> {
        Rc::new(Io_statementContextAll::Dump_dataContext(
            BaseParserRuleContext::copy_from(
                ctx,
                Dump_dataContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type Load_dataContext<'input> = BaseParserRuleContext<'input, Load_dataContextExt<'input>>;

pub trait Load_dataContextAttrs<'input>: SQLParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token String
    /// Returns `None` if there is no child corresponding to token String
    fn String(&self) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(String, 0)
    }
    /// Retrieves first TerminalNode corresponding to token Identifier
    /// Returns `None` if there is no child corresponding to token Identifier
    fn Identifier(&self) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Identifier, 0)
    }
}

impl<'input> Load_dataContextAttrs<'input> for Load_dataContext<'input> {}

pub struct Load_dataContextExt<'input> {
    base: Io_statementContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {Load_dataContextExt<'a>}

impl<'input> SQLParserContext<'input> for Load_dataContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for Load_dataContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_load_data(self);
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.exit_load_data(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for Load_dataContext<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        visitor.visit_load_data(self);
    }
}

impl<'input> CustomRuleContext<'input> for Load_dataContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_io_statement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_io_statement }
}

impl<'input> Borrow<Io_statementContextExt<'input>> for Load_dataContext<'input> {
    fn borrow(&self) -> &Io_statementContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<Io_statementContextExt<'input>> for Load_dataContext<'input> {
    fn borrow_mut(&mut self) -> &mut Io_statementContextExt<'input> {
        &mut self.base
    }
}

impl<'input> Io_statementContextAttrs<'input> for Load_dataContext<'input> {}

impl<'input> Load_dataContextExt<'input> {
    fn new(ctx: &dyn Io_statementContextAttrs<'input>) -> Rc<Io_statementContextAll<'input>> {
        Rc::new(Io_statementContextAll::Load_dataContext(
            BaseParserRuleContext::copy_from(
                ctx,
                Load_dataContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

impl<'input, I, H> SQLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn io_statement(&mut self) -> Result<Rc<Io_statementContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Io_statementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 6, RULE_io_statement);
        let mut _localctx: Rc<Io_statementContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(102);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                T__9 => {
                    let tmp = Load_dataContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 1);
                    _localctx = tmp;
                    {
                        recog.base.set_state(88);
                        recog.base.match_token(T__9, &mut recog.err_handler)?;

                        recog.base.set_state(89);
                        recog.base.match_token(T__10, &mut recog.err_handler)?;

                        recog.base.set_state(90);
                        recog.base.match_token(T__11, &mut recog.err_handler)?;

                        recog.base.set_state(91);
                        recog.base.match_token(String, &mut recog.err_handler)?;

                        recog.base.set_state(92);
                        recog.base.match_token(T__12, &mut recog.err_handler)?;

                        recog.base.set_state(93);
                        recog.base.match_token(T__13, &mut recog.err_handler)?;

                        recog.base.set_state(94);
                        recog.base.match_token(Identifier, &mut recog.err_handler)?;
                    }
                }

                T__14 => {
                    let tmp = Dump_dataContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 2);
                    _localctx = tmp;
                    {
                        recog.base.set_state(95);
                        recog.base.match_token(T__14, &mut recog.err_handler)?;

                        recog.base.set_state(96);
                        recog.base.match_token(T__12, &mut recog.err_handler)?;

                        recog.base.set_state(97);
                        recog.base.match_token(T__11, &mut recog.err_handler)?;

                        recog.base.set_state(98);
                        recog.base.match_token(String, &mut recog.err_handler)?;

                        recog.base.set_state(99);
                        recog.base.match_token(T__10, &mut recog.err_handler)?;

                        recog.base.set_state(100);
                        recog.base.match_token(T__13, &mut recog.err_handler)?;

                        recog.base.set_state(101);
                        recog.base.match_token(Identifier, &mut recog.err_handler)?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- table_statement ----------------
#[derive(Debug)]
pub enum Table_statementContextAll<'input> {
    Delete_from_tableContext(Delete_from_tableContext<'input>),
    Insert_into_tableContext(Insert_into_tableContext<'input>),
    Create_tableContext(Create_tableContext<'input>),
    Describe_tableContext(Describe_tableContext<'input>),
    Select_table_Context(Select_table_Context<'input>),
    Drop_tableContext(Drop_tableContext<'input>),
    Update_tableContext(Update_tableContext<'input>),
    Error(Table_statementContext<'input>),
}
antlr_rust::tid! {Table_statementContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for Table_statementContextAll<'input> {}

impl<'input> SQLParserContext<'input> for Table_statementContextAll<'input> {}

impl<'input> Deref for Table_statementContextAll<'input> {
    type Target = dyn Table_statementContextAttrs<'input> + 'input;
    fn deref(&self) -> &Self::Target {
        use Table_statementContextAll::*;
        match self {
            Delete_from_tableContext(inner) => inner,
            Insert_into_tableContext(inner) => inner,
            Create_tableContext(inner) => inner,
            Describe_tableContext(inner) => inner,
            Select_table_Context(inner) => inner,
            Drop_tableContext(inner) => inner,
            Update_tableContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}
impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for Table_statementContextAll<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        self.deref().accept(visitor)
    }
}
impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for Table_statementContextAll<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        self.deref().enter(listener)
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        self.deref().exit(listener)
    }
}

pub type Table_statementContext<'input> =
    BaseParserRuleContext<'input, Table_statementContextExt<'input>>;

#[derive(Clone)]
pub struct Table_statementContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLParserContext<'input> for Table_statementContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for Table_statementContext<'input> {}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for Table_statementContext<'input> {}

impl<'input> CustomRuleContext<'input> for Table_statementContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_table_statement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_table_statement }
}
antlr_rust::tid! {Table_statementContextExt<'a>}

impl<'input> Table_statementContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Table_statementContextAll<'input>> {
        Rc::new(Table_statementContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(
                parent,
                invoking_state,
                Table_statementContextExt { ph: PhantomData },
            ),
        ))
    }
}

pub trait Table_statementContextAttrs<'input>:
    SQLParserContext<'input> + BorrowMut<Table_statementContextExt<'input>>
{
}

impl<'input> Table_statementContextAttrs<'input> for Table_statementContext<'input> {}

pub type Delete_from_tableContext<'input> =
    BaseParserRuleContext<'input, Delete_from_tableContextExt<'input>>;

pub trait Delete_from_tableContextAttrs<'input>: SQLParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token Identifier
    /// Returns `None` if there is no child corresponding to token Identifier
    fn Identifier(&self) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Identifier, 0)
    }
    fn where_and_clause(&self) -> Option<Rc<Where_and_clauseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Delete_from_tableContextAttrs<'input> for Delete_from_tableContext<'input> {}

pub struct Delete_from_tableContextExt<'input> {
    base: Table_statementContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {Delete_from_tableContextExt<'a>}

impl<'input> SQLParserContext<'input> for Delete_from_tableContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for Delete_from_tableContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_delete_from_table(self);
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.exit_delete_from_table(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for Delete_from_tableContext<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        visitor.visit_delete_from_table(self);
    }
}

impl<'input> CustomRuleContext<'input> for Delete_from_tableContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_table_statement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_table_statement }
}

impl<'input> Borrow<Table_statementContextExt<'input>> for Delete_from_tableContext<'input> {
    fn borrow(&self) -> &Table_statementContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<Table_statementContextExt<'input>> for Delete_from_tableContext<'input> {
    fn borrow_mut(&mut self) -> &mut Table_statementContextExt<'input> {
        &mut self.base
    }
}

impl<'input> Table_statementContextAttrs<'input> for Delete_from_tableContext<'input> {}

impl<'input> Delete_from_tableContextExt<'input> {
    fn new(ctx: &dyn Table_statementContextAttrs<'input>) -> Rc<Table_statementContextAll<'input>> {
        Rc::new(Table_statementContextAll::Delete_from_tableContext(
            BaseParserRuleContext::copy_from(
                ctx,
                Delete_from_tableContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type Insert_into_tableContext<'input> =
    BaseParserRuleContext<'input, Insert_into_tableContextExt<'input>>;

pub trait Insert_into_tableContextAttrs<'input>: SQLParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token Identifier
    /// Returns `None` if there is no child corresponding to token Identifier
    fn Identifier(&self) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Identifier, 0)
    }
    fn value_lists(&self) -> Option<Rc<Value_listsContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Insert_into_tableContextAttrs<'input> for Insert_into_tableContext<'input> {}

pub struct Insert_into_tableContextExt<'input> {
    base: Table_statementContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {Insert_into_tableContextExt<'a>}

impl<'input> SQLParserContext<'input> for Insert_into_tableContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for Insert_into_tableContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_insert_into_table(self);
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.exit_insert_into_table(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for Insert_into_tableContext<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        visitor.visit_insert_into_table(self);
    }
}

impl<'input> CustomRuleContext<'input> for Insert_into_tableContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_table_statement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_table_statement }
}

impl<'input> Borrow<Table_statementContextExt<'input>> for Insert_into_tableContext<'input> {
    fn borrow(&self) -> &Table_statementContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<Table_statementContextExt<'input>> for Insert_into_tableContext<'input> {
    fn borrow_mut(&mut self) -> &mut Table_statementContextExt<'input> {
        &mut self.base
    }
}

impl<'input> Table_statementContextAttrs<'input> for Insert_into_tableContext<'input> {}

impl<'input> Insert_into_tableContextExt<'input> {
    fn new(ctx: &dyn Table_statementContextAttrs<'input>) -> Rc<Table_statementContextAll<'input>> {
        Rc::new(Table_statementContextAll::Insert_into_tableContext(
            BaseParserRuleContext::copy_from(
                ctx,
                Insert_into_tableContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type Create_tableContext<'input> =
    BaseParserRuleContext<'input, Create_tableContextExt<'input>>;

pub trait Create_tableContextAttrs<'input>: SQLParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token Identifier
    /// Returns `None` if there is no child corresponding to token Identifier
    fn Identifier(&self) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Identifier, 0)
    }
    fn field_list(&self) -> Option<Rc<Field_listContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Create_tableContextAttrs<'input> for Create_tableContext<'input> {}

pub struct Create_tableContextExt<'input> {
    base: Table_statementContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {Create_tableContextExt<'a>}

impl<'input> SQLParserContext<'input> for Create_tableContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for Create_tableContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_create_table(self);
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.exit_create_table(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for Create_tableContext<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        visitor.visit_create_table(self);
    }
}

impl<'input> CustomRuleContext<'input> for Create_tableContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_table_statement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_table_statement }
}

impl<'input> Borrow<Table_statementContextExt<'input>> for Create_tableContext<'input> {
    fn borrow(&self) -> &Table_statementContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<Table_statementContextExt<'input>> for Create_tableContext<'input> {
    fn borrow_mut(&mut self) -> &mut Table_statementContextExt<'input> {
        &mut self.base
    }
}

impl<'input> Table_statementContextAttrs<'input> for Create_tableContext<'input> {}

impl<'input> Create_tableContextExt<'input> {
    fn new(ctx: &dyn Table_statementContextAttrs<'input>) -> Rc<Table_statementContextAll<'input>> {
        Rc::new(Table_statementContextAll::Create_tableContext(
            BaseParserRuleContext::copy_from(
                ctx,
                Create_tableContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type Describe_tableContext<'input> =
    BaseParserRuleContext<'input, Describe_tableContextExt<'input>>;

pub trait Describe_tableContextAttrs<'input>: SQLParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token Identifier
    /// Returns `None` if there is no child corresponding to token Identifier
    fn Identifier(&self) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Identifier, 0)
    }
}

impl<'input> Describe_tableContextAttrs<'input> for Describe_tableContext<'input> {}

pub struct Describe_tableContextExt<'input> {
    base: Table_statementContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {Describe_tableContextExt<'a>}

impl<'input> SQLParserContext<'input> for Describe_tableContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for Describe_tableContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_describe_table(self);
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.exit_describe_table(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for Describe_tableContext<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        visitor.visit_describe_table(self);
    }
}

impl<'input> CustomRuleContext<'input> for Describe_tableContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_table_statement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_table_statement }
}

impl<'input> Borrow<Table_statementContextExt<'input>> for Describe_tableContext<'input> {
    fn borrow(&self) -> &Table_statementContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<Table_statementContextExt<'input>> for Describe_tableContext<'input> {
    fn borrow_mut(&mut self) -> &mut Table_statementContextExt<'input> {
        &mut self.base
    }
}

impl<'input> Table_statementContextAttrs<'input> for Describe_tableContext<'input> {}

impl<'input> Describe_tableContextExt<'input> {
    fn new(ctx: &dyn Table_statementContextAttrs<'input>) -> Rc<Table_statementContextAll<'input>> {
        Rc::new(Table_statementContextAll::Describe_tableContext(
            BaseParserRuleContext::copy_from(
                ctx,
                Describe_tableContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type Select_table_Context<'input> =
    BaseParserRuleContext<'input, Select_table_ContextExt<'input>>;

pub trait Select_table_ContextAttrs<'input>: SQLParserContext<'input> {
    fn select_table(&self) -> Option<Rc<Select_tableContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Select_table_ContextAttrs<'input> for Select_table_Context<'input> {}

pub struct Select_table_ContextExt<'input> {
    base: Table_statementContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {Select_table_ContextExt<'a>}

impl<'input> SQLParserContext<'input> for Select_table_Context<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for Select_table_Context<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_select_table_(self);
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.exit_select_table_(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for Select_table_Context<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        visitor.visit_select_table_(self);
    }
}

impl<'input> CustomRuleContext<'input> for Select_table_ContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_table_statement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_table_statement }
}

impl<'input> Borrow<Table_statementContextExt<'input>> for Select_table_Context<'input> {
    fn borrow(&self) -> &Table_statementContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<Table_statementContextExt<'input>> for Select_table_Context<'input> {
    fn borrow_mut(&mut self) -> &mut Table_statementContextExt<'input> {
        &mut self.base
    }
}

impl<'input> Table_statementContextAttrs<'input> for Select_table_Context<'input> {}

impl<'input> Select_table_ContextExt<'input> {
    fn new(ctx: &dyn Table_statementContextAttrs<'input>) -> Rc<Table_statementContextAll<'input>> {
        Rc::new(Table_statementContextAll::Select_table_Context(
            BaseParserRuleContext::copy_from(
                ctx,
                Select_table_ContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type Drop_tableContext<'input> = BaseParserRuleContext<'input, Drop_tableContextExt<'input>>;

pub trait Drop_tableContextAttrs<'input>: SQLParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token Identifier
    /// Returns `None` if there is no child corresponding to token Identifier
    fn Identifier(&self) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Identifier, 0)
    }
}

impl<'input> Drop_tableContextAttrs<'input> for Drop_tableContext<'input> {}

pub struct Drop_tableContextExt<'input> {
    base: Table_statementContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {Drop_tableContextExt<'a>}

impl<'input> SQLParserContext<'input> for Drop_tableContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for Drop_tableContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_drop_table(self);
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.exit_drop_table(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for Drop_tableContext<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        visitor.visit_drop_table(self);
    }
}

impl<'input> CustomRuleContext<'input> for Drop_tableContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_table_statement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_table_statement }
}

impl<'input> Borrow<Table_statementContextExt<'input>> for Drop_tableContext<'input> {
    fn borrow(&self) -> &Table_statementContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<Table_statementContextExt<'input>> for Drop_tableContext<'input> {
    fn borrow_mut(&mut self) -> &mut Table_statementContextExt<'input> {
        &mut self.base
    }
}

impl<'input> Table_statementContextAttrs<'input> for Drop_tableContext<'input> {}

impl<'input> Drop_tableContextExt<'input> {
    fn new(ctx: &dyn Table_statementContextAttrs<'input>) -> Rc<Table_statementContextAll<'input>> {
        Rc::new(Table_statementContextAll::Drop_tableContext(
            BaseParserRuleContext::copy_from(
                ctx,
                Drop_tableContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type Update_tableContext<'input> =
    BaseParserRuleContext<'input, Update_tableContextExt<'input>>;

pub trait Update_tableContextAttrs<'input>: SQLParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token Identifier
    /// Returns `None` if there is no child corresponding to token Identifier
    fn Identifier(&self) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Identifier, 0)
    }
    fn set_clause(&self) -> Option<Rc<Set_clauseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn where_and_clause(&self) -> Option<Rc<Where_and_clauseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Update_tableContextAttrs<'input> for Update_tableContext<'input> {}

pub struct Update_tableContextExt<'input> {
    base: Table_statementContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {Update_tableContextExt<'a>}

impl<'input> SQLParserContext<'input> for Update_tableContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for Update_tableContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_update_table(self);
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.exit_update_table(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for Update_tableContext<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        visitor.visit_update_table(self);
    }
}

impl<'input> CustomRuleContext<'input> for Update_tableContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_table_statement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_table_statement }
}

impl<'input> Borrow<Table_statementContextExt<'input>> for Update_tableContext<'input> {
    fn borrow(&self) -> &Table_statementContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<Table_statementContextExt<'input>> for Update_tableContext<'input> {
    fn borrow_mut(&mut self) -> &mut Table_statementContextExt<'input> {
        &mut self.base
    }
}

impl<'input> Table_statementContextAttrs<'input> for Update_tableContext<'input> {}

impl<'input> Update_tableContextExt<'input> {
    fn new(ctx: &dyn Table_statementContextAttrs<'input>) -> Rc<Table_statementContextAll<'input>> {
        Rc::new(Table_statementContextAll::Update_tableContext(
            BaseParserRuleContext::copy_from(
                ctx,
                Update_tableContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

impl<'input, I, H> SQLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn table_statement(&mut self) -> Result<Rc<Table_statementContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Table_statementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 8, RULE_table_statement);
        let mut _localctx: Rc<Table_statementContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(134);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                T__1 => {
                    let tmp = Create_tableContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 1);
                    _localctx = tmp;
                    {
                        recog.base.set_state(104);
                        recog.base.match_token(T__1, &mut recog.err_handler)?;

                        recog.base.set_state(105);
                        recog.base.match_token(T__13, &mut recog.err_handler)?;

                        recog.base.set_state(106);
                        recog.base.match_token(Identifier, &mut recog.err_handler)?;

                        recog.base.set_state(107);
                        recog.base.match_token(T__15, &mut recog.err_handler)?;

                        /*InvokeRule field_list*/
                        recog.base.set_state(108);
                        recog.field_list()?;

                        recog.base.set_state(109);
                        recog.base.match_token(T__16, &mut recog.err_handler)?;
                    }
                }

                T__3 => {
                    let tmp = Drop_tableContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 2);
                    _localctx = tmp;
                    {
                        recog.base.set_state(111);
                        recog.base.match_token(T__3, &mut recog.err_handler)?;

                        recog.base.set_state(112);
                        recog.base.match_token(T__13, &mut recog.err_handler)?;

                        recog.base.set_state(113);
                        recog.base.match_token(Identifier, &mut recog.err_handler)?;
                    }
                }

                T__17 => {
                    let tmp = Describe_tableContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 3);
                    _localctx = tmp;
                    {
                        recog.base.set_state(114);
                        recog.base.match_token(T__17, &mut recog.err_handler)?;

                        recog.base.set_state(115);
                        recog.base.match_token(Identifier, &mut recog.err_handler)?;
                    }
                }

                T__18 => {
                    let tmp = Insert_into_tableContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 4);
                    _localctx = tmp;
                    {
                        recog.base.set_state(116);
                        recog.base.match_token(T__18, &mut recog.err_handler)?;

                        recog.base.set_state(117);
                        recog.base.match_token(T__19, &mut recog.err_handler)?;

                        recog.base.set_state(118);
                        recog.base.match_token(Identifier, &mut recog.err_handler)?;

                        recog.base.set_state(119);
                        recog.base.match_token(T__20, &mut recog.err_handler)?;

                        /*InvokeRule value_lists*/
                        recog.base.set_state(120);
                        recog.value_lists()?;
                    }
                }

                T__21 => {
                    let tmp = Delete_from_tableContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 5);
                    _localctx = tmp;
                    {
                        recog.base.set_state(121);
                        recog.base.match_token(T__21, &mut recog.err_handler)?;

                        recog.base.set_state(122);
                        recog.base.match_token(T__10, &mut recog.err_handler)?;

                        recog.base.set_state(123);
                        recog.base.match_token(Identifier, &mut recog.err_handler)?;

                        recog.base.set_state(124);
                        recog.base.match_token(T__22, &mut recog.err_handler)?;

                        /*InvokeRule where_and_clause*/
                        recog.base.set_state(125);
                        recog.where_and_clause()?;
                    }
                }

                T__23 => {
                    let tmp = Update_tableContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 6);
                    _localctx = tmp;
                    {
                        recog.base.set_state(126);
                        recog.base.match_token(T__23, &mut recog.err_handler)?;

                        recog.base.set_state(127);
                        recog.base.match_token(Identifier, &mut recog.err_handler)?;

                        recog.base.set_state(128);
                        recog.base.match_token(T__24, &mut recog.err_handler)?;

                        /*InvokeRule set_clause*/
                        recog.base.set_state(129);
                        recog.set_clause()?;

                        recog.base.set_state(130);
                        recog.base.match_token(T__22, &mut recog.err_handler)?;

                        /*InvokeRule where_and_clause*/
                        recog.base.set_state(131);
                        recog.where_and_clause()?;
                    }
                }

                T__25 => {
                    let tmp = Select_table_ContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 7);
                    _localctx = tmp;
                    {
                        /*InvokeRule select_table*/
                        recog.base.set_state(133);
                        recog.select_table()?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- select_table ----------------
pub type Select_tableContextAll<'input> = Select_tableContext<'input>;

pub type Select_tableContext<'input> =
    BaseParserRuleContext<'input, Select_tableContextExt<'input>>;

#[derive(Clone)]
pub struct Select_tableContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLParserContext<'input> for Select_tableContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for Select_tableContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_select_table(self);
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.exit_select_table(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for Select_tableContext<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        visitor.visit_select_table(self);
    }
}

impl<'input> CustomRuleContext<'input> for Select_tableContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_select_table
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_select_table }
}
antlr_rust::tid! {Select_tableContextExt<'a>}

impl<'input> Select_tableContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Select_tableContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Select_tableContextExt { ph: PhantomData },
        ))
    }
}

pub trait Select_tableContextAttrs<'input>:
    SQLParserContext<'input> + BorrowMut<Select_tableContextExt<'input>>
{
    fn selectors(&self) -> Option<Rc<SelectorsContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn identifiers(&self) -> Option<Rc<IdentifiersContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn where_and_clause(&self) -> Option<Rc<Where_and_clauseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn column(&self) -> Option<Rc<ColumnContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token Integer in current rule
    fn Integer_all(&self) -> Vec<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token Integer, starting from 0.
    /// Returns `None` if number of children corresponding to token Integer is less or equal than `i`.
    fn Integer(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Integer, i)
    }
}

impl<'input> Select_tableContextAttrs<'input> for Select_tableContext<'input> {}

impl<'input, I, H> SQLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn select_table(&mut self) -> Result<Rc<Select_tableContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Select_tableContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 10, RULE_select_table);
        let mut _localctx: Rc<Select_tableContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(136);
                recog.base.match_token(T__25, &mut recog.err_handler)?;

                /*InvokeRule selectors*/
                recog.base.set_state(137);
                recog.selectors()?;

                recog.base.set_state(138);
                recog.base.match_token(T__10, &mut recog.err_handler)?;

                /*InvokeRule identifiers*/
                recog.base.set_state(139);
                recog.identifiers()?;

                recog.base.set_state(142);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == T__22 {
                    {
                        recog.base.set_state(140);
                        recog.base.match_token(T__22, &mut recog.err_handler)?;

                        /*InvokeRule where_and_clause*/
                        recog.base.set_state(141);
                        recog.where_and_clause()?;
                    }
                }

                recog.base.set_state(147);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == T__26 {
                    {
                        recog.base.set_state(144);
                        recog.base.match_token(T__26, &mut recog.err_handler)?;

                        recog.base.set_state(145);
                        recog.base.match_token(T__27, &mut recog.err_handler)?;

                        /*InvokeRule column*/
                        recog.base.set_state(146);
                        recog.column()?;
                    }
                }

                recog.base.set_state(155);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == T__28 {
                    {
                        recog.base.set_state(149);
                        recog.base.match_token(T__28, &mut recog.err_handler)?;

                        recog.base.set_state(150);
                        recog.base.match_token(Integer, &mut recog.err_handler)?;

                        recog.base.set_state(153);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == T__29 {
                            {
                                recog.base.set_state(151);
                                recog.base.match_token(T__29, &mut recog.err_handler)?;

                                recog.base.set_state(152);
                                recog.base.match_token(Integer, &mut recog.err_handler)?;
                            }
                        }
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- alter_statement ----------------
#[derive(Debug)]
pub enum Alter_statementContextAll<'input> {
    Alter_table_drop_pkContext(Alter_table_drop_pkContext<'input>),
    Alter_table_add_foreign_keyContext(Alter_table_add_foreign_keyContext<'input>),
    Alter_table_add_uniqueContext(Alter_table_add_uniqueContext<'input>),
    Alter_drop_indexContext(Alter_drop_indexContext<'input>),
    Alter_add_indexContext(Alter_add_indexContext<'input>),
    Alter_table_drop_foreign_keyContext(Alter_table_drop_foreign_keyContext<'input>),
    Alter_table_add_pkContext(Alter_table_add_pkContext<'input>),
    Error(Alter_statementContext<'input>),
}
antlr_rust::tid! {Alter_statementContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for Alter_statementContextAll<'input> {}

impl<'input> SQLParserContext<'input> for Alter_statementContextAll<'input> {}

impl<'input> Deref for Alter_statementContextAll<'input> {
    type Target = dyn Alter_statementContextAttrs<'input> + 'input;
    fn deref(&self) -> &Self::Target {
        use Alter_statementContextAll::*;
        match self {
            Alter_table_drop_pkContext(inner) => inner,
            Alter_table_add_foreign_keyContext(inner) => inner,
            Alter_table_add_uniqueContext(inner) => inner,
            Alter_drop_indexContext(inner) => inner,
            Alter_add_indexContext(inner) => inner,
            Alter_table_drop_foreign_keyContext(inner) => inner,
            Alter_table_add_pkContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}
impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for Alter_statementContextAll<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        self.deref().accept(visitor)
    }
}
impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for Alter_statementContextAll<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        self.deref().enter(listener)
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        self.deref().exit(listener)
    }
}

pub type Alter_statementContext<'input> =
    BaseParserRuleContext<'input, Alter_statementContextExt<'input>>;

#[derive(Clone)]
pub struct Alter_statementContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLParserContext<'input> for Alter_statementContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for Alter_statementContext<'input> {}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for Alter_statementContext<'input> {}

impl<'input> CustomRuleContext<'input> for Alter_statementContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_alter_statement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_alter_statement }
}
antlr_rust::tid! {Alter_statementContextExt<'a>}

impl<'input> Alter_statementContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Alter_statementContextAll<'input>> {
        Rc::new(Alter_statementContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(
                parent,
                invoking_state,
                Alter_statementContextExt { ph: PhantomData },
            ),
        ))
    }
}

pub trait Alter_statementContextAttrs<'input>:
    SQLParserContext<'input> + BorrowMut<Alter_statementContextExt<'input>>
{
}

impl<'input> Alter_statementContextAttrs<'input> for Alter_statementContext<'input> {}

pub type Alter_table_drop_pkContext<'input> =
    BaseParserRuleContext<'input, Alter_table_drop_pkContextExt<'input>>;

pub trait Alter_table_drop_pkContextAttrs<'input>: SQLParserContext<'input> {
    /// Retrieves all `TerminalNode`s corresponding to token Identifier in current rule
    fn Identifier_all(&self) -> Vec<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token Identifier, starting from 0.
    /// Returns `None` if number of children corresponding to token Identifier is less or equal than `i`.
    fn Identifier(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Identifier, i)
    }
}

impl<'input> Alter_table_drop_pkContextAttrs<'input> for Alter_table_drop_pkContext<'input> {}

pub struct Alter_table_drop_pkContextExt<'input> {
    base: Alter_statementContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {Alter_table_drop_pkContextExt<'a>}

impl<'input> SQLParserContext<'input> for Alter_table_drop_pkContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for Alter_table_drop_pkContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_alter_table_drop_pk(self);
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.exit_alter_table_drop_pk(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for Alter_table_drop_pkContext<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        visitor.visit_alter_table_drop_pk(self);
    }
}

impl<'input> CustomRuleContext<'input> for Alter_table_drop_pkContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_alter_statement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_alter_statement }
}

impl<'input> Borrow<Alter_statementContextExt<'input>> for Alter_table_drop_pkContext<'input> {
    fn borrow(&self) -> &Alter_statementContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<Alter_statementContextExt<'input>> for Alter_table_drop_pkContext<'input> {
    fn borrow_mut(&mut self) -> &mut Alter_statementContextExt<'input> {
        &mut self.base
    }
}

impl<'input> Alter_statementContextAttrs<'input> for Alter_table_drop_pkContext<'input> {}

impl<'input> Alter_table_drop_pkContextExt<'input> {
    fn new(ctx: &dyn Alter_statementContextAttrs<'input>) -> Rc<Alter_statementContextAll<'input>> {
        Rc::new(Alter_statementContextAll::Alter_table_drop_pkContext(
            BaseParserRuleContext::copy_from(
                ctx,
                Alter_table_drop_pkContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type Alter_table_add_foreign_keyContext<'input> =
    BaseParserRuleContext<'input, Alter_table_add_foreign_keyContextExt<'input>>;

pub trait Alter_table_add_foreign_keyContextAttrs<'input>: SQLParserContext<'input> {
    /// Retrieves all `TerminalNode`s corresponding to token Identifier in current rule
    fn Identifier_all(&self) -> Vec<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token Identifier, starting from 0.
    /// Returns `None` if number of children corresponding to token Identifier is less or equal than `i`.
    fn Identifier(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Identifier, i)
    }
    fn identifiers_all(&self) -> Vec<Rc<IdentifiersContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn identifiers(&self, i: usize) -> Option<Rc<IdentifiersContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Alter_table_add_foreign_keyContextAttrs<'input>
    for Alter_table_add_foreign_keyContext<'input>
{
}

pub struct Alter_table_add_foreign_keyContextExt<'input> {
    base: Alter_statementContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {Alter_table_add_foreign_keyContextExt<'a>}

impl<'input> SQLParserContext<'input> for Alter_table_add_foreign_keyContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a>
    for Alter_table_add_foreign_keyContext<'input>
{
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_alter_table_add_foreign_key(self);
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.exit_alter_table_add_foreign_key(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a>
    for Alter_table_add_foreign_keyContext<'input>
{
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        visitor.visit_alter_table_add_foreign_key(self);
    }
}

impl<'input> CustomRuleContext<'input> for Alter_table_add_foreign_keyContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_alter_statement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_alter_statement }
}

impl<'input> Borrow<Alter_statementContextExt<'input>>
    for Alter_table_add_foreign_keyContext<'input>
{
    fn borrow(&self) -> &Alter_statementContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<Alter_statementContextExt<'input>>
    for Alter_table_add_foreign_keyContext<'input>
{
    fn borrow_mut(&mut self) -> &mut Alter_statementContextExt<'input> {
        &mut self.base
    }
}

impl<'input> Alter_statementContextAttrs<'input> for Alter_table_add_foreign_keyContext<'input> {}

impl<'input> Alter_table_add_foreign_keyContextExt<'input> {
    fn new(ctx: &dyn Alter_statementContextAttrs<'input>) -> Rc<Alter_statementContextAll<'input>> {
        Rc::new(
            Alter_statementContextAll::Alter_table_add_foreign_keyContext(
                BaseParserRuleContext::copy_from(
                    ctx,
                    Alter_table_add_foreign_keyContextExt {
                        base: ctx.borrow().clone(),
                        ph: PhantomData,
                    },
                ),
            ),
        )
    }
}

pub type Alter_table_add_uniqueContext<'input> =
    BaseParserRuleContext<'input, Alter_table_add_uniqueContextExt<'input>>;

pub trait Alter_table_add_uniqueContextAttrs<'input>: SQLParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token Identifier
    /// Returns `None` if there is no child corresponding to token Identifier
    fn Identifier(&self) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Identifier, 0)
    }
    fn identifiers(&self) -> Option<Rc<IdentifiersContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Alter_table_add_uniqueContextAttrs<'input> for Alter_table_add_uniqueContext<'input> {}

pub struct Alter_table_add_uniqueContextExt<'input> {
    base: Alter_statementContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {Alter_table_add_uniqueContextExt<'a>}

impl<'input> SQLParserContext<'input> for Alter_table_add_uniqueContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a>
    for Alter_table_add_uniqueContext<'input>
{
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_alter_table_add_unique(self);
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.exit_alter_table_add_unique(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for Alter_table_add_uniqueContext<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        visitor.visit_alter_table_add_unique(self);
    }
}

impl<'input> CustomRuleContext<'input> for Alter_table_add_uniqueContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_alter_statement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_alter_statement }
}

impl<'input> Borrow<Alter_statementContextExt<'input>> for Alter_table_add_uniqueContext<'input> {
    fn borrow(&self) -> &Alter_statementContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<Alter_statementContextExt<'input>>
    for Alter_table_add_uniqueContext<'input>
{
    fn borrow_mut(&mut self) -> &mut Alter_statementContextExt<'input> {
        &mut self.base
    }
}

impl<'input> Alter_statementContextAttrs<'input> for Alter_table_add_uniqueContext<'input> {}

impl<'input> Alter_table_add_uniqueContextExt<'input> {
    fn new(ctx: &dyn Alter_statementContextAttrs<'input>) -> Rc<Alter_statementContextAll<'input>> {
        Rc::new(Alter_statementContextAll::Alter_table_add_uniqueContext(
            BaseParserRuleContext::copy_from(
                ctx,
                Alter_table_add_uniqueContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type Alter_drop_indexContext<'input> =
    BaseParserRuleContext<'input, Alter_drop_indexContextExt<'input>>;

pub trait Alter_drop_indexContextAttrs<'input>: SQLParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token Identifier
    /// Returns `None` if there is no child corresponding to token Identifier
    fn Identifier(&self) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Identifier, 0)
    }
    fn identifiers(&self) -> Option<Rc<IdentifiersContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Alter_drop_indexContextAttrs<'input> for Alter_drop_indexContext<'input> {}

pub struct Alter_drop_indexContextExt<'input> {
    base: Alter_statementContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {Alter_drop_indexContextExt<'a>}

impl<'input> SQLParserContext<'input> for Alter_drop_indexContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for Alter_drop_indexContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_alter_drop_index(self);
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.exit_alter_drop_index(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for Alter_drop_indexContext<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        visitor.visit_alter_drop_index(self);
    }
}

impl<'input> CustomRuleContext<'input> for Alter_drop_indexContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_alter_statement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_alter_statement }
}

impl<'input> Borrow<Alter_statementContextExt<'input>> for Alter_drop_indexContext<'input> {
    fn borrow(&self) -> &Alter_statementContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<Alter_statementContextExt<'input>> for Alter_drop_indexContext<'input> {
    fn borrow_mut(&mut self) -> &mut Alter_statementContextExt<'input> {
        &mut self.base
    }
}

impl<'input> Alter_statementContextAttrs<'input> for Alter_drop_indexContext<'input> {}

impl<'input> Alter_drop_indexContextExt<'input> {
    fn new(ctx: &dyn Alter_statementContextAttrs<'input>) -> Rc<Alter_statementContextAll<'input>> {
        Rc::new(Alter_statementContextAll::Alter_drop_indexContext(
            BaseParserRuleContext::copy_from(
                ctx,
                Alter_drop_indexContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type Alter_add_indexContext<'input> =
    BaseParserRuleContext<'input, Alter_add_indexContextExt<'input>>;

pub trait Alter_add_indexContextAttrs<'input>: SQLParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token Identifier
    /// Returns `None` if there is no child corresponding to token Identifier
    fn Identifier(&self) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Identifier, 0)
    }
    fn identifiers(&self) -> Option<Rc<IdentifiersContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Alter_add_indexContextAttrs<'input> for Alter_add_indexContext<'input> {}

pub struct Alter_add_indexContextExt<'input> {
    base: Alter_statementContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {Alter_add_indexContextExt<'a>}

impl<'input> SQLParserContext<'input> for Alter_add_indexContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for Alter_add_indexContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_alter_add_index(self);
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.exit_alter_add_index(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for Alter_add_indexContext<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        visitor.visit_alter_add_index(self);
    }
}

impl<'input> CustomRuleContext<'input> for Alter_add_indexContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_alter_statement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_alter_statement }
}

impl<'input> Borrow<Alter_statementContextExt<'input>> for Alter_add_indexContext<'input> {
    fn borrow(&self) -> &Alter_statementContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<Alter_statementContextExt<'input>> for Alter_add_indexContext<'input> {
    fn borrow_mut(&mut self) -> &mut Alter_statementContextExt<'input> {
        &mut self.base
    }
}

impl<'input> Alter_statementContextAttrs<'input> for Alter_add_indexContext<'input> {}

impl<'input> Alter_add_indexContextExt<'input> {
    fn new(ctx: &dyn Alter_statementContextAttrs<'input>) -> Rc<Alter_statementContextAll<'input>> {
        Rc::new(Alter_statementContextAll::Alter_add_indexContext(
            BaseParserRuleContext::copy_from(
                ctx,
                Alter_add_indexContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type Alter_table_drop_foreign_keyContext<'input> =
    BaseParserRuleContext<'input, Alter_table_drop_foreign_keyContextExt<'input>>;

pub trait Alter_table_drop_foreign_keyContextAttrs<'input>: SQLParserContext<'input> {
    /// Retrieves all `TerminalNode`s corresponding to token Identifier in current rule
    fn Identifier_all(&self) -> Vec<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token Identifier, starting from 0.
    /// Returns `None` if number of children corresponding to token Identifier is less or equal than `i`.
    fn Identifier(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Identifier, i)
    }
}

impl<'input> Alter_table_drop_foreign_keyContextAttrs<'input>
    for Alter_table_drop_foreign_keyContext<'input>
{
}

pub struct Alter_table_drop_foreign_keyContextExt<'input> {
    base: Alter_statementContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {Alter_table_drop_foreign_keyContextExt<'a>}

impl<'input> SQLParserContext<'input> for Alter_table_drop_foreign_keyContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a>
    for Alter_table_drop_foreign_keyContext<'input>
{
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_alter_table_drop_foreign_key(self);
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.exit_alter_table_drop_foreign_key(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a>
    for Alter_table_drop_foreign_keyContext<'input>
{
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        visitor.visit_alter_table_drop_foreign_key(self);
    }
}

impl<'input> CustomRuleContext<'input> for Alter_table_drop_foreign_keyContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_alter_statement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_alter_statement }
}

impl<'input> Borrow<Alter_statementContextExt<'input>>
    for Alter_table_drop_foreign_keyContext<'input>
{
    fn borrow(&self) -> &Alter_statementContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<Alter_statementContextExt<'input>>
    for Alter_table_drop_foreign_keyContext<'input>
{
    fn borrow_mut(&mut self) -> &mut Alter_statementContextExt<'input> {
        &mut self.base
    }
}

impl<'input> Alter_statementContextAttrs<'input> for Alter_table_drop_foreign_keyContext<'input> {}

impl<'input> Alter_table_drop_foreign_keyContextExt<'input> {
    fn new(ctx: &dyn Alter_statementContextAttrs<'input>) -> Rc<Alter_statementContextAll<'input>> {
        Rc::new(
            Alter_statementContextAll::Alter_table_drop_foreign_keyContext(
                BaseParserRuleContext::copy_from(
                    ctx,
                    Alter_table_drop_foreign_keyContextExt {
                        base: ctx.borrow().clone(),
                        ph: PhantomData,
                    },
                ),
            ),
        )
    }
}

pub type Alter_table_add_pkContext<'input> =
    BaseParserRuleContext<'input, Alter_table_add_pkContextExt<'input>>;

pub trait Alter_table_add_pkContextAttrs<'input>: SQLParserContext<'input> {
    /// Retrieves all `TerminalNode`s corresponding to token Identifier in current rule
    fn Identifier_all(&self) -> Vec<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token Identifier, starting from 0.
    /// Returns `None` if number of children corresponding to token Identifier is less or equal than `i`.
    fn Identifier(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Identifier, i)
    }
    fn identifiers(&self) -> Option<Rc<IdentifiersContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Alter_table_add_pkContextAttrs<'input> for Alter_table_add_pkContext<'input> {}

pub struct Alter_table_add_pkContextExt<'input> {
    base: Alter_statementContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {Alter_table_add_pkContextExt<'a>}

impl<'input> SQLParserContext<'input> for Alter_table_add_pkContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for Alter_table_add_pkContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_alter_table_add_pk(self);
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.exit_alter_table_add_pk(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for Alter_table_add_pkContext<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        visitor.visit_alter_table_add_pk(self);
    }
}

impl<'input> CustomRuleContext<'input> for Alter_table_add_pkContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_alter_statement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_alter_statement }
}

impl<'input> Borrow<Alter_statementContextExt<'input>> for Alter_table_add_pkContext<'input> {
    fn borrow(&self) -> &Alter_statementContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<Alter_statementContextExt<'input>> for Alter_table_add_pkContext<'input> {
    fn borrow_mut(&mut self) -> &mut Alter_statementContextExt<'input> {
        &mut self.base
    }
}

impl<'input> Alter_statementContextAttrs<'input> for Alter_table_add_pkContext<'input> {}

impl<'input> Alter_table_add_pkContextExt<'input> {
    fn new(ctx: &dyn Alter_statementContextAttrs<'input>) -> Rc<Alter_statementContextAll<'input>> {
        Rc::new(Alter_statementContextAll::Alter_table_add_pkContext(
            BaseParserRuleContext::copy_from(
                ctx,
                Alter_table_add_pkContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

impl<'input, I, H> SQLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn alter_statement(&mut self) -> Result<Rc<Alter_statementContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Alter_statementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 12, RULE_alter_statement);
        let mut _localctx: Rc<Alter_statementContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(233);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(12, &mut recog.base)? {
                1 => {
                    let tmp = Alter_add_indexContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 1);
                    _localctx = tmp;
                    {
                        recog.base.set_state(157);
                        recog.base.match_token(T__30, &mut recog.err_handler)?;

                        recog.base.set_state(158);
                        recog.base.match_token(T__13, &mut recog.err_handler)?;

                        recog.base.set_state(159);
                        recog.base.match_token(Identifier, &mut recog.err_handler)?;

                        recog.base.set_state(160);
                        recog.base.match_token(T__31, &mut recog.err_handler)?;

                        recog.base.set_state(161);
                        recog.base.match_token(T__32, &mut recog.err_handler)?;

                        recog.base.set_state(162);
                        recog.base.match_token(T__15, &mut recog.err_handler)?;

                        /*InvokeRule identifiers*/
                        recog.base.set_state(163);
                        recog.identifiers()?;

                        recog.base.set_state(164);
                        recog.base.match_token(T__16, &mut recog.err_handler)?;
                    }
                }
                2 => {
                    let tmp = Alter_drop_indexContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 2);
                    _localctx = tmp;
                    {
                        recog.base.set_state(166);
                        recog.base.match_token(T__30, &mut recog.err_handler)?;

                        recog.base.set_state(167);
                        recog.base.match_token(T__13, &mut recog.err_handler)?;

                        recog.base.set_state(168);
                        recog.base.match_token(Identifier, &mut recog.err_handler)?;

                        recog.base.set_state(169);
                        recog.base.match_token(T__3, &mut recog.err_handler)?;

                        recog.base.set_state(170);
                        recog.base.match_token(T__32, &mut recog.err_handler)?;

                        recog.base.set_state(171);
                        recog.base.match_token(T__15, &mut recog.err_handler)?;

                        /*InvokeRule identifiers*/
                        recog.base.set_state(172);
                        recog.identifiers()?;

                        recog.base.set_state(173);
                        recog.base.match_token(T__16, &mut recog.err_handler)?;
                    }
                }
                3 => {
                    let tmp = Alter_table_drop_pkContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 3);
                    _localctx = tmp;
                    {
                        recog.base.set_state(175);
                        recog.base.match_token(T__30, &mut recog.err_handler)?;

                        recog.base.set_state(176);
                        recog.base.match_token(T__13, &mut recog.err_handler)?;

                        recog.base.set_state(177);
                        recog.base.match_token(Identifier, &mut recog.err_handler)?;

                        recog.base.set_state(178);
                        recog.base.match_token(T__3, &mut recog.err_handler)?;

                        recog.base.set_state(179);
                        recog.base.match_token(T__33, &mut recog.err_handler)?;

                        recog.base.set_state(180);
                        recog.base.match_token(T__34, &mut recog.err_handler)?;

                        recog.base.set_state(182);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == Identifier {
                            {
                                recog.base.set_state(181);
                                recog.base.match_token(Identifier, &mut recog.err_handler)?;
                            }
                        }
                    }
                }
                4 => {
                    let tmp = Alter_table_drop_foreign_keyContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 4);
                    _localctx = tmp;
                    {
                        recog.base.set_state(184);
                        recog.base.match_token(T__30, &mut recog.err_handler)?;

                        recog.base.set_state(185);
                        recog.base.match_token(T__13, &mut recog.err_handler)?;

                        recog.base.set_state(186);
                        recog.base.match_token(Identifier, &mut recog.err_handler)?;

                        recog.base.set_state(187);
                        recog.base.match_token(T__3, &mut recog.err_handler)?;

                        recog.base.set_state(188);
                        recog.base.match_token(T__35, &mut recog.err_handler)?;

                        recog.base.set_state(189);
                        recog.base.match_token(T__34, &mut recog.err_handler)?;

                        recog.base.set_state(190);
                        recog.base.match_token(Identifier, &mut recog.err_handler)?;
                    }
                }
                5 => {
                    let tmp = Alter_table_add_pkContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 5);
                    _localctx = tmp;
                    {
                        recog.base.set_state(191);
                        recog.base.match_token(T__30, &mut recog.err_handler)?;

                        recog.base.set_state(192);
                        recog.base.match_token(T__13, &mut recog.err_handler)?;

                        recog.base.set_state(193);
                        recog.base.match_token(Identifier, &mut recog.err_handler)?;

                        recog.base.set_state(194);
                        recog.base.match_token(T__31, &mut recog.err_handler)?;

                        recog.base.set_state(195);
                        recog.base.match_token(T__36, &mut recog.err_handler)?;

                        recog.base.set_state(197);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == Identifier {
                            {
                                recog.base.set_state(196);
                                recog.base.match_token(Identifier, &mut recog.err_handler)?;
                            }
                        }

                        recog.base.set_state(199);
                        recog.base.match_token(T__33, &mut recog.err_handler)?;

                        recog.base.set_state(200);
                        recog.base.match_token(T__34, &mut recog.err_handler)?;

                        recog.base.set_state(201);
                        recog.base.match_token(T__15, &mut recog.err_handler)?;

                        /*InvokeRule identifiers*/
                        recog.base.set_state(202);
                        recog.identifiers()?;

                        recog.base.set_state(203);
                        recog.base.match_token(T__16, &mut recog.err_handler)?;
                    }
                }
                6 => {
                    let tmp = Alter_table_add_foreign_keyContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 6);
                    _localctx = tmp;
                    {
                        recog.base.set_state(205);
                        recog.base.match_token(T__30, &mut recog.err_handler)?;

                        recog.base.set_state(206);
                        recog.base.match_token(T__13, &mut recog.err_handler)?;

                        recog.base.set_state(207);
                        recog.base.match_token(Identifier, &mut recog.err_handler)?;

                        recog.base.set_state(208);
                        recog.base.match_token(T__31, &mut recog.err_handler)?;

                        recog.base.set_state(209);
                        recog.base.match_token(T__36, &mut recog.err_handler)?;

                        recog.base.set_state(211);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == Identifier {
                            {
                                recog.base.set_state(210);
                                recog.base.match_token(Identifier, &mut recog.err_handler)?;
                            }
                        }

                        recog.base.set_state(213);
                        recog.base.match_token(T__35, &mut recog.err_handler)?;

                        recog.base.set_state(214);
                        recog.base.match_token(T__34, &mut recog.err_handler)?;

                        recog.base.set_state(215);
                        recog.base.match_token(T__15, &mut recog.err_handler)?;

                        /*InvokeRule identifiers*/
                        recog.base.set_state(216);
                        recog.identifiers()?;

                        recog.base.set_state(217);
                        recog.base.match_token(T__16, &mut recog.err_handler)?;

                        recog.base.set_state(218);
                        recog.base.match_token(T__37, &mut recog.err_handler)?;

                        recog.base.set_state(219);
                        recog.base.match_token(Identifier, &mut recog.err_handler)?;

                        recog.base.set_state(220);
                        recog.base.match_token(T__15, &mut recog.err_handler)?;

                        /*InvokeRule identifiers*/
                        recog.base.set_state(221);
                        recog.identifiers()?;

                        recog.base.set_state(222);
                        recog.base.match_token(T__16, &mut recog.err_handler)?;
                    }
                }
                7 => {
                    let tmp = Alter_table_add_uniqueContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 7);
                    _localctx = tmp;
                    {
                        recog.base.set_state(224);
                        recog.base.match_token(T__30, &mut recog.err_handler)?;

                        recog.base.set_state(225);
                        recog.base.match_token(T__13, &mut recog.err_handler)?;

                        recog.base.set_state(226);
                        recog.base.match_token(Identifier, &mut recog.err_handler)?;

                        recog.base.set_state(227);
                        recog.base.match_token(T__31, &mut recog.err_handler)?;

                        recog.base.set_state(228);
                        recog.base.match_token(T__38, &mut recog.err_handler)?;

                        recog.base.set_state(229);
                        recog.base.match_token(T__15, &mut recog.err_handler)?;

                        /*InvokeRule identifiers*/
                        recog.base.set_state(230);
                        recog.identifiers()?;

                        recog.base.set_state(231);
                        recog.base.match_token(T__16, &mut recog.err_handler)?;
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- field_list ----------------
pub type Field_listContextAll<'input> = Field_listContext<'input>;

pub type Field_listContext<'input> = BaseParserRuleContext<'input, Field_listContextExt<'input>>;

#[derive(Clone)]
pub struct Field_listContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLParserContext<'input> for Field_listContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for Field_listContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_field_list(self);
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.exit_field_list(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for Field_listContext<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        visitor.visit_field_list(self);
    }
}

impl<'input> CustomRuleContext<'input> for Field_listContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_field_list
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_field_list }
}
antlr_rust::tid! {Field_listContextExt<'a>}

impl<'input> Field_listContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Field_listContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Field_listContextExt { ph: PhantomData },
        ))
    }
}

pub trait Field_listContextAttrs<'input>:
    SQLParserContext<'input> + BorrowMut<Field_listContextExt<'input>>
{
    fn field_all(&self) -> Vec<Rc<FieldContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn field(&self, i: usize) -> Option<Rc<FieldContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Field_listContextAttrs<'input> for Field_listContext<'input> {}

impl<'input, I, H> SQLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn field_list(&mut self) -> Result<Rc<Field_listContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Field_listContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 14, RULE_field_list);
        let mut _localctx: Rc<Field_listContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule field*/
                recog.base.set_state(235);
                recog.field()?;

                recog.base.set_state(240);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == T__39 {
                    {
                        {
                            recog.base.set_state(236);
                            recog.base.match_token(T__39, &mut recog.err_handler)?;

                            /*InvokeRule field*/
                            recog.base.set_state(237);
                            recog.field()?;
                        }
                    }
                    recog.base.set_state(242);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- field ----------------
#[derive(Debug)]
pub enum FieldContextAll<'input> {
    Primary_key_fieldContext(Primary_key_fieldContext<'input>),
    Foreign_key_fieldContext(Foreign_key_fieldContext<'input>),
    Normal_fieldContext(Normal_fieldContext<'input>),
    Error(FieldContext<'input>),
}
antlr_rust::tid! {FieldContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for FieldContextAll<'input> {}

impl<'input> SQLParserContext<'input> for FieldContextAll<'input> {}

impl<'input> Deref for FieldContextAll<'input> {
    type Target = dyn FieldContextAttrs<'input> + 'input;
    fn deref(&self) -> &Self::Target {
        use FieldContextAll::*;
        match self {
            Primary_key_fieldContext(inner) => inner,
            Foreign_key_fieldContext(inner) => inner,
            Normal_fieldContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}
impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for FieldContextAll<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        self.deref().accept(visitor)
    }
}
impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for FieldContextAll<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        self.deref().enter(listener)
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        self.deref().exit(listener)
    }
}

pub type FieldContext<'input> = BaseParserRuleContext<'input, FieldContextExt<'input>>;

#[derive(Clone)]
pub struct FieldContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLParserContext<'input> for FieldContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for FieldContext<'input> {}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for FieldContext<'input> {}

impl<'input> CustomRuleContext<'input> for FieldContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_field
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_field }
}
antlr_rust::tid! {FieldContextExt<'a>}

impl<'input> FieldContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<FieldContextAll<'input>> {
        Rc::new(FieldContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(
                parent,
                invoking_state,
                FieldContextExt { ph: PhantomData },
            ),
        ))
    }
}

pub trait FieldContextAttrs<'input>:
    SQLParserContext<'input> + BorrowMut<FieldContextExt<'input>>
{
}

impl<'input> FieldContextAttrs<'input> for FieldContext<'input> {}

pub type Primary_key_fieldContext<'input> =
    BaseParserRuleContext<'input, Primary_key_fieldContextExt<'input>>;

pub trait Primary_key_fieldContextAttrs<'input>: SQLParserContext<'input> {
    fn identifiers(&self) -> Option<Rc<IdentifiersContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token Identifier
    /// Returns `None` if there is no child corresponding to token Identifier
    fn Identifier(&self) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Identifier, 0)
    }
}

impl<'input> Primary_key_fieldContextAttrs<'input> for Primary_key_fieldContext<'input> {}

pub struct Primary_key_fieldContextExt<'input> {
    base: FieldContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {Primary_key_fieldContextExt<'a>}

impl<'input> SQLParserContext<'input> for Primary_key_fieldContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for Primary_key_fieldContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_primary_key_field(self);
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.exit_primary_key_field(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for Primary_key_fieldContext<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        visitor.visit_primary_key_field(self);
    }
}

impl<'input> CustomRuleContext<'input> for Primary_key_fieldContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_field
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_field }
}

impl<'input> Borrow<FieldContextExt<'input>> for Primary_key_fieldContext<'input> {
    fn borrow(&self) -> &FieldContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<FieldContextExt<'input>> for Primary_key_fieldContext<'input> {
    fn borrow_mut(&mut self) -> &mut FieldContextExt<'input> {
        &mut self.base
    }
}

impl<'input> FieldContextAttrs<'input> for Primary_key_fieldContext<'input> {}

impl<'input> Primary_key_fieldContextExt<'input> {
    fn new(ctx: &dyn FieldContextAttrs<'input>) -> Rc<FieldContextAll<'input>> {
        Rc::new(FieldContextAll::Primary_key_fieldContext(
            BaseParserRuleContext::copy_from(
                ctx,
                Primary_key_fieldContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type Foreign_key_fieldContext<'input> =
    BaseParserRuleContext<'input, Foreign_key_fieldContextExt<'input>>;

pub trait Foreign_key_fieldContextAttrs<'input>: SQLParserContext<'input> {
    fn identifiers_all(&self) -> Vec<Rc<IdentifiersContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn identifiers(&self, i: usize) -> Option<Rc<IdentifiersContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token Identifier in current rule
    fn Identifier_all(&self) -> Vec<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token Identifier, starting from 0.
    /// Returns `None` if number of children corresponding to token Identifier is less or equal than `i`.
    fn Identifier(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Identifier, i)
    }
}

impl<'input> Foreign_key_fieldContextAttrs<'input> for Foreign_key_fieldContext<'input> {}

pub struct Foreign_key_fieldContextExt<'input> {
    base: FieldContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {Foreign_key_fieldContextExt<'a>}

impl<'input> SQLParserContext<'input> for Foreign_key_fieldContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for Foreign_key_fieldContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_foreign_key_field(self);
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.exit_foreign_key_field(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for Foreign_key_fieldContext<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        visitor.visit_foreign_key_field(self);
    }
}

impl<'input> CustomRuleContext<'input> for Foreign_key_fieldContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_field
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_field }
}

impl<'input> Borrow<FieldContextExt<'input>> for Foreign_key_fieldContext<'input> {
    fn borrow(&self) -> &FieldContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<FieldContextExt<'input>> for Foreign_key_fieldContext<'input> {
    fn borrow_mut(&mut self) -> &mut FieldContextExt<'input> {
        &mut self.base
    }
}

impl<'input> FieldContextAttrs<'input> for Foreign_key_fieldContext<'input> {}

impl<'input> Foreign_key_fieldContextExt<'input> {
    fn new(ctx: &dyn FieldContextAttrs<'input>) -> Rc<FieldContextAll<'input>> {
        Rc::new(FieldContextAll::Foreign_key_fieldContext(
            BaseParserRuleContext::copy_from(
                ctx,
                Foreign_key_fieldContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type Normal_fieldContext<'input> =
    BaseParserRuleContext<'input, Normal_fieldContextExt<'input>>;

pub trait Normal_fieldContextAttrs<'input>: SQLParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token Identifier
    /// Returns `None` if there is no child corresponding to token Identifier
    fn Identifier(&self) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Identifier, 0)
    }
    fn type_(&self) -> Option<Rc<Type_ContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token Null
    /// Returns `None` if there is no child corresponding to token Null
    fn Null(&self) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Null, 0)
    }
    fn value(&self) -> Option<Rc<ValueContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Normal_fieldContextAttrs<'input> for Normal_fieldContext<'input> {}

pub struct Normal_fieldContextExt<'input> {
    base: FieldContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {Normal_fieldContextExt<'a>}

impl<'input> SQLParserContext<'input> for Normal_fieldContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for Normal_fieldContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_normal_field(self);
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.exit_normal_field(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for Normal_fieldContext<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        visitor.visit_normal_field(self);
    }
}

impl<'input> CustomRuleContext<'input> for Normal_fieldContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_field
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_field }
}

impl<'input> Borrow<FieldContextExt<'input>> for Normal_fieldContext<'input> {
    fn borrow(&self) -> &FieldContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<FieldContextExt<'input>> for Normal_fieldContext<'input> {
    fn borrow_mut(&mut self) -> &mut FieldContextExt<'input> {
        &mut self.base
    }
}

impl<'input> FieldContextAttrs<'input> for Normal_fieldContext<'input> {}

impl<'input> Normal_fieldContextExt<'input> {
    fn new(ctx: &dyn FieldContextAttrs<'input>) -> Rc<FieldContextAll<'input>> {
        Rc::new(FieldContextAll::Normal_fieldContext(
            BaseParserRuleContext::copy_from(
                ctx,
                Normal_fieldContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

impl<'input, I, H> SQLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn field(&mut self) -> Result<Rc<FieldContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = FieldContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_field);
        let mut _localctx: Rc<FieldContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(276);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                Identifier => {
                    let tmp = Normal_fieldContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 1);
                    _localctx = tmp;
                    {
                        recog.base.set_state(243);
                        recog.base.match_token(Identifier, &mut recog.err_handler)?;

                        /*InvokeRule type_*/
                        recog.base.set_state(244);
                        recog.type_()?;

                        recog.base.set_state(247);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == T__40 {
                            {
                                recog.base.set_state(245);
                                recog.base.match_token(T__40, &mut recog.err_handler)?;

                                recog.base.set_state(246);
                                recog.base.match_token(Null, &mut recog.err_handler)?;
                            }
                        }

                        recog.base.set_state(251);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == T__41 {
                            {
                                recog.base.set_state(249);
                                recog.base.match_token(T__41, &mut recog.err_handler)?;

                                /*InvokeRule value*/
                                recog.base.set_state(250);
                                recog.value()?;
                            }
                        }
                    }
                }

                T__33 => {
                    let tmp = Primary_key_fieldContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 2);
                    _localctx = tmp;
                    {
                        recog.base.set_state(253);
                        recog.base.match_token(T__33, &mut recog.err_handler)?;

                        recog.base.set_state(254);
                        recog.base.match_token(T__34, &mut recog.err_handler)?;

                        recog.base.set_state(256);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == Identifier {
                            {
                                recog.base.set_state(255);
                                recog.base.match_token(Identifier, &mut recog.err_handler)?;
                            }
                        }

                        recog.base.set_state(258);
                        recog.base.match_token(T__15, &mut recog.err_handler)?;

                        /*InvokeRule identifiers*/
                        recog.base.set_state(259);
                        recog.identifiers()?;

                        recog.base.set_state(260);
                        recog.base.match_token(T__16, &mut recog.err_handler)?;
                    }
                }

                T__35 => {
                    let tmp = Foreign_key_fieldContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 3);
                    _localctx = tmp;
                    {
                        recog.base.set_state(262);
                        recog.base.match_token(T__35, &mut recog.err_handler)?;

                        recog.base.set_state(263);
                        recog.base.match_token(T__34, &mut recog.err_handler)?;

                        recog.base.set_state(265);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == Identifier {
                            {
                                recog.base.set_state(264);
                                recog.base.match_token(Identifier, &mut recog.err_handler)?;
                            }
                        }

                        recog.base.set_state(267);
                        recog.base.match_token(T__15, &mut recog.err_handler)?;

                        /*InvokeRule identifiers*/
                        recog.base.set_state(268);
                        recog.identifiers()?;

                        recog.base.set_state(269);
                        recog.base.match_token(T__16, &mut recog.err_handler)?;

                        recog.base.set_state(270);
                        recog.base.match_token(T__37, &mut recog.err_handler)?;

                        recog.base.set_state(271);
                        recog.base.match_token(Identifier, &mut recog.err_handler)?;

                        recog.base.set_state(272);
                        recog.base.match_token(T__15, &mut recog.err_handler)?;

                        /*InvokeRule identifiers*/
                        recog.base.set_state(273);
                        recog.identifiers()?;

                        recog.base.set_state(274);
                        recog.base.match_token(T__16, &mut recog.err_handler)?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- type_ ----------------
pub type Type_ContextAll<'input> = Type_Context<'input>;

pub type Type_Context<'input> = BaseParserRuleContext<'input, Type_ContextExt<'input>>;

#[derive(Clone)]
pub struct Type_ContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLParserContext<'input> for Type_Context<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for Type_Context<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_type_(self);
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.exit_type_(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for Type_Context<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        visitor.visit_type_(self);
    }
}

impl<'input> CustomRuleContext<'input> for Type_ContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_type_
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_type_ }
}
antlr_rust::tid! {Type_ContextExt<'a>}

impl<'input> Type_ContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Type_ContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Type_ContextExt { ph: PhantomData },
        ))
    }
}

pub trait Type_ContextAttrs<'input>:
    SQLParserContext<'input> + BorrowMut<Type_ContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token Integer
    /// Returns `None` if there is no child corresponding to token Integer
    fn Integer(&self) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Integer, 0)
    }
}

impl<'input> Type_ContextAttrs<'input> for Type_Context<'input> {}

impl<'input, I, H> SQLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn type_(&mut self) -> Result<Rc<Type_ContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Type_ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_type_);
        let mut _localctx: Rc<Type_ContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(284);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                T__42 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        recog.base.set_state(278);
                        recog.base.match_token(T__42, &mut recog.err_handler)?;
                    }
                }

                T__43 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        recog.base.set_state(279);
                        recog.base.match_token(T__43, &mut recog.err_handler)?;

                        recog.base.set_state(280);
                        recog.base.match_token(T__15, &mut recog.err_handler)?;

                        recog.base.set_state(281);
                        recog.base.match_token(Integer, &mut recog.err_handler)?;

                        recog.base.set_state(282);
                        recog.base.match_token(T__16, &mut recog.err_handler)?;
                    }
                }

                T__44 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        recog.base.set_state(283);
                        recog.base.match_token(T__44, &mut recog.err_handler)?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- value_lists ----------------
pub type Value_listsContextAll<'input> = Value_listsContext<'input>;

pub type Value_listsContext<'input> = BaseParserRuleContext<'input, Value_listsContextExt<'input>>;

#[derive(Clone)]
pub struct Value_listsContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLParserContext<'input> for Value_listsContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for Value_listsContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_value_lists(self);
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.exit_value_lists(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for Value_listsContext<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        visitor.visit_value_lists(self);
    }
}

impl<'input> CustomRuleContext<'input> for Value_listsContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_value_lists
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_value_lists }
}
antlr_rust::tid! {Value_listsContextExt<'a>}

impl<'input> Value_listsContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Value_listsContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Value_listsContextExt { ph: PhantomData },
        ))
    }
}

pub trait Value_listsContextAttrs<'input>:
    SQLParserContext<'input> + BorrowMut<Value_listsContextExt<'input>>
{
    fn value_list_all(&self) -> Vec<Rc<Value_listContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn value_list(&self, i: usize) -> Option<Rc<Value_listContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Value_listsContextAttrs<'input> for Value_listsContext<'input> {}

impl<'input, I, H> SQLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn value_lists(&mut self) -> Result<Rc<Value_listsContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Value_listsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 20, RULE_value_lists);
        let mut _localctx: Rc<Value_listsContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule value_list*/
                recog.base.set_state(286);
                recog.value_list()?;

                recog.base.set_state(291);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == T__39 {
                    {
                        {
                            recog.base.set_state(287);
                            recog.base.match_token(T__39, &mut recog.err_handler)?;

                            /*InvokeRule value_list*/
                            recog.base.set_state(288);
                            recog.value_list()?;
                        }
                    }
                    recog.base.set_state(293);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- value_list ----------------
pub type Value_listContextAll<'input> = Value_listContext<'input>;

pub type Value_listContext<'input> = BaseParserRuleContext<'input, Value_listContextExt<'input>>;

#[derive(Clone)]
pub struct Value_listContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLParserContext<'input> for Value_listContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for Value_listContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_value_list(self);
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.exit_value_list(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for Value_listContext<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        visitor.visit_value_list(self);
    }
}

impl<'input> CustomRuleContext<'input> for Value_listContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_value_list
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_value_list }
}
antlr_rust::tid! {Value_listContextExt<'a>}

impl<'input> Value_listContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Value_listContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Value_listContextExt { ph: PhantomData },
        ))
    }
}

pub trait Value_listContextAttrs<'input>:
    SQLParserContext<'input> + BorrowMut<Value_listContextExt<'input>>
{
    fn value_all(&self) -> Vec<Rc<ValueContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn value(&self, i: usize) -> Option<Rc<ValueContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Value_listContextAttrs<'input> for Value_listContext<'input> {}

impl<'input, I, H> SQLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn value_list(&mut self) -> Result<Rc<Value_listContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Value_listContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 22, RULE_value_list);
        let mut _localctx: Rc<Value_listContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(294);
                recog.base.match_token(T__15, &mut recog.err_handler)?;

                /*InvokeRule value*/
                recog.base.set_state(295);
                recog.value()?;

                recog.base.set_state(300);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == T__39 {
                    {
                        {
                            recog.base.set_state(296);
                            recog.base.match_token(T__39, &mut recog.err_handler)?;

                            /*InvokeRule value*/
                            recog.base.set_state(297);
                            recog.value()?;
                        }
                    }
                    recog.base.set_state(302);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(303);
                recog.base.match_token(T__16, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- value ----------------
pub type ValueContextAll<'input> = ValueContext<'input>;

pub type ValueContext<'input> = BaseParserRuleContext<'input, ValueContextExt<'input>>;

#[derive(Clone)]
pub struct ValueContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLParserContext<'input> for ValueContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for ValueContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_value(self);
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.exit_value(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for ValueContext<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        visitor.visit_value(self);
    }
}

impl<'input> CustomRuleContext<'input> for ValueContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_value
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_value }
}
antlr_rust::tid! {ValueContextExt<'a>}

impl<'input> ValueContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ValueContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ValueContextExt { ph: PhantomData },
        ))
    }
}

pub trait ValueContextAttrs<'input>:
    SQLParserContext<'input> + BorrowMut<ValueContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token Integer
    /// Returns `None` if there is no child corresponding to token Integer
    fn Integer(&self) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Integer, 0)
    }
    /// Retrieves first TerminalNode corresponding to token String
    /// Returns `None` if there is no child corresponding to token String
    fn String(&self) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(String, 0)
    }
    /// Retrieves first TerminalNode corresponding to token Float
    /// Returns `None` if there is no child corresponding to token Float
    fn Float(&self) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Float, 0)
    }
    /// Retrieves first TerminalNode corresponding to token Null
    /// Returns `None` if there is no child corresponding to token Null
    fn Null(&self) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Null, 0)
    }
}

impl<'input> ValueContextAttrs<'input> for ValueContext<'input> {}

impl<'input, I, H> SQLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn value(&mut self) -> Result<Rc<ValueContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ValueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_value);
        let mut _localctx: Rc<ValueContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(305);
                _la = recog.base.input.la(1);
                if {
                    !(((_la - 63) & !0x3f) == 0
                        && ((1usize << (_la - 63))
                            & ((1usize << (Null - 63))
                                | (1usize << (Integer - 63))
                                | (1usize << (String - 63))
                                | (1usize << (Float - 63))))
                            != 0)
                } {
                    recog.err_handler.recover_inline(&mut recog.base)?;
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- where_and_clause ----------------
pub type Where_and_clauseContextAll<'input> = Where_and_clauseContext<'input>;

pub type Where_and_clauseContext<'input> =
    BaseParserRuleContext<'input, Where_and_clauseContextExt<'input>>;

#[derive(Clone)]
pub struct Where_and_clauseContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLParserContext<'input> for Where_and_clauseContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for Where_and_clauseContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_where_and_clause(self);
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.exit_where_and_clause(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for Where_and_clauseContext<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        visitor.visit_where_and_clause(self);
    }
}

impl<'input> CustomRuleContext<'input> for Where_and_clauseContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_where_and_clause
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_where_and_clause }
}
antlr_rust::tid! {Where_and_clauseContextExt<'a>}

impl<'input> Where_and_clauseContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Where_and_clauseContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Where_and_clauseContextExt { ph: PhantomData },
        ))
    }
}

pub trait Where_and_clauseContextAttrs<'input>:
    SQLParserContext<'input> + BorrowMut<Where_and_clauseContextExt<'input>>
{
    fn where_clause_all(&self) -> Vec<Rc<Where_clauseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn where_clause(&self, i: usize) -> Option<Rc<Where_clauseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Where_and_clauseContextAttrs<'input> for Where_and_clauseContext<'input> {}

impl<'input, I, H> SQLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn where_and_clause(
        &mut self,
    ) -> Result<Rc<Where_and_clauseContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Where_and_clauseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 26, RULE_where_and_clause);
        let mut _localctx: Rc<Where_and_clauseContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule where_clause*/
                recog.base.set_state(307);
                recog.where_clause()?;

                recog.base.set_state(312);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == T__45 {
                    {
                        {
                            recog.base.set_state(308);
                            recog.base.match_token(T__45, &mut recog.err_handler)?;

                            /*InvokeRule where_clause*/
                            recog.base.set_state(309);
                            recog.where_clause()?;
                        }
                    }
                    recog.base.set_state(314);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- where_clause ----------------
#[derive(Debug)]
pub enum Where_clauseContextAll<'input> {
    Where_in_listContext(Where_in_listContext<'input>),
    Where_operator_selectContext(Where_operator_selectContext<'input>),
    Where_nullContext(Where_nullContext<'input>),
    Where_operator_expressionContext(Where_operator_expressionContext<'input>),
    Where_in_selectContext(Where_in_selectContext<'input>),
    Where_like_stringContext(Where_like_stringContext<'input>),
    Error(Where_clauseContext<'input>),
}
antlr_rust::tid! {Where_clauseContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for Where_clauseContextAll<'input> {}

impl<'input> SQLParserContext<'input> for Where_clauseContextAll<'input> {}

impl<'input> Deref for Where_clauseContextAll<'input> {
    type Target = dyn Where_clauseContextAttrs<'input> + 'input;
    fn deref(&self) -> &Self::Target {
        use Where_clauseContextAll::*;
        match self {
            Where_in_listContext(inner) => inner,
            Where_operator_selectContext(inner) => inner,
            Where_nullContext(inner) => inner,
            Where_operator_expressionContext(inner) => inner,
            Where_in_selectContext(inner) => inner,
            Where_like_stringContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}
impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for Where_clauseContextAll<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        self.deref().accept(visitor)
    }
}
impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for Where_clauseContextAll<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        self.deref().enter(listener)
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        self.deref().exit(listener)
    }
}

pub type Where_clauseContext<'input> =
    BaseParserRuleContext<'input, Where_clauseContextExt<'input>>;

#[derive(Clone)]
pub struct Where_clauseContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLParserContext<'input> for Where_clauseContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for Where_clauseContext<'input> {}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for Where_clauseContext<'input> {}

impl<'input> CustomRuleContext<'input> for Where_clauseContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_where_clause
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_where_clause }
}
antlr_rust::tid! {Where_clauseContextExt<'a>}

impl<'input> Where_clauseContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Where_clauseContextAll<'input>> {
        Rc::new(Where_clauseContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(
                parent,
                invoking_state,
                Where_clauseContextExt { ph: PhantomData },
            ),
        ))
    }
}

pub trait Where_clauseContextAttrs<'input>:
    SQLParserContext<'input> + BorrowMut<Where_clauseContextExt<'input>>
{
}

impl<'input> Where_clauseContextAttrs<'input> for Where_clauseContext<'input> {}

pub type Where_in_listContext<'input> =
    BaseParserRuleContext<'input, Where_in_listContextExt<'input>>;

pub trait Where_in_listContextAttrs<'input>: SQLParserContext<'input> {
    fn column(&self) -> Option<Rc<ColumnContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn value_list(&self) -> Option<Rc<Value_listContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Where_in_listContextAttrs<'input> for Where_in_listContext<'input> {}

pub struct Where_in_listContextExt<'input> {
    base: Where_clauseContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {Where_in_listContextExt<'a>}

impl<'input> SQLParserContext<'input> for Where_in_listContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for Where_in_listContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_where_in_list(self);
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.exit_where_in_list(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for Where_in_listContext<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        visitor.visit_where_in_list(self);
    }
}

impl<'input> CustomRuleContext<'input> for Where_in_listContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_where_clause
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_where_clause }
}

impl<'input> Borrow<Where_clauseContextExt<'input>> for Where_in_listContext<'input> {
    fn borrow(&self) -> &Where_clauseContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<Where_clauseContextExt<'input>> for Where_in_listContext<'input> {
    fn borrow_mut(&mut self) -> &mut Where_clauseContextExt<'input> {
        &mut self.base
    }
}

impl<'input> Where_clauseContextAttrs<'input> for Where_in_listContext<'input> {}

impl<'input> Where_in_listContextExt<'input> {
    fn new(ctx: &dyn Where_clauseContextAttrs<'input>) -> Rc<Where_clauseContextAll<'input>> {
        Rc::new(Where_clauseContextAll::Where_in_listContext(
            BaseParserRuleContext::copy_from(
                ctx,
                Where_in_listContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type Where_operator_selectContext<'input> =
    BaseParserRuleContext<'input, Where_operator_selectContextExt<'input>>;

pub trait Where_operator_selectContextAttrs<'input>: SQLParserContext<'input> {
    fn column(&self) -> Option<Rc<ColumnContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn operator_(&self) -> Option<Rc<Operator_ContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn select_table(&self) -> Option<Rc<Select_tableContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Where_operator_selectContextAttrs<'input> for Where_operator_selectContext<'input> {}

pub struct Where_operator_selectContextExt<'input> {
    base: Where_clauseContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {Where_operator_selectContextExt<'a>}

impl<'input> SQLParserContext<'input> for Where_operator_selectContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for Where_operator_selectContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_where_operator_select(self);
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.exit_where_operator_select(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for Where_operator_selectContext<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        visitor.visit_where_operator_select(self);
    }
}

impl<'input> CustomRuleContext<'input> for Where_operator_selectContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_where_clause
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_where_clause }
}

impl<'input> Borrow<Where_clauseContextExt<'input>> for Where_operator_selectContext<'input> {
    fn borrow(&self) -> &Where_clauseContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<Where_clauseContextExt<'input>> for Where_operator_selectContext<'input> {
    fn borrow_mut(&mut self) -> &mut Where_clauseContextExt<'input> {
        &mut self.base
    }
}

impl<'input> Where_clauseContextAttrs<'input> for Where_operator_selectContext<'input> {}

impl<'input> Where_operator_selectContextExt<'input> {
    fn new(ctx: &dyn Where_clauseContextAttrs<'input>) -> Rc<Where_clauseContextAll<'input>> {
        Rc::new(Where_clauseContextAll::Where_operator_selectContext(
            BaseParserRuleContext::copy_from(
                ctx,
                Where_operator_selectContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type Where_nullContext<'input> = BaseParserRuleContext<'input, Where_nullContextExt<'input>>;

pub trait Where_nullContextAttrs<'input>: SQLParserContext<'input> {
    fn column(&self) -> Option<Rc<ColumnContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token Null
    /// Returns `None` if there is no child corresponding to token Null
    fn Null(&self) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Null, 0)
    }
}

impl<'input> Where_nullContextAttrs<'input> for Where_nullContext<'input> {}

pub struct Where_nullContextExt<'input> {
    base: Where_clauseContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {Where_nullContextExt<'a>}

impl<'input> SQLParserContext<'input> for Where_nullContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for Where_nullContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_where_null(self);
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.exit_where_null(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for Where_nullContext<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        visitor.visit_where_null(self);
    }
}

impl<'input> CustomRuleContext<'input> for Where_nullContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_where_clause
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_where_clause }
}

impl<'input> Borrow<Where_clauseContextExt<'input>> for Where_nullContext<'input> {
    fn borrow(&self) -> &Where_clauseContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<Where_clauseContextExt<'input>> for Where_nullContext<'input> {
    fn borrow_mut(&mut self) -> &mut Where_clauseContextExt<'input> {
        &mut self.base
    }
}

impl<'input> Where_clauseContextAttrs<'input> for Where_nullContext<'input> {}

impl<'input> Where_nullContextExt<'input> {
    fn new(ctx: &dyn Where_clauseContextAttrs<'input>) -> Rc<Where_clauseContextAll<'input>> {
        Rc::new(Where_clauseContextAll::Where_nullContext(
            BaseParserRuleContext::copy_from(
                ctx,
                Where_nullContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type Where_operator_expressionContext<'input> =
    BaseParserRuleContext<'input, Where_operator_expressionContextExt<'input>>;

pub trait Where_operator_expressionContextAttrs<'input>: SQLParserContext<'input> {
    fn column(&self) -> Option<Rc<ColumnContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn operator_(&self) -> Option<Rc<Operator_ContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Where_operator_expressionContextAttrs<'input>
    for Where_operator_expressionContext<'input>
{
}

pub struct Where_operator_expressionContextExt<'input> {
    base: Where_clauseContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {Where_operator_expressionContextExt<'a>}

impl<'input> SQLParserContext<'input> for Where_operator_expressionContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a>
    for Where_operator_expressionContext<'input>
{
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_where_operator_expression(self);
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.exit_where_operator_expression(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a>
    for Where_operator_expressionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        visitor.visit_where_operator_expression(self);
    }
}

impl<'input> CustomRuleContext<'input> for Where_operator_expressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_where_clause
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_where_clause }
}

impl<'input> Borrow<Where_clauseContextExt<'input>> for Where_operator_expressionContext<'input> {
    fn borrow(&self) -> &Where_clauseContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<Where_clauseContextExt<'input>>
    for Where_operator_expressionContext<'input>
{
    fn borrow_mut(&mut self) -> &mut Where_clauseContextExt<'input> {
        &mut self.base
    }
}

impl<'input> Where_clauseContextAttrs<'input> for Where_operator_expressionContext<'input> {}

impl<'input> Where_operator_expressionContextExt<'input> {
    fn new(ctx: &dyn Where_clauseContextAttrs<'input>) -> Rc<Where_clauseContextAll<'input>> {
        Rc::new(Where_clauseContextAll::Where_operator_expressionContext(
            BaseParserRuleContext::copy_from(
                ctx,
                Where_operator_expressionContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type Where_in_selectContext<'input> =
    BaseParserRuleContext<'input, Where_in_selectContextExt<'input>>;

pub trait Where_in_selectContextAttrs<'input>: SQLParserContext<'input> {
    fn column(&self) -> Option<Rc<ColumnContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn select_table(&self) -> Option<Rc<Select_tableContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Where_in_selectContextAttrs<'input> for Where_in_selectContext<'input> {}

pub struct Where_in_selectContextExt<'input> {
    base: Where_clauseContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {Where_in_selectContextExt<'a>}

impl<'input> SQLParserContext<'input> for Where_in_selectContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for Where_in_selectContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_where_in_select(self);
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.exit_where_in_select(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for Where_in_selectContext<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        visitor.visit_where_in_select(self);
    }
}

impl<'input> CustomRuleContext<'input> for Where_in_selectContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_where_clause
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_where_clause }
}

impl<'input> Borrow<Where_clauseContextExt<'input>> for Where_in_selectContext<'input> {
    fn borrow(&self) -> &Where_clauseContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<Where_clauseContextExt<'input>> for Where_in_selectContext<'input> {
    fn borrow_mut(&mut self) -> &mut Where_clauseContextExt<'input> {
        &mut self.base
    }
}

impl<'input> Where_clauseContextAttrs<'input> for Where_in_selectContext<'input> {}

impl<'input> Where_in_selectContextExt<'input> {
    fn new(ctx: &dyn Where_clauseContextAttrs<'input>) -> Rc<Where_clauseContextAll<'input>> {
        Rc::new(Where_clauseContextAll::Where_in_selectContext(
            BaseParserRuleContext::copy_from(
                ctx,
                Where_in_selectContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type Where_like_stringContext<'input> =
    BaseParserRuleContext<'input, Where_like_stringContextExt<'input>>;

pub trait Where_like_stringContextAttrs<'input>: SQLParserContext<'input> {
    fn column(&self) -> Option<Rc<ColumnContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token String
    /// Returns `None` if there is no child corresponding to token String
    fn String(&self) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(String, 0)
    }
}

impl<'input> Where_like_stringContextAttrs<'input> for Where_like_stringContext<'input> {}

pub struct Where_like_stringContextExt<'input> {
    base: Where_clauseContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {Where_like_stringContextExt<'a>}

impl<'input> SQLParserContext<'input> for Where_like_stringContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for Where_like_stringContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_where_like_string(self);
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.exit_where_like_string(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for Where_like_stringContext<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        visitor.visit_where_like_string(self);
    }
}

impl<'input> CustomRuleContext<'input> for Where_like_stringContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_where_clause
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_where_clause }
}

impl<'input> Borrow<Where_clauseContextExt<'input>> for Where_like_stringContext<'input> {
    fn borrow(&self) -> &Where_clauseContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<Where_clauseContextExt<'input>> for Where_like_stringContext<'input> {
    fn borrow_mut(&mut self) -> &mut Where_clauseContextExt<'input> {
        &mut self.base
    }
}

impl<'input> Where_clauseContextAttrs<'input> for Where_like_stringContext<'input> {}

impl<'input> Where_like_stringContextExt<'input> {
    fn new(ctx: &dyn Where_clauseContextAttrs<'input>) -> Rc<Where_clauseContextAll<'input>> {
        Rc::new(Where_clauseContextAll::Where_like_stringContext(
            BaseParserRuleContext::copy_from(
                ctx,
                Where_like_stringContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

impl<'input, I, H> SQLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn where_clause(&mut self) -> Result<Rc<Where_clauseContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Where_clauseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 28, RULE_where_clause);
        let mut _localctx: Rc<Where_clauseContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(346);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(24, &mut recog.base)? {
                1 => {
                    let tmp = Where_operator_expressionContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 1);
                    _localctx = tmp;
                    {
                        /*InvokeRule column*/
                        recog.base.set_state(315);
                        recog.column()?;

                        /*InvokeRule operator_*/
                        recog.base.set_state(316);
                        recog.operator_()?;

                        /*InvokeRule expression*/
                        recog.base.set_state(317);
                        recog.expression()?;
                    }
                }
                2 => {
                    let tmp = Where_operator_selectContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 2);
                    _localctx = tmp;
                    {
                        /*InvokeRule column*/
                        recog.base.set_state(319);
                        recog.column()?;

                        /*InvokeRule operator_*/
                        recog.base.set_state(320);
                        recog.operator_()?;

                        recog.base.set_state(321);
                        recog.base.match_token(T__15, &mut recog.err_handler)?;

                        /*InvokeRule select_table*/
                        recog.base.set_state(322);
                        recog.select_table()?;

                        recog.base.set_state(323);
                        recog.base.match_token(T__16, &mut recog.err_handler)?;
                    }
                }
                3 => {
                    let tmp = Where_nullContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 3);
                    _localctx = tmp;
                    {
                        /*InvokeRule column*/
                        recog.base.set_state(325);
                        recog.column()?;

                        recog.base.set_state(326);
                        recog.base.match_token(T__46, &mut recog.err_handler)?;

                        recog.base.set_state(328);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == T__40 {
                            {
                                recog.base.set_state(327);
                                recog.base.match_token(T__40, &mut recog.err_handler)?;
                            }
                        }

                        recog.base.set_state(330);
                        recog.base.match_token(Null, &mut recog.err_handler)?;
                    }
                }
                4 => {
                    let tmp = Where_in_listContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 4);
                    _localctx = tmp;
                    {
                        /*InvokeRule column*/
                        recog.base.set_state(332);
                        recog.column()?;

                        recog.base.set_state(333);
                        recog.base.match_token(T__47, &mut recog.err_handler)?;

                        /*InvokeRule value_list*/
                        recog.base.set_state(334);
                        recog.value_list()?;
                    }
                }
                5 => {
                    let tmp = Where_in_selectContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 5);
                    _localctx = tmp;
                    {
                        /*InvokeRule column*/
                        recog.base.set_state(336);
                        recog.column()?;

                        recog.base.set_state(337);
                        recog.base.match_token(T__47, &mut recog.err_handler)?;

                        recog.base.set_state(338);
                        recog.base.match_token(T__15, &mut recog.err_handler)?;

                        /*InvokeRule select_table*/
                        recog.base.set_state(339);
                        recog.select_table()?;

                        recog.base.set_state(340);
                        recog.base.match_token(T__16, &mut recog.err_handler)?;
                    }
                }
                6 => {
                    let tmp = Where_like_stringContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 6);
                    _localctx = tmp;
                    {
                        /*InvokeRule column*/
                        recog.base.set_state(342);
                        recog.column()?;

                        recog.base.set_state(343);
                        recog.base.match_token(T__48, &mut recog.err_handler)?;

                        recog.base.set_state(344);
                        recog.base.match_token(String, &mut recog.err_handler)?;
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- column ----------------
pub type ColumnContextAll<'input> = ColumnContext<'input>;

pub type ColumnContext<'input> = BaseParserRuleContext<'input, ColumnContextExt<'input>>;

#[derive(Clone)]
pub struct ColumnContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLParserContext<'input> for ColumnContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for ColumnContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_column(self);
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.exit_column(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for ColumnContext<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        visitor.visit_column(self);
    }
}

impl<'input> CustomRuleContext<'input> for ColumnContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_column
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_column }
}
antlr_rust::tid! {ColumnContextExt<'a>}

impl<'input> ColumnContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ColumnContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ColumnContextExt { ph: PhantomData },
        ))
    }
}

pub trait ColumnContextAttrs<'input>:
    SQLParserContext<'input> + BorrowMut<ColumnContextExt<'input>>
{
    /// Retrieves all `TerminalNode`s corresponding to token Identifier in current rule
    fn Identifier_all(&self) -> Vec<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token Identifier, starting from 0.
    /// Returns `None` if number of children corresponding to token Identifier is less or equal than `i`.
    fn Identifier(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Identifier, i)
    }
}

impl<'input> ColumnContextAttrs<'input> for ColumnContext<'input> {}

impl<'input, I, H> SQLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn column(&mut self) -> Result<Rc<ColumnContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ColumnContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_column);
        let mut _localctx: Rc<ColumnContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(350);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(25, &mut recog.base)? {
                    x if x == 1 => {
                        recog.base.set_state(348);
                        recog.base.match_token(Identifier, &mut recog.err_handler)?;

                        recog.base.set_state(349);
                        recog.base.match_token(T__49, &mut recog.err_handler)?;
                    }

                    _ => {}
                }
                recog.base.set_state(352);
                recog.base.match_token(Identifier, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- expression ----------------
pub type ExpressionContextAll<'input> = ExpressionContext<'input>;

pub type ExpressionContext<'input> = BaseParserRuleContext<'input, ExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ExpressionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLParserContext<'input> for ExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for ExpressionContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_expression(self);
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.exit_expression(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for ExpressionContext<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        visitor.visit_expression(self);
    }
}

impl<'input> CustomRuleContext<'input> for ExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}
antlr_rust::tid! {ExpressionContextExt<'a>}

impl<'input> ExpressionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ExpressionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ExpressionContextExt { ph: PhantomData },
        ))
    }
}

pub trait ExpressionContextAttrs<'input>:
    SQLParserContext<'input> + BorrowMut<ExpressionContextExt<'input>>
{
    fn value(&self) -> Option<Rc<ValueContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn column(&self) -> Option<Rc<ColumnContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> ExpressionContextAttrs<'input> for ExpressionContext<'input> {}

impl<'input, I, H> SQLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn expression(&mut self) -> Result<Rc<ExpressionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 32, RULE_expression);
        let mut _localctx: Rc<ExpressionContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(356);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                Null | Integer | String | Float => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule value*/
                        recog.base.set_state(354);
                        recog.value()?;
                    }
                }

                Identifier => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule column*/
                        recog.base.set_state(355);
                        recog.column()?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- set_clause ----------------
pub type Set_clauseContextAll<'input> = Set_clauseContext<'input>;

pub type Set_clauseContext<'input> = BaseParserRuleContext<'input, Set_clauseContextExt<'input>>;

#[derive(Clone)]
pub struct Set_clauseContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLParserContext<'input> for Set_clauseContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for Set_clauseContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_set_clause(self);
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.exit_set_clause(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for Set_clauseContext<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        visitor.visit_set_clause(self);
    }
}

impl<'input> CustomRuleContext<'input> for Set_clauseContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_set_clause
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_set_clause }
}
antlr_rust::tid! {Set_clauseContextExt<'a>}

impl<'input> Set_clauseContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Set_clauseContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Set_clauseContextExt { ph: PhantomData },
        ))
    }
}

pub trait Set_clauseContextAttrs<'input>:
    SQLParserContext<'input> + BorrowMut<Set_clauseContextExt<'input>>
{
    /// Retrieves all `TerminalNode`s corresponding to token Identifier in current rule
    fn Identifier_all(&self) -> Vec<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token Identifier, starting from 0.
    /// Returns `None` if number of children corresponding to token Identifier is less or equal than `i`.
    fn Identifier(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Identifier, i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token EqualOrAssign in current rule
    fn EqualOrAssign_all(&self) -> Vec<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token EqualOrAssign, starting from 0.
    /// Returns `None` if number of children corresponding to token EqualOrAssign is less or equal than `i`.
    fn EqualOrAssign(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(EqualOrAssign, i)
    }
    fn value_all(&self) -> Vec<Rc<ValueContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn value(&self, i: usize) -> Option<Rc<ValueContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Set_clauseContextAttrs<'input> for Set_clauseContext<'input> {}

impl<'input, I, H> SQLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn set_clause(&mut self) -> Result<Rc<Set_clauseContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Set_clauseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 34, RULE_set_clause);
        let mut _localctx: Rc<Set_clauseContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(358);
                recog.base.match_token(Identifier, &mut recog.err_handler)?;

                recog.base.set_state(359);
                recog
                    .base
                    .match_token(EqualOrAssign, &mut recog.err_handler)?;

                /*InvokeRule value*/
                recog.base.set_state(360);
                recog.value()?;

                recog.base.set_state(367);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == T__39 {
                    {
                        {
                            recog.base.set_state(361);
                            recog.base.match_token(T__39, &mut recog.err_handler)?;

                            recog.base.set_state(362);
                            recog.base.match_token(Identifier, &mut recog.err_handler)?;

                            recog.base.set_state(363);
                            recog
                                .base
                                .match_token(EqualOrAssign, &mut recog.err_handler)?;

                            /*InvokeRule value*/
                            recog.base.set_state(364);
                            recog.value()?;
                        }
                    }
                    recog.base.set_state(369);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- selectors ----------------
pub type SelectorsContextAll<'input> = SelectorsContext<'input>;

pub type SelectorsContext<'input> = BaseParserRuleContext<'input, SelectorsContextExt<'input>>;

#[derive(Clone)]
pub struct SelectorsContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLParserContext<'input> for SelectorsContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for SelectorsContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_selectors(self);
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.exit_selectors(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for SelectorsContext<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        visitor.visit_selectors(self);
    }
}

impl<'input> CustomRuleContext<'input> for SelectorsContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_selectors
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_selectors }
}
antlr_rust::tid! {SelectorsContextExt<'a>}

impl<'input> SelectorsContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<SelectorsContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            SelectorsContextExt { ph: PhantomData },
        ))
    }
}

pub trait SelectorsContextAttrs<'input>:
    SQLParserContext<'input> + BorrowMut<SelectorsContextExt<'input>>
{
    fn selector_all(&self) -> Vec<Rc<SelectorContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn selector(&self, i: usize) -> Option<Rc<SelectorContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> SelectorsContextAttrs<'input> for SelectorsContext<'input> {}

impl<'input, I, H> SQLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn selectors(&mut self) -> Result<Rc<SelectorsContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = SelectorsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 36, RULE_selectors);
        let mut _localctx: Rc<SelectorsContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(379);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                T__50 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        recog.base.set_state(370);
                        recog.base.match_token(T__50, &mut recog.err_handler)?;
                    }
                }

                Count | Average | Max | Min | Sum | Identifier => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule selector*/
                        recog.base.set_state(371);
                        recog.selector()?;

                        recog.base.set_state(376);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        while _la == T__39 {
                            {
                                {
                                    recog.base.set_state(372);
                                    recog.base.match_token(T__39, &mut recog.err_handler)?;

                                    /*InvokeRule selector*/
                                    recog.base.set_state(373);
                                    recog.selector()?;
                                }
                            }
                            recog.base.set_state(378);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                        }
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- selector ----------------
pub type SelectorContextAll<'input> = SelectorContext<'input>;

pub type SelectorContext<'input> = BaseParserRuleContext<'input, SelectorContextExt<'input>>;

#[derive(Clone)]
pub struct SelectorContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLParserContext<'input> for SelectorContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for SelectorContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_selector(self);
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.exit_selector(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for SelectorContext<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        visitor.visit_selector(self);
    }
}

impl<'input> CustomRuleContext<'input> for SelectorContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_selector
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_selector }
}
antlr_rust::tid! {SelectorContextExt<'a>}

impl<'input> SelectorContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<SelectorContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            SelectorContextExt { ph: PhantomData },
        ))
    }
}

pub trait SelectorContextAttrs<'input>:
    SQLParserContext<'input> + BorrowMut<SelectorContextExt<'input>>
{
    fn column(&self) -> Option<Rc<ColumnContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn aggregator(&self) -> Option<Rc<AggregatorContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token Count
    /// Returns `None` if there is no child corresponding to token Count
    fn Count(&self) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Count, 0)
    }
}

impl<'input> SelectorContextAttrs<'input> for SelectorContext<'input> {}

impl<'input, I, H> SQLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn selector(&mut self) -> Result<Rc<SelectorContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = SelectorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 38, RULE_selector);
        let mut _localctx: Rc<SelectorContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(391);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(30, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule column*/
                        recog.base.set_state(381);
                        recog.column()?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule aggregator*/
                        recog.base.set_state(382);
                        recog.aggregator()?;

                        recog.base.set_state(383);
                        recog.base.match_token(T__15, &mut recog.err_handler)?;

                        /*InvokeRule column*/
                        recog.base.set_state(384);
                        recog.column()?;

                        recog.base.set_state(385);
                        recog.base.match_token(T__16, &mut recog.err_handler)?;
                    }
                }
                3 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        recog.base.set_state(387);
                        recog.base.match_token(Count, &mut recog.err_handler)?;

                        recog.base.set_state(388);
                        recog.base.match_token(T__15, &mut recog.err_handler)?;

                        recog.base.set_state(389);
                        recog.base.match_token(T__50, &mut recog.err_handler)?;

                        recog.base.set_state(390);
                        recog.base.match_token(T__16, &mut recog.err_handler)?;
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- identifiers ----------------
pub type IdentifiersContextAll<'input> = IdentifiersContext<'input>;

pub type IdentifiersContext<'input> = BaseParserRuleContext<'input, IdentifiersContextExt<'input>>;

#[derive(Clone)]
pub struct IdentifiersContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLParserContext<'input> for IdentifiersContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for IdentifiersContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_identifiers(self);
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.exit_identifiers(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for IdentifiersContext<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        visitor.visit_identifiers(self);
    }
}

impl<'input> CustomRuleContext<'input> for IdentifiersContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_identifiers
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_identifiers }
}
antlr_rust::tid! {IdentifiersContextExt<'a>}

impl<'input> IdentifiersContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<IdentifiersContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            IdentifiersContextExt { ph: PhantomData },
        ))
    }
}

pub trait IdentifiersContextAttrs<'input>:
    SQLParserContext<'input> + BorrowMut<IdentifiersContextExt<'input>>
{
    /// Retrieves all `TerminalNode`s corresponding to token Identifier in current rule
    fn Identifier_all(&self) -> Vec<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token Identifier, starting from 0.
    /// Returns `None` if number of children corresponding to token Identifier is less or equal than `i`.
    fn Identifier(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Identifier, i)
    }
}

impl<'input> IdentifiersContextAttrs<'input> for IdentifiersContext<'input> {}

impl<'input, I, H> SQLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn identifiers(&mut self) -> Result<Rc<IdentifiersContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = IdentifiersContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 40, RULE_identifiers);
        let mut _localctx: Rc<IdentifiersContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(393);
                recog.base.match_token(Identifier, &mut recog.err_handler)?;

                recog.base.set_state(398);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == T__39 {
                    {
                        {
                            recog.base.set_state(394);
                            recog.base.match_token(T__39, &mut recog.err_handler)?;

                            recog.base.set_state(395);
                            recog.base.match_token(Identifier, &mut recog.err_handler)?;
                        }
                    }
                    recog.base.set_state(400);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- operator_ ----------------
pub type Operator_ContextAll<'input> = Operator_Context<'input>;

pub type Operator_Context<'input> = BaseParserRuleContext<'input, Operator_ContextExt<'input>>;

#[derive(Clone)]
pub struct Operator_ContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLParserContext<'input> for Operator_Context<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for Operator_Context<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_operator_(self);
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.exit_operator_(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for Operator_Context<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        visitor.visit_operator_(self);
    }
}

impl<'input> CustomRuleContext<'input> for Operator_ContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_operator_
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_operator_ }
}
antlr_rust::tid! {Operator_ContextExt<'a>}

impl<'input> Operator_ContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Operator_ContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Operator_ContextExt { ph: PhantomData },
        ))
    }
}

pub trait Operator_ContextAttrs<'input>:
    SQLParserContext<'input> + BorrowMut<Operator_ContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token EqualOrAssign
    /// Returns `None` if there is no child corresponding to token EqualOrAssign
    fn EqualOrAssign(&self) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(EqualOrAssign, 0)
    }
    /// Retrieves first TerminalNode corresponding to token Less
    /// Returns `None` if there is no child corresponding to token Less
    fn Less(&self) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Less, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LessEqual
    /// Returns `None` if there is no child corresponding to token LessEqual
    fn LessEqual(&self) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LessEqual, 0)
    }
    /// Retrieves first TerminalNode corresponding to token Greater
    /// Returns `None` if there is no child corresponding to token Greater
    fn Greater(&self) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Greater, 0)
    }
    /// Retrieves first TerminalNode corresponding to token GreaterEqual
    /// Returns `None` if there is no child corresponding to token GreaterEqual
    fn GreaterEqual(&self) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(GreaterEqual, 0)
    }
    /// Retrieves first TerminalNode corresponding to token NotEqual
    /// Returns `None` if there is no child corresponding to token NotEqual
    fn NotEqual(&self) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NotEqual, 0)
    }
}

impl<'input> Operator_ContextAttrs<'input> for Operator_Context<'input> {}

impl<'input, I, H> SQLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn operator_(&mut self) -> Result<Rc<Operator_ContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Operator_ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 42, RULE_operator_);
        let mut _localctx: Rc<Operator_ContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(401);
                _la = recog.base.input.la(1);
                if {
                    !(((_la - 52) & !0x3f) == 0
                        && ((1usize << (_la - 52))
                            & ((1usize << (EqualOrAssign - 52))
                                | (1usize << (Less - 52))
                                | (1usize << (LessEqual - 52))
                                | (1usize << (Greater - 52))
                                | (1usize << (GreaterEqual - 52))
                                | (1usize << (NotEqual - 52))))
                            != 0)
                } {
                    recog.err_handler.recover_inline(&mut recog.base)?;
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- aggregator ----------------
pub type AggregatorContextAll<'input> = AggregatorContext<'input>;

pub type AggregatorContext<'input> = BaseParserRuleContext<'input, AggregatorContextExt<'input>>;

#[derive(Clone)]
pub struct AggregatorContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLParserContext<'input> for AggregatorContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLListener<'input> + 'a> for AggregatorContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_aggregator(self);
    }
    fn exit(&self, listener: &mut (dyn SQLListener<'input> + 'a)) {
        listener.exit_aggregator(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn SQLVisitor<'input> + 'a> for AggregatorContext<'input> {
    fn accept(&self, visitor: &mut (dyn SQLVisitor<'input> + 'a)) {
        visitor.visit_aggregator(self);
    }
}

impl<'input> CustomRuleContext<'input> for AggregatorContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_aggregator
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_aggregator }
}
antlr_rust::tid! {AggregatorContextExt<'a>}

impl<'input> AggregatorContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<AggregatorContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            AggregatorContextExt { ph: PhantomData },
        ))
    }
}

pub trait AggregatorContextAttrs<'input>:
    SQLParserContext<'input> + BorrowMut<AggregatorContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token Count
    /// Returns `None` if there is no child corresponding to token Count
    fn Count(&self) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Count, 0)
    }
    /// Retrieves first TerminalNode corresponding to token Average
    /// Returns `None` if there is no child corresponding to token Average
    fn Average(&self) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Average, 0)
    }
    /// Retrieves first TerminalNode corresponding to token Max
    /// Returns `None` if there is no child corresponding to token Max
    fn Max(&self) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Max, 0)
    }
    /// Retrieves first TerminalNode corresponding to token Min
    /// Returns `None` if there is no child corresponding to token Min
    fn Min(&self) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Min, 0)
    }
    /// Retrieves first TerminalNode corresponding to token Sum
    /// Returns `None` if there is no child corresponding to token Sum
    fn Sum(&self) -> Option<Rc<TerminalNode<'input, SQLParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(Sum, 0)
    }
}

impl<'input> AggregatorContextAttrs<'input> for AggregatorContext<'input> {}

impl<'input, I, H> SQLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn aggregator(&mut self) -> Result<Rc<AggregatorContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = AggregatorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 44, RULE_aggregator);
        let mut _localctx: Rc<AggregatorContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(403);
                _la = recog.base.input.la(1);
                if {
                    !(((_la - 58) & !0x3f) == 0
                        && ((1usize << (_la - 58))
                            & ((1usize << (Count - 58))
                                | (1usize << (Average - 58))
                                | (1usize << (Max - 58))
                                | (1usize << (Min - 58))
                                | (1usize << (Sum - 58))))
                            != 0)
                } {
                    recog.err_handler.recover_inline(&mut recog.base)?;
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(_ATN.clone(), _ATN.get_decision_state(i), i as isize).into())
        }
        Arc::new(dfa)
    };
}

const _serializedATN: &'static str =
    "\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x47\u{198}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\
	\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\
	\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\
	\x0e\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x04\
	\x13\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x04\x16\x09\x16\x04\x17\x09\
	\x17\x04\x18\x09\x18\x03\x02\x07\x02\x32\x0a\x02\x0c\x02\x0e\x02\x35\x0b\
	\x02\x03\x02\x03\x02\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\
	\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\
	\x03\x05\x03\x49\x0a\x03\x03\x04\x03\x04\x03\x04\x03\x04\x03\x04\x03\x04\
	\x03\x04\x03\x04\x03\x04\x03\x04\x03\x04\x03\x04\x03\x04\x03\x04\x05\x04\
	\x59\x0a\x04\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\
	\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x05\x05\x69\x0a\x05\
	\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\
	\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\
	\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\
	\x03\x06\x03\x06\x03\x06\x05\x06\u{89}\x0a\x06\x03\x07\x03\x07\x03\x07\x03\
	\x07\x03\x07\x03\x07\x05\x07\u{91}\x0a\x07\x03\x07\x03\x07\x03\x07\x05\x07\
	\u{96}\x0a\x07\x03\x07\x03\x07\x03\x07\x03\x07\x05\x07\u{9c}\x0a\x07\x05\
	\x07\u{9e}\x0a\x07\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\
	\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\
	\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\
	\x05\x08\u{b9}\x0a\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\
	\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x05\x08\u{c8}\x0a\x08\
	\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\
	\x03\x08\x03\x08\x03\x08\x05\x08\u{d6}\x0a\x08\x03\x08\x03\x08\x03\x08\x03\
	\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\
	\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x05\x08\u{ec}\
	\x0a\x08\x03\x09\x03\x09\x03\x09\x07\x09\u{f1}\x0a\x09\x0c\x09\x0e\x09\u{f4}\
	\x0b\x09\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x05\x0a\u{fa}\x0a\x0a\x03\x0a\x03\
	\x0a\x05\x0a\u{fe}\x0a\x0a\x03\x0a\x03\x0a\x03\x0a\x05\x0a\u{103}\x0a\x0a\
	\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x05\x0a\u{10c}\
	\x0a\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\
	\x03\x0a\x05\x0a\u{117}\x0a\x0a\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\
	\x03\x0b\x05\x0b\u{11f}\x0a\x0b\x03\x0c\x03\x0c\x03\x0c\x07\x0c\u{124}\x0a\
	\x0c\x0c\x0c\x0e\x0c\u{127}\x0b\x0c\x03\x0d\x03\x0d\x03\x0d\x03\x0d\x07\
	\x0d\u{12d}\x0a\x0d\x0c\x0d\x0e\x0d\u{130}\x0b\x0d\x03\x0d\x03\x0d\x03\x0e\
	\x03\x0e\x03\x0f\x03\x0f\x03\x0f\x07\x0f\u{139}\x0a\x0f\x0c\x0f\x0e\x0f\
	\u{13c}\x0b\x0f\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\
	\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\x05\x10\u{14b}\x0a\x10\
	\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\
	\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\x05\x10\u{15d}\
	\x0a\x10\x03\x11\x03\x11\x05\x11\u{161}\x0a\x11\x03\x11\x03\x11\x03\x12\
	\x03\x12\x05\x12\u{167}\x0a\x12\x03\x13\x03\x13\x03\x13\x03\x13\x03\x13\
	\x03\x13\x03\x13\x07\x13\u{170}\x0a\x13\x0c\x13\x0e\x13\u{173}\x0b\x13\x03\
	\x14\x03\x14\x03\x14\x03\x14\x07\x14\u{179}\x0a\x14\x0c\x14\x0e\x14\u{17c}\
	\x0b\x14\x05\x14\u{17e}\x0a\x14\x03\x15\x03\x15\x03\x15\x03\x15\x03\x15\
	\x03\x15\x03\x15\x03\x15\x03\x15\x03\x15\x05\x15\u{18a}\x0a\x15\x03\x16\
	\x03\x16\x03\x16\x07\x16\u{18f}\x0a\x16\x0c\x16\x0e\x16\u{192}\x0b\x16\x03\
	\x17\x03\x17\x03\x18\x03\x18\x03\x18\x02\x02\x19\x02\x04\x06\x08\x0a\x0c\
	\x0e\x10\x12\x14\x16\x18\x1a\x1c\x1e\x20\x22\x24\x26\x28\x2a\x2c\x2e\x02\
	\x05\x04\x02\x41\x41\x43\x45\x03\x02\x36\x3b\x03\x02\x3c\x40\x02\u{1b9}\
	\x02\x33\x03\x02\x02\x02\x04\x48\x03\x02\x02\x02\x06\x58\x03\x02\x02\x02\
	\x08\x68\x03\x02\x02\x02\x0a\u{88}\x03\x02\x02\x02\x0c\u{8a}\x03\x02\x02\
	\x02\x0e\u{eb}\x03\x02\x02\x02\x10\u{ed}\x03\x02\x02\x02\x12\u{116}\x03\
	\x02\x02\x02\x14\u{11e}\x03\x02\x02\x02\x16\u{120}\x03\x02\x02\x02\x18\u{128}\
	\x03\x02\x02\x02\x1a\u{133}\x03\x02\x02\x02\x1c\u{135}\x03\x02\x02\x02\x1e\
	\u{15c}\x03\x02\x02\x02\x20\u{160}\x03\x02\x02\x02\x22\u{166}\x03\x02\x02\
	\x02\x24\u{168}\x03\x02\x02\x02\x26\u{17d}\x03\x02\x02\x02\x28\u{189}\x03\
	\x02\x02\x02\x2a\u{18b}\x03\x02\x02\x02\x2c\u{193}\x03\x02\x02\x02\x2e\u{195}\
	\x03\x02\x02\x02\x30\x32\x05\x04\x03\x02\x31\x30\x03\x02\x02\x02\x32\x35\
	\x03\x02\x02\x02\x33\x31\x03\x02\x02\x02\x33\x34\x03\x02\x02\x02\x34\x36\
	\x03\x02\x02\x02\x35\x33\x03\x02\x02\x02\x36\x37\x07\x02\x02\x03\x37\x03\
	\x03\x02\x02\x02\x38\x39\x05\x06\x04\x02\x39\x3a\x07\x03\x02\x02\x3a\x49\
	\x03\x02\x02\x02\x3b\x3c\x05\x08\x05\x02\x3c\x3d\x07\x03\x02\x02\x3d\x49\
	\x03\x02\x02\x02\x3e\x3f\x05\x0a\x06\x02\x3f\x40\x07\x03\x02\x02\x40\x49\
	\x03\x02\x02\x02\x41\x42\x05\x0e\x08\x02\x42\x43\x07\x03\x02\x02\x43\x49\
	\x03\x02\x02\x02\x44\x45\x07\x47\x02\x02\x45\x49\x07\x03\x02\x02\x46\x47\
	\x07\x41\x02\x02\x47\x49\x07\x03\x02\x02\x48\x38\x03\x02\x02\x02\x48\x3b\
	\x03\x02\x02\x02\x48\x3e\x03\x02\x02\x02\x48\x41\x03\x02\x02\x02\x48\x44\
	\x03\x02\x02\x02\x48\x46\x03\x02\x02\x02\x49\x05\x03\x02\x02\x02\x4a\x4b\
	\x07\x04\x02\x02\x4b\x4c\x07\x05\x02\x02\x4c\x59\x07\x42\x02\x02\x4d\x4e\
	\x07\x06\x02\x02\x4e\x4f\x07\x05\x02\x02\x4f\x59\x07\x42\x02\x02\x50\x51\
	\x07\x07\x02\x02\x51\x59\x07\x08\x02\x02\x52\x53\x07\x09\x02\x02\x53\x59\
	\x07\x42\x02\x02\x54\x55\x07\x07\x02\x02\x55\x59\x07\x0a\x02\x02\x56\x57\
	\x07\x07\x02\x02\x57\x59\x07\x0b\x02\x02\x58\x4a\x03\x02\x02\x02\x58\x4d\
	\x03\x02\x02\x02\x58\x50\x03\x02\x02\x02\x58\x52\x03\x02\x02\x02\x58\x54\
	\x03\x02\x02\x02\x58\x56\x03\x02\x02\x02\x59\x07\x03\x02\x02\x02\x5a\x5b\
	\x07\x0c\x02\x02\x5b\x5c\x07\x0d\x02\x02\x5c\x5d\x07\x0e\x02\x02\x5d\x5e\
	\x07\x44\x02\x02\x5e\x5f\x07\x0f\x02\x02\x5f\x60\x07\x10\x02\x02\x60\x69\
	\x07\x42\x02\x02\x61\x62\x07\x11\x02\x02\x62\x63\x07\x0f\x02\x02\x63\x64\
	\x07\x0e\x02\x02\x64\x65\x07\x44\x02\x02\x65\x66\x07\x0d\x02\x02\x66\x67\
	\x07\x10\x02\x02\x67\x69\x07\x42\x02\x02\x68\x5a\x03\x02\x02\x02\x68\x61\
	\x03\x02\x02\x02\x69\x09\x03\x02\x02\x02\x6a\x6b\x07\x04\x02\x02\x6b\x6c\
	\x07\x10\x02\x02\x6c\x6d\x07\x42\x02\x02\x6d\x6e\x07\x12\x02\x02\x6e\x6f\
	\x05\x10\x09\x02\x6f\x70\x07\x13\x02\x02\x70\u{89}\x03\x02\x02\x02\x71\x72\
	\x07\x06\x02\x02\x72\x73\x07\x10\x02\x02\x73\u{89}\x07\x42\x02\x02\x74\x75\
	\x07\x14\x02\x02\x75\u{89}\x07\x42\x02\x02\x76\x77\x07\x15\x02\x02\x77\x78\
	\x07\x16\x02\x02\x78\x79\x07\x42\x02\x02\x79\x7a\x07\x17\x02\x02\x7a\u{89}\
	\x05\x16\x0c\x02\x7b\x7c\x07\x18\x02\x02\x7c\x7d\x07\x0d\x02\x02\x7d\x7e\
	\x07\x42\x02\x02\x7e\x7f\x07\x19\x02\x02\x7f\u{89}\x05\x1c\x0f\x02\u{80}\
	\u{81}\x07\x1a\x02\x02\u{81}\u{82}\x07\x42\x02\x02\u{82}\u{83}\x07\x1b\x02\
	\x02\u{83}\u{84}\x05\x24\x13\x02\u{84}\u{85}\x07\x19\x02\x02\u{85}\u{86}\
	\x05\x1c\x0f\x02\u{86}\u{89}\x03\x02\x02\x02\u{87}\u{89}\x05\x0c\x07\x02\
	\u{88}\x6a\x03\x02\x02\x02\u{88}\x71\x03\x02\x02\x02\u{88}\x74\x03\x02\x02\
	\x02\u{88}\x76\x03\x02\x02\x02\u{88}\x7b\x03\x02\x02\x02\u{88}\u{80}\x03\
	\x02\x02\x02\u{88}\u{87}\x03\x02\x02\x02\u{89}\x0b\x03\x02\x02\x02\u{8a}\
	\u{8b}\x07\x1c\x02\x02\u{8b}\u{8c}\x05\x26\x14\x02\u{8c}\u{8d}\x07\x0d\x02\
	\x02\u{8d}\u{90}\x05\x2a\x16\x02\u{8e}\u{8f}\x07\x19\x02\x02\u{8f}\u{91}\
	\x05\x1c\x0f\x02\u{90}\u{8e}\x03\x02\x02\x02\u{90}\u{91}\x03\x02\x02\x02\
	\u{91}\u{95}\x03\x02\x02\x02\u{92}\u{93}\x07\x1d\x02\x02\u{93}\u{94}\x07\
	\x1e\x02\x02\u{94}\u{96}\x05\x20\x11\x02\u{95}\u{92}\x03\x02\x02\x02\u{95}\
	\u{96}\x03\x02\x02\x02\u{96}\u{9d}\x03\x02\x02\x02\u{97}\u{98}\x07\x1f\x02\
	\x02\u{98}\u{9b}\x07\x43\x02\x02\u{99}\u{9a}\x07\x20\x02\x02\u{9a}\u{9c}\
	\x07\x43\x02\x02\u{9b}\u{99}\x03\x02\x02\x02\u{9b}\u{9c}\x03\x02\x02\x02\
	\u{9c}\u{9e}\x03\x02\x02\x02\u{9d}\u{97}\x03\x02\x02\x02\u{9d}\u{9e}\x03\
	\x02\x02\x02\u{9e}\x0d\x03\x02\x02\x02\u{9f}\u{a0}\x07\x21\x02\x02\u{a0}\
	\u{a1}\x07\x10\x02\x02\u{a1}\u{a2}\x07\x42\x02\x02\u{a2}\u{a3}\x07\x22\x02\
	\x02\u{a3}\u{a4}\x07\x23\x02\x02\u{a4}\u{a5}\x07\x12\x02\x02\u{a5}\u{a6}\
	\x05\x2a\x16\x02\u{a6}\u{a7}\x07\x13\x02\x02\u{a7}\u{ec}\x03\x02\x02\x02\
	\u{a8}\u{a9}\x07\x21\x02\x02\u{a9}\u{aa}\x07\x10\x02\x02\u{aa}\u{ab}\x07\
	\x42\x02\x02\u{ab}\u{ac}\x07\x06\x02\x02\u{ac}\u{ad}\x07\x23\x02\x02\u{ad}\
	\u{ae}\x07\x12\x02\x02\u{ae}\u{af}\x05\x2a\x16\x02\u{af}\u{b0}\x07\x13\x02\
	\x02\u{b0}\u{ec}\x03\x02\x02\x02\u{b1}\u{b2}\x07\x21\x02\x02\u{b2}\u{b3}\
	\x07\x10\x02\x02\u{b3}\u{b4}\x07\x42\x02\x02\u{b4}\u{b5}\x07\x06\x02\x02\
	\u{b5}\u{b6}\x07\x24\x02\x02\u{b6}\u{b8}\x07\x25\x02\x02\u{b7}\u{b9}\x07\
	\x42\x02\x02\u{b8}\u{b7}\x03\x02\x02\x02\u{b8}\u{b9}\x03\x02\x02\x02\u{b9}\
	\u{ec}\x03\x02\x02\x02\u{ba}\u{bb}\x07\x21\x02\x02\u{bb}\u{bc}\x07\x10\x02\
	\x02\u{bc}\u{bd}\x07\x42\x02\x02\u{bd}\u{be}\x07\x06\x02\x02\u{be}\u{bf}\
	\x07\x26\x02\x02\u{bf}\u{c0}\x07\x25\x02\x02\u{c0}\u{ec}\x07\x42\x02\x02\
	\u{c1}\u{c2}\x07\x21\x02\x02\u{c2}\u{c3}\x07\x10\x02\x02\u{c3}\u{c4}\x07\
	\x42\x02\x02\u{c4}\u{c5}\x07\x22\x02\x02\u{c5}\u{c7}\x07\x27\x02\x02\u{c6}\
	\u{c8}\x07\x42\x02\x02\u{c7}\u{c6}\x03\x02\x02\x02\u{c7}\u{c8}\x03\x02\x02\
	\x02\u{c8}\u{c9}\x03\x02\x02\x02\u{c9}\u{ca}\x07\x24\x02\x02\u{ca}\u{cb}\
	\x07\x25\x02\x02\u{cb}\u{cc}\x07\x12\x02\x02\u{cc}\u{cd}\x05\x2a\x16\x02\
	\u{cd}\u{ce}\x07\x13\x02\x02\u{ce}\u{ec}\x03\x02\x02\x02\u{cf}\u{d0}\x07\
	\x21\x02\x02\u{d0}\u{d1}\x07\x10\x02\x02\u{d1}\u{d2}\x07\x42\x02\x02\u{d2}\
	\u{d3}\x07\x22\x02\x02\u{d3}\u{d5}\x07\x27\x02\x02\u{d4}\u{d6}\x07\x42\x02\
	\x02\u{d5}\u{d4}\x03\x02\x02\x02\u{d5}\u{d6}\x03\x02\x02\x02\u{d6}\u{d7}\
	\x03\x02\x02\x02\u{d7}\u{d8}\x07\x26\x02\x02\u{d8}\u{d9}\x07\x25\x02\x02\
	\u{d9}\u{da}\x07\x12\x02\x02\u{da}\u{db}\x05\x2a\x16\x02\u{db}\u{dc}\x07\
	\x13\x02\x02\u{dc}\u{dd}\x07\x28\x02\x02\u{dd}\u{de}\x07\x42\x02\x02\u{de}\
	\u{df}\x07\x12\x02\x02\u{df}\u{e0}\x05\x2a\x16\x02\u{e0}\u{e1}\x07\x13\x02\
	\x02\u{e1}\u{ec}\x03\x02\x02\x02\u{e2}\u{e3}\x07\x21\x02\x02\u{e3}\u{e4}\
	\x07\x10\x02\x02\u{e4}\u{e5}\x07\x42\x02\x02\u{e5}\u{e6}\x07\x22\x02\x02\
	\u{e6}\u{e7}\x07\x29\x02\x02\u{e7}\u{e8}\x07\x12\x02\x02\u{e8}\u{e9}\x05\
	\x2a\x16\x02\u{e9}\u{ea}\x07\x13\x02\x02\u{ea}\u{ec}\x03\x02\x02\x02\u{eb}\
	\u{9f}\x03\x02\x02\x02\u{eb}\u{a8}\x03\x02\x02\x02\u{eb}\u{b1}\x03\x02\x02\
	\x02\u{eb}\u{ba}\x03\x02\x02\x02\u{eb}\u{c1}\x03\x02\x02\x02\u{eb}\u{cf}\
	\x03\x02\x02\x02\u{eb}\u{e2}\x03\x02\x02\x02\u{ec}\x0f\x03\x02\x02\x02\u{ed}\
	\u{f2}\x05\x12\x0a\x02\u{ee}\u{ef}\x07\x2a\x02\x02\u{ef}\u{f1}\x05\x12\x0a\
	\x02\u{f0}\u{ee}\x03\x02\x02\x02\u{f1}\u{f4}\x03\x02\x02\x02\u{f2}\u{f0}\
	\x03\x02\x02\x02\u{f2}\u{f3}\x03\x02\x02\x02\u{f3}\x11\x03\x02\x02\x02\u{f4}\
	\u{f2}\x03\x02\x02\x02\u{f5}\u{f6}\x07\x42\x02\x02\u{f6}\u{f9}\x05\x14\x0b\
	\x02\u{f7}\u{f8}\x07\x2b\x02\x02\u{f8}\u{fa}\x07\x41\x02\x02\u{f9}\u{f7}\
	\x03\x02\x02\x02\u{f9}\u{fa}\x03\x02\x02\x02\u{fa}\u{fd}\x03\x02\x02\x02\
	\u{fb}\u{fc}\x07\x2c\x02\x02\u{fc}\u{fe}\x05\x1a\x0e\x02\u{fd}\u{fb}\x03\
	\x02\x02\x02\u{fd}\u{fe}\x03\x02\x02\x02\u{fe}\u{117}\x03\x02\x02\x02\u{ff}\
	\u{100}\x07\x24\x02\x02\u{100}\u{102}\x07\x25\x02\x02\u{101}\u{103}\x07\
	\x42\x02\x02\u{102}\u{101}\x03\x02\x02\x02\u{102}\u{103}\x03\x02\x02\x02\
	\u{103}\u{104}\x03\x02\x02\x02\u{104}\u{105}\x07\x12\x02\x02\u{105}\u{106}\
	\x05\x2a\x16\x02\u{106}\u{107}\x07\x13\x02\x02\u{107}\u{117}\x03\x02\x02\
	\x02\u{108}\u{109}\x07\x26\x02\x02\u{109}\u{10b}\x07\x25\x02\x02\u{10a}\
	\u{10c}\x07\x42\x02\x02\u{10b}\u{10a}\x03\x02\x02\x02\u{10b}\u{10c}\x03\
	\x02\x02\x02\u{10c}\u{10d}\x03\x02\x02\x02\u{10d}\u{10e}\x07\x12\x02\x02\
	\u{10e}\u{10f}\x05\x2a\x16\x02\u{10f}\u{110}\x07\x13\x02\x02\u{110}\u{111}\
	\x07\x28\x02\x02\u{111}\u{112}\x07\x42\x02\x02\u{112}\u{113}\x07\x12\x02\
	\x02\u{113}\u{114}\x05\x2a\x16\x02\u{114}\u{115}\x07\x13\x02\x02\u{115}\
	\u{117}\x03\x02\x02\x02\u{116}\u{f5}\x03\x02\x02\x02\u{116}\u{ff}\x03\x02\
	\x02\x02\u{116}\u{108}\x03\x02\x02\x02\u{117}\x13\x03\x02\x02\x02\u{118}\
	\u{11f}\x07\x2d\x02\x02\u{119}\u{11a}\x07\x2e\x02\x02\u{11a}\u{11b}\x07\
	\x12\x02\x02\u{11b}\u{11c}\x07\x43\x02\x02\u{11c}\u{11f}\x07\x13\x02\x02\
	\u{11d}\u{11f}\x07\x2f\x02\x02\u{11e}\u{118}\x03\x02\x02\x02\u{11e}\u{119}\
	\x03\x02\x02\x02\u{11e}\u{11d}\x03\x02\x02\x02\u{11f}\x15\x03\x02\x02\x02\
	\u{120}\u{125}\x05\x18\x0d\x02\u{121}\u{122}\x07\x2a\x02\x02\u{122}\u{124}\
	\x05\x18\x0d\x02\u{123}\u{121}\x03\x02\x02\x02\u{124}\u{127}\x03\x02\x02\
	\x02\u{125}\u{123}\x03\x02\x02\x02\u{125}\u{126}\x03\x02\x02\x02\u{126}\
	\x17\x03\x02\x02\x02\u{127}\u{125}\x03\x02\x02\x02\u{128}\u{129}\x07\x12\
	\x02\x02\u{129}\u{12e}\x05\x1a\x0e\x02\u{12a}\u{12b}\x07\x2a\x02\x02\u{12b}\
	\u{12d}\x05\x1a\x0e\x02\u{12c}\u{12a}\x03\x02\x02\x02\u{12d}\u{130}\x03\
	\x02\x02\x02\u{12e}\u{12c}\x03\x02\x02\x02\u{12e}\u{12f}\x03\x02\x02\x02\
	\u{12f}\u{131}\x03\x02\x02\x02\u{130}\u{12e}\x03\x02\x02\x02\u{131}\u{132}\
	\x07\x13\x02\x02\u{132}\x19\x03\x02\x02\x02\u{133}\u{134}\x09\x02\x02\x02\
	\u{134}\x1b\x03\x02\x02\x02\u{135}\u{13a}\x05\x1e\x10\x02\u{136}\u{137}\
	\x07\x30\x02\x02\u{137}\u{139}\x05\x1e\x10\x02\u{138}\u{136}\x03\x02\x02\
	\x02\u{139}\u{13c}\x03\x02\x02\x02\u{13a}\u{138}\x03\x02\x02\x02\u{13a}\
	\u{13b}\x03\x02\x02\x02\u{13b}\x1d\x03\x02\x02\x02\u{13c}\u{13a}\x03\x02\
	\x02\x02\u{13d}\u{13e}\x05\x20\x11\x02\u{13e}\u{13f}\x05\x2c\x17\x02\u{13f}\
	\u{140}\x05\x22\x12\x02\u{140}\u{15d}\x03\x02\x02\x02\u{141}\u{142}\x05\
	\x20\x11\x02\u{142}\u{143}\x05\x2c\x17\x02\u{143}\u{144}\x07\x12\x02\x02\
	\u{144}\u{145}\x05\x0c\x07\x02\u{145}\u{146}\x07\x13\x02\x02\u{146}\u{15d}\
	\x03\x02\x02\x02\u{147}\u{148}\x05\x20\x11\x02\u{148}\u{14a}\x07\x31\x02\
	\x02\u{149}\u{14b}\x07\x2b\x02\x02\u{14a}\u{149}\x03\x02\x02\x02\u{14a}\
	\u{14b}\x03\x02\x02\x02\u{14b}\u{14c}\x03\x02\x02\x02\u{14c}\u{14d}\x07\
	\x41\x02\x02\u{14d}\u{15d}\x03\x02\x02\x02\u{14e}\u{14f}\x05\x20\x11\x02\
	\u{14f}\u{150}\x07\x32\x02\x02\u{150}\u{151}\x05\x18\x0d\x02\u{151}\u{15d}\
	\x03\x02\x02\x02\u{152}\u{153}\x05\x20\x11\x02\u{153}\u{154}\x07\x32\x02\
	\x02\u{154}\u{155}\x07\x12\x02\x02\u{155}\u{156}\x05\x0c\x07\x02\u{156}\
	\u{157}\x07\x13\x02\x02\u{157}\u{15d}\x03\x02\x02\x02\u{158}\u{159}\x05\
	\x20\x11\x02\u{159}\u{15a}\x07\x33\x02\x02\u{15a}\u{15b}\x07\x44\x02\x02\
	\u{15b}\u{15d}\x03\x02\x02\x02\u{15c}\u{13d}\x03\x02\x02\x02\u{15c}\u{141}\
	\x03\x02\x02\x02\u{15c}\u{147}\x03\x02\x02\x02\u{15c}\u{14e}\x03\x02\x02\
	\x02\u{15c}\u{152}\x03\x02\x02\x02\u{15c}\u{158}\x03\x02\x02\x02\u{15d}\
	\x1f\x03\x02\x02\x02\u{15e}\u{15f}\x07\x42\x02\x02\u{15f}\u{161}\x07\x34\
	\x02\x02\u{160}\u{15e}\x03\x02\x02\x02\u{160}\u{161}\x03\x02\x02\x02\u{161}\
	\u{162}\x03\x02\x02\x02\u{162}\u{163}\x07\x42\x02\x02\u{163}\x21\x03\x02\
	\x02\x02\u{164}\u{167}\x05\x1a\x0e\x02\u{165}\u{167}\x05\x20\x11\x02\u{166}\
	\u{164}\x03\x02\x02\x02\u{166}\u{165}\x03\x02\x02\x02\u{167}\x23\x03\x02\
	\x02\x02\u{168}\u{169}\x07\x42\x02\x02\u{169}\u{16a}\x07\x36\x02\x02\u{16a}\
	\u{171}\x05\x1a\x0e\x02\u{16b}\u{16c}\x07\x2a\x02\x02\u{16c}\u{16d}\x07\
	\x42\x02\x02\u{16d}\u{16e}\x07\x36\x02\x02\u{16e}\u{170}\x05\x1a\x0e\x02\
	\u{16f}\u{16b}\x03\x02\x02\x02\u{170}\u{173}\x03\x02\x02\x02\u{171}\u{16f}\
	\x03\x02\x02\x02\u{171}\u{172}\x03\x02\x02\x02\u{172}\x25\x03\x02\x02\x02\
	\u{173}\u{171}\x03\x02\x02\x02\u{174}\u{17e}\x07\x35\x02\x02\u{175}\u{17a}\
	\x05\x28\x15\x02\u{176}\u{177}\x07\x2a\x02\x02\u{177}\u{179}\x05\x28\x15\
	\x02\u{178}\u{176}\x03\x02\x02\x02\u{179}\u{17c}\x03\x02\x02\x02\u{17a}\
	\u{178}\x03\x02\x02\x02\u{17a}\u{17b}\x03\x02\x02\x02\u{17b}\u{17e}\x03\
	\x02\x02\x02\u{17c}\u{17a}\x03\x02\x02\x02\u{17d}\u{174}\x03\x02\x02\x02\
	\u{17d}\u{175}\x03\x02\x02\x02\u{17e}\x27\x03\x02\x02\x02\u{17f}\u{18a}\
	\x05\x20\x11\x02\u{180}\u{181}\x05\x2e\x18\x02\u{181}\u{182}\x07\x12\x02\
	\x02\u{182}\u{183}\x05\x20\x11\x02\u{183}\u{184}\x07\x13\x02\x02\u{184}\
	\u{18a}\x03\x02\x02\x02\u{185}\u{186}\x07\x3c\x02\x02\u{186}\u{187}\x07\
	\x12\x02\x02\u{187}\u{188}\x07\x35\x02\x02\u{188}\u{18a}\x07\x13\x02\x02\
	\u{189}\u{17f}\x03\x02\x02\x02\u{189}\u{180}\x03\x02\x02\x02\u{189}\u{185}\
	\x03\x02\x02\x02\u{18a}\x29\x03\x02\x02\x02\u{18b}\u{190}\x07\x42\x02\x02\
	\u{18c}\u{18d}\x07\x2a\x02\x02\u{18d}\u{18f}\x07\x42\x02\x02\u{18e}\u{18c}\
	\x03\x02\x02\x02\u{18f}\u{192}\x03\x02\x02\x02\u{190}\u{18e}\x03\x02\x02\
	\x02\u{190}\u{191}\x03\x02\x02\x02\u{191}\x2b\x03\x02\x02\x02\u{192}\u{190}\
	\x03\x02\x02\x02\u{193}\u{194}\x09\x03\x02\x02\u{194}\x2d\x03\x02\x02\x02\
	\u{195}\u{196}\x09\x04\x02\x02\u{196}\x2f\x03\x02\x02\x02\x22\x33\x48\x58\
	\x68\u{88}\u{90}\u{95}\u{9b}\u{9d}\u{b8}\u{c7}\u{d5}\u{eb}\u{f2}\u{f9}\u{fd}\
	\u{102}\u{10b}\u{116}\u{11e}\u{125}\u{12e}\u{13a}\u{14a}\u{15c}\u{160}\u{166}\
	\u{171}\u{17a}\u{17d}\u{189}\u{190}";
