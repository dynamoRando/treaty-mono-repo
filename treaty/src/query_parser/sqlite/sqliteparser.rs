// Generated from SQLite.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
use super::sqlitelistener::*;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::error_strategy::{DefaultErrorStrategy, ErrorStrategy};
use antlr_rust::errors::*;
use antlr_rust::int_stream::EOF;
use antlr_rust::lazy_static;
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
use antlr_rust::{TidAble, TidExt};

use std::any::{Any, TypeId};
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::convert::TryFrom;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;

pub const SCOL: isize = 1;
pub const DOT: isize = 2;
pub const OPEN_PAR: isize = 3;
pub const CLOSE_PAR: isize = 4;
pub const COMMA: isize = 5;
pub const ASSIGN: isize = 6;
pub const STAR: isize = 7;
pub const PLUS: isize = 8;
pub const MINUS: isize = 9;
pub const TILDE: isize = 10;
pub const PIPE2: isize = 11;
pub const DIV: isize = 12;
pub const MOD: isize = 13;
pub const LT2: isize = 14;
pub const GT2: isize = 15;
pub const AMP: isize = 16;
pub const PIPE: isize = 17;
pub const LT: isize = 18;
pub const LT_EQ: isize = 19;
pub const GT: isize = 20;
pub const GT_EQ: isize = 21;
pub const EQ: isize = 22;
pub const NOT_EQ1: isize = 23;
pub const NOT_EQ2: isize = 24;
pub const K_ABORT: isize = 25;
pub const K_ACTION: isize = 26;
pub const K_ADD: isize = 27;
pub const K_AFTER: isize = 28;
pub const K_ALL: isize = 29;
pub const K_ALTER: isize = 30;
pub const K_ANALYZE: isize = 31;
pub const K_AND: isize = 32;
pub const K_AS: isize = 33;
pub const K_ASC: isize = 34;
pub const K_ATTACH: isize = 35;
pub const K_AUTOINCREMENT: isize = 36;
pub const K_BEFORE: isize = 37;
pub const K_BEGIN: isize = 38;
pub const K_BETWEEN: isize = 39;
pub const K_BY: isize = 40;
pub const K_CASCADE: isize = 41;
pub const K_CASE: isize = 42;
pub const K_CAST: isize = 43;
pub const K_CHECK: isize = 44;
pub const K_COLLATE: isize = 45;
pub const K_COLUMN: isize = 46;
pub const K_COMMIT: isize = 47;
pub const K_CONFLICT: isize = 48;
pub const K_CONSTRAINT: isize = 49;
pub const K_CREATE: isize = 50;
pub const K_CROSS: isize = 51;
pub const K_CURRENT_DATE: isize = 52;
pub const K_CURRENT_TIME: isize = 53;
pub const K_CURRENT_TIMESTAMP: isize = 54;
pub const K_DATABASE: isize = 55;
pub const K_DEFAULT: isize = 56;
pub const K_DEFERRABLE: isize = 57;
pub const K_DEFERRED: isize = 58;
pub const K_DELETE: isize = 59;
pub const K_DESC: isize = 60;
pub const K_DETACH: isize = 61;
pub const K_DISTINCT: isize = 62;
pub const K_DROP: isize = 63;
pub const K_EACH: isize = 64;
pub const K_ELSE: isize = 65;
pub const K_END: isize = 66;
pub const K_ESCAPE: isize = 67;
pub const K_EXCEPT: isize = 68;
pub const K_EXCLUSIVE: isize = 69;
pub const K_EXISTS: isize = 70;
pub const K_EXPLAIN: isize = 71;
pub const K_FAIL: isize = 72;
pub const K_FOR: isize = 73;
pub const K_FOREIGN: isize = 74;
pub const K_FROM: isize = 75;
pub const K_FULL: isize = 76;
pub const K_GLOB: isize = 77;
pub const K_GROUP: isize = 78;
pub const K_HAVING: isize = 79;
pub const K_IF: isize = 80;
pub const K_IGNORE: isize = 81;
pub const K_IMMEDIATE: isize = 82;
pub const K_IN: isize = 83;
pub const K_INDEX: isize = 84;
pub const K_INDEXED: isize = 85;
pub const K_INITIALLY: isize = 86;
pub const K_INNER: isize = 87;
pub const K_INSERT: isize = 88;
pub const K_INSTEAD: isize = 89;
pub const K_INTERSECT: isize = 90;
pub const K_INTO: isize = 91;
pub const K_IS: isize = 92;
pub const K_ISNULL: isize = 93;
pub const K_JOIN: isize = 94;
pub const K_KEY: isize = 95;
pub const K_LEFT: isize = 96;
pub const K_LIKE: isize = 97;
pub const K_LIMIT: isize = 98;
pub const K_MATCH: isize = 99;
pub const K_NATURAL: isize = 100;
pub const K_NO: isize = 101;
pub const K_NOT: isize = 102;
pub const K_NOTNULL: isize = 103;
pub const K_NULL: isize = 104;
pub const K_OF: isize = 105;
pub const K_OFFSET: isize = 106;
pub const K_ON: isize = 107;
pub const K_OR: isize = 108;
pub const K_ORDER: isize = 109;
pub const K_OUTER: isize = 110;
pub const K_PLAN: isize = 111;
pub const K_PRAGMA: isize = 112;
pub const K_PRIMARY: isize = 113;
pub const K_QUERY: isize = 114;
pub const K_RAISE: isize = 115;
pub const K_RECURSIVE: isize = 116;
pub const K_REFERENCES: isize = 117;
pub const K_REGEXP: isize = 118;
pub const K_REINDEX: isize = 119;
pub const K_RELEASE: isize = 120;
pub const K_RENAME: isize = 121;
pub const K_REPLACE: isize = 122;
pub const K_RESTRICT: isize = 123;
pub const K_RIGHT: isize = 124;
pub const K_ROLLBACK: isize = 125;
pub const K_ROW: isize = 126;
pub const K_SAVEPOINT: isize = 127;
pub const K_SELECT: isize = 128;
pub const K_SET: isize = 129;
pub const K_TABLE: isize = 130;
pub const K_TEMP: isize = 131;
pub const K_TEMPORARY: isize = 132;
pub const K_THEN: isize = 133;
pub const K_TO: isize = 134;
pub const K_TRANSACTION: isize = 135;
pub const K_TRIGGER: isize = 136;
pub const K_UNION: isize = 137;
pub const K_UNIQUE: isize = 138;
pub const K_UPDATE: isize = 139;
pub const K_USING: isize = 140;
pub const K_VACUUM: isize = 141;
pub const K_VALUES: isize = 142;
pub const K_VIEW: isize = 143;
pub const K_VIRTUAL: isize = 144;
pub const K_WHEN: isize = 145;
pub const K_WHERE: isize = 146;
pub const K_WITH: isize = 147;
pub const K_WITHOUT: isize = 148;
pub const IDENTIFIER: isize = 149;
pub const NUMERIC_LITERAL: isize = 150;
pub const BIND_PARAMETER: isize = 151;
pub const STRING_LITERAL: isize = 152;
pub const BLOB_LITERAL: isize = 153;
pub const SINGLE_LINE_COMMENT: isize = 154;
pub const MULTILINE_COMMENT: isize = 155;
pub const SPACES: isize = 156;
pub const UNEXPECTED_CHAR: isize = 157;
pub const RULE_parse: usize = 0;
pub const RULE_error: usize = 1;
pub const RULE_sql_stmt_list: usize = 2;
pub const RULE_sql_stmt: usize = 3;
pub const RULE_alter_table_stmt: usize = 4;
pub const RULE_analyze_stmt: usize = 5;
pub const RULE_attach_stmt: usize = 6;
pub const RULE_begin_stmt: usize = 7;
pub const RULE_commit_stmt: usize = 8;
pub const RULE_compound_select_stmt: usize = 9;
pub const RULE_create_index_stmt: usize = 10;
pub const RULE_create_table_stmt: usize = 11;
pub const RULE_create_trigger_stmt: usize = 12;
pub const RULE_create_view_stmt: usize = 13;
pub const RULE_create_virtual_table_stmt: usize = 14;
pub const RULE_delete_stmt: usize = 15;
pub const RULE_delete_stmt_limited: usize = 16;
pub const RULE_detach_stmt: usize = 17;
pub const RULE_drop_index_stmt: usize = 18;
pub const RULE_drop_table_stmt: usize = 19;
pub const RULE_drop_trigger_stmt: usize = 20;
pub const RULE_drop_view_stmt: usize = 21;
pub const RULE_factored_select_stmt: usize = 22;
pub const RULE_insert_stmt: usize = 23;
pub const RULE_pragma_stmt: usize = 24;
pub const RULE_reindex_stmt: usize = 25;
pub const RULE_release_stmt: usize = 26;
pub const RULE_rollback_stmt: usize = 27;
pub const RULE_savepoint_stmt: usize = 28;
pub const RULE_simple_select_stmt: usize = 29;
pub const RULE_select_stmt: usize = 30;
pub const RULE_select_or_values: usize = 31;
pub const RULE_update_stmt: usize = 32;
pub const RULE_update_stmt_limited: usize = 33;
pub const RULE_vacuum_stmt: usize = 34;
pub const RULE_column_def: usize = 35;
pub const RULE_type_name: usize = 36;
pub const RULE_column_constraint: usize = 37;
pub const RULE_conflict_clause: usize = 38;
pub const RULE_expr: usize = 39;
pub const RULE_foreign_key_clause: usize = 40;
pub const RULE_raise_function: usize = 41;
pub const RULE_indexed_column: usize = 42;
pub const RULE_table_constraint: usize = 43;
pub const RULE_with_clause: usize = 44;
pub const RULE_qualified_table_name: usize = 45;
pub const RULE_ordering_term: usize = 46;
pub const RULE_pragma_value: usize = 47;
pub const RULE_common_table_expression: usize = 48;
pub const RULE_result_column: usize = 49;
pub const RULE_table_or_subquery: usize = 50;
pub const RULE_join_clause: usize = 51;
pub const RULE_join_operator: usize = 52;
pub const RULE_join_constraint: usize = 53;
pub const RULE_select_core: usize = 54;
pub const RULE_compound_operator: usize = 55;
pub const RULE_signed_number: usize = 56;
pub const RULE_literal_value: usize = 57;
pub const RULE_unary_operator: usize = 58;
pub const RULE_error_message: usize = 59;
pub const RULE_module_argument: usize = 60;
pub const RULE_column_alias: usize = 61;
pub const RULE_keyword: usize = 62;
pub const RULE_name: usize = 63;
pub const RULE_function_name: usize = 64;
pub const RULE_database_name: usize = 65;
pub const RULE_schema_name: usize = 66;
pub const RULE_table_function_name: usize = 67;
pub const RULE_table_name: usize = 68;
pub const RULE_table_or_index_name: usize = 69;
pub const RULE_new_table_name: usize = 70;
pub const RULE_column_name: usize = 71;
pub const RULE_collation_name: usize = 72;
pub const RULE_foreign_table: usize = 73;
pub const RULE_index_name: usize = 74;
pub const RULE_trigger_name: usize = 75;
pub const RULE_view_name: usize = 76;
pub const RULE_module_name: usize = 77;
pub const RULE_pragma_name: usize = 78;
pub const RULE_savepoint_name: usize = 79;
pub const RULE_table_alias: usize = 80;
pub const RULE_transaction_name: usize = 81;
pub const RULE_any_name: usize = 82;
pub const ruleNames: [&str; 83] = [
    "parse",
    "error",
    "sql_stmt_list",
    "sql_stmt",
    "alter_table_stmt",
    "analyze_stmt",
    "attach_stmt",
    "begin_stmt",
    "commit_stmt",
    "compound_select_stmt",
    "create_index_stmt",
    "create_table_stmt",
    "create_trigger_stmt",
    "create_view_stmt",
    "create_virtual_table_stmt",
    "delete_stmt",
    "delete_stmt_limited",
    "detach_stmt",
    "drop_index_stmt",
    "drop_table_stmt",
    "drop_trigger_stmt",
    "drop_view_stmt",
    "factored_select_stmt",
    "insert_stmt",
    "pragma_stmt",
    "reindex_stmt",
    "release_stmt",
    "rollback_stmt",
    "savepoint_stmt",
    "simple_select_stmt",
    "select_stmt",
    "select_or_values",
    "update_stmt",
    "update_stmt_limited",
    "vacuum_stmt",
    "column_def",
    "type_name",
    "column_constraint",
    "conflict_clause",
    "expr",
    "foreign_key_clause",
    "raise_function",
    "indexed_column",
    "table_constraint",
    "with_clause",
    "qualified_table_name",
    "ordering_term",
    "pragma_value",
    "common_table_expression",
    "result_column",
    "table_or_subquery",
    "join_clause",
    "join_operator",
    "join_constraint",
    "select_core",
    "compound_operator",
    "signed_number",
    "literal_value",
    "unary_operator",
    "error_message",
    "module_argument",
    "column_alias",
    "keyword",
    "name",
    "function_name",
    "database_name",
    "schema_name",
    "table_function_name",
    "table_name",
    "table_or_index_name",
    "new_table_name",
    "column_name",
    "collation_name",
    "foreign_table",
    "index_name",
    "trigger_name",
    "view_name",
    "module_name",
    "pragma_name",
    "savepoint_name",
    "table_alias",
    "transaction_name",
    "any_name",
];

pub const _LITERAL_NAMES: [Option<&'static str>; 25] = [
    None,
    Some("';'"),
    Some("'.'"),
    Some("'('"),
    Some("')'"),
    Some("','"),
    Some("'='"),
    Some("'*'"),
    Some("'+'"),
    Some("'-'"),
    Some("'~'"),
    Some("'||'"),
    Some("'/'"),
    Some("'%'"),
    Some("'<<'"),
    Some("'>>'"),
    Some("'&'"),
    Some("'|'"),
    Some("'<'"),
    Some("'<='"),
    Some("'>'"),
    Some("'>='"),
    Some("'=='"),
    Some("'!='"),
    Some("'<>'"),
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>; 158] = [
    None,
    Some("SCOL"),
    Some("DOT"),
    Some("OPEN_PAR"),
    Some("CLOSE_PAR"),
    Some("COMMA"),
    Some("ASSIGN"),
    Some("STAR"),
    Some("PLUS"),
    Some("MINUS"),
    Some("TILDE"),
    Some("PIPE2"),
    Some("DIV"),
    Some("MOD"),
    Some("LT2"),
    Some("GT2"),
    Some("AMP"),
    Some("PIPE"),
    Some("LT"),
    Some("LT_EQ"),
    Some("GT"),
    Some("GT_EQ"),
    Some("EQ"),
    Some("NOT_EQ1"),
    Some("NOT_EQ2"),
    Some("K_ABORT"),
    Some("K_ACTION"),
    Some("K_ADD"),
    Some("K_AFTER"),
    Some("K_ALL"),
    Some("K_ALTER"),
    Some("K_ANALYZE"),
    Some("K_AND"),
    Some("K_AS"),
    Some("K_ASC"),
    Some("K_ATTACH"),
    Some("K_AUTOINCREMENT"),
    Some("K_BEFORE"),
    Some("K_BEGIN"),
    Some("K_BETWEEN"),
    Some("K_BY"),
    Some("K_CASCADE"),
    Some("K_CASE"),
    Some("K_CAST"),
    Some("K_CHECK"),
    Some("K_COLLATE"),
    Some("K_COLUMN"),
    Some("K_COMMIT"),
    Some("K_CONFLICT"),
    Some("K_CONSTRAINT"),
    Some("K_CREATE"),
    Some("K_CROSS"),
    Some("K_CURRENT_DATE"),
    Some("K_CURRENT_TIME"),
    Some("K_CURRENT_TIMESTAMP"),
    Some("K_DATABASE"),
    Some("K_DEFAULT"),
    Some("K_DEFERRABLE"),
    Some("K_DEFERRED"),
    Some("K_DELETE"),
    Some("K_DESC"),
    Some("K_DETACH"),
    Some("K_DISTINCT"),
    Some("K_DROP"),
    Some("K_EACH"),
    Some("K_ELSE"),
    Some("K_END"),
    Some("K_ESCAPE"),
    Some("K_EXCEPT"),
    Some("K_EXCLUSIVE"),
    Some("K_EXISTS"),
    Some("K_EXPLAIN"),
    Some("K_FAIL"),
    Some("K_FOR"),
    Some("K_FOREIGN"),
    Some("K_FROM"),
    Some("K_FULL"),
    Some("K_GLOB"),
    Some("K_GROUP"),
    Some("K_HAVING"),
    Some("K_IF"),
    Some("K_IGNORE"),
    Some("K_IMMEDIATE"),
    Some("K_IN"),
    Some("K_INDEX"),
    Some("K_INDEXED"),
    Some("K_INITIALLY"),
    Some("K_INNER"),
    Some("K_INSERT"),
    Some("K_INSTEAD"),
    Some("K_INTERSECT"),
    Some("K_INTO"),
    Some("K_IS"),
    Some("K_ISNULL"),
    Some("K_JOIN"),
    Some("K_KEY"),
    Some("K_LEFT"),
    Some("K_LIKE"),
    Some("K_LIMIT"),
    Some("K_MATCH"),
    Some("K_NATURAL"),
    Some("K_NO"),
    Some("K_NOT"),
    Some("K_NOTNULL"),
    Some("K_NULL"),
    Some("K_OF"),
    Some("K_OFFSET"),
    Some("K_ON"),
    Some("K_OR"),
    Some("K_ORDER"),
    Some("K_OUTER"),
    Some("K_PLAN"),
    Some("K_PRAGMA"),
    Some("K_PRIMARY"),
    Some("K_QUERY"),
    Some("K_RAISE"),
    Some("K_RECURSIVE"),
    Some("K_REFERENCES"),
    Some("K_REGEXP"),
    Some("K_REINDEX"),
    Some("K_RELEASE"),
    Some("K_RENAME"),
    Some("K_REPLACE"),
    Some("K_RESTRICT"),
    Some("K_RIGHT"),
    Some("K_ROLLBACK"),
    Some("K_ROW"),
    Some("K_SAVEPOINT"),
    Some("K_SELECT"),
    Some("K_SET"),
    Some("K_TABLE"),
    Some("K_TEMP"),
    Some("K_TEMPORARY"),
    Some("K_THEN"),
    Some("K_TO"),
    Some("K_TRANSACTION"),
    Some("K_TRIGGER"),
    Some("K_UNION"),
    Some("K_UNIQUE"),
    Some("K_UPDATE"),
    Some("K_USING"),
    Some("K_VACUUM"),
    Some("K_VALUES"),
    Some("K_VIEW"),
    Some("K_VIRTUAL"),
    Some("K_WHEN"),
    Some("K_WHERE"),
    Some("K_WITH"),
    Some("K_WITHOUT"),
    Some("IDENTIFIER"),
    Some("NUMERIC_LITERAL"),
    Some("BIND_PARAMETER"),
    Some("STRING_LITERAL"),
    Some("BLOB_LITERAL"),
    Some("SINGLE_LINE_COMMENT"),
    Some("MULTILINE_COMMENT"),
    Some("SPACES"),
    Some("UNEXPECTED_CHAR"),
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
    SQLiteParserExt<'input>,
    I,
    SQLiteParserContextType,
    dyn SQLiteListener<'input> + 'input,
>;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type SQLiteTreeWalker<'input, 'a> =
    ParseTreeWalker<'input, 'a, SQLiteParserContextType, dyn SQLiteListener<'input> + 'a>;

/// Parser for SQLite grammar
pub struct SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    base: BaseParserType<'input, I>,
    interpreter: Arc<ParserATNSimulator>,
    _shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> SQLiteParser<'input, I, H>
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
                SQLiteParserExt {
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

impl<'input, I> SQLiteParser<'input, I, DynStrategy<'input, I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self {
        Self::with_strategy(input, Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> SQLiteParser<'input, I, DefaultErrorStrategy<'input, SQLiteParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn new(input: I) -> Self {
        Self::with_strategy(input, DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for SQLiteParser
pub trait SQLiteParserContext<'input>:
    for<'x> Listenable<dyn SQLiteListener<'input> + 'x>
    + ParserRuleContext<'input, TF = LocalTokenFactory<'input>, Ctx = SQLiteParserContextType>
{
}

antlr_rust::coerce_from! { 'input : SQLiteParserContext<'input> }

impl<'input> SQLiteParserContext<'input> for TerminalNode<'input, SQLiteParserContextType> {}
impl<'input> SQLiteParserContext<'input> for ErrorNode<'input, SQLiteParserContextType> {}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn SQLiteParserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn SQLiteListener<'input> + 'input }

pub struct SQLiteParserContextType;
antlr_rust::tid! {SQLiteParserContextType}

impl<'input> ParserNodeType<'input> for SQLiteParserContextType {
    type TF = LocalTokenFactory<'input>;
    type Type = dyn SQLiteParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    type Target = BaseParserType<'input, I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct SQLiteParserExt<'input> {
    _pd: PhantomData<&'input str>,
}

impl<'input> SQLiteParserExt<'input> {}
antlr_rust::tid! { SQLiteParserExt<'a> }

impl<'input> TokenAware<'input> for SQLiteParserExt<'input> {
    type TF = LocalTokenFactory<'input>;
}

impl<'input, I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>>
    ParserRecog<'input, BaseParserType<'input, I>> for SQLiteParserExt<'input>
{
}

impl<'input, I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>>
    Actions<'input, BaseParserType<'input, I>> for SQLiteParserExt<'input>
{
    fn get_grammar_file_name(&self) -> &str {
        "SQLite.g4"
    }

    fn get_rule_names(&self) -> &[&str] {
        &ruleNames
    }

    fn get_vocabulary(&self) -> &dyn Vocabulary {
        &**VOCABULARY
    }
    fn sempred(
        _localctx: Option<&(dyn SQLiteParserContext<'input> + 'input)>,
        rule_index: isize,
        pred_index: isize,
        recog: &mut BaseParserType<'input, I>,
    ) -> bool {
        match rule_index {
            39 => SQLiteParser::<'input, I, _>::expr_sempred(
                _localctx.and_then(|x| x.downcast_ref()),
                pred_index,
                recog,
            ),
            _ => true,
        }
    }
}

impl<'input, I> SQLiteParser<'input, I, DefaultErrorStrategy<'input, SQLiteParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    fn expr_sempred(
        _localctx: Option<&ExprContext<'input>>,
        pred_index: isize,
        recog: &mut <Self as Deref>::Target,
    ) -> bool {
        match pred_index {
            0 => recog.precpred(None, 20),
            1 => recog.precpred(None, 19),
            2 => recog.precpred(None, 18),
            3 => recog.precpred(None, 17),
            4 => recog.precpred(None, 16),
            5 => recog.precpred(None, 15),
            6 => recog.precpred(None, 13),
            7 => recog.precpred(None, 12),
            8 => recog.precpred(None, 5),
            9 => recog.precpred(None, 4),
            10 => recog.precpred(None, 14),
            11 => recog.precpred(None, 8),
            12 => recog.precpred(None, 7),
            13 => recog.precpred(None, 6),
            _ => true,
        }
    }
}
//------------------- parse ----------------
pub type ParseContextAll<'input> = ParseContext<'input>;

pub type ParseContext<'input> = BaseParserRuleContext<'input, ParseContextExt<'input>>;

#[derive(Clone)]
pub struct ParseContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for ParseContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for ParseContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_parse(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_parse(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for ParseContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_parse
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_parse }
}
antlr_rust::tid! {ParseContextExt<'a>}

impl<'input> ParseContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ParseContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ParseContextExt { ph: PhantomData },
        ))
    }
}

pub trait ParseContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<ParseContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(EOF, 0)
    }
    fn sql_stmt_list_all(&self) -> Vec<Rc<Sql_stmt_listContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn sql_stmt_list(&self, i: usize) -> Option<Rc<Sql_stmt_listContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn error_all(&self) -> Vec<Rc<ErrorContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn error(&self, i: usize) -> Option<Rc<ErrorContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> ParseContextAttrs<'input> for ParseContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn parse(&mut self) -> Result<Rc<ParseContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ParseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_parse);
        let mut _localctx: Rc<ParseContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(170);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while (((_la) & !0x3f) == 0
                    && ((1usize << _la)
                        & ((1usize << SCOL) | (1usize << K_ALTER) | (1usize << K_ANALYZE)))
                        != 0)
                    || (((_la - 35) & !0x3f) == 0
                        && ((1usize << (_la - 35))
                            & ((1usize << (K_ATTACH - 35))
                                | (1usize << (K_BEGIN - 35))
                                | (1usize << (K_COMMIT - 35))
                                | (1usize << (K_CREATE - 35))
                                | (1usize << (K_DELETE - 35))
                                | (1usize << (K_DETACH - 35))
                                | (1usize << (K_DROP - 35))
                                | (1usize << (K_END - 35))))
                            != 0)
                    || _la == K_EXPLAIN
                    || _la == K_INSERT
                    || (((_la - 112) & !0x3f) == 0
                        && ((1usize << (_la - 112))
                            & ((1usize << (K_PRAGMA - 112))
                                | (1usize << (K_REINDEX - 112))
                                | (1usize << (K_RELEASE - 112))
                                | (1usize << (K_REPLACE - 112))
                                | (1usize << (K_ROLLBACK - 112))
                                | (1usize << (K_SAVEPOINT - 112))
                                | (1usize << (K_SELECT - 112))
                                | (1usize << (K_UPDATE - 112))
                                | (1usize << (K_VACUUM - 112))
                                | (1usize << (K_VALUES - 112))))
                            != 0)
                    || _la == K_WITH
                    || _la == UNEXPECTED_CHAR
                {
                    {
                        recog.base.set_state(168);
                        recog.err_handler.sync(&mut recog.base)?;
                        match recog.base.input.la(1) {
                            SCOL | K_ALTER | K_ANALYZE | K_ATTACH | K_BEGIN | K_COMMIT
                            | K_CREATE | K_DELETE | K_DETACH | K_DROP | K_END | K_EXPLAIN
                            | K_INSERT | K_PRAGMA | K_REINDEX | K_RELEASE | K_REPLACE
                            | K_ROLLBACK | K_SAVEPOINT | K_SELECT | K_UPDATE | K_VACUUM
                            | K_VALUES | K_WITH => {
                                {
                                    /*InvokeRule sql_stmt_list*/
                                    recog.base.set_state(166);
                                    recog.sql_stmt_list()?;
                                }
                            }

                            UNEXPECTED_CHAR => {
                                {
                                    /*InvokeRule error*/
                                    recog.base.set_state(167);
                                    recog.error()?;
                                }
                            }

                            _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                                &mut recog.base,
                            )))?,
                        }
                    }
                    recog.base.set_state(172);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(173);
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
//------------------- error ----------------
pub type ErrorContextAll<'input> = ErrorContext<'input>;

pub type ErrorContext<'input> = BaseParserRuleContext<'input, ErrorContextExt<'input>>;

#[derive(Clone)]
pub struct ErrorContextExt<'input> {
    pub UNEXPECTED_CHAR: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for ErrorContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for ErrorContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_error(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_error(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for ErrorContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_error
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_error }
}
antlr_rust::tid! {ErrorContextExt<'a>}

impl<'input> ErrorContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ErrorContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ErrorContextExt {
                UNEXPECTED_CHAR: None,
                ph: PhantomData,
            },
        ))
    }
}

pub trait ErrorContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<ErrorContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token UNEXPECTED_CHAR
    /// Returns `None` if there is no child corresponding to token UNEXPECTED_CHAR
    fn UNEXPECTED_CHAR(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(UNEXPECTED_CHAR, 0)
    }
}

impl<'input> ErrorContextAttrs<'input> for ErrorContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn error(&mut self) -> Result<Rc<ErrorContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ErrorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_error);
        let mut _localctx: Rc<ErrorContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(175);
                let tmp = recog
                    .base
                    .match_token(UNEXPECTED_CHAR, &mut recog.err_handler)?;
                cast_mut::<_, ErrorContext>(&mut _localctx).UNEXPECTED_CHAR = Some(tmp.clone());

                // throw new RuntimeException("UNEXPECTED_CHAR=" + if let Some(it) = &(cast::<_,ErrorContext >(&*_localctx))
                // .UNEXPECTED_CHAR { it.get_text() } else { "null" } );
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
//------------------- sql_stmt_list ----------------
pub type Sql_stmt_listContextAll<'input> = Sql_stmt_listContext<'input>;

pub type Sql_stmt_listContext<'input> =
    BaseParserRuleContext<'input, Sql_stmt_listContextExt<'input>>;

#[derive(Clone)]
pub struct Sql_stmt_listContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Sql_stmt_listContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Sql_stmt_listContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_sql_stmt_list(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_sql_stmt_list(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Sql_stmt_listContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_sql_stmt_list
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_sql_stmt_list }
}
antlr_rust::tid! {Sql_stmt_listContextExt<'a>}

impl<'input> Sql_stmt_listContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Sql_stmt_listContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Sql_stmt_listContextExt { ph: PhantomData },
        ))
    }
}

pub trait Sql_stmt_listContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Sql_stmt_listContextExt<'input>>
{
    fn sql_stmt_all(&self) -> Vec<Rc<Sql_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn sql_stmt(&self, i: usize) -> Option<Rc<Sql_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SCOL in current rule
    fn SCOL_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SCOL, starting from 0.
    /// Returns `None` if number of children corresponding to token SCOL is less or equal than `i`.
    fn SCOL(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SCOL, i)
    }
}

impl<'input> Sql_stmt_listContextAttrs<'input> for Sql_stmt_listContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn sql_stmt_list(&mut self) -> Result<Rc<Sql_stmt_listContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Sql_stmt_listContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 4, RULE_sql_stmt_list);
        let mut _localctx: Rc<Sql_stmt_listContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(181);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == SCOL {
                    {
                        {
                            recog.base.set_state(178);
                            recog.base.match_token(SCOL, &mut recog.err_handler)?;
                        }
                    }
                    recog.base.set_state(183);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                /*InvokeRule sql_stmt*/
                recog.base.set_state(184);
                recog.sql_stmt()?;

                recog.base.set_state(193);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(4, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            {
                                recog.base.set_state(186);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                                loop {
                                    {
                                        {
                                            recog.base.set_state(185);
                                            recog.base.match_token(SCOL, &mut recog.err_handler)?;
                                        }
                                    }
                                    recog.base.set_state(188);
                                    recog.err_handler.sync(&mut recog.base)?;
                                    _la = recog.base.input.la(1);
                                    if _la != SCOL {
                                        break;
                                    }
                                }
                                /*InvokeRule sql_stmt*/
                                recog.base.set_state(190);
                                recog.sql_stmt()?;
                            }
                        }
                    }
                    recog.base.set_state(195);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(4, &mut recog.base)?;
                }
                recog.base.set_state(199);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(5, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            {
                                recog.base.set_state(196);
                                recog.base.match_token(SCOL, &mut recog.err_handler)?;
                            }
                        }
                    }
                    recog.base.set_state(201);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(5, &mut recog.base)?;
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
//------------------- sql_stmt ----------------
pub type Sql_stmtContextAll<'input> = Sql_stmtContext<'input>;

pub type Sql_stmtContext<'input> = BaseParserRuleContext<'input, Sql_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct Sql_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Sql_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Sql_stmtContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_sql_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_sql_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Sql_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_sql_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_sql_stmt }
}
antlr_rust::tid! {Sql_stmtContextExt<'a>}

impl<'input> Sql_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Sql_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Sql_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait Sql_stmtContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Sql_stmtContextExt<'input>>
{
    fn alter_table_stmt(&self) -> Option<Rc<Alter_table_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn analyze_stmt(&self) -> Option<Rc<Analyze_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn attach_stmt(&self) -> Option<Rc<Attach_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn begin_stmt(&self) -> Option<Rc<Begin_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn commit_stmt(&self) -> Option<Rc<Commit_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn compound_select_stmt(&self) -> Option<Rc<Compound_select_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn create_index_stmt(&self) -> Option<Rc<Create_index_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn create_table_stmt(&self) -> Option<Rc<Create_table_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn create_trigger_stmt(&self) -> Option<Rc<Create_trigger_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn create_view_stmt(&self) -> Option<Rc<Create_view_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn create_virtual_table_stmt(&self) -> Option<Rc<Create_virtual_table_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn delete_stmt(&self) -> Option<Rc<Delete_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn delete_stmt_limited(&self) -> Option<Rc<Delete_stmt_limitedContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn detach_stmt(&self) -> Option<Rc<Detach_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn drop_index_stmt(&self) -> Option<Rc<Drop_index_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn drop_table_stmt(&self) -> Option<Rc<Drop_table_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn drop_trigger_stmt(&self) -> Option<Rc<Drop_trigger_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn drop_view_stmt(&self) -> Option<Rc<Drop_view_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn factored_select_stmt(&self) -> Option<Rc<Factored_select_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn insert_stmt(&self) -> Option<Rc<Insert_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn pragma_stmt(&self) -> Option<Rc<Pragma_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn reindex_stmt(&self) -> Option<Rc<Reindex_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn release_stmt(&self) -> Option<Rc<Release_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn rollback_stmt(&self) -> Option<Rc<Rollback_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn savepoint_stmt(&self) -> Option<Rc<Savepoint_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn simple_select_stmt(&self) -> Option<Rc<Simple_select_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn select_stmt(&self) -> Option<Rc<Select_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn update_stmt(&self) -> Option<Rc<Update_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn update_stmt_limited(&self) -> Option<Rc<Update_stmt_limitedContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn vacuum_stmt(&self) -> Option<Rc<Vacuum_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_EXPLAIN
    /// Returns `None` if there is no child corresponding to token K_EXPLAIN
    fn K_EXPLAIN(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_EXPLAIN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_QUERY
    /// Returns `None` if there is no child corresponding to token K_QUERY
    fn K_QUERY(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_QUERY, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_PLAN
    /// Returns `None` if there is no child corresponding to token K_PLAN
    fn K_PLAN(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_PLAN, 0)
    }
}

impl<'input> Sql_stmtContextAttrs<'input> for Sql_stmtContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn sql_stmt(&mut self) -> Result<Rc<Sql_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Sql_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_sql_stmt);
        let mut _localctx: Rc<Sql_stmtContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(207);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == K_EXPLAIN {
                    {
                        recog.base.set_state(202);
                        recog.base.match_token(K_EXPLAIN, &mut recog.err_handler)?;

                        recog.base.set_state(205);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == K_QUERY {
                            {
                                recog.base.set_state(203);
                                recog.base.match_token(K_QUERY, &mut recog.err_handler)?;

                                recog.base.set_state(204);
                                recog.base.match_token(K_PLAN, &mut recog.err_handler)?;
                            }
                        }
                    }
                }

                recog.base.set_state(239);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(8, &mut recog.base)? {
                    1 => {
                        {
                            /*InvokeRule alter_table_stmt*/
                            recog.base.set_state(209);
                            recog.alter_table_stmt()?;
                        }
                    }
                    2 => {
                        {
                            /*InvokeRule analyze_stmt*/
                            recog.base.set_state(210);
                            recog.analyze_stmt()?;
                        }
                    }
                    3 => {
                        {
                            /*InvokeRule attach_stmt*/
                            recog.base.set_state(211);
                            recog.attach_stmt()?;
                        }
                    }
                    4 => {
                        {
                            /*InvokeRule begin_stmt*/
                            recog.base.set_state(212);
                            recog.begin_stmt()?;
                        }
                    }
                    5 => {
                        {
                            /*InvokeRule commit_stmt*/
                            recog.base.set_state(213);
                            recog.commit_stmt()?;
                        }
                    }
                    6 => {
                        {
                            /*InvokeRule compound_select_stmt*/
                            recog.base.set_state(214);
                            recog.compound_select_stmt()?;
                        }
                    }
                    7 => {
                        {
                            /*InvokeRule create_index_stmt*/
                            recog.base.set_state(215);
                            recog.create_index_stmt()?;
                        }
                    }
                    8 => {
                        {
                            /*InvokeRule create_table_stmt*/
                            recog.base.set_state(216);
                            recog.create_table_stmt()?;
                        }
                    }
                    9 => {
                        {
                            /*InvokeRule create_trigger_stmt*/
                            recog.base.set_state(217);
                            recog.create_trigger_stmt()?;
                        }
                    }
                    10 => {
                        {
                            /*InvokeRule create_view_stmt*/
                            recog.base.set_state(218);
                            recog.create_view_stmt()?;
                        }
                    }
                    11 => {
                        {
                            /*InvokeRule create_virtual_table_stmt*/
                            recog.base.set_state(219);
                            recog.create_virtual_table_stmt()?;
                        }
                    }
                    12 => {
                        {
                            /*InvokeRule delete_stmt*/
                            recog.base.set_state(220);
                            recog.delete_stmt()?;
                        }
                    }
                    13 => {
                        {
                            /*InvokeRule delete_stmt_limited*/
                            recog.base.set_state(221);
                            recog.delete_stmt_limited()?;
                        }
                    }
                    14 => {
                        {
                            /*InvokeRule detach_stmt*/
                            recog.base.set_state(222);
                            recog.detach_stmt()?;
                        }
                    }
                    15 => {
                        {
                            /*InvokeRule drop_index_stmt*/
                            recog.base.set_state(223);
                            recog.drop_index_stmt()?;
                        }
                    }
                    16 => {
                        {
                            /*InvokeRule drop_table_stmt*/
                            recog.base.set_state(224);
                            recog.drop_table_stmt()?;
                        }
                    }
                    17 => {
                        {
                            /*InvokeRule drop_trigger_stmt*/
                            recog.base.set_state(225);
                            recog.drop_trigger_stmt()?;
                        }
                    }
                    18 => {
                        {
                            /*InvokeRule drop_view_stmt*/
                            recog.base.set_state(226);
                            recog.drop_view_stmt()?;
                        }
                    }
                    19 => {
                        {
                            /*InvokeRule factored_select_stmt*/
                            recog.base.set_state(227);
                            recog.factored_select_stmt()?;
                        }
                    }
                    20 => {
                        {
                            /*InvokeRule insert_stmt*/
                            recog.base.set_state(228);
                            recog.insert_stmt()?;
                        }
                    }
                    21 => {
                        {
                            /*InvokeRule pragma_stmt*/
                            recog.base.set_state(229);
                            recog.pragma_stmt()?;
                        }
                    }
                    22 => {
                        {
                            /*InvokeRule reindex_stmt*/
                            recog.base.set_state(230);
                            recog.reindex_stmt()?;
                        }
                    }
                    23 => {
                        {
                            /*InvokeRule release_stmt*/
                            recog.base.set_state(231);
                            recog.release_stmt()?;
                        }
                    }
                    24 => {
                        {
                            /*InvokeRule rollback_stmt*/
                            recog.base.set_state(232);
                            recog.rollback_stmt()?;
                        }
                    }
                    25 => {
                        {
                            /*InvokeRule savepoint_stmt*/
                            recog.base.set_state(233);
                            recog.savepoint_stmt()?;
                        }
                    }
                    26 => {
                        {
                            /*InvokeRule simple_select_stmt*/
                            recog.base.set_state(234);
                            recog.simple_select_stmt()?;
                        }
                    }
                    27 => {
                        {
                            /*InvokeRule select_stmt*/
                            recog.base.set_state(235);
                            recog.select_stmt()?;
                        }
                    }
                    28 => {
                        {
                            /*InvokeRule update_stmt*/
                            recog.base.set_state(236);
                            recog.update_stmt()?;
                        }
                    }
                    29 => {
                        {
                            /*InvokeRule update_stmt_limited*/
                            recog.base.set_state(237);
                            recog.update_stmt_limited()?;
                        }
                    }
                    30 => {
                        {
                            /*InvokeRule vacuum_stmt*/
                            recog.base.set_state(238);
                            recog.vacuum_stmt()?;
                        }
                    }

                    _ => {}
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
//------------------- alter_table_stmt ----------------
pub type Alter_table_stmtContextAll<'input> = Alter_table_stmtContext<'input>;

pub type Alter_table_stmtContext<'input> =
    BaseParserRuleContext<'input, Alter_table_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct Alter_table_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Alter_table_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Alter_table_stmtContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_alter_table_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_alter_table_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Alter_table_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_alter_table_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_alter_table_stmt }
}
antlr_rust::tid! {Alter_table_stmtContextExt<'a>}

impl<'input> Alter_table_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Alter_table_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Alter_table_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait Alter_table_stmtContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Alter_table_stmtContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token K_ALTER
    /// Returns `None` if there is no child corresponding to token K_ALTER
    fn K_ALTER(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ALTER, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_TABLE
    /// Returns `None` if there is no child corresponding to token K_TABLE
    fn K_TABLE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_TABLE, 0)
    }
    fn table_name(&self) -> Option<Rc<Table_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_RENAME
    /// Returns `None` if there is no child corresponding to token K_RENAME
    fn K_RENAME(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_RENAME, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_TO
    /// Returns `None` if there is no child corresponding to token K_TO
    fn K_TO(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_TO, 0)
    }
    fn new_table_name(&self) -> Option<Rc<New_table_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_ADD
    /// Returns `None` if there is no child corresponding to token K_ADD
    fn K_ADD(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ADD, 0)
    }
    fn column_def(&self) -> Option<Rc<Column_defContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn database_name(&self) -> Option<Rc<Database_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token DOT
    /// Returns `None` if there is no child corresponding to token DOT
    fn DOT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DOT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_COLUMN
    /// Returns `None` if there is no child corresponding to token K_COLUMN
    fn K_COLUMN(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_COLUMN, 0)
    }
}

impl<'input> Alter_table_stmtContextAttrs<'input> for Alter_table_stmtContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn alter_table_stmt(
        &mut self,
    ) -> Result<Rc<Alter_table_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Alter_table_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 8, RULE_alter_table_stmt);
        let mut _localctx: Rc<Alter_table_stmtContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(241);
                recog.base.match_token(K_ALTER, &mut recog.err_handler)?;

                recog.base.set_state(242);
                recog.base.match_token(K_TABLE, &mut recog.err_handler)?;

                recog.base.set_state(246);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(9, &mut recog.base)? {
                    x if x == 1 => {
                        {
                            /*InvokeRule database_name*/
                            recog.base.set_state(243);
                            recog.database_name()?;

                            recog.base.set_state(244);
                            recog.base.match_token(DOT, &mut recog.err_handler)?;
                        }
                    }

                    _ => {}
                }
                /*InvokeRule table_name*/
                recog.base.set_state(248);
                recog.table_name()?;

                recog.base.set_state(257);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.base.input.la(1) {
                    K_RENAME => {
                        {
                            recog.base.set_state(249);
                            recog.base.match_token(K_RENAME, &mut recog.err_handler)?;

                            recog.base.set_state(250);
                            recog.base.match_token(K_TO, &mut recog.err_handler)?;

                            /*InvokeRule new_table_name*/
                            recog.base.set_state(251);
                            recog.new_table_name()?;
                        }
                    }

                    K_ADD => {
                        {
                            recog.base.set_state(252);
                            recog.base.match_token(K_ADD, &mut recog.err_handler)?;

                            recog.base.set_state(254);
                            recog.err_handler.sync(&mut recog.base)?;
                            match recog.interpreter.adaptive_predict(10, &mut recog.base)? {
                                x if x == 1 => {
                                    recog.base.set_state(253);
                                    recog.base.match_token(K_COLUMN, &mut recog.err_handler)?;
                                }

                                _ => {}
                            }
                            /*InvokeRule column_def*/
                            recog.base.set_state(256);
                            recog.column_def()?;
                        }
                    }

                    _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                        &mut recog.base,
                    )))?,
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
//------------------- analyze_stmt ----------------
pub type Analyze_stmtContextAll<'input> = Analyze_stmtContext<'input>;

pub type Analyze_stmtContext<'input> =
    BaseParserRuleContext<'input, Analyze_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct Analyze_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Analyze_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Analyze_stmtContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_analyze_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_analyze_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Analyze_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_analyze_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_analyze_stmt }
}
antlr_rust::tid! {Analyze_stmtContextExt<'a>}

impl<'input> Analyze_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Analyze_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Analyze_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait Analyze_stmtContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Analyze_stmtContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token K_ANALYZE
    /// Returns `None` if there is no child corresponding to token K_ANALYZE
    fn K_ANALYZE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ANALYZE, 0)
    }
    fn database_name(&self) -> Option<Rc<Database_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn table_or_index_name(&self) -> Option<Rc<Table_or_index_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token DOT
    /// Returns `None` if there is no child corresponding to token DOT
    fn DOT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DOT, 0)
    }
}

impl<'input> Analyze_stmtContextAttrs<'input> for Analyze_stmtContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn analyze_stmt(&mut self) -> Result<Rc<Analyze_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Analyze_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 10, RULE_analyze_stmt);
        let mut _localctx: Rc<Analyze_stmtContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(259);
                recog.base.match_token(K_ANALYZE, &mut recog.err_handler)?;

                recog.base.set_state(266);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(12, &mut recog.base)? {
                    x if x == 1 => {
                        {
                            /*InvokeRule database_name*/
                            recog.base.set_state(260);
                            recog.database_name()?;
                        }
                    }

                    x if x == 2 => {
                        {
                            /*InvokeRule table_or_index_name*/
                            recog.base.set_state(261);
                            recog.table_or_index_name()?;
                        }
                    }

                    x if x == 3 => {
                        {
                            /*InvokeRule database_name*/
                            recog.base.set_state(262);
                            recog.database_name()?;

                            recog.base.set_state(263);
                            recog.base.match_token(DOT, &mut recog.err_handler)?;

                            /*InvokeRule table_or_index_name*/
                            recog.base.set_state(264);
                            recog.table_or_index_name()?;
                        }
                    }

                    _ => {}
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
//------------------- attach_stmt ----------------
pub type Attach_stmtContextAll<'input> = Attach_stmtContext<'input>;

pub type Attach_stmtContext<'input> = BaseParserRuleContext<'input, Attach_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct Attach_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Attach_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Attach_stmtContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_attach_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_attach_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Attach_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_attach_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_attach_stmt }
}
antlr_rust::tid! {Attach_stmtContextExt<'a>}

impl<'input> Attach_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Attach_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Attach_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait Attach_stmtContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Attach_stmtContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token K_ATTACH
    /// Returns `None` if there is no child corresponding to token K_ATTACH
    fn K_ATTACH(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ATTACH, 0)
    }
    fn expr(&self) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_AS
    /// Returns `None` if there is no child corresponding to token K_AS
    fn K_AS(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_AS, 0)
    }
    fn database_name(&self) -> Option<Rc<Database_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_DATABASE
    /// Returns `None` if there is no child corresponding to token K_DATABASE
    fn K_DATABASE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_DATABASE, 0)
    }
}

impl<'input> Attach_stmtContextAttrs<'input> for Attach_stmtContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn attach_stmt(&mut self) -> Result<Rc<Attach_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Attach_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 12, RULE_attach_stmt);
        let mut _localctx: Rc<Attach_stmtContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(268);
                recog.base.match_token(K_ATTACH, &mut recog.err_handler)?;

                recog.base.set_state(270);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(13, &mut recog.base)? {
                    x if x == 1 => {
                        recog.base.set_state(269);
                        recog.base.match_token(K_DATABASE, &mut recog.err_handler)?;
                    }

                    _ => {}
                }
                /*InvokeRule expr*/
                recog.base.set_state(272);
                recog.expr_rec(0)?;

                recog.base.set_state(273);
                recog.base.match_token(K_AS, &mut recog.err_handler)?;

                /*InvokeRule database_name*/
                recog.base.set_state(274);
                recog.database_name()?;
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
//------------------- begin_stmt ----------------
pub type Begin_stmtContextAll<'input> = Begin_stmtContext<'input>;

pub type Begin_stmtContext<'input> = BaseParserRuleContext<'input, Begin_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct Begin_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Begin_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Begin_stmtContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_begin_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_begin_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Begin_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_begin_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_begin_stmt }
}
antlr_rust::tid! {Begin_stmtContextExt<'a>}

impl<'input> Begin_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Begin_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Begin_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait Begin_stmtContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Begin_stmtContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token K_BEGIN
    /// Returns `None` if there is no child corresponding to token K_BEGIN
    fn K_BEGIN(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_BEGIN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_TRANSACTION
    /// Returns `None` if there is no child corresponding to token K_TRANSACTION
    fn K_TRANSACTION(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_TRANSACTION, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_DEFERRED
    /// Returns `None` if there is no child corresponding to token K_DEFERRED
    fn K_DEFERRED(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_DEFERRED, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_IMMEDIATE
    /// Returns `None` if there is no child corresponding to token K_IMMEDIATE
    fn K_IMMEDIATE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_IMMEDIATE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_EXCLUSIVE
    /// Returns `None` if there is no child corresponding to token K_EXCLUSIVE
    fn K_EXCLUSIVE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_EXCLUSIVE, 0)
    }
    fn transaction_name(&self) -> Option<Rc<Transaction_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Begin_stmtContextAttrs<'input> for Begin_stmtContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn begin_stmt(&mut self) -> Result<Rc<Begin_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Begin_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 14, RULE_begin_stmt);
        let mut _localctx: Rc<Begin_stmtContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(276);
                recog.base.match_token(K_BEGIN, &mut recog.err_handler)?;

                recog.base.set_state(278);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if ((_la - 58) & !0x3f) == 0
                    && ((1usize << (_la - 58))
                        & ((1usize << (K_DEFERRED - 58))
                            | (1usize << (K_EXCLUSIVE - 58))
                            | (1usize << (K_IMMEDIATE - 58))))
                        != 0
                {
                    {
                        recog.base.set_state(277);
                        _la = recog.base.input.la(1);
                        if !(((_la - 58) & !0x3f) == 0
                            && ((1usize << (_la - 58))
                                & ((1usize << (K_DEFERRED - 58))
                                    | (1usize << (K_EXCLUSIVE - 58))
                                    | (1usize << (K_IMMEDIATE - 58))))
                                != 0)
                        {
                            recog.err_handler.recover_inline(&mut recog.base)?;
                        } else {
                            if recog.base.input.la(1) == TOKEN_EOF {
                                recog.base.matched_eof = true
                            };
                            recog.err_handler.report_match(&mut recog.base);
                            recog.base.consume(&mut recog.err_handler);
                        }
                    }
                }

                recog.base.set_state(284);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == K_TRANSACTION {
                    {
                        recog.base.set_state(280);
                        recog
                            .base
                            .match_token(K_TRANSACTION, &mut recog.err_handler)?;

                        recog.base.set_state(282);
                        recog.err_handler.sync(&mut recog.base)?;
                        match recog.interpreter.adaptive_predict(15, &mut recog.base)? {
                            x if x == 1 => {
                                {
                                    /*InvokeRule transaction_name*/
                                    recog.base.set_state(281);
                                    recog.transaction_name()?;
                                }
                            }

                            _ => {}
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
//------------------- commit_stmt ----------------
pub type Commit_stmtContextAll<'input> = Commit_stmtContext<'input>;

pub type Commit_stmtContext<'input> = BaseParserRuleContext<'input, Commit_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct Commit_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Commit_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Commit_stmtContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_commit_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_commit_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Commit_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_commit_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_commit_stmt }
}
antlr_rust::tid! {Commit_stmtContextExt<'a>}

impl<'input> Commit_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Commit_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Commit_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait Commit_stmtContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Commit_stmtContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token K_COMMIT
    /// Returns `None` if there is no child corresponding to token K_COMMIT
    fn K_COMMIT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_COMMIT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_END
    /// Returns `None` if there is no child corresponding to token K_END
    fn K_END(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_END, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_TRANSACTION
    /// Returns `None` if there is no child corresponding to token K_TRANSACTION
    fn K_TRANSACTION(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_TRANSACTION, 0)
    }
    fn transaction_name(&self) -> Option<Rc<Transaction_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Commit_stmtContextAttrs<'input> for Commit_stmtContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn commit_stmt(&mut self) -> Result<Rc<Commit_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Commit_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 16, RULE_commit_stmt);
        let mut _localctx: Rc<Commit_stmtContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(286);
                _la = recog.base.input.la(1);
                if !(_la == K_COMMIT || _la == K_END) {
                    recog.err_handler.recover_inline(&mut recog.base)?;
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
                recog.base.set_state(291);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == K_TRANSACTION {
                    {
                        recog.base.set_state(287);
                        recog
                            .base
                            .match_token(K_TRANSACTION, &mut recog.err_handler)?;

                        recog.base.set_state(289);
                        recog.err_handler.sync(&mut recog.base)?;
                        match recog.interpreter.adaptive_predict(17, &mut recog.base)? {
                            x if x == 1 => {
                                {
                                    /*InvokeRule transaction_name*/
                                    recog.base.set_state(288);
                                    recog.transaction_name()?;
                                }
                            }

                            _ => {}
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
//------------------- compound_select_stmt ----------------
pub type Compound_select_stmtContextAll<'input> = Compound_select_stmtContext<'input>;

pub type Compound_select_stmtContext<'input> =
    BaseParserRuleContext<'input, Compound_select_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct Compound_select_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Compound_select_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a>
    for Compound_select_stmtContext<'input>
{
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_compound_select_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_compound_select_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Compound_select_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_compound_select_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_compound_select_stmt }
}
antlr_rust::tid! {Compound_select_stmtContextExt<'a>}

impl<'input> Compound_select_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Compound_select_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Compound_select_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait Compound_select_stmtContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Compound_select_stmtContextExt<'input>>
{
    fn select_core_all(&self) -> Vec<Rc<Select_coreContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn select_core(&self, i: usize) -> Option<Rc<Select_coreContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn with_clause(&self) -> Option<Rc<With_clauseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_ORDER
    /// Returns `None` if there is no child corresponding to token K_ORDER
    fn K_ORDER(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ORDER, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_BY
    /// Returns `None` if there is no child corresponding to token K_BY
    fn K_BY(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_BY, 0)
    }
    fn ordering_term_all(&self) -> Vec<Rc<Ordering_termContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn ordering_term(&self, i: usize) -> Option<Rc<Ordering_termContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token K_LIMIT
    /// Returns `None` if there is no child corresponding to token K_LIMIT
    fn K_LIMIT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_LIMIT, 0)
    }
    fn expr_all(&self) -> Vec<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token K_UNION in current rule
    fn K_UNION_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token K_UNION, starting from 0.
    /// Returns `None` if number of children corresponding to token K_UNION is less or equal than `i`.
    fn K_UNION(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_UNION, i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token K_INTERSECT in current rule
    fn K_INTERSECT_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token K_INTERSECT, starting from 0.
    /// Returns `None` if number of children corresponding to token K_INTERSECT is less or equal than `i`.
    fn K_INTERSECT(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_INTERSECT, i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token K_EXCEPT in current rule
    fn K_EXCEPT_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token K_EXCEPT, starting from 0.
    /// Returns `None` if number of children corresponding to token K_EXCEPT is less or equal than `i`.
    fn K_EXCEPT(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_EXCEPT, i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
    /// Retrieves first TerminalNode corresponding to token K_OFFSET
    /// Returns `None` if there is no child corresponding to token K_OFFSET
    fn K_OFFSET(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_OFFSET, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token K_ALL in current rule
    fn K_ALL_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token K_ALL, starting from 0.
    /// Returns `None` if number of children corresponding to token K_ALL is less or equal than `i`.
    fn K_ALL(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ALL, i)
    }
}

impl<'input> Compound_select_stmtContextAttrs<'input> for Compound_select_stmtContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn compound_select_stmt(
        &mut self,
    ) -> Result<Rc<Compound_select_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Compound_select_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 18, RULE_compound_select_stmt);
        let mut _localctx: Rc<Compound_select_stmtContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(294);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == K_WITH {
                    {
                        /*InvokeRule with_clause*/
                        recog.base.set_state(293);
                        recog.with_clause()?;
                    }
                }

                /*InvokeRule select_core*/
                recog.base.set_state(296);
                recog.select_core()?;

                recog.base.set_state(306);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                loop {
                    {
                        {
                            recog.base.set_state(303);
                            recog.err_handler.sync(&mut recog.base)?;
                            match recog.base.input.la(1) {
                                K_UNION => {
                                    recog.base.set_state(297);
                                    recog.base.match_token(K_UNION, &mut recog.err_handler)?;

                                    recog.base.set_state(299);
                                    recog.err_handler.sync(&mut recog.base)?;
                                    _la = recog.base.input.la(1);
                                    if _la == K_ALL {
                                        {
                                            recog.base.set_state(298);
                                            recog
                                                .base
                                                .match_token(K_ALL, &mut recog.err_handler)?;
                                        }
                                    }
                                }

                                K_INTERSECT => {
                                    recog.base.set_state(301);
                                    recog
                                        .base
                                        .match_token(K_INTERSECT, &mut recog.err_handler)?;
                                }

                                K_EXCEPT => {
                                    recog.base.set_state(302);
                                    recog.base.match_token(K_EXCEPT, &mut recog.err_handler)?;
                                }

                                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                                    &mut recog.base,
                                )))?,
                            }
                            /*InvokeRule select_core*/
                            recog.base.set_state(305);
                            recog.select_core()?;
                        }
                    }
                    recog.base.set_state(308);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                    if !(_la == K_EXCEPT || _la == K_INTERSECT || _la == K_UNION) {
                        break;
                    }
                }
                recog.base.set_state(320);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == K_ORDER {
                    {
                        recog.base.set_state(310);
                        recog.base.match_token(K_ORDER, &mut recog.err_handler)?;

                        recog.base.set_state(311);
                        recog.base.match_token(K_BY, &mut recog.err_handler)?;

                        /*InvokeRule ordering_term*/
                        recog.base.set_state(312);
                        recog.ordering_term()?;

                        recog.base.set_state(317);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        while _la == COMMA {
                            {
                                {
                                    recog.base.set_state(313);
                                    recog.base.match_token(COMMA, &mut recog.err_handler)?;

                                    /*InvokeRule ordering_term*/
                                    recog.base.set_state(314);
                                    recog.ordering_term()?;
                                }
                            }
                            recog.base.set_state(319);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                        }
                    }
                }

                recog.base.set_state(328);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == K_LIMIT {
                    {
                        recog.base.set_state(322);
                        recog.base.match_token(K_LIMIT, &mut recog.err_handler)?;

                        /*InvokeRule expr*/
                        recog.base.set_state(323);
                        recog.expr_rec(0)?;

                        recog.base.set_state(326);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == COMMA || _la == K_OFFSET {
                            {
                                recog.base.set_state(324);
                                _la = recog.base.input.la(1);
                                if !(_la == COMMA || _la == K_OFFSET) {
                                    recog.err_handler.recover_inline(&mut recog.base)?;
                                } else {
                                    if recog.base.input.la(1) == TOKEN_EOF {
                                        recog.base.matched_eof = true
                                    };
                                    recog.err_handler.report_match(&mut recog.base);
                                    recog.base.consume(&mut recog.err_handler);
                                }
                                /*InvokeRule expr*/
                                recog.base.set_state(325);
                                recog.expr_rec(0)?;
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
//------------------- create_index_stmt ----------------
pub type Create_index_stmtContextAll<'input> = Create_index_stmtContext<'input>;

pub type Create_index_stmtContext<'input> =
    BaseParserRuleContext<'input, Create_index_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct Create_index_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Create_index_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Create_index_stmtContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_create_index_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_create_index_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Create_index_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_create_index_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_create_index_stmt }
}
antlr_rust::tid! {Create_index_stmtContextExt<'a>}

impl<'input> Create_index_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Create_index_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Create_index_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait Create_index_stmtContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Create_index_stmtContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token K_CREATE
    /// Returns `None` if there is no child corresponding to token K_CREATE
    fn K_CREATE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_CREATE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_INDEX
    /// Returns `None` if there is no child corresponding to token K_INDEX
    fn K_INDEX(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_INDEX, 0)
    }
    fn index_name(&self) -> Option<Rc<Index_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_ON
    /// Returns `None` if there is no child corresponding to token K_ON
    fn K_ON(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ON, 0)
    }
    fn table_name(&self) -> Option<Rc<Table_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token OPEN_PAR
    /// Returns `None` if there is no child corresponding to token OPEN_PAR
    fn OPEN_PAR(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPEN_PAR, 0)
    }
    fn indexed_column_all(&self) -> Vec<Rc<Indexed_columnContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn indexed_column(&self, i: usize) -> Option<Rc<Indexed_columnContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token CLOSE_PAR
    /// Returns `None` if there is no child corresponding to token CLOSE_PAR
    fn CLOSE_PAR(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CLOSE_PAR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_UNIQUE
    /// Returns `None` if there is no child corresponding to token K_UNIQUE
    fn K_UNIQUE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_UNIQUE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_IF
    /// Returns `None` if there is no child corresponding to token K_IF
    fn K_IF(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_IF, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_NOT
    /// Returns `None` if there is no child corresponding to token K_NOT
    fn K_NOT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_NOT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_EXISTS
    /// Returns `None` if there is no child corresponding to token K_EXISTS
    fn K_EXISTS(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_EXISTS, 0)
    }
    fn database_name(&self) -> Option<Rc<Database_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token DOT
    /// Returns `None` if there is no child corresponding to token DOT
    fn DOT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DOT, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
    /// Retrieves first TerminalNode corresponding to token K_WHERE
    /// Returns `None` if there is no child corresponding to token K_WHERE
    fn K_WHERE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_WHERE, 0)
    }
    fn expr(&self) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Create_index_stmtContextAttrs<'input> for Create_index_stmtContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn create_index_stmt(
        &mut self,
    ) -> Result<Rc<Create_index_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Create_index_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 20, RULE_create_index_stmt);
        let mut _localctx: Rc<Create_index_stmtContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(330);
                recog.base.match_token(K_CREATE, &mut recog.err_handler)?;

                recog.base.set_state(332);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == K_UNIQUE {
                    {
                        recog.base.set_state(331);
                        recog.base.match_token(K_UNIQUE, &mut recog.err_handler)?;
                    }
                }

                recog.base.set_state(334);
                recog.base.match_token(K_INDEX, &mut recog.err_handler)?;

                recog.base.set_state(338);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(28, &mut recog.base)? {
                    x if x == 1 => {
                        recog.base.set_state(335);
                        recog.base.match_token(K_IF, &mut recog.err_handler)?;

                        recog.base.set_state(336);
                        recog.base.match_token(K_NOT, &mut recog.err_handler)?;

                        recog.base.set_state(337);
                        recog.base.match_token(K_EXISTS, &mut recog.err_handler)?;
                    }

                    _ => {}
                }
                recog.base.set_state(343);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(29, &mut recog.base)? {
                    x if x == 1 => {
                        {
                            /*InvokeRule database_name*/
                            recog.base.set_state(340);
                            recog.database_name()?;

                            recog.base.set_state(341);
                            recog.base.match_token(DOT, &mut recog.err_handler)?;
                        }
                    }

                    _ => {}
                }
                /*InvokeRule index_name*/
                recog.base.set_state(345);
                recog.index_name()?;

                recog.base.set_state(346);
                recog.base.match_token(K_ON, &mut recog.err_handler)?;

                /*InvokeRule table_name*/
                recog.base.set_state(347);
                recog.table_name()?;

                recog.base.set_state(348);
                recog.base.match_token(OPEN_PAR, &mut recog.err_handler)?;

                /*InvokeRule indexed_column*/
                recog.base.set_state(349);
                recog.indexed_column()?;

                recog.base.set_state(354);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == COMMA {
                    {
                        {
                            recog.base.set_state(350);
                            recog.base.match_token(COMMA, &mut recog.err_handler)?;

                            /*InvokeRule indexed_column*/
                            recog.base.set_state(351);
                            recog.indexed_column()?;
                        }
                    }
                    recog.base.set_state(356);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(357);
                recog.base.match_token(CLOSE_PAR, &mut recog.err_handler)?;

                recog.base.set_state(360);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == K_WHERE {
                    {
                        recog.base.set_state(358);
                        recog.base.match_token(K_WHERE, &mut recog.err_handler)?;

                        /*InvokeRule expr*/
                        recog.base.set_state(359);
                        recog.expr_rec(0)?;
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
//------------------- create_table_stmt ----------------
pub type Create_table_stmtContextAll<'input> = Create_table_stmtContext<'input>;

pub type Create_table_stmtContext<'input> =
    BaseParserRuleContext<'input, Create_table_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct Create_table_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Create_table_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Create_table_stmtContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_create_table_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_create_table_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Create_table_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_create_table_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_create_table_stmt }
}
antlr_rust::tid! {Create_table_stmtContextExt<'a>}

impl<'input> Create_table_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Create_table_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Create_table_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait Create_table_stmtContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Create_table_stmtContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token K_CREATE
    /// Returns `None` if there is no child corresponding to token K_CREATE
    fn K_CREATE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_CREATE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_TABLE
    /// Returns `None` if there is no child corresponding to token K_TABLE
    fn K_TABLE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_TABLE, 0)
    }
    fn table_name(&self) -> Option<Rc<Table_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token OPEN_PAR
    /// Returns `None` if there is no child corresponding to token OPEN_PAR
    fn OPEN_PAR(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPEN_PAR, 0)
    }
    fn column_def_all(&self) -> Vec<Rc<Column_defContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn column_def(&self, i: usize) -> Option<Rc<Column_defContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token CLOSE_PAR
    /// Returns `None` if there is no child corresponding to token CLOSE_PAR
    fn CLOSE_PAR(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CLOSE_PAR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_AS
    /// Returns `None` if there is no child corresponding to token K_AS
    fn K_AS(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_AS, 0)
    }
    fn select_stmt(&self) -> Option<Rc<Select_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_IF
    /// Returns `None` if there is no child corresponding to token K_IF
    fn K_IF(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_IF, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_NOT
    /// Returns `None` if there is no child corresponding to token K_NOT
    fn K_NOT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_NOT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_EXISTS
    /// Returns `None` if there is no child corresponding to token K_EXISTS
    fn K_EXISTS(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_EXISTS, 0)
    }
    fn database_name(&self) -> Option<Rc<Database_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token DOT
    /// Returns `None` if there is no child corresponding to token DOT
    fn DOT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DOT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_TEMP
    /// Returns `None` if there is no child corresponding to token K_TEMP
    fn K_TEMP(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_TEMP, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_TEMPORARY
    /// Returns `None` if there is no child corresponding to token K_TEMPORARY
    fn K_TEMPORARY(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_TEMPORARY, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
    fn table_constraint_all(&self) -> Vec<Rc<Table_constraintContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn table_constraint(&self, i: usize) -> Option<Rc<Table_constraintContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token K_WITHOUT
    /// Returns `None` if there is no child corresponding to token K_WITHOUT
    fn K_WITHOUT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_WITHOUT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token IDENTIFIER
    /// Returns `None` if there is no child corresponding to token IDENTIFIER
    fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(IDENTIFIER, 0)
    }
}

impl<'input> Create_table_stmtContextAttrs<'input> for Create_table_stmtContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn create_table_stmt(
        &mut self,
    ) -> Result<Rc<Create_table_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Create_table_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 22, RULE_create_table_stmt);
        let mut _localctx: Rc<Create_table_stmtContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(362);
                recog.base.match_token(K_CREATE, &mut recog.err_handler)?;

                recog.base.set_state(364);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == K_TEMP || _la == K_TEMPORARY {
                    {
                        recog.base.set_state(363);
                        _la = recog.base.input.la(1);
                        if !(_la == K_TEMP || _la == K_TEMPORARY) {
                            recog.err_handler.recover_inline(&mut recog.base)?;
                        } else {
                            if recog.base.input.la(1) == TOKEN_EOF {
                                recog.base.matched_eof = true
                            };
                            recog.err_handler.report_match(&mut recog.base);
                            recog.base.consume(&mut recog.err_handler);
                        }
                    }
                }

                recog.base.set_state(366);
                recog.base.match_token(K_TABLE, &mut recog.err_handler)?;

                recog.base.set_state(370);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(33, &mut recog.base)? {
                    x if x == 1 => {
                        recog.base.set_state(367);
                        recog.base.match_token(K_IF, &mut recog.err_handler)?;

                        recog.base.set_state(368);
                        recog.base.match_token(K_NOT, &mut recog.err_handler)?;

                        recog.base.set_state(369);
                        recog.base.match_token(K_EXISTS, &mut recog.err_handler)?;
                    }

                    _ => {}
                }
                recog.base.set_state(375);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(34, &mut recog.base)? {
                    x if x == 1 => {
                        {
                            /*InvokeRule database_name*/
                            recog.base.set_state(372);
                            recog.database_name()?;

                            recog.base.set_state(373);
                            recog.base.match_token(DOT, &mut recog.err_handler)?;
                        }
                    }

                    _ => {}
                }
                /*InvokeRule table_name*/
                recog.base.set_state(377);
                recog.table_name()?;

                recog.base.set_state(401);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.base.input.la(1) {
                    OPEN_PAR => {
                        {
                            recog.base.set_state(378);
                            recog.base.match_token(OPEN_PAR, &mut recog.err_handler)?;

                            /*InvokeRule column_def*/
                            recog.base.set_state(379);
                            recog.column_def()?;

                            recog.base.set_state(384);
                            recog.err_handler.sync(&mut recog.base)?;
                            _alt = recog.interpreter.adaptive_predict(35, &mut recog.base)?;
                            while { _alt != 1 && _alt != INVALID_ALT } {
                                if _alt == 1 + 1 {
                                    {
                                        {
                                            recog.base.set_state(380);
                                            recog
                                                .base
                                                .match_token(COMMA, &mut recog.err_handler)?;

                                            /*InvokeRule column_def*/
                                            recog.base.set_state(381);
                                            recog.column_def()?;
                                        }
                                    }
                                }
                                recog.base.set_state(386);
                                recog.err_handler.sync(&mut recog.base)?;
                                _alt = recog.interpreter.adaptive_predict(35, &mut recog.base)?;
                            }
                            recog.base.set_state(391);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            while _la == COMMA {
                                {
                                    {
                                        recog.base.set_state(387);
                                        recog.base.match_token(COMMA, &mut recog.err_handler)?;

                                        /*InvokeRule table_constraint*/
                                        recog.base.set_state(388);
                                        recog.table_constraint()?;
                                    }
                                }
                                recog.base.set_state(393);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                            }
                            recog.base.set_state(394);
                            recog.base.match_token(CLOSE_PAR, &mut recog.err_handler)?;

                            recog.base.set_state(397);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == K_WITHOUT {
                                {
                                    recog.base.set_state(395);
                                    recog.base.match_token(K_WITHOUT, &mut recog.err_handler)?;

                                    recog.base.set_state(396);
                                    recog.base.match_token(IDENTIFIER, &mut recog.err_handler)?;
                                }
                            }
                        }
                    }

                    K_AS => {
                        {
                            recog.base.set_state(399);
                            recog.base.match_token(K_AS, &mut recog.err_handler)?;

                            /*InvokeRule select_stmt*/
                            recog.base.set_state(400);
                            recog.select_stmt()?;
                        }
                    }

                    _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                        &mut recog.base,
                    )))?,
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
//------------------- create_trigger_stmt ----------------
pub type Create_trigger_stmtContextAll<'input> = Create_trigger_stmtContext<'input>;

pub type Create_trigger_stmtContext<'input> =
    BaseParserRuleContext<'input, Create_trigger_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct Create_trigger_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Create_trigger_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a>
    for Create_trigger_stmtContext<'input>
{
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_create_trigger_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_create_trigger_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Create_trigger_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_create_trigger_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_create_trigger_stmt }
}
antlr_rust::tid! {Create_trigger_stmtContextExt<'a>}

impl<'input> Create_trigger_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Create_trigger_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Create_trigger_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait Create_trigger_stmtContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Create_trigger_stmtContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token K_CREATE
    /// Returns `None` if there is no child corresponding to token K_CREATE
    fn K_CREATE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_CREATE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_TRIGGER
    /// Returns `None` if there is no child corresponding to token K_TRIGGER
    fn K_TRIGGER(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_TRIGGER, 0)
    }
    fn trigger_name(&self) -> Option<Rc<Trigger_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_ON
    /// Returns `None` if there is no child corresponding to token K_ON
    fn K_ON(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ON, 0)
    }
    fn table_name(&self) -> Option<Rc<Table_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_BEGIN
    /// Returns `None` if there is no child corresponding to token K_BEGIN
    fn K_BEGIN(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_BEGIN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_END
    /// Returns `None` if there is no child corresponding to token K_END
    fn K_END(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_END, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_DELETE
    /// Returns `None` if there is no child corresponding to token K_DELETE
    fn K_DELETE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_DELETE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_INSERT
    /// Returns `None` if there is no child corresponding to token K_INSERT
    fn K_INSERT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_INSERT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_UPDATE
    /// Returns `None` if there is no child corresponding to token K_UPDATE
    fn K_UPDATE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_UPDATE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_IF
    /// Returns `None` if there is no child corresponding to token K_IF
    fn K_IF(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_IF, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_NOT
    /// Returns `None` if there is no child corresponding to token K_NOT
    fn K_NOT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_NOT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_EXISTS
    /// Returns `None` if there is no child corresponding to token K_EXISTS
    fn K_EXISTS(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_EXISTS, 0)
    }
    fn database_name_all(&self) -> Vec<Rc<Database_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn database_name(&self, i: usize) -> Option<Rc<Database_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token DOT in current rule
    fn DOT_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token DOT, starting from 0.
    /// Returns `None` if number of children corresponding to token DOT is less or equal than `i`.
    fn DOT(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DOT, i)
    }
    /// Retrieves first TerminalNode corresponding to token K_BEFORE
    /// Returns `None` if there is no child corresponding to token K_BEFORE
    fn K_BEFORE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_BEFORE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_AFTER
    /// Returns `None` if there is no child corresponding to token K_AFTER
    fn K_AFTER(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_AFTER, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_INSTEAD
    /// Returns `None` if there is no child corresponding to token K_INSTEAD
    fn K_INSTEAD(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_INSTEAD, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token K_OF in current rule
    fn K_OF_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token K_OF, starting from 0.
    /// Returns `None` if number of children corresponding to token K_OF is less or equal than `i`.
    fn K_OF(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_OF, i)
    }
    /// Retrieves first TerminalNode corresponding to token K_FOR
    /// Returns `None` if there is no child corresponding to token K_FOR
    fn K_FOR(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_FOR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_EACH
    /// Returns `None` if there is no child corresponding to token K_EACH
    fn K_EACH(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_EACH, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_ROW
    /// Returns `None` if there is no child corresponding to token K_ROW
    fn K_ROW(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ROW, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_WHEN
    /// Returns `None` if there is no child corresponding to token K_WHEN
    fn K_WHEN(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_WHEN, 0)
    }
    fn expr(&self) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SCOL in current rule
    fn SCOL_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SCOL, starting from 0.
    /// Returns `None` if number of children corresponding to token SCOL is less or equal than `i`.
    fn SCOL(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SCOL, i)
    }
    /// Retrieves first TerminalNode corresponding to token K_TEMP
    /// Returns `None` if there is no child corresponding to token K_TEMP
    fn K_TEMP(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_TEMP, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_TEMPORARY
    /// Returns `None` if there is no child corresponding to token K_TEMPORARY
    fn K_TEMPORARY(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_TEMPORARY, 0)
    }
    fn column_name_all(&self) -> Vec<Rc<Column_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn column_name(&self, i: usize) -> Option<Rc<Column_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn update_stmt_all(&self) -> Vec<Rc<Update_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn update_stmt(&self, i: usize) -> Option<Rc<Update_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn insert_stmt_all(&self) -> Vec<Rc<Insert_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn insert_stmt(&self, i: usize) -> Option<Rc<Insert_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn delete_stmt_all(&self) -> Vec<Rc<Delete_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn delete_stmt(&self, i: usize) -> Option<Rc<Delete_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn select_stmt_all(&self) -> Vec<Rc<Select_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn select_stmt(&self, i: usize) -> Option<Rc<Select_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
}

impl<'input> Create_trigger_stmtContextAttrs<'input> for Create_trigger_stmtContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn create_trigger_stmt(
        &mut self,
    ) -> Result<Rc<Create_trigger_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Create_trigger_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 24, RULE_create_trigger_stmt);
        let mut _localctx: Rc<Create_trigger_stmtContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(403);
                recog.base.match_token(K_CREATE, &mut recog.err_handler)?;

                recog.base.set_state(405);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == K_TEMP || _la == K_TEMPORARY {
                    {
                        recog.base.set_state(404);
                        _la = recog.base.input.la(1);
                        if !(_la == K_TEMP || _la == K_TEMPORARY) {
                            recog.err_handler.recover_inline(&mut recog.base)?;
                        } else {
                            if recog.base.input.la(1) == TOKEN_EOF {
                                recog.base.matched_eof = true
                            };
                            recog.err_handler.report_match(&mut recog.base);
                            recog.base.consume(&mut recog.err_handler);
                        }
                    }
                }

                recog.base.set_state(407);
                recog.base.match_token(K_TRIGGER, &mut recog.err_handler)?;

                recog.base.set_state(411);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(40, &mut recog.base)? {
                    x if x == 1 => {
                        recog.base.set_state(408);
                        recog.base.match_token(K_IF, &mut recog.err_handler)?;

                        recog.base.set_state(409);
                        recog.base.match_token(K_NOT, &mut recog.err_handler)?;

                        recog.base.set_state(410);
                        recog.base.match_token(K_EXISTS, &mut recog.err_handler)?;
                    }

                    _ => {}
                }
                recog.base.set_state(416);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(41, &mut recog.base)? {
                    x if x == 1 => {
                        {
                            /*InvokeRule database_name*/
                            recog.base.set_state(413);
                            recog.database_name()?;

                            recog.base.set_state(414);
                            recog.base.match_token(DOT, &mut recog.err_handler)?;
                        }
                    }

                    _ => {}
                }
                /*InvokeRule trigger_name*/
                recog.base.set_state(418);
                recog.trigger_name()?;

                recog.base.set_state(423);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.base.input.la(1) {
                    K_BEFORE => {
                        recog.base.set_state(419);
                        recog.base.match_token(K_BEFORE, &mut recog.err_handler)?;
                    }

                    K_AFTER => {
                        recog.base.set_state(420);
                        recog.base.match_token(K_AFTER, &mut recog.err_handler)?;
                    }

                    K_INSTEAD => {
                        recog.base.set_state(421);
                        recog.base.match_token(K_INSTEAD, &mut recog.err_handler)?;

                        recog.base.set_state(422);
                        recog.base.match_token(K_OF, &mut recog.err_handler)?;
                    }

                    K_DELETE | K_INSERT | K_UPDATE => {}

                    _ => {}
                }
                recog.base.set_state(439);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.base.input.la(1) {
                    K_DELETE => {
                        recog.base.set_state(425);
                        recog.base.match_token(K_DELETE, &mut recog.err_handler)?;
                    }

                    K_INSERT => {
                        recog.base.set_state(426);
                        recog.base.match_token(K_INSERT, &mut recog.err_handler)?;
                    }

                    K_UPDATE => {
                        {
                            recog.base.set_state(427);
                            recog.base.match_token(K_UPDATE, &mut recog.err_handler)?;

                            recog.base.set_state(437);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == K_OF {
                                {
                                    recog.base.set_state(428);
                                    recog.base.match_token(K_OF, &mut recog.err_handler)?;

                                    /*InvokeRule column_name*/
                                    recog.base.set_state(429);
                                    recog.column_name()?;

                                    recog.base.set_state(434);
                                    recog.err_handler.sync(&mut recog.base)?;
                                    _la = recog.base.input.la(1);
                                    while _la == COMMA {
                                        {
                                            {
                                                recog.base.set_state(430);
                                                recog
                                                    .base
                                                    .match_token(COMMA, &mut recog.err_handler)?;

                                                /*InvokeRule column_name*/
                                                recog.base.set_state(431);
                                                recog.column_name()?;
                                            }
                                        }
                                        recog.base.set_state(436);
                                        recog.err_handler.sync(&mut recog.base)?;
                                        _la = recog.base.input.la(1);
                                    }
                                }
                            }
                        }
                    }

                    _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                        &mut recog.base,
                    )))?,
                }
                recog.base.set_state(441);
                recog.base.match_token(K_ON, &mut recog.err_handler)?;

                recog.base.set_state(445);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(46, &mut recog.base)? {
                    x if x == 1 => {
                        {
                            /*InvokeRule database_name*/
                            recog.base.set_state(442);
                            recog.database_name()?;

                            recog.base.set_state(443);
                            recog.base.match_token(DOT, &mut recog.err_handler)?;
                        }
                    }

                    _ => {}
                }
                /*InvokeRule table_name*/
                recog.base.set_state(447);
                recog.table_name()?;

                recog.base.set_state(451);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == K_FOR {
                    {
                        recog.base.set_state(448);
                        recog.base.match_token(K_FOR, &mut recog.err_handler)?;

                        recog.base.set_state(449);
                        recog.base.match_token(K_EACH, &mut recog.err_handler)?;

                        recog.base.set_state(450);
                        recog.base.match_token(K_ROW, &mut recog.err_handler)?;
                    }
                }

                recog.base.set_state(455);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == K_WHEN {
                    {
                        recog.base.set_state(453);
                        recog.base.match_token(K_WHEN, &mut recog.err_handler)?;

                        /*InvokeRule expr*/
                        recog.base.set_state(454);
                        recog.expr_rec(0)?;
                    }
                }

                recog.base.set_state(457);
                recog.base.match_token(K_BEGIN, &mut recog.err_handler)?;

                recog.base.set_state(466);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                loop {
                    {
                        {
                            recog.base.set_state(462);
                            recog.err_handler.sync(&mut recog.base)?;
                            match recog.interpreter.adaptive_predict(49, &mut recog.base)? {
                                1 => {
                                    {
                                        /*InvokeRule update_stmt*/
                                        recog.base.set_state(458);
                                        recog.update_stmt()?;
                                    }
                                }
                                2 => {
                                    {
                                        /*InvokeRule insert_stmt*/
                                        recog.base.set_state(459);
                                        recog.insert_stmt()?;
                                    }
                                }
                                3 => {
                                    {
                                        /*InvokeRule delete_stmt*/
                                        recog.base.set_state(460);
                                        recog.delete_stmt()?;
                                    }
                                }
                                4 => {
                                    {
                                        /*InvokeRule select_stmt*/
                                        recog.base.set_state(461);
                                        recog.select_stmt()?;
                                    }
                                }

                                _ => {}
                            }
                            recog.base.set_state(464);
                            recog.base.match_token(SCOL, &mut recog.err_handler)?;
                        }
                    }
                    recog.base.set_state(468);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                    if !(_la == K_DELETE
                        || _la == K_INSERT
                        || (((_la - 122) & !0x3f) == 0
                            && ((1usize << (_la - 122))
                                & ((1usize << (K_REPLACE - 122))
                                    | (1usize << (K_SELECT - 122))
                                    | (1usize << (K_UPDATE - 122))
                                    | (1usize << (K_VALUES - 122))
                                    | (1usize << (K_WITH - 122))))
                                != 0))
                    {
                        break;
                    }
                }
                recog.base.set_state(470);
                recog.base.match_token(K_END, &mut recog.err_handler)?;
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
//------------------- create_view_stmt ----------------
pub type Create_view_stmtContextAll<'input> = Create_view_stmtContext<'input>;

pub type Create_view_stmtContext<'input> =
    BaseParserRuleContext<'input, Create_view_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct Create_view_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Create_view_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Create_view_stmtContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_create_view_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_create_view_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Create_view_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_create_view_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_create_view_stmt }
}
antlr_rust::tid! {Create_view_stmtContextExt<'a>}

impl<'input> Create_view_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Create_view_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Create_view_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait Create_view_stmtContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Create_view_stmtContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token K_CREATE
    /// Returns `None` if there is no child corresponding to token K_CREATE
    fn K_CREATE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_CREATE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_VIEW
    /// Returns `None` if there is no child corresponding to token K_VIEW
    fn K_VIEW(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_VIEW, 0)
    }
    fn view_name(&self) -> Option<Rc<View_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_AS
    /// Returns `None` if there is no child corresponding to token K_AS
    fn K_AS(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_AS, 0)
    }
    fn select_stmt(&self) -> Option<Rc<Select_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_IF
    /// Returns `None` if there is no child corresponding to token K_IF
    fn K_IF(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_IF, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_NOT
    /// Returns `None` if there is no child corresponding to token K_NOT
    fn K_NOT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_NOT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_EXISTS
    /// Returns `None` if there is no child corresponding to token K_EXISTS
    fn K_EXISTS(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_EXISTS, 0)
    }
    fn database_name(&self) -> Option<Rc<Database_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token DOT
    /// Returns `None` if there is no child corresponding to token DOT
    fn DOT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DOT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_TEMP
    /// Returns `None` if there is no child corresponding to token K_TEMP
    fn K_TEMP(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_TEMP, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_TEMPORARY
    /// Returns `None` if there is no child corresponding to token K_TEMPORARY
    fn K_TEMPORARY(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_TEMPORARY, 0)
    }
}

impl<'input> Create_view_stmtContextAttrs<'input> for Create_view_stmtContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn create_view_stmt(
        &mut self,
    ) -> Result<Rc<Create_view_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Create_view_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 26, RULE_create_view_stmt);
        let mut _localctx: Rc<Create_view_stmtContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(472);
                recog.base.match_token(K_CREATE, &mut recog.err_handler)?;

                recog.base.set_state(474);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == K_TEMP || _la == K_TEMPORARY {
                    {
                        recog.base.set_state(473);
                        _la = recog.base.input.la(1);
                        if !(_la == K_TEMP || _la == K_TEMPORARY) {
                            recog.err_handler.recover_inline(&mut recog.base)?;
                        } else {
                            if recog.base.input.la(1) == TOKEN_EOF {
                                recog.base.matched_eof = true
                            };
                            recog.err_handler.report_match(&mut recog.base);
                            recog.base.consume(&mut recog.err_handler);
                        }
                    }
                }

                recog.base.set_state(476);
                recog.base.match_token(K_VIEW, &mut recog.err_handler)?;

                recog.base.set_state(480);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(52, &mut recog.base)? {
                    x if x == 1 => {
                        recog.base.set_state(477);
                        recog.base.match_token(K_IF, &mut recog.err_handler)?;

                        recog.base.set_state(478);
                        recog.base.match_token(K_NOT, &mut recog.err_handler)?;

                        recog.base.set_state(479);
                        recog.base.match_token(K_EXISTS, &mut recog.err_handler)?;
                    }

                    _ => {}
                }
                recog.base.set_state(485);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(53, &mut recog.base)? {
                    x if x == 1 => {
                        {
                            /*InvokeRule database_name*/
                            recog.base.set_state(482);
                            recog.database_name()?;

                            recog.base.set_state(483);
                            recog.base.match_token(DOT, &mut recog.err_handler)?;
                        }
                    }

                    _ => {}
                }
                /*InvokeRule view_name*/
                recog.base.set_state(487);
                recog.view_name()?;

                recog.base.set_state(488);
                recog.base.match_token(K_AS, &mut recog.err_handler)?;

                /*InvokeRule select_stmt*/
                recog.base.set_state(489);
                recog.select_stmt()?;
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
//------------------- create_virtual_table_stmt ----------------
pub type Create_virtual_table_stmtContextAll<'input> = Create_virtual_table_stmtContext<'input>;

pub type Create_virtual_table_stmtContext<'input> =
    BaseParserRuleContext<'input, Create_virtual_table_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct Create_virtual_table_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Create_virtual_table_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a>
    for Create_virtual_table_stmtContext<'input>
{
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_create_virtual_table_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_create_virtual_table_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Create_virtual_table_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_create_virtual_table_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_create_virtual_table_stmt }
}
antlr_rust::tid! {Create_virtual_table_stmtContextExt<'a>}

impl<'input> Create_virtual_table_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Create_virtual_table_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Create_virtual_table_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait Create_virtual_table_stmtContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Create_virtual_table_stmtContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token K_CREATE
    /// Returns `None` if there is no child corresponding to token K_CREATE
    fn K_CREATE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_CREATE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_VIRTUAL
    /// Returns `None` if there is no child corresponding to token K_VIRTUAL
    fn K_VIRTUAL(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_VIRTUAL, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_TABLE
    /// Returns `None` if there is no child corresponding to token K_TABLE
    fn K_TABLE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_TABLE, 0)
    }
    fn table_name(&self) -> Option<Rc<Table_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_USING
    /// Returns `None` if there is no child corresponding to token K_USING
    fn K_USING(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_USING, 0)
    }
    fn module_name(&self) -> Option<Rc<Module_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_IF
    /// Returns `None` if there is no child corresponding to token K_IF
    fn K_IF(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_IF, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_NOT
    /// Returns `None` if there is no child corresponding to token K_NOT
    fn K_NOT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_NOT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_EXISTS
    /// Returns `None` if there is no child corresponding to token K_EXISTS
    fn K_EXISTS(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_EXISTS, 0)
    }
    fn database_name(&self) -> Option<Rc<Database_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token DOT
    /// Returns `None` if there is no child corresponding to token DOT
    fn DOT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DOT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token OPEN_PAR
    /// Returns `None` if there is no child corresponding to token OPEN_PAR
    fn OPEN_PAR(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPEN_PAR, 0)
    }
    fn module_argument_all(&self) -> Vec<Rc<Module_argumentContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn module_argument(&self, i: usize) -> Option<Rc<Module_argumentContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token CLOSE_PAR
    /// Returns `None` if there is no child corresponding to token CLOSE_PAR
    fn CLOSE_PAR(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CLOSE_PAR, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
}

impl<'input> Create_virtual_table_stmtContextAttrs<'input>
    for Create_virtual_table_stmtContext<'input>
{
}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn create_virtual_table_stmt(
        &mut self,
    ) -> Result<Rc<Create_virtual_table_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Create_virtual_table_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 28, RULE_create_virtual_table_stmt);
        let mut _localctx: Rc<Create_virtual_table_stmtContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(491);
                recog.base.match_token(K_CREATE, &mut recog.err_handler)?;

                recog.base.set_state(492);
                recog.base.match_token(K_VIRTUAL, &mut recog.err_handler)?;

                recog.base.set_state(493);
                recog.base.match_token(K_TABLE, &mut recog.err_handler)?;

                recog.base.set_state(497);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(54, &mut recog.base)? {
                    x if x == 1 => {
                        recog.base.set_state(494);
                        recog.base.match_token(K_IF, &mut recog.err_handler)?;

                        recog.base.set_state(495);
                        recog.base.match_token(K_NOT, &mut recog.err_handler)?;

                        recog.base.set_state(496);
                        recog.base.match_token(K_EXISTS, &mut recog.err_handler)?;
                    }

                    _ => {}
                }
                recog.base.set_state(502);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(55, &mut recog.base)? {
                    x if x == 1 => {
                        {
                            /*InvokeRule database_name*/
                            recog.base.set_state(499);
                            recog.database_name()?;

                            recog.base.set_state(500);
                            recog.base.match_token(DOT, &mut recog.err_handler)?;
                        }
                    }

                    _ => {}
                }
                /*InvokeRule table_name*/
                recog.base.set_state(504);
                recog.table_name()?;

                recog.base.set_state(505);
                recog.base.match_token(K_USING, &mut recog.err_handler)?;

                /*InvokeRule module_name*/
                recog.base.set_state(506);
                recog.module_name()?;

                recog.base.set_state(518);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == OPEN_PAR {
                    {
                        recog.base.set_state(507);
                        recog.base.match_token(OPEN_PAR, &mut recog.err_handler)?;

                        /*InvokeRule module_argument*/
                        recog.base.set_state(508);
                        recog.module_argument()?;

                        recog.base.set_state(513);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        while _la == COMMA {
                            {
                                {
                                    recog.base.set_state(509);
                                    recog.base.match_token(COMMA, &mut recog.err_handler)?;

                                    /*InvokeRule module_argument*/
                                    recog.base.set_state(510);
                                    recog.module_argument()?;
                                }
                            }
                            recog.base.set_state(515);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                        }
                        recog.base.set_state(516);
                        recog.base.match_token(CLOSE_PAR, &mut recog.err_handler)?;
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
//------------------- delete_stmt ----------------
pub type Delete_stmtContextAll<'input> = Delete_stmtContext<'input>;

pub type Delete_stmtContext<'input> = BaseParserRuleContext<'input, Delete_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct Delete_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Delete_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Delete_stmtContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_delete_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_delete_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Delete_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_delete_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_delete_stmt }
}
antlr_rust::tid! {Delete_stmtContextExt<'a>}

impl<'input> Delete_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Delete_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Delete_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait Delete_stmtContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Delete_stmtContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token K_DELETE
    /// Returns `None` if there is no child corresponding to token K_DELETE
    fn K_DELETE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_DELETE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_FROM
    /// Returns `None` if there is no child corresponding to token K_FROM
    fn K_FROM(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_FROM, 0)
    }
    fn qualified_table_name(&self) -> Option<Rc<Qualified_table_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn with_clause(&self) -> Option<Rc<With_clauseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_WHERE
    /// Returns `None` if there is no child corresponding to token K_WHERE
    fn K_WHERE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_WHERE, 0)
    }
    fn expr(&self) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Delete_stmtContextAttrs<'input> for Delete_stmtContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn delete_stmt(&mut self) -> Result<Rc<Delete_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Delete_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 30, RULE_delete_stmt);
        let mut _localctx: Rc<Delete_stmtContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(521);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == K_WITH {
                    {
                        /*InvokeRule with_clause*/
                        recog.base.set_state(520);
                        recog.with_clause()?;
                    }
                }

                recog.base.set_state(523);
                recog.base.match_token(K_DELETE, &mut recog.err_handler)?;

                recog.base.set_state(524);
                recog.base.match_token(K_FROM, &mut recog.err_handler)?;

                /*InvokeRule qualified_table_name*/
                recog.base.set_state(525);
                recog.qualified_table_name()?;

                recog.base.set_state(528);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == K_WHERE {
                    {
                        recog.base.set_state(526);
                        recog.base.match_token(K_WHERE, &mut recog.err_handler)?;

                        /*InvokeRule expr*/
                        recog.base.set_state(527);
                        recog.expr_rec(0)?;
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
//------------------- delete_stmt_limited ----------------
pub type Delete_stmt_limitedContextAll<'input> = Delete_stmt_limitedContext<'input>;

pub type Delete_stmt_limitedContext<'input> =
    BaseParserRuleContext<'input, Delete_stmt_limitedContextExt<'input>>;

#[derive(Clone)]
pub struct Delete_stmt_limitedContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Delete_stmt_limitedContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a>
    for Delete_stmt_limitedContext<'input>
{
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_delete_stmt_limited(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_delete_stmt_limited(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Delete_stmt_limitedContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_delete_stmt_limited
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_delete_stmt_limited }
}
antlr_rust::tid! {Delete_stmt_limitedContextExt<'a>}

impl<'input> Delete_stmt_limitedContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Delete_stmt_limitedContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Delete_stmt_limitedContextExt { ph: PhantomData },
        ))
    }
}

pub trait Delete_stmt_limitedContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Delete_stmt_limitedContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token K_DELETE
    /// Returns `None` if there is no child corresponding to token K_DELETE
    fn K_DELETE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_DELETE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_FROM
    /// Returns `None` if there is no child corresponding to token K_FROM
    fn K_FROM(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_FROM, 0)
    }
    fn qualified_table_name(&self) -> Option<Rc<Qualified_table_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn with_clause(&self) -> Option<Rc<With_clauseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_WHERE
    /// Returns `None` if there is no child corresponding to token K_WHERE
    fn K_WHERE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_WHERE, 0)
    }
    fn expr_all(&self) -> Vec<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token K_LIMIT
    /// Returns `None` if there is no child corresponding to token K_LIMIT
    fn K_LIMIT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_LIMIT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_ORDER
    /// Returns `None` if there is no child corresponding to token K_ORDER
    fn K_ORDER(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ORDER, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_BY
    /// Returns `None` if there is no child corresponding to token K_BY
    fn K_BY(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_BY, 0)
    }
    fn ordering_term_all(&self) -> Vec<Rc<Ordering_termContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn ordering_term(&self, i: usize) -> Option<Rc<Ordering_termContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token K_OFFSET
    /// Returns `None` if there is no child corresponding to token K_OFFSET
    fn K_OFFSET(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_OFFSET, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
}

impl<'input> Delete_stmt_limitedContextAttrs<'input> for Delete_stmt_limitedContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn delete_stmt_limited(
        &mut self,
    ) -> Result<Rc<Delete_stmt_limitedContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Delete_stmt_limitedContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 32, RULE_delete_stmt_limited);
        let mut _localctx: Rc<Delete_stmt_limitedContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(531);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == K_WITH {
                    {
                        /*InvokeRule with_clause*/
                        recog.base.set_state(530);
                        recog.with_clause()?;
                    }
                }

                recog.base.set_state(533);
                recog.base.match_token(K_DELETE, &mut recog.err_handler)?;

                recog.base.set_state(534);
                recog.base.match_token(K_FROM, &mut recog.err_handler)?;

                /*InvokeRule qualified_table_name*/
                recog.base.set_state(535);
                recog.qualified_table_name()?;

                recog.base.set_state(538);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == K_WHERE {
                    {
                        recog.base.set_state(536);
                        recog.base.match_token(K_WHERE, &mut recog.err_handler)?;

                        /*InvokeRule expr*/
                        recog.base.set_state(537);
                        recog.expr_rec(0)?;
                    }
                }

                recog.base.set_state(558);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == K_LIMIT || _la == K_ORDER {
                    {
                        recog.base.set_state(550);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == K_ORDER {
                            {
                                recog.base.set_state(540);
                                recog.base.match_token(K_ORDER, &mut recog.err_handler)?;

                                recog.base.set_state(541);
                                recog.base.match_token(K_BY, &mut recog.err_handler)?;

                                /*InvokeRule ordering_term*/
                                recog.base.set_state(542);
                                recog.ordering_term()?;

                                recog.base.set_state(547);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                                while _la == COMMA {
                                    {
                                        {
                                            recog.base.set_state(543);
                                            recog
                                                .base
                                                .match_token(COMMA, &mut recog.err_handler)?;

                                            /*InvokeRule ordering_term*/
                                            recog.base.set_state(544);
                                            recog.ordering_term()?;
                                        }
                                    }
                                    recog.base.set_state(549);
                                    recog.err_handler.sync(&mut recog.base)?;
                                    _la = recog.base.input.la(1);
                                }
                            }
                        }

                        recog.base.set_state(552);
                        recog.base.match_token(K_LIMIT, &mut recog.err_handler)?;

                        /*InvokeRule expr*/
                        recog.base.set_state(553);
                        recog.expr_rec(0)?;

                        recog.base.set_state(556);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == COMMA || _la == K_OFFSET {
                            {
                                recog.base.set_state(554);
                                _la = recog.base.input.la(1);
                                if !(_la == COMMA || _la == K_OFFSET) {
                                    recog.err_handler.recover_inline(&mut recog.base)?;
                                } else {
                                    if recog.base.input.la(1) == TOKEN_EOF {
                                        recog.base.matched_eof = true
                                    };
                                    recog.err_handler.report_match(&mut recog.base);
                                    recog.base.consume(&mut recog.err_handler);
                                }
                                /*InvokeRule expr*/
                                recog.base.set_state(555);
                                recog.expr_rec(0)?;
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
//------------------- detach_stmt ----------------
pub type Detach_stmtContextAll<'input> = Detach_stmtContext<'input>;

pub type Detach_stmtContext<'input> = BaseParserRuleContext<'input, Detach_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct Detach_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Detach_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Detach_stmtContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_detach_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_detach_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Detach_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_detach_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_detach_stmt }
}
antlr_rust::tid! {Detach_stmtContextExt<'a>}

impl<'input> Detach_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Detach_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Detach_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait Detach_stmtContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Detach_stmtContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token K_DETACH
    /// Returns `None` if there is no child corresponding to token K_DETACH
    fn K_DETACH(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_DETACH, 0)
    }
    fn database_name(&self) -> Option<Rc<Database_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_DATABASE
    /// Returns `None` if there is no child corresponding to token K_DATABASE
    fn K_DATABASE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_DATABASE, 0)
    }
}

impl<'input> Detach_stmtContextAttrs<'input> for Detach_stmtContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn detach_stmt(&mut self) -> Result<Rc<Detach_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Detach_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 34, RULE_detach_stmt);
        let mut _localctx: Rc<Detach_stmtContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(560);
                recog.base.match_token(K_DETACH, &mut recog.err_handler)?;

                recog.base.set_state(562);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(66, &mut recog.base)? {
                    x if x == 1 => {
                        recog.base.set_state(561);
                        recog.base.match_token(K_DATABASE, &mut recog.err_handler)?;
                    }

                    _ => {}
                }
                /*InvokeRule database_name*/
                recog.base.set_state(564);
                recog.database_name()?;
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
//------------------- drop_index_stmt ----------------
pub type Drop_index_stmtContextAll<'input> = Drop_index_stmtContext<'input>;

pub type Drop_index_stmtContext<'input> =
    BaseParserRuleContext<'input, Drop_index_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct Drop_index_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Drop_index_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Drop_index_stmtContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_drop_index_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_drop_index_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Drop_index_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_drop_index_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_drop_index_stmt }
}
antlr_rust::tid! {Drop_index_stmtContextExt<'a>}

impl<'input> Drop_index_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Drop_index_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Drop_index_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait Drop_index_stmtContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Drop_index_stmtContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token K_DROP
    /// Returns `None` if there is no child corresponding to token K_DROP
    fn K_DROP(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_DROP, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_INDEX
    /// Returns `None` if there is no child corresponding to token K_INDEX
    fn K_INDEX(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_INDEX, 0)
    }
    fn index_name(&self) -> Option<Rc<Index_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_IF
    /// Returns `None` if there is no child corresponding to token K_IF
    fn K_IF(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_IF, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_EXISTS
    /// Returns `None` if there is no child corresponding to token K_EXISTS
    fn K_EXISTS(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_EXISTS, 0)
    }
    fn database_name(&self) -> Option<Rc<Database_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token DOT
    /// Returns `None` if there is no child corresponding to token DOT
    fn DOT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DOT, 0)
    }
}

impl<'input> Drop_index_stmtContextAttrs<'input> for Drop_index_stmtContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn drop_index_stmt(&mut self) -> Result<Rc<Drop_index_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Drop_index_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 36, RULE_drop_index_stmt);
        let mut _localctx: Rc<Drop_index_stmtContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(566);
                recog.base.match_token(K_DROP, &mut recog.err_handler)?;

                recog.base.set_state(567);
                recog.base.match_token(K_INDEX, &mut recog.err_handler)?;

                recog.base.set_state(570);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(67, &mut recog.base)? {
                    x if x == 1 => {
                        recog.base.set_state(568);
                        recog.base.match_token(K_IF, &mut recog.err_handler)?;

                        recog.base.set_state(569);
                        recog.base.match_token(K_EXISTS, &mut recog.err_handler)?;
                    }

                    _ => {}
                }
                recog.base.set_state(575);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(68, &mut recog.base)? {
                    x if x == 1 => {
                        {
                            /*InvokeRule database_name*/
                            recog.base.set_state(572);
                            recog.database_name()?;

                            recog.base.set_state(573);
                            recog.base.match_token(DOT, &mut recog.err_handler)?;
                        }
                    }

                    _ => {}
                }
                /*InvokeRule index_name*/
                recog.base.set_state(577);
                recog.index_name()?;
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
//------------------- drop_table_stmt ----------------
pub type Drop_table_stmtContextAll<'input> = Drop_table_stmtContext<'input>;

pub type Drop_table_stmtContext<'input> =
    BaseParserRuleContext<'input, Drop_table_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct Drop_table_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Drop_table_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Drop_table_stmtContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_drop_table_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_drop_table_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Drop_table_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_drop_table_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_drop_table_stmt }
}
antlr_rust::tid! {Drop_table_stmtContextExt<'a>}

impl<'input> Drop_table_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Drop_table_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Drop_table_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait Drop_table_stmtContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Drop_table_stmtContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token K_DROP
    /// Returns `None` if there is no child corresponding to token K_DROP
    fn K_DROP(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_DROP, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_TABLE
    /// Returns `None` if there is no child corresponding to token K_TABLE
    fn K_TABLE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_TABLE, 0)
    }
    fn table_name(&self) -> Option<Rc<Table_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_IF
    /// Returns `None` if there is no child corresponding to token K_IF
    fn K_IF(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_IF, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_EXISTS
    /// Returns `None` if there is no child corresponding to token K_EXISTS
    fn K_EXISTS(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_EXISTS, 0)
    }
    fn database_name(&self) -> Option<Rc<Database_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token DOT
    /// Returns `None` if there is no child corresponding to token DOT
    fn DOT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DOT, 0)
    }
}

impl<'input> Drop_table_stmtContextAttrs<'input> for Drop_table_stmtContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn drop_table_stmt(&mut self) -> Result<Rc<Drop_table_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Drop_table_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 38, RULE_drop_table_stmt);
        let mut _localctx: Rc<Drop_table_stmtContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(579);
                recog.base.match_token(K_DROP, &mut recog.err_handler)?;

                recog.base.set_state(580);
                recog.base.match_token(K_TABLE, &mut recog.err_handler)?;

                recog.base.set_state(583);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(69, &mut recog.base)? {
                    x if x == 1 => {
                        recog.base.set_state(581);
                        recog.base.match_token(K_IF, &mut recog.err_handler)?;

                        recog.base.set_state(582);
                        recog.base.match_token(K_EXISTS, &mut recog.err_handler)?;
                    }

                    _ => {}
                }
                recog.base.set_state(588);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(70, &mut recog.base)? {
                    x if x == 1 => {
                        {
                            /*InvokeRule database_name*/
                            recog.base.set_state(585);
                            recog.database_name()?;

                            recog.base.set_state(586);
                            recog.base.match_token(DOT, &mut recog.err_handler)?;
                        }
                    }

                    _ => {}
                }
                /*InvokeRule table_name*/
                recog.base.set_state(590);
                recog.table_name()?;
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
//------------------- drop_trigger_stmt ----------------
pub type Drop_trigger_stmtContextAll<'input> = Drop_trigger_stmtContext<'input>;

pub type Drop_trigger_stmtContext<'input> =
    BaseParserRuleContext<'input, Drop_trigger_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct Drop_trigger_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Drop_trigger_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Drop_trigger_stmtContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_drop_trigger_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_drop_trigger_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Drop_trigger_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_drop_trigger_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_drop_trigger_stmt }
}
antlr_rust::tid! {Drop_trigger_stmtContextExt<'a>}

impl<'input> Drop_trigger_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Drop_trigger_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Drop_trigger_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait Drop_trigger_stmtContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Drop_trigger_stmtContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token K_DROP
    /// Returns `None` if there is no child corresponding to token K_DROP
    fn K_DROP(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_DROP, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_TRIGGER
    /// Returns `None` if there is no child corresponding to token K_TRIGGER
    fn K_TRIGGER(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_TRIGGER, 0)
    }
    fn trigger_name(&self) -> Option<Rc<Trigger_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_IF
    /// Returns `None` if there is no child corresponding to token K_IF
    fn K_IF(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_IF, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_EXISTS
    /// Returns `None` if there is no child corresponding to token K_EXISTS
    fn K_EXISTS(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_EXISTS, 0)
    }
    fn database_name(&self) -> Option<Rc<Database_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token DOT
    /// Returns `None` if there is no child corresponding to token DOT
    fn DOT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DOT, 0)
    }
}

impl<'input> Drop_trigger_stmtContextAttrs<'input> for Drop_trigger_stmtContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn drop_trigger_stmt(
        &mut self,
    ) -> Result<Rc<Drop_trigger_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Drop_trigger_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 40, RULE_drop_trigger_stmt);
        let mut _localctx: Rc<Drop_trigger_stmtContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(592);
                recog.base.match_token(K_DROP, &mut recog.err_handler)?;

                recog.base.set_state(593);
                recog.base.match_token(K_TRIGGER, &mut recog.err_handler)?;

                recog.base.set_state(596);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(71, &mut recog.base)? {
                    x if x == 1 => {
                        recog.base.set_state(594);
                        recog.base.match_token(K_IF, &mut recog.err_handler)?;

                        recog.base.set_state(595);
                        recog.base.match_token(K_EXISTS, &mut recog.err_handler)?;
                    }

                    _ => {}
                }
                recog.base.set_state(601);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(72, &mut recog.base)? {
                    x if x == 1 => {
                        {
                            /*InvokeRule database_name*/
                            recog.base.set_state(598);
                            recog.database_name()?;

                            recog.base.set_state(599);
                            recog.base.match_token(DOT, &mut recog.err_handler)?;
                        }
                    }

                    _ => {}
                }
                /*InvokeRule trigger_name*/
                recog.base.set_state(603);
                recog.trigger_name()?;
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
//------------------- drop_view_stmt ----------------
pub type Drop_view_stmtContextAll<'input> = Drop_view_stmtContext<'input>;

pub type Drop_view_stmtContext<'input> =
    BaseParserRuleContext<'input, Drop_view_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct Drop_view_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Drop_view_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Drop_view_stmtContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_drop_view_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_drop_view_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Drop_view_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_drop_view_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_drop_view_stmt }
}
antlr_rust::tid! {Drop_view_stmtContextExt<'a>}

impl<'input> Drop_view_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Drop_view_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Drop_view_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait Drop_view_stmtContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Drop_view_stmtContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token K_DROP
    /// Returns `None` if there is no child corresponding to token K_DROP
    fn K_DROP(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_DROP, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_VIEW
    /// Returns `None` if there is no child corresponding to token K_VIEW
    fn K_VIEW(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_VIEW, 0)
    }
    fn view_name(&self) -> Option<Rc<View_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_IF
    /// Returns `None` if there is no child corresponding to token K_IF
    fn K_IF(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_IF, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_EXISTS
    /// Returns `None` if there is no child corresponding to token K_EXISTS
    fn K_EXISTS(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_EXISTS, 0)
    }
    fn database_name(&self) -> Option<Rc<Database_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token DOT
    /// Returns `None` if there is no child corresponding to token DOT
    fn DOT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DOT, 0)
    }
}

impl<'input> Drop_view_stmtContextAttrs<'input> for Drop_view_stmtContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn drop_view_stmt(&mut self) -> Result<Rc<Drop_view_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Drop_view_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 42, RULE_drop_view_stmt);
        let mut _localctx: Rc<Drop_view_stmtContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(605);
                recog.base.match_token(K_DROP, &mut recog.err_handler)?;

                recog.base.set_state(606);
                recog.base.match_token(K_VIEW, &mut recog.err_handler)?;

                recog.base.set_state(609);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(73, &mut recog.base)? {
                    x if x == 1 => {
                        recog.base.set_state(607);
                        recog.base.match_token(K_IF, &mut recog.err_handler)?;

                        recog.base.set_state(608);
                        recog.base.match_token(K_EXISTS, &mut recog.err_handler)?;
                    }

                    _ => {}
                }
                recog.base.set_state(614);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(74, &mut recog.base)? {
                    x if x == 1 => {
                        {
                            /*InvokeRule database_name*/
                            recog.base.set_state(611);
                            recog.database_name()?;

                            recog.base.set_state(612);
                            recog.base.match_token(DOT, &mut recog.err_handler)?;
                        }
                    }

                    _ => {}
                }
                /*InvokeRule view_name*/
                recog.base.set_state(616);
                recog.view_name()?;
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
//------------------- factored_select_stmt ----------------
pub type Factored_select_stmtContextAll<'input> = Factored_select_stmtContext<'input>;

pub type Factored_select_stmtContext<'input> =
    BaseParserRuleContext<'input, Factored_select_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct Factored_select_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Factored_select_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a>
    for Factored_select_stmtContext<'input>
{
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_factored_select_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_factored_select_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Factored_select_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_factored_select_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_factored_select_stmt }
}
antlr_rust::tid! {Factored_select_stmtContextExt<'a>}

impl<'input> Factored_select_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Factored_select_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Factored_select_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait Factored_select_stmtContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Factored_select_stmtContextExt<'input>>
{
    fn select_core_all(&self) -> Vec<Rc<Select_coreContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn select_core(&self, i: usize) -> Option<Rc<Select_coreContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn with_clause(&self) -> Option<Rc<With_clauseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn compound_operator_all(&self) -> Vec<Rc<Compound_operatorContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn compound_operator(&self, i: usize) -> Option<Rc<Compound_operatorContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token K_ORDER
    /// Returns `None` if there is no child corresponding to token K_ORDER
    fn K_ORDER(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ORDER, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_BY
    /// Returns `None` if there is no child corresponding to token K_BY
    fn K_BY(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_BY, 0)
    }
    fn ordering_term_all(&self) -> Vec<Rc<Ordering_termContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn ordering_term(&self, i: usize) -> Option<Rc<Ordering_termContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token K_LIMIT
    /// Returns `None` if there is no child corresponding to token K_LIMIT
    fn K_LIMIT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_LIMIT, 0)
    }
    fn expr_all(&self) -> Vec<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
    /// Retrieves first TerminalNode corresponding to token K_OFFSET
    /// Returns `None` if there is no child corresponding to token K_OFFSET
    fn K_OFFSET(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_OFFSET, 0)
    }
}

impl<'input> Factored_select_stmtContextAttrs<'input> for Factored_select_stmtContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn factored_select_stmt(
        &mut self,
    ) -> Result<Rc<Factored_select_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Factored_select_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 44, RULE_factored_select_stmt);
        let mut _localctx: Rc<Factored_select_stmtContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(619);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == K_WITH {
                    {
                        /*InvokeRule with_clause*/
                        recog.base.set_state(618);
                        recog.with_clause()?;
                    }
                }

                /*InvokeRule select_core*/
                recog.base.set_state(621);
                recog.select_core()?;

                recog.base.set_state(627);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == K_EXCEPT || _la == K_INTERSECT || _la == K_UNION {
                    {
                        {
                            /*InvokeRule compound_operator*/
                            recog.base.set_state(622);
                            recog.compound_operator()?;

                            /*InvokeRule select_core*/
                            recog.base.set_state(623);
                            recog.select_core()?;
                        }
                    }
                    recog.base.set_state(629);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(640);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == K_ORDER {
                    {
                        recog.base.set_state(630);
                        recog.base.match_token(K_ORDER, &mut recog.err_handler)?;

                        recog.base.set_state(631);
                        recog.base.match_token(K_BY, &mut recog.err_handler)?;

                        /*InvokeRule ordering_term*/
                        recog.base.set_state(632);
                        recog.ordering_term()?;

                        recog.base.set_state(637);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        while _la == COMMA {
                            {
                                {
                                    recog.base.set_state(633);
                                    recog.base.match_token(COMMA, &mut recog.err_handler)?;

                                    /*InvokeRule ordering_term*/
                                    recog.base.set_state(634);
                                    recog.ordering_term()?;
                                }
                            }
                            recog.base.set_state(639);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                        }
                    }
                }

                recog.base.set_state(648);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == K_LIMIT {
                    {
                        recog.base.set_state(642);
                        recog.base.match_token(K_LIMIT, &mut recog.err_handler)?;

                        /*InvokeRule expr*/
                        recog.base.set_state(643);
                        recog.expr_rec(0)?;

                        recog.base.set_state(646);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == COMMA || _la == K_OFFSET {
                            {
                                recog.base.set_state(644);
                                _la = recog.base.input.la(1);
                                if !(_la == COMMA || _la == K_OFFSET) {
                                    recog.err_handler.recover_inline(&mut recog.base)?;
                                } else {
                                    if recog.base.input.la(1) == TOKEN_EOF {
                                        recog.base.matched_eof = true
                                    };
                                    recog.err_handler.report_match(&mut recog.base);
                                    recog.base.consume(&mut recog.err_handler);
                                }
                                /*InvokeRule expr*/
                                recog.base.set_state(645);
                                recog.expr_rec(0)?;
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
//------------------- insert_stmt ----------------
pub type Insert_stmtContextAll<'input> = Insert_stmtContext<'input>;

pub type Insert_stmtContext<'input> = BaseParserRuleContext<'input, Insert_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct Insert_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Insert_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Insert_stmtContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_insert_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_insert_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Insert_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_insert_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_insert_stmt }
}
antlr_rust::tid! {Insert_stmtContextExt<'a>}

impl<'input> Insert_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Insert_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Insert_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait Insert_stmtContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Insert_stmtContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token K_INTO
    /// Returns `None` if there is no child corresponding to token K_INTO
    fn K_INTO(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_INTO, 0)
    }
    fn table_name(&self) -> Option<Rc<Table_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_INSERT
    /// Returns `None` if there is no child corresponding to token K_INSERT
    fn K_INSERT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_INSERT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_REPLACE
    /// Returns `None` if there is no child corresponding to token K_REPLACE
    fn K_REPLACE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_REPLACE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_OR
    /// Returns `None` if there is no child corresponding to token K_OR
    fn K_OR(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_OR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_ROLLBACK
    /// Returns `None` if there is no child corresponding to token K_ROLLBACK
    fn K_ROLLBACK(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ROLLBACK, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_ABORT
    /// Returns `None` if there is no child corresponding to token K_ABORT
    fn K_ABORT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ABORT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_FAIL
    /// Returns `None` if there is no child corresponding to token K_FAIL
    fn K_FAIL(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_FAIL, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_IGNORE
    /// Returns `None` if there is no child corresponding to token K_IGNORE
    fn K_IGNORE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_IGNORE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_VALUES
    /// Returns `None` if there is no child corresponding to token K_VALUES
    fn K_VALUES(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_VALUES, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token OPEN_PAR in current rule
    fn OPEN_PAR_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token OPEN_PAR, starting from 0.
    /// Returns `None` if number of children corresponding to token OPEN_PAR is less or equal than `i`.
    fn OPEN_PAR(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPEN_PAR, i)
    }
    fn expr_all(&self) -> Vec<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token CLOSE_PAR in current rule
    fn CLOSE_PAR_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token CLOSE_PAR, starting from 0.
    /// Returns `None` if number of children corresponding to token CLOSE_PAR is less or equal than `i`.
    fn CLOSE_PAR(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CLOSE_PAR, i)
    }
    fn select_stmt(&self) -> Option<Rc<Select_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_DEFAULT
    /// Returns `None` if there is no child corresponding to token K_DEFAULT
    fn K_DEFAULT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_DEFAULT, 0)
    }
    fn with_clause(&self) -> Option<Rc<With_clauseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn database_name(&self) -> Option<Rc<Database_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token DOT
    /// Returns `None` if there is no child corresponding to token DOT
    fn DOT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DOT, 0)
    }
    fn column_name_all(&self) -> Vec<Rc<Column_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn column_name(&self, i: usize) -> Option<Rc<Column_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
}

impl<'input> Insert_stmtContextAttrs<'input> for Insert_stmtContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn insert_stmt(&mut self) -> Result<Rc<Insert_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Insert_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 46, RULE_insert_stmt);
        let mut _localctx: Rc<Insert_stmtContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(651);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == K_WITH {
                    {
                        /*InvokeRule with_clause*/
                        recog.base.set_state(650);
                        recog.with_clause()?;
                    }
                }

                recog.base.set_state(670);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(82, &mut recog.base)? {
                    1 => {
                        recog.base.set_state(653);
                        recog.base.match_token(K_INSERT, &mut recog.err_handler)?;
                    }
                    2 => {
                        recog.base.set_state(654);
                        recog.base.match_token(K_REPLACE, &mut recog.err_handler)?;
                    }
                    3 => {
                        recog.base.set_state(655);
                        recog.base.match_token(K_INSERT, &mut recog.err_handler)?;

                        recog.base.set_state(656);
                        recog.base.match_token(K_OR, &mut recog.err_handler)?;

                        recog.base.set_state(657);
                        recog.base.match_token(K_REPLACE, &mut recog.err_handler)?;
                    }
                    4 => {
                        recog.base.set_state(658);
                        recog.base.match_token(K_INSERT, &mut recog.err_handler)?;

                        recog.base.set_state(659);
                        recog.base.match_token(K_OR, &mut recog.err_handler)?;

                        recog.base.set_state(660);
                        recog.base.match_token(K_ROLLBACK, &mut recog.err_handler)?;
                    }
                    5 => {
                        recog.base.set_state(661);
                        recog.base.match_token(K_INSERT, &mut recog.err_handler)?;

                        recog.base.set_state(662);
                        recog.base.match_token(K_OR, &mut recog.err_handler)?;

                        recog.base.set_state(663);
                        recog.base.match_token(K_ABORT, &mut recog.err_handler)?;
                    }
                    6 => {
                        recog.base.set_state(664);
                        recog.base.match_token(K_INSERT, &mut recog.err_handler)?;

                        recog.base.set_state(665);
                        recog.base.match_token(K_OR, &mut recog.err_handler)?;

                        recog.base.set_state(666);
                        recog.base.match_token(K_FAIL, &mut recog.err_handler)?;
                    }
                    7 => {
                        recog.base.set_state(667);
                        recog.base.match_token(K_INSERT, &mut recog.err_handler)?;

                        recog.base.set_state(668);
                        recog.base.match_token(K_OR, &mut recog.err_handler)?;

                        recog.base.set_state(669);
                        recog.base.match_token(K_IGNORE, &mut recog.err_handler)?;
                    }

                    _ => {}
                }
                recog.base.set_state(672);
                recog.base.match_token(K_INTO, &mut recog.err_handler)?;

                recog.base.set_state(676);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(83, &mut recog.base)? {
                    x if x == 1 => {
                        {
                            /*InvokeRule database_name*/
                            recog.base.set_state(673);
                            recog.database_name()?;

                            recog.base.set_state(674);
                            recog.base.match_token(DOT, &mut recog.err_handler)?;
                        }
                    }

                    _ => {}
                }
                /*InvokeRule table_name*/
                recog.base.set_state(678);
                recog.table_name()?;

                recog.base.set_state(690);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == OPEN_PAR {
                    {
                        recog.base.set_state(679);
                        recog.base.match_token(OPEN_PAR, &mut recog.err_handler)?;

                        /*InvokeRule column_name*/
                        recog.base.set_state(680);
                        recog.column_name()?;

                        recog.base.set_state(685);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        while _la == COMMA {
                            {
                                {
                                    recog.base.set_state(681);
                                    recog.base.match_token(COMMA, &mut recog.err_handler)?;

                                    /*InvokeRule column_name*/
                                    recog.base.set_state(682);
                                    recog.column_name()?;
                                }
                            }
                            recog.base.set_state(687);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                        }
                        recog.base.set_state(688);
                        recog.base.match_token(CLOSE_PAR, &mut recog.err_handler)?;
                    }
                }

                recog.base.set_state(723);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(89, &mut recog.base)? {
                    1 => {
                        {
                            recog.base.set_state(692);
                            recog.base.match_token(K_VALUES, &mut recog.err_handler)?;

                            recog.base.set_state(693);
                            recog.base.match_token(OPEN_PAR, &mut recog.err_handler)?;

                            /*InvokeRule expr*/
                            recog.base.set_state(694);
                            recog.expr_rec(0)?;

                            recog.base.set_state(699);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            while _la == COMMA {
                                {
                                    {
                                        recog.base.set_state(695);
                                        recog.base.match_token(COMMA, &mut recog.err_handler)?;

                                        /*InvokeRule expr*/
                                        recog.base.set_state(696);
                                        recog.expr_rec(0)?;
                                    }
                                }
                                recog.base.set_state(701);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                            }
                            recog.base.set_state(702);
                            recog.base.match_token(CLOSE_PAR, &mut recog.err_handler)?;

                            recog.base.set_state(717);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            while _la == COMMA {
                                {
                                    {
                                        recog.base.set_state(703);
                                        recog.base.match_token(COMMA, &mut recog.err_handler)?;

                                        recog.base.set_state(704);
                                        recog.base.match_token(OPEN_PAR, &mut recog.err_handler)?;

                                        /*InvokeRule expr*/
                                        recog.base.set_state(705);
                                        recog.expr_rec(0)?;

                                        recog.base.set_state(710);
                                        recog.err_handler.sync(&mut recog.base)?;
                                        _la = recog.base.input.la(1);
                                        while _la == COMMA {
                                            {
                                                {
                                                    recog.base.set_state(706);
                                                    recog.base.match_token(
                                                        COMMA,
                                                        &mut recog.err_handler,
                                                    )?;

                                                    /*InvokeRule expr*/
                                                    recog.base.set_state(707);
                                                    recog.expr_rec(0)?;
                                                }
                                            }
                                            recog.base.set_state(712);
                                            recog.err_handler.sync(&mut recog.base)?;
                                            _la = recog.base.input.la(1);
                                        }
                                        recog.base.set_state(713);
                                        recog
                                            .base
                                            .match_token(CLOSE_PAR, &mut recog.err_handler)?;
                                    }
                                }
                                recog.base.set_state(719);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                            }
                        }
                    }
                    2 => {
                        {
                            /*InvokeRule select_stmt*/
                            recog.base.set_state(720);
                            recog.select_stmt()?;
                        }
                    }
                    3 => {
                        recog.base.set_state(721);
                        recog.base.match_token(K_DEFAULT, &mut recog.err_handler)?;

                        recog.base.set_state(722);
                        recog.base.match_token(K_VALUES, &mut recog.err_handler)?;
                    }

                    _ => {}
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
//------------------- pragma_stmt ----------------
pub type Pragma_stmtContextAll<'input> = Pragma_stmtContext<'input>;

pub type Pragma_stmtContext<'input> = BaseParserRuleContext<'input, Pragma_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct Pragma_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Pragma_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Pragma_stmtContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_pragma_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_pragma_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Pragma_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_pragma_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_pragma_stmt }
}
antlr_rust::tid! {Pragma_stmtContextExt<'a>}

impl<'input> Pragma_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Pragma_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Pragma_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait Pragma_stmtContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Pragma_stmtContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token K_PRAGMA
    /// Returns `None` if there is no child corresponding to token K_PRAGMA
    fn K_PRAGMA(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_PRAGMA, 0)
    }
    fn pragma_name(&self) -> Option<Rc<Pragma_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn database_name(&self) -> Option<Rc<Database_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token DOT
    /// Returns `None` if there is no child corresponding to token DOT
    fn DOT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DOT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ASSIGN
    /// Returns `None` if there is no child corresponding to token ASSIGN
    fn ASSIGN(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ASSIGN, 0)
    }
    fn pragma_value(&self) -> Option<Rc<Pragma_valueContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token OPEN_PAR
    /// Returns `None` if there is no child corresponding to token OPEN_PAR
    fn OPEN_PAR(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPEN_PAR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token CLOSE_PAR
    /// Returns `None` if there is no child corresponding to token CLOSE_PAR
    fn CLOSE_PAR(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CLOSE_PAR, 0)
    }
}

impl<'input> Pragma_stmtContextAttrs<'input> for Pragma_stmtContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn pragma_stmt(&mut self) -> Result<Rc<Pragma_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Pragma_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 48, RULE_pragma_stmt);
        let mut _localctx: Rc<Pragma_stmtContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(725);
                recog.base.match_token(K_PRAGMA, &mut recog.err_handler)?;

                recog.base.set_state(729);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(90, &mut recog.base)? {
                    x if x == 1 => {
                        {
                            /*InvokeRule database_name*/
                            recog.base.set_state(726);
                            recog.database_name()?;

                            recog.base.set_state(727);
                            recog.base.match_token(DOT, &mut recog.err_handler)?;
                        }
                    }

                    _ => {}
                }
                /*InvokeRule pragma_name*/
                recog.base.set_state(731);
                recog.pragma_name()?;

                recog.base.set_state(738);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.base.input.la(1) {
                    ASSIGN => {
                        {
                            recog.base.set_state(732);
                            recog.base.match_token(ASSIGN, &mut recog.err_handler)?;

                            /*InvokeRule pragma_value*/
                            recog.base.set_state(733);
                            recog.pragma_value()?;
                        }
                    }

                    OPEN_PAR => {
                        {
                            recog.base.set_state(734);
                            recog.base.match_token(OPEN_PAR, &mut recog.err_handler)?;

                            /*InvokeRule pragma_value*/
                            recog.base.set_state(735);
                            recog.pragma_value()?;

                            recog.base.set_state(736);
                            recog.base.match_token(CLOSE_PAR, &mut recog.err_handler)?;
                        }
                    }

                    EOF | SCOL | K_ALTER | K_ANALYZE | K_ATTACH | K_BEGIN | K_COMMIT | K_CREATE
                    | K_DELETE | K_DETACH | K_DROP | K_END | K_EXPLAIN | K_INSERT | K_PRAGMA
                    | K_REINDEX | K_RELEASE | K_REPLACE | K_ROLLBACK | K_SAVEPOINT | K_SELECT
                    | K_UPDATE | K_VACUUM | K_VALUES | K_WITH | UNEXPECTED_CHAR => {}

                    _ => {}
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
//------------------- reindex_stmt ----------------
pub type Reindex_stmtContextAll<'input> = Reindex_stmtContext<'input>;

pub type Reindex_stmtContext<'input> =
    BaseParserRuleContext<'input, Reindex_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct Reindex_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Reindex_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Reindex_stmtContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_reindex_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_reindex_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Reindex_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_reindex_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_reindex_stmt }
}
antlr_rust::tid! {Reindex_stmtContextExt<'a>}

impl<'input> Reindex_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Reindex_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Reindex_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait Reindex_stmtContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Reindex_stmtContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token K_REINDEX
    /// Returns `None` if there is no child corresponding to token K_REINDEX
    fn K_REINDEX(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_REINDEX, 0)
    }
    fn collation_name(&self) -> Option<Rc<Collation_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn table_name(&self) -> Option<Rc<Table_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn index_name(&self) -> Option<Rc<Index_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn database_name(&self) -> Option<Rc<Database_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token DOT
    /// Returns `None` if there is no child corresponding to token DOT
    fn DOT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DOT, 0)
    }
}

impl<'input> Reindex_stmtContextAttrs<'input> for Reindex_stmtContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn reindex_stmt(&mut self) -> Result<Rc<Reindex_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Reindex_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 50, RULE_reindex_stmt);
        let mut _localctx: Rc<Reindex_stmtContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(740);
                recog.base.match_token(K_REINDEX, &mut recog.err_handler)?;

                recog.base.set_state(751);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(94, &mut recog.base)? {
                    x if x == 1 => {
                        {
                            /*InvokeRule collation_name*/
                            recog.base.set_state(741);
                            recog.collation_name()?;
                        }
                    }

                    x if x == 2 => {
                        {
                            recog.base.set_state(745);
                            recog.err_handler.sync(&mut recog.base)?;
                            match recog.interpreter.adaptive_predict(92, &mut recog.base)? {
                                x if x == 1 => {
                                    {
                                        /*InvokeRule database_name*/
                                        recog.base.set_state(742);
                                        recog.database_name()?;

                                        recog.base.set_state(743);
                                        recog.base.match_token(DOT, &mut recog.err_handler)?;
                                    }
                                }

                                _ => {}
                            }
                            recog.base.set_state(749);
                            recog.err_handler.sync(&mut recog.base)?;
                            match recog.interpreter.adaptive_predict(93, &mut recog.base)? {
                                1 => {
                                    {
                                        /*InvokeRule table_name*/
                                        recog.base.set_state(747);
                                        recog.table_name()?;
                                    }
                                }
                                2 => {
                                    {
                                        /*InvokeRule index_name*/
                                        recog.base.set_state(748);
                                        recog.index_name()?;
                                    }
                                }

                                _ => {}
                            }
                        }
                    }

                    _ => {}
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
//------------------- release_stmt ----------------
pub type Release_stmtContextAll<'input> = Release_stmtContext<'input>;

pub type Release_stmtContext<'input> =
    BaseParserRuleContext<'input, Release_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct Release_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Release_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Release_stmtContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_release_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_release_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Release_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_release_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_release_stmt }
}
antlr_rust::tid! {Release_stmtContextExt<'a>}

impl<'input> Release_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Release_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Release_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait Release_stmtContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Release_stmtContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token K_RELEASE
    /// Returns `None` if there is no child corresponding to token K_RELEASE
    fn K_RELEASE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_RELEASE, 0)
    }
    fn savepoint_name(&self) -> Option<Rc<Savepoint_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_SAVEPOINT
    /// Returns `None` if there is no child corresponding to token K_SAVEPOINT
    fn K_SAVEPOINT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_SAVEPOINT, 0)
    }
}

impl<'input> Release_stmtContextAttrs<'input> for Release_stmtContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn release_stmt(&mut self) -> Result<Rc<Release_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Release_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 52, RULE_release_stmt);
        let mut _localctx: Rc<Release_stmtContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(753);
                recog.base.match_token(K_RELEASE, &mut recog.err_handler)?;

                recog.base.set_state(755);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(95, &mut recog.base)? {
                    x if x == 1 => {
                        recog.base.set_state(754);
                        recog
                            .base
                            .match_token(K_SAVEPOINT, &mut recog.err_handler)?;
                    }

                    _ => {}
                }
                /*InvokeRule savepoint_name*/
                recog.base.set_state(757);
                recog.savepoint_name()?;
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
//------------------- rollback_stmt ----------------
pub type Rollback_stmtContextAll<'input> = Rollback_stmtContext<'input>;

pub type Rollback_stmtContext<'input> =
    BaseParserRuleContext<'input, Rollback_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct Rollback_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Rollback_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Rollback_stmtContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_rollback_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_rollback_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Rollback_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_rollback_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_rollback_stmt }
}
antlr_rust::tid! {Rollback_stmtContextExt<'a>}

impl<'input> Rollback_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Rollback_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Rollback_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait Rollback_stmtContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Rollback_stmtContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token K_ROLLBACK
    /// Returns `None` if there is no child corresponding to token K_ROLLBACK
    fn K_ROLLBACK(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ROLLBACK, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_TRANSACTION
    /// Returns `None` if there is no child corresponding to token K_TRANSACTION
    fn K_TRANSACTION(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_TRANSACTION, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_TO
    /// Returns `None` if there is no child corresponding to token K_TO
    fn K_TO(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_TO, 0)
    }
    fn savepoint_name(&self) -> Option<Rc<Savepoint_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn transaction_name(&self) -> Option<Rc<Transaction_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_SAVEPOINT
    /// Returns `None` if there is no child corresponding to token K_SAVEPOINT
    fn K_SAVEPOINT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_SAVEPOINT, 0)
    }
}

impl<'input> Rollback_stmtContextAttrs<'input> for Rollback_stmtContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn rollback_stmt(&mut self) -> Result<Rc<Rollback_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Rollback_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 54, RULE_rollback_stmt);
        let mut _localctx: Rc<Rollback_stmtContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(759);
                recog.base.match_token(K_ROLLBACK, &mut recog.err_handler)?;

                recog.base.set_state(764);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == K_TRANSACTION {
                    {
                        recog.base.set_state(760);
                        recog
                            .base
                            .match_token(K_TRANSACTION, &mut recog.err_handler)?;

                        recog.base.set_state(762);
                        recog.err_handler.sync(&mut recog.base)?;
                        match recog.interpreter.adaptive_predict(96, &mut recog.base)? {
                            x if x == 1 => {
                                {
                                    /*InvokeRule transaction_name*/
                                    recog.base.set_state(761);
                                    recog.transaction_name()?;
                                }
                            }

                            _ => {}
                        }
                    }
                }

                recog.base.set_state(771);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == K_TO {
                    {
                        recog.base.set_state(766);
                        recog.base.match_token(K_TO, &mut recog.err_handler)?;

                        recog.base.set_state(768);
                        recog.err_handler.sync(&mut recog.base)?;
                        match recog.interpreter.adaptive_predict(98, &mut recog.base)? {
                            x if x == 1 => {
                                recog.base.set_state(767);
                                recog
                                    .base
                                    .match_token(K_SAVEPOINT, &mut recog.err_handler)?;
                            }

                            _ => {}
                        }
                        /*InvokeRule savepoint_name*/
                        recog.base.set_state(770);
                        recog.savepoint_name()?;
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
//------------------- savepoint_stmt ----------------
pub type Savepoint_stmtContextAll<'input> = Savepoint_stmtContext<'input>;

pub type Savepoint_stmtContext<'input> =
    BaseParserRuleContext<'input, Savepoint_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct Savepoint_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Savepoint_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Savepoint_stmtContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_savepoint_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_savepoint_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Savepoint_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_savepoint_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_savepoint_stmt }
}
antlr_rust::tid! {Savepoint_stmtContextExt<'a>}

impl<'input> Savepoint_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Savepoint_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Savepoint_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait Savepoint_stmtContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Savepoint_stmtContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token K_SAVEPOINT
    /// Returns `None` if there is no child corresponding to token K_SAVEPOINT
    fn K_SAVEPOINT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_SAVEPOINT, 0)
    }
    fn savepoint_name(&self) -> Option<Rc<Savepoint_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Savepoint_stmtContextAttrs<'input> for Savepoint_stmtContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn savepoint_stmt(&mut self) -> Result<Rc<Savepoint_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Savepoint_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 56, RULE_savepoint_stmt);
        let mut _localctx: Rc<Savepoint_stmtContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(773);
                recog
                    .base
                    .match_token(K_SAVEPOINT, &mut recog.err_handler)?;

                /*InvokeRule savepoint_name*/
                recog.base.set_state(774);
                recog.savepoint_name()?;
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
//------------------- simple_select_stmt ----------------
pub type Simple_select_stmtContextAll<'input> = Simple_select_stmtContext<'input>;

pub type Simple_select_stmtContext<'input> =
    BaseParserRuleContext<'input, Simple_select_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct Simple_select_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Simple_select_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Simple_select_stmtContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_simple_select_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_simple_select_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Simple_select_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_simple_select_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_simple_select_stmt }
}
antlr_rust::tid! {Simple_select_stmtContextExt<'a>}

impl<'input> Simple_select_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Simple_select_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Simple_select_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait Simple_select_stmtContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Simple_select_stmtContextExt<'input>>
{
    fn select_core(&self) -> Option<Rc<Select_coreContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn with_clause(&self) -> Option<Rc<With_clauseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_ORDER
    /// Returns `None` if there is no child corresponding to token K_ORDER
    fn K_ORDER(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ORDER, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_BY
    /// Returns `None` if there is no child corresponding to token K_BY
    fn K_BY(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_BY, 0)
    }
    fn ordering_term_all(&self) -> Vec<Rc<Ordering_termContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn ordering_term(&self, i: usize) -> Option<Rc<Ordering_termContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token K_LIMIT
    /// Returns `None` if there is no child corresponding to token K_LIMIT
    fn K_LIMIT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_LIMIT, 0)
    }
    fn expr_all(&self) -> Vec<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
    /// Retrieves first TerminalNode corresponding to token K_OFFSET
    /// Returns `None` if there is no child corresponding to token K_OFFSET
    fn K_OFFSET(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_OFFSET, 0)
    }
}

impl<'input> Simple_select_stmtContextAttrs<'input> for Simple_select_stmtContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn simple_select_stmt(
        &mut self,
    ) -> Result<Rc<Simple_select_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Simple_select_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 58, RULE_simple_select_stmt);
        let mut _localctx: Rc<Simple_select_stmtContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(777);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == K_WITH {
                    {
                        /*InvokeRule with_clause*/
                        recog.base.set_state(776);
                        recog.with_clause()?;
                    }
                }

                /*InvokeRule select_core*/
                recog.base.set_state(779);
                recog.select_core()?;

                recog.base.set_state(790);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == K_ORDER {
                    {
                        recog.base.set_state(780);
                        recog.base.match_token(K_ORDER, &mut recog.err_handler)?;

                        recog.base.set_state(781);
                        recog.base.match_token(K_BY, &mut recog.err_handler)?;

                        /*InvokeRule ordering_term*/
                        recog.base.set_state(782);
                        recog.ordering_term()?;

                        recog.base.set_state(787);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        while _la == COMMA {
                            {
                                {
                                    recog.base.set_state(783);
                                    recog.base.match_token(COMMA, &mut recog.err_handler)?;

                                    /*InvokeRule ordering_term*/
                                    recog.base.set_state(784);
                                    recog.ordering_term()?;
                                }
                            }
                            recog.base.set_state(789);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                        }
                    }
                }

                recog.base.set_state(798);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == K_LIMIT {
                    {
                        recog.base.set_state(792);
                        recog.base.match_token(K_LIMIT, &mut recog.err_handler)?;

                        /*InvokeRule expr*/
                        recog.base.set_state(793);
                        recog.expr_rec(0)?;

                        recog.base.set_state(796);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == COMMA || _la == K_OFFSET {
                            {
                                recog.base.set_state(794);
                                _la = recog.base.input.la(1);
                                if !(_la == COMMA || _la == K_OFFSET) {
                                    recog.err_handler.recover_inline(&mut recog.base)?;
                                } else {
                                    if recog.base.input.la(1) == TOKEN_EOF {
                                        recog.base.matched_eof = true
                                    };
                                    recog.err_handler.report_match(&mut recog.base);
                                    recog.base.consume(&mut recog.err_handler);
                                }
                                /*InvokeRule expr*/
                                recog.base.set_state(795);
                                recog.expr_rec(0)?;
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
//------------------- select_stmt ----------------
pub type Select_stmtContextAll<'input> = Select_stmtContext<'input>;

pub type Select_stmtContext<'input> = BaseParserRuleContext<'input, Select_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct Select_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Select_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Select_stmtContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_select_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_select_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Select_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_select_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_select_stmt }
}
antlr_rust::tid! {Select_stmtContextExt<'a>}

impl<'input> Select_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Select_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Select_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait Select_stmtContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Select_stmtContextExt<'input>>
{
    fn select_or_values_all(&self) -> Vec<Rc<Select_or_valuesContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn select_or_values(&self, i: usize) -> Option<Rc<Select_or_valuesContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn with_clause(&self) -> Option<Rc<With_clauseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn compound_operator_all(&self) -> Vec<Rc<Compound_operatorContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn compound_operator(&self, i: usize) -> Option<Rc<Compound_operatorContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token K_ORDER
    /// Returns `None` if there is no child corresponding to token K_ORDER
    fn K_ORDER(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ORDER, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_BY
    /// Returns `None` if there is no child corresponding to token K_BY
    fn K_BY(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_BY, 0)
    }
    fn ordering_term_all(&self) -> Vec<Rc<Ordering_termContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn ordering_term(&self, i: usize) -> Option<Rc<Ordering_termContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token K_LIMIT
    /// Returns `None` if there is no child corresponding to token K_LIMIT
    fn K_LIMIT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_LIMIT, 0)
    }
    fn expr_all(&self) -> Vec<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
    /// Retrieves first TerminalNode corresponding to token K_OFFSET
    /// Returns `None` if there is no child corresponding to token K_OFFSET
    fn K_OFFSET(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_OFFSET, 0)
    }
}

impl<'input> Select_stmtContextAttrs<'input> for Select_stmtContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn select_stmt(&mut self) -> Result<Rc<Select_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Select_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 60, RULE_select_stmt);
        let mut _localctx: Rc<Select_stmtContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(801);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == K_WITH {
                    {
                        /*InvokeRule with_clause*/
                        recog.base.set_state(800);
                        recog.with_clause()?;
                    }
                }

                /*InvokeRule select_or_values*/
                recog.base.set_state(803);
                recog.select_or_values()?;

                recog.base.set_state(809);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == K_EXCEPT || _la == K_INTERSECT || _la == K_UNION {
                    {
                        {
                            /*InvokeRule compound_operator*/
                            recog.base.set_state(804);
                            recog.compound_operator()?;

                            /*InvokeRule select_or_values*/
                            recog.base.set_state(805);
                            recog.select_or_values()?;
                        }
                    }
                    recog.base.set_state(811);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(822);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == K_ORDER {
                    {
                        recog.base.set_state(812);
                        recog.base.match_token(K_ORDER, &mut recog.err_handler)?;

                        recog.base.set_state(813);
                        recog.base.match_token(K_BY, &mut recog.err_handler)?;

                        /*InvokeRule ordering_term*/
                        recog.base.set_state(814);
                        recog.ordering_term()?;

                        recog.base.set_state(819);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        while _la == COMMA {
                            {
                                {
                                    recog.base.set_state(815);
                                    recog.base.match_token(COMMA, &mut recog.err_handler)?;

                                    /*InvokeRule ordering_term*/
                                    recog.base.set_state(816);
                                    recog.ordering_term()?;
                                }
                            }
                            recog.base.set_state(821);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                        }
                    }
                }

                recog.base.set_state(830);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == K_LIMIT {
                    {
                        recog.base.set_state(824);
                        recog.base.match_token(K_LIMIT, &mut recog.err_handler)?;

                        /*InvokeRule expr*/
                        recog.base.set_state(825);
                        recog.expr_rec(0)?;

                        recog.base.set_state(828);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == COMMA || _la == K_OFFSET {
                            {
                                recog.base.set_state(826);
                                _la = recog.base.input.la(1);
                                if !(_la == COMMA || _la == K_OFFSET) {
                                    recog.err_handler.recover_inline(&mut recog.base)?;
                                } else {
                                    if recog.base.input.la(1) == TOKEN_EOF {
                                        recog.base.matched_eof = true
                                    };
                                    recog.err_handler.report_match(&mut recog.base);
                                    recog.base.consume(&mut recog.err_handler);
                                }
                                /*InvokeRule expr*/
                                recog.base.set_state(827);
                                recog.expr_rec(0)?;
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
//------------------- select_or_values ----------------
pub type Select_or_valuesContextAll<'input> = Select_or_valuesContext<'input>;

pub type Select_or_valuesContext<'input> =
    BaseParserRuleContext<'input, Select_or_valuesContextExt<'input>>;

#[derive(Clone)]
pub struct Select_or_valuesContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Select_or_valuesContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Select_or_valuesContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_select_or_values(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_select_or_values(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Select_or_valuesContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_select_or_values
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_select_or_values }
}
antlr_rust::tid! {Select_or_valuesContextExt<'a>}

impl<'input> Select_or_valuesContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Select_or_valuesContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Select_or_valuesContextExt { ph: PhantomData },
        ))
    }
}

pub trait Select_or_valuesContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Select_or_valuesContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token K_SELECT
    /// Returns `None` if there is no child corresponding to token K_SELECT
    fn K_SELECT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_SELECT, 0)
    }
    fn result_column_all(&self) -> Vec<Rc<Result_columnContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn result_column(&self, i: usize) -> Option<Rc<Result_columnContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
    /// Retrieves first TerminalNode corresponding to token K_FROM
    /// Returns `None` if there is no child corresponding to token K_FROM
    fn K_FROM(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_FROM, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_WHERE
    /// Returns `None` if there is no child corresponding to token K_WHERE
    fn K_WHERE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_WHERE, 0)
    }
    fn expr_all(&self) -> Vec<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token K_GROUP
    /// Returns `None` if there is no child corresponding to token K_GROUP
    fn K_GROUP(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_GROUP, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_BY
    /// Returns `None` if there is no child corresponding to token K_BY
    fn K_BY(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_BY, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_DISTINCT
    /// Returns `None` if there is no child corresponding to token K_DISTINCT
    fn K_DISTINCT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_DISTINCT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_ALL
    /// Returns `None` if there is no child corresponding to token K_ALL
    fn K_ALL(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ALL, 0)
    }
    fn table_or_subquery_all(&self) -> Vec<Rc<Table_or_subqueryContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn table_or_subquery(&self, i: usize) -> Option<Rc<Table_or_subqueryContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn join_clause(&self) -> Option<Rc<Join_clauseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_HAVING
    /// Returns `None` if there is no child corresponding to token K_HAVING
    fn K_HAVING(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_HAVING, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_VALUES
    /// Returns `None` if there is no child corresponding to token K_VALUES
    fn K_VALUES(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_VALUES, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token OPEN_PAR in current rule
    fn OPEN_PAR_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token OPEN_PAR, starting from 0.
    /// Returns `None` if number of children corresponding to token OPEN_PAR is less or equal than `i`.
    fn OPEN_PAR(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPEN_PAR, i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token CLOSE_PAR in current rule
    fn CLOSE_PAR_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token CLOSE_PAR, starting from 0.
    /// Returns `None` if number of children corresponding to token CLOSE_PAR is less or equal than `i`.
    fn CLOSE_PAR(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CLOSE_PAR, i)
    }
}

impl<'input> Select_or_valuesContextAttrs<'input> for Select_or_valuesContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn select_or_values(
        &mut self,
    ) -> Result<Rc<Select_or_valuesContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Select_or_valuesContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 62, RULE_select_or_values);
        let mut _localctx: Rc<Select_or_valuesContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(906);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                K_SELECT => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        recog.base.set_state(832);
                        recog.base.match_token(K_SELECT, &mut recog.err_handler)?;

                        recog.base.set_state(834);
                        recog.err_handler.sync(&mut recog.base)?;
                        match recog.interpreter.adaptive_predict(111, &mut recog.base)? {
                            x if x == 1 => {
                                recog.base.set_state(833);
                                _la = recog.base.input.la(1);
                                if !(_la == K_ALL || _la == K_DISTINCT) {
                                    recog.err_handler.recover_inline(&mut recog.base)?;
                                } else {
                                    if recog.base.input.la(1) == TOKEN_EOF {
                                        recog.base.matched_eof = true
                                    };
                                    recog.err_handler.report_match(&mut recog.base);
                                    recog.base.consume(&mut recog.err_handler);
                                }
                            }

                            _ => {}
                        }
                        /*InvokeRule result_column*/
                        recog.base.set_state(836);
                        recog.result_column()?;

                        recog.base.set_state(841);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        while _la == COMMA {
                            {
                                {
                                    recog.base.set_state(837);
                                    recog.base.match_token(COMMA, &mut recog.err_handler)?;

                                    /*InvokeRule result_column*/
                                    recog.base.set_state(838);
                                    recog.result_column()?;
                                }
                            }
                            recog.base.set_state(843);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                        }
                        recog.base.set_state(856);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == K_FROM {
                            {
                                recog.base.set_state(844);
                                recog.base.match_token(K_FROM, &mut recog.err_handler)?;

                                recog.base.set_state(854);
                                recog.err_handler.sync(&mut recog.base)?;
                                match recog.interpreter.adaptive_predict(114, &mut recog.base)? {
                                    1 => {
                                        {
                                            /*InvokeRule table_or_subquery*/
                                            recog.base.set_state(845);
                                            recog.table_or_subquery()?;

                                            recog.base.set_state(850);
                                            recog.err_handler.sync(&mut recog.base)?;
                                            _la = recog.base.input.la(1);
                                            while _la == COMMA {
                                                {
                                                    {
                                                        recog.base.set_state(846);
                                                        recog.base.match_token(
                                                            COMMA,
                                                            &mut recog.err_handler,
                                                        )?;

                                                        /*InvokeRule table_or_subquery*/
                                                        recog.base.set_state(847);
                                                        recog.table_or_subquery()?;
                                                    }
                                                }
                                                recog.base.set_state(852);
                                                recog.err_handler.sync(&mut recog.base)?;
                                                _la = recog.base.input.la(1);
                                            }
                                        }
                                    }
                                    2 => {
                                        {
                                            /*InvokeRule join_clause*/
                                            recog.base.set_state(853);
                                            recog.join_clause()?;
                                        }
                                    }

                                    _ => {}
                                }
                            }
                        }

                        recog.base.set_state(860);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == K_WHERE {
                            {
                                recog.base.set_state(858);
                                recog.base.match_token(K_WHERE, &mut recog.err_handler)?;

                                /*InvokeRule expr*/
                                recog.base.set_state(859);
                                recog.expr_rec(0)?;
                            }
                        }

                        recog.base.set_state(876);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == K_GROUP {
                            {
                                recog.base.set_state(862);
                                recog.base.match_token(K_GROUP, &mut recog.err_handler)?;

                                recog.base.set_state(863);
                                recog.base.match_token(K_BY, &mut recog.err_handler)?;

                                /*InvokeRule expr*/
                                recog.base.set_state(864);
                                recog.expr_rec(0)?;

                                recog.base.set_state(869);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                                while _la == COMMA {
                                    {
                                        {
                                            recog.base.set_state(865);
                                            recog
                                                .base
                                                .match_token(COMMA, &mut recog.err_handler)?;

                                            /*InvokeRule expr*/
                                            recog.base.set_state(866);
                                            recog.expr_rec(0)?;
                                        }
                                    }
                                    recog.base.set_state(871);
                                    recog.err_handler.sync(&mut recog.base)?;
                                    _la = recog.base.input.la(1);
                                }
                                recog.base.set_state(874);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                                if _la == K_HAVING {
                                    {
                                        recog.base.set_state(872);
                                        recog.base.match_token(K_HAVING, &mut recog.err_handler)?;

                                        /*InvokeRule expr*/
                                        recog.base.set_state(873);
                                        recog.expr_rec(0)?;
                                    }
                                }
                            }
                        }
                    }
                }

                K_VALUES => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        recog.base.set_state(878);
                        recog.base.match_token(K_VALUES, &mut recog.err_handler)?;

                        recog.base.set_state(879);
                        recog.base.match_token(OPEN_PAR, &mut recog.err_handler)?;

                        /*InvokeRule expr*/
                        recog.base.set_state(880);
                        recog.expr_rec(0)?;

                        recog.base.set_state(885);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        while _la == COMMA {
                            {
                                {
                                    recog.base.set_state(881);
                                    recog.base.match_token(COMMA, &mut recog.err_handler)?;

                                    /*InvokeRule expr*/
                                    recog.base.set_state(882);
                                    recog.expr_rec(0)?;
                                }
                            }
                            recog.base.set_state(887);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                        }
                        recog.base.set_state(888);
                        recog.base.match_token(CLOSE_PAR, &mut recog.err_handler)?;

                        recog.base.set_state(903);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        while _la == COMMA {
                            {
                                {
                                    recog.base.set_state(889);
                                    recog.base.match_token(COMMA, &mut recog.err_handler)?;

                                    recog.base.set_state(890);
                                    recog.base.match_token(OPEN_PAR, &mut recog.err_handler)?;

                                    /*InvokeRule expr*/
                                    recog.base.set_state(891);
                                    recog.expr_rec(0)?;

                                    recog.base.set_state(896);
                                    recog.err_handler.sync(&mut recog.base)?;
                                    _la = recog.base.input.la(1);
                                    while _la == COMMA {
                                        {
                                            {
                                                recog.base.set_state(892);
                                                recog
                                                    .base
                                                    .match_token(COMMA, &mut recog.err_handler)?;

                                                /*InvokeRule expr*/
                                                recog.base.set_state(893);
                                                recog.expr_rec(0)?;
                                            }
                                        }
                                        recog.base.set_state(898);
                                        recog.err_handler.sync(&mut recog.base)?;
                                        _la = recog.base.input.la(1);
                                    }
                                    recog.base.set_state(899);
                                    recog.base.match_token(CLOSE_PAR, &mut recog.err_handler)?;
                                }
                            }
                            recog.base.set_state(905);
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
//------------------- update_stmt ----------------
pub type Update_stmtContextAll<'input> = Update_stmtContext<'input>;

pub type Update_stmtContext<'input> = BaseParserRuleContext<'input, Update_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct Update_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Update_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Update_stmtContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_update_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_update_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Update_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_update_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_update_stmt }
}
antlr_rust::tid! {Update_stmtContextExt<'a>}

impl<'input> Update_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Update_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Update_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait Update_stmtContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Update_stmtContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token K_UPDATE
    /// Returns `None` if there is no child corresponding to token K_UPDATE
    fn K_UPDATE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_UPDATE, 0)
    }
    fn qualified_table_name(&self) -> Option<Rc<Qualified_table_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_SET
    /// Returns `None` if there is no child corresponding to token K_SET
    fn K_SET(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_SET, 0)
    }
    fn column_name_all(&self) -> Vec<Rc<Column_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn column_name(&self, i: usize) -> Option<Rc<Column_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token ASSIGN in current rule
    fn ASSIGN_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token ASSIGN, starting from 0.
    /// Returns `None` if number of children corresponding to token ASSIGN is less or equal than `i`.
    fn ASSIGN(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ASSIGN, i)
    }
    fn expr_all(&self) -> Vec<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn with_clause(&self) -> Option<Rc<With_clauseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_OR
    /// Returns `None` if there is no child corresponding to token K_OR
    fn K_OR(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_OR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_ROLLBACK
    /// Returns `None` if there is no child corresponding to token K_ROLLBACK
    fn K_ROLLBACK(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ROLLBACK, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_ABORT
    /// Returns `None` if there is no child corresponding to token K_ABORT
    fn K_ABORT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ABORT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_REPLACE
    /// Returns `None` if there is no child corresponding to token K_REPLACE
    fn K_REPLACE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_REPLACE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_FAIL
    /// Returns `None` if there is no child corresponding to token K_FAIL
    fn K_FAIL(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_FAIL, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_IGNORE
    /// Returns `None` if there is no child corresponding to token K_IGNORE
    fn K_IGNORE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_IGNORE, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
    /// Retrieves first TerminalNode corresponding to token K_WHERE
    /// Returns `None` if there is no child corresponding to token K_WHERE
    fn K_WHERE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_WHERE, 0)
    }
}

impl<'input> Update_stmtContextAttrs<'input> for Update_stmtContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn update_stmt(&mut self) -> Result<Rc<Update_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Update_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 64, RULE_update_stmt);
        let mut _localctx: Rc<Update_stmtContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(909);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == K_WITH {
                    {
                        /*InvokeRule with_clause*/
                        recog.base.set_state(908);
                        recog.with_clause()?;
                    }
                }

                recog.base.set_state(911);
                recog.base.match_token(K_UPDATE, &mut recog.err_handler)?;

                recog.base.set_state(922);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(125, &mut recog.base)? {
                    x if x == 1 => {
                        recog.base.set_state(912);
                        recog.base.match_token(K_OR, &mut recog.err_handler)?;

                        recog.base.set_state(913);
                        recog.base.match_token(K_ROLLBACK, &mut recog.err_handler)?;
                    }

                    x if x == 2 => {
                        recog.base.set_state(914);
                        recog.base.match_token(K_OR, &mut recog.err_handler)?;

                        recog.base.set_state(915);
                        recog.base.match_token(K_ABORT, &mut recog.err_handler)?;
                    }

                    x if x == 3 => {
                        recog.base.set_state(916);
                        recog.base.match_token(K_OR, &mut recog.err_handler)?;

                        recog.base.set_state(917);
                        recog.base.match_token(K_REPLACE, &mut recog.err_handler)?;
                    }

                    x if x == 4 => {
                        recog.base.set_state(918);
                        recog.base.match_token(K_OR, &mut recog.err_handler)?;

                        recog.base.set_state(919);
                        recog.base.match_token(K_FAIL, &mut recog.err_handler)?;
                    }

                    x if x == 5 => {
                        recog.base.set_state(920);
                        recog.base.match_token(K_OR, &mut recog.err_handler)?;

                        recog.base.set_state(921);
                        recog.base.match_token(K_IGNORE, &mut recog.err_handler)?;
                    }

                    _ => {}
                }
                /*InvokeRule qualified_table_name*/
                recog.base.set_state(924);
                recog.qualified_table_name()?;

                recog.base.set_state(925);
                recog.base.match_token(K_SET, &mut recog.err_handler)?;

                /*InvokeRule column_name*/
                recog.base.set_state(926);
                recog.column_name()?;

                recog.base.set_state(927);
                recog.base.match_token(ASSIGN, &mut recog.err_handler)?;

                /*InvokeRule expr*/
                recog.base.set_state(928);
                recog.expr_rec(0)?;

                recog.base.set_state(936);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == COMMA {
                    {
                        {
                            recog.base.set_state(929);
                            recog.base.match_token(COMMA, &mut recog.err_handler)?;

                            /*InvokeRule column_name*/
                            recog.base.set_state(930);
                            recog.column_name()?;

                            recog.base.set_state(931);
                            recog.base.match_token(ASSIGN, &mut recog.err_handler)?;

                            /*InvokeRule expr*/
                            recog.base.set_state(932);
                            recog.expr_rec(0)?;
                        }
                    }
                    recog.base.set_state(938);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(941);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == K_WHERE {
                    {
                        recog.base.set_state(939);
                        recog.base.match_token(K_WHERE, &mut recog.err_handler)?;

                        /*InvokeRule expr*/
                        recog.base.set_state(940);
                        recog.expr_rec(0)?;
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
//------------------- update_stmt_limited ----------------
pub type Update_stmt_limitedContextAll<'input> = Update_stmt_limitedContext<'input>;

pub type Update_stmt_limitedContext<'input> =
    BaseParserRuleContext<'input, Update_stmt_limitedContextExt<'input>>;

#[derive(Clone)]
pub struct Update_stmt_limitedContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Update_stmt_limitedContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a>
    for Update_stmt_limitedContext<'input>
{
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_update_stmt_limited(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_update_stmt_limited(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Update_stmt_limitedContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_update_stmt_limited
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_update_stmt_limited }
}
antlr_rust::tid! {Update_stmt_limitedContextExt<'a>}

impl<'input> Update_stmt_limitedContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Update_stmt_limitedContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Update_stmt_limitedContextExt { ph: PhantomData },
        ))
    }
}

pub trait Update_stmt_limitedContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Update_stmt_limitedContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token K_UPDATE
    /// Returns `None` if there is no child corresponding to token K_UPDATE
    fn K_UPDATE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_UPDATE, 0)
    }
    fn qualified_table_name(&self) -> Option<Rc<Qualified_table_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_SET
    /// Returns `None` if there is no child corresponding to token K_SET
    fn K_SET(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_SET, 0)
    }
    fn column_name_all(&self) -> Vec<Rc<Column_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn column_name(&self, i: usize) -> Option<Rc<Column_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token ASSIGN in current rule
    fn ASSIGN_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token ASSIGN, starting from 0.
    /// Returns `None` if number of children corresponding to token ASSIGN is less or equal than `i`.
    fn ASSIGN(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ASSIGN, i)
    }
    fn expr_all(&self) -> Vec<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn with_clause(&self) -> Option<Rc<With_clauseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_OR
    /// Returns `None` if there is no child corresponding to token K_OR
    fn K_OR(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_OR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_ROLLBACK
    /// Returns `None` if there is no child corresponding to token K_ROLLBACK
    fn K_ROLLBACK(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ROLLBACK, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_ABORT
    /// Returns `None` if there is no child corresponding to token K_ABORT
    fn K_ABORT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ABORT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_REPLACE
    /// Returns `None` if there is no child corresponding to token K_REPLACE
    fn K_REPLACE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_REPLACE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_FAIL
    /// Returns `None` if there is no child corresponding to token K_FAIL
    fn K_FAIL(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_FAIL, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_IGNORE
    /// Returns `None` if there is no child corresponding to token K_IGNORE
    fn K_IGNORE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_IGNORE, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
    /// Retrieves first TerminalNode corresponding to token K_WHERE
    /// Returns `None` if there is no child corresponding to token K_WHERE
    fn K_WHERE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_WHERE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_LIMIT
    /// Returns `None` if there is no child corresponding to token K_LIMIT
    fn K_LIMIT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_LIMIT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_ORDER
    /// Returns `None` if there is no child corresponding to token K_ORDER
    fn K_ORDER(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ORDER, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_BY
    /// Returns `None` if there is no child corresponding to token K_BY
    fn K_BY(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_BY, 0)
    }
    fn ordering_term_all(&self) -> Vec<Rc<Ordering_termContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn ordering_term(&self, i: usize) -> Option<Rc<Ordering_termContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token K_OFFSET
    /// Returns `None` if there is no child corresponding to token K_OFFSET
    fn K_OFFSET(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_OFFSET, 0)
    }
}

impl<'input> Update_stmt_limitedContextAttrs<'input> for Update_stmt_limitedContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn update_stmt_limited(
        &mut self,
    ) -> Result<Rc<Update_stmt_limitedContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Update_stmt_limitedContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 66, RULE_update_stmt_limited);
        let mut _localctx: Rc<Update_stmt_limitedContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(944);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == K_WITH {
                    {
                        /*InvokeRule with_clause*/
                        recog.base.set_state(943);
                        recog.with_clause()?;
                    }
                }

                recog.base.set_state(946);
                recog.base.match_token(K_UPDATE, &mut recog.err_handler)?;

                recog.base.set_state(957);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(129, &mut recog.base)? {
                    x if x == 1 => {
                        recog.base.set_state(947);
                        recog.base.match_token(K_OR, &mut recog.err_handler)?;

                        recog.base.set_state(948);
                        recog.base.match_token(K_ROLLBACK, &mut recog.err_handler)?;
                    }

                    x if x == 2 => {
                        recog.base.set_state(949);
                        recog.base.match_token(K_OR, &mut recog.err_handler)?;

                        recog.base.set_state(950);
                        recog.base.match_token(K_ABORT, &mut recog.err_handler)?;
                    }

                    x if x == 3 => {
                        recog.base.set_state(951);
                        recog.base.match_token(K_OR, &mut recog.err_handler)?;

                        recog.base.set_state(952);
                        recog.base.match_token(K_REPLACE, &mut recog.err_handler)?;
                    }

                    x if x == 4 => {
                        recog.base.set_state(953);
                        recog.base.match_token(K_OR, &mut recog.err_handler)?;

                        recog.base.set_state(954);
                        recog.base.match_token(K_FAIL, &mut recog.err_handler)?;
                    }

                    x if x == 5 => {
                        recog.base.set_state(955);
                        recog.base.match_token(K_OR, &mut recog.err_handler)?;

                        recog.base.set_state(956);
                        recog.base.match_token(K_IGNORE, &mut recog.err_handler)?;
                    }

                    _ => {}
                }
                /*InvokeRule qualified_table_name*/
                recog.base.set_state(959);
                recog.qualified_table_name()?;

                recog.base.set_state(960);
                recog.base.match_token(K_SET, &mut recog.err_handler)?;

                /*InvokeRule column_name*/
                recog.base.set_state(961);
                recog.column_name()?;

                recog.base.set_state(962);
                recog.base.match_token(ASSIGN, &mut recog.err_handler)?;

                /*InvokeRule expr*/
                recog.base.set_state(963);
                recog.expr_rec(0)?;

                recog.base.set_state(971);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == COMMA {
                    {
                        {
                            recog.base.set_state(964);
                            recog.base.match_token(COMMA, &mut recog.err_handler)?;

                            /*InvokeRule column_name*/
                            recog.base.set_state(965);
                            recog.column_name()?;

                            recog.base.set_state(966);
                            recog.base.match_token(ASSIGN, &mut recog.err_handler)?;

                            /*InvokeRule expr*/
                            recog.base.set_state(967);
                            recog.expr_rec(0)?;
                        }
                    }
                    recog.base.set_state(973);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(976);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == K_WHERE {
                    {
                        recog.base.set_state(974);
                        recog.base.match_token(K_WHERE, &mut recog.err_handler)?;

                        /*InvokeRule expr*/
                        recog.base.set_state(975);
                        recog.expr_rec(0)?;
                    }
                }

                recog.base.set_state(996);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == K_LIMIT || _la == K_ORDER {
                    {
                        recog.base.set_state(988);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == K_ORDER {
                            {
                                recog.base.set_state(978);
                                recog.base.match_token(K_ORDER, &mut recog.err_handler)?;

                                recog.base.set_state(979);
                                recog.base.match_token(K_BY, &mut recog.err_handler)?;

                                /*InvokeRule ordering_term*/
                                recog.base.set_state(980);
                                recog.ordering_term()?;

                                recog.base.set_state(985);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                                while _la == COMMA {
                                    {
                                        {
                                            recog.base.set_state(981);
                                            recog
                                                .base
                                                .match_token(COMMA, &mut recog.err_handler)?;

                                            /*InvokeRule ordering_term*/
                                            recog.base.set_state(982);
                                            recog.ordering_term()?;
                                        }
                                    }
                                    recog.base.set_state(987);
                                    recog.err_handler.sync(&mut recog.base)?;
                                    _la = recog.base.input.la(1);
                                }
                            }
                        }

                        recog.base.set_state(990);
                        recog.base.match_token(K_LIMIT, &mut recog.err_handler)?;

                        /*InvokeRule expr*/
                        recog.base.set_state(991);
                        recog.expr_rec(0)?;

                        recog.base.set_state(994);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == COMMA || _la == K_OFFSET {
                            {
                                recog.base.set_state(992);
                                _la = recog.base.input.la(1);
                                if !(_la == COMMA || _la == K_OFFSET) {
                                    recog.err_handler.recover_inline(&mut recog.base)?;
                                } else {
                                    if recog.base.input.la(1) == TOKEN_EOF {
                                        recog.base.matched_eof = true
                                    };
                                    recog.err_handler.report_match(&mut recog.base);
                                    recog.base.consume(&mut recog.err_handler);
                                }
                                /*InvokeRule expr*/
                                recog.base.set_state(993);
                                recog.expr_rec(0)?;
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
//------------------- vacuum_stmt ----------------
pub type Vacuum_stmtContextAll<'input> = Vacuum_stmtContext<'input>;

pub type Vacuum_stmtContext<'input> = BaseParserRuleContext<'input, Vacuum_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct Vacuum_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Vacuum_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Vacuum_stmtContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_vacuum_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_vacuum_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Vacuum_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_vacuum_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_vacuum_stmt }
}
antlr_rust::tid! {Vacuum_stmtContextExt<'a>}

impl<'input> Vacuum_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Vacuum_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Vacuum_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait Vacuum_stmtContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Vacuum_stmtContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token K_VACUUM
    /// Returns `None` if there is no child corresponding to token K_VACUUM
    fn K_VACUUM(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_VACUUM, 0)
    }
}

impl<'input> Vacuum_stmtContextAttrs<'input> for Vacuum_stmtContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn vacuum_stmt(&mut self) -> Result<Rc<Vacuum_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Vacuum_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 68, RULE_vacuum_stmt);
        let mut _localctx: Rc<Vacuum_stmtContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(998);
                recog.base.match_token(K_VACUUM, &mut recog.err_handler)?;
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
//------------------- column_def ----------------
pub type Column_defContextAll<'input> = Column_defContext<'input>;

pub type Column_defContext<'input> = BaseParserRuleContext<'input, Column_defContextExt<'input>>;

#[derive(Clone)]
pub struct Column_defContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Column_defContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Column_defContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_column_def(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_column_def(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Column_defContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_column_def
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_column_def }
}
antlr_rust::tid! {Column_defContextExt<'a>}

impl<'input> Column_defContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Column_defContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Column_defContextExt { ph: PhantomData },
        ))
    }
}

pub trait Column_defContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Column_defContextExt<'input>>
{
    fn column_name(&self) -> Option<Rc<Column_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn type_name(&self) -> Option<Rc<Type_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn column_constraint_all(&self) -> Vec<Rc<Column_constraintContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn column_constraint(&self, i: usize) -> Option<Rc<Column_constraintContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Column_defContextAttrs<'input> for Column_defContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn column_def(&mut self) -> Result<Rc<Column_defContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Column_defContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 70, RULE_column_def);
        let mut _localctx: Rc<Column_defContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule column_name*/
                recog.base.set_state(1000);
                recog.column_name()?;

                recog.base.set_state(1002);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(136, &mut recog.base)? {
                    x if x == 1 => {
                        {
                            /*InvokeRule type_name*/
                            recog.base.set_state(1001);
                            recog.type_name()?;
                        }
                    }

                    _ => {}
                }
                recog.base.set_state(1007);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while (((_la - 44) & !0x3f) == 0
                    && ((1usize << (_la - 44))
                        & ((1usize << (K_CHECK - 44))
                            | (1usize << (K_COLLATE - 44))
                            | (1usize << (K_CONSTRAINT - 44))
                            | (1usize << (K_DEFAULT - 44))))
                        != 0)
                    || (((_la - 102) & !0x3f) == 0
                        && ((1usize << (_la - 102))
                            & ((1usize << (K_NOT - 102))
                                | (1usize << (K_NULL - 102))
                                | (1usize << (K_PRIMARY - 102))
                                | (1usize << (K_REFERENCES - 102))))
                            != 0)
                    || _la == K_UNIQUE
                {
                    {
                        {
                            /*InvokeRule column_constraint*/
                            recog.base.set_state(1004);
                            recog.column_constraint()?;
                        }
                    }
                    recog.base.set_state(1009);
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
//------------------- type_name ----------------
pub type Type_nameContextAll<'input> = Type_nameContext<'input>;

pub type Type_nameContext<'input> = BaseParserRuleContext<'input, Type_nameContextExt<'input>>;

#[derive(Clone)]
pub struct Type_nameContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Type_nameContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Type_nameContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_type_name(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_type_name(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Type_nameContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_type_name
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_type_name }
}
antlr_rust::tid! {Type_nameContextExt<'a>}

impl<'input> Type_nameContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Type_nameContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Type_nameContextExt { ph: PhantomData },
        ))
    }
}

pub trait Type_nameContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Type_nameContextExt<'input>>
{
    fn name_all(&self) -> Vec<Rc<NameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn name(&self, i: usize) -> Option<Rc<NameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token OPEN_PAR
    /// Returns `None` if there is no child corresponding to token OPEN_PAR
    fn OPEN_PAR(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPEN_PAR, 0)
    }
    fn signed_number_all(&self) -> Vec<Rc<Signed_numberContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn signed_number(&self, i: usize) -> Option<Rc<Signed_numberContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token CLOSE_PAR
    /// Returns `None` if there is no child corresponding to token CLOSE_PAR
    fn CLOSE_PAR(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CLOSE_PAR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token COMMA
    /// Returns `None` if there is no child corresponding to token COMMA
    fn COMMA(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, 0)
    }
}

impl<'input> Type_nameContextAttrs<'input> for Type_nameContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn type_name(&mut self) -> Result<Rc<Type_nameContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Type_nameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 72, RULE_type_name);
        let mut _localctx: Rc<Type_nameContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1011);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = 1 + 1;
                loop {
                    match _alt {
                        x if x == 1 + 1 => {
                            {
                                /*InvokeRule name*/
                                recog.base.set_state(1010);
                                recog.name()?;
                            }
                        }

                        _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                            &mut recog.base,
                        )))?,
                    }
                    recog.base.set_state(1013);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(138, &mut recog.base)?;
                    if _alt == 1 || _alt == INVALID_ALT {
                        break;
                    }
                }
                recog.base.set_state(1025);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(139, &mut recog.base)? {
                    x if x == 1 => {
                        {
                            recog.base.set_state(1015);
                            recog.base.match_token(OPEN_PAR, &mut recog.err_handler)?;

                            /*InvokeRule signed_number*/
                            recog.base.set_state(1016);
                            recog.signed_number()?;

                            recog.base.set_state(1017);
                            recog.base.match_token(CLOSE_PAR, &mut recog.err_handler)?;
                        }
                    }

                    x if x == 2 => {
                        {
                            recog.base.set_state(1019);
                            recog.base.match_token(OPEN_PAR, &mut recog.err_handler)?;

                            /*InvokeRule signed_number*/
                            recog.base.set_state(1020);
                            recog.signed_number()?;

                            recog.base.set_state(1021);
                            recog.base.match_token(COMMA, &mut recog.err_handler)?;

                            /*InvokeRule signed_number*/
                            recog.base.set_state(1022);
                            recog.signed_number()?;

                            recog.base.set_state(1023);
                            recog.base.match_token(CLOSE_PAR, &mut recog.err_handler)?;
                        }
                    }

                    _ => {}
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
//------------------- column_constraint ----------------
pub type Column_constraintContextAll<'input> = Column_constraintContext<'input>;

pub type Column_constraintContext<'input> =
    BaseParserRuleContext<'input, Column_constraintContextExt<'input>>;

#[derive(Clone)]
pub struct Column_constraintContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Column_constraintContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Column_constraintContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_column_constraint(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_column_constraint(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Column_constraintContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_column_constraint
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_column_constraint }
}
antlr_rust::tid! {Column_constraintContextExt<'a>}

impl<'input> Column_constraintContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Column_constraintContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Column_constraintContextExt { ph: PhantomData },
        ))
    }
}

pub trait Column_constraintContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Column_constraintContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token K_PRIMARY
    /// Returns `None` if there is no child corresponding to token K_PRIMARY
    fn K_PRIMARY(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_PRIMARY, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_KEY
    /// Returns `None` if there is no child corresponding to token K_KEY
    fn K_KEY(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_KEY, 0)
    }
    fn conflict_clause(&self) -> Option<Rc<Conflict_clauseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_NULL
    /// Returns `None` if there is no child corresponding to token K_NULL
    fn K_NULL(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_NULL, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_UNIQUE
    /// Returns `None` if there is no child corresponding to token K_UNIQUE
    fn K_UNIQUE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_UNIQUE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_CHECK
    /// Returns `None` if there is no child corresponding to token K_CHECK
    fn K_CHECK(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_CHECK, 0)
    }
    /// Retrieves first TerminalNode corresponding to token OPEN_PAR
    /// Returns `None` if there is no child corresponding to token OPEN_PAR
    fn OPEN_PAR(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPEN_PAR, 0)
    }
    fn expr(&self) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token CLOSE_PAR
    /// Returns `None` if there is no child corresponding to token CLOSE_PAR
    fn CLOSE_PAR(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CLOSE_PAR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_DEFAULT
    /// Returns `None` if there is no child corresponding to token K_DEFAULT
    fn K_DEFAULT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_DEFAULT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_COLLATE
    /// Returns `None` if there is no child corresponding to token K_COLLATE
    fn K_COLLATE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_COLLATE, 0)
    }
    fn collation_name(&self) -> Option<Rc<Collation_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn foreign_key_clause(&self) -> Option<Rc<Foreign_key_clauseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_CONSTRAINT
    /// Returns `None` if there is no child corresponding to token K_CONSTRAINT
    fn K_CONSTRAINT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_CONSTRAINT, 0)
    }
    fn name(&self) -> Option<Rc<NameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn signed_number(&self) -> Option<Rc<Signed_numberContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn literal_value(&self) -> Option<Rc<Literal_valueContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_AUTOINCREMENT
    /// Returns `None` if there is no child corresponding to token K_AUTOINCREMENT
    fn K_AUTOINCREMENT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_AUTOINCREMENT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_NOT
    /// Returns `None` if there is no child corresponding to token K_NOT
    fn K_NOT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_NOT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_ASC
    /// Returns `None` if there is no child corresponding to token K_ASC
    fn K_ASC(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ASC, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_DESC
    /// Returns `None` if there is no child corresponding to token K_DESC
    fn K_DESC(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_DESC, 0)
    }
}

impl<'input> Column_constraintContextAttrs<'input> for Column_constraintContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn column_constraint(
        &mut self,
    ) -> Result<Rc<Column_constraintContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Column_constraintContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 74, RULE_column_constraint);
        let mut _localctx: Rc<Column_constraintContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1029);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == K_CONSTRAINT {
                    {
                        recog.base.set_state(1027);
                        recog
                            .base
                            .match_token(K_CONSTRAINT, &mut recog.err_handler)?;

                        /*InvokeRule name*/
                        recog.base.set_state(1028);
                        recog.name()?;
                    }
                }

                recog.base.set_state(1064);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.base.input.la(1) {
                    K_PRIMARY => {
                        {
                            recog.base.set_state(1031);
                            recog.base.match_token(K_PRIMARY, &mut recog.err_handler)?;

                            recog.base.set_state(1032);
                            recog.base.match_token(K_KEY, &mut recog.err_handler)?;

                            recog.base.set_state(1034);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == K_ASC || _la == K_DESC {
                                {
                                    recog.base.set_state(1033);
                                    _la = recog.base.input.la(1);
                                    if !(_la == K_ASC || _la == K_DESC) {
                                        recog.err_handler.recover_inline(&mut recog.base)?;
                                    } else {
                                        if recog.base.input.la(1) == TOKEN_EOF {
                                            recog.base.matched_eof = true
                                        };
                                        recog.err_handler.report_match(&mut recog.base);
                                        recog.base.consume(&mut recog.err_handler);
                                    }
                                }
                            }

                            /*InvokeRule conflict_clause*/
                            recog.base.set_state(1036);
                            recog.conflict_clause()?;

                            recog.base.set_state(1038);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == K_AUTOINCREMENT {
                                {
                                    recog.base.set_state(1037);
                                    recog
                                        .base
                                        .match_token(K_AUTOINCREMENT, &mut recog.err_handler)?;
                                }
                            }
                        }
                    }

                    K_NOT | K_NULL => {
                        {
                            recog.base.set_state(1041);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == K_NOT {
                                {
                                    recog.base.set_state(1040);
                                    recog.base.match_token(K_NOT, &mut recog.err_handler)?;
                                }
                            }

                            recog.base.set_state(1043);
                            recog.base.match_token(K_NULL, &mut recog.err_handler)?;

                            /*InvokeRule conflict_clause*/
                            recog.base.set_state(1044);
                            recog.conflict_clause()?;
                        }
                    }

                    K_UNIQUE => {
                        {
                            recog.base.set_state(1045);
                            recog.base.match_token(K_UNIQUE, &mut recog.err_handler)?;

                            /*InvokeRule conflict_clause*/
                            recog.base.set_state(1046);
                            recog.conflict_clause()?;
                        }
                    }

                    K_CHECK => {
                        {
                            recog.base.set_state(1047);
                            recog.base.match_token(K_CHECK, &mut recog.err_handler)?;

                            recog.base.set_state(1048);
                            recog.base.match_token(OPEN_PAR, &mut recog.err_handler)?;

                            /*InvokeRule expr*/
                            recog.base.set_state(1049);
                            recog.expr_rec(0)?;

                            recog.base.set_state(1050);
                            recog.base.match_token(CLOSE_PAR, &mut recog.err_handler)?;
                        }
                    }

                    K_DEFAULT => {
                        {
                            recog.base.set_state(1052);
                            recog.base.match_token(K_DEFAULT, &mut recog.err_handler)?;

                            recog.base.set_state(1059);
                            recog.err_handler.sync(&mut recog.base)?;
                            match recog.interpreter.adaptive_predict(144, &mut recog.base)? {
                                1 => {
                                    {
                                        /*InvokeRule signed_number*/
                                        recog.base.set_state(1053);
                                        recog.signed_number()?;
                                    }
                                }
                                2 => {
                                    {
                                        /*InvokeRule literal_value*/
                                        recog.base.set_state(1054);
                                        recog.literal_value()?;
                                    }
                                }
                                3 => {
                                    {
                                        recog.base.set_state(1055);
                                        recog.base.match_token(OPEN_PAR, &mut recog.err_handler)?;

                                        /*InvokeRule expr*/
                                        recog.base.set_state(1056);
                                        recog.expr_rec(0)?;

                                        recog.base.set_state(1057);
                                        recog
                                            .base
                                            .match_token(CLOSE_PAR, &mut recog.err_handler)?;
                                    }
                                }

                                _ => {}
                            }
                        }
                    }

                    K_COLLATE => {
                        {
                            recog.base.set_state(1061);
                            recog.base.match_token(K_COLLATE, &mut recog.err_handler)?;

                            /*InvokeRule collation_name*/
                            recog.base.set_state(1062);
                            recog.collation_name()?;
                        }
                    }

                    K_REFERENCES => {
                        {
                            /*InvokeRule foreign_key_clause*/
                            recog.base.set_state(1063);
                            recog.foreign_key_clause()?;
                        }
                    }

                    _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                        &mut recog.base,
                    )))?,
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
//------------------- conflict_clause ----------------
pub type Conflict_clauseContextAll<'input> = Conflict_clauseContext<'input>;

pub type Conflict_clauseContext<'input> =
    BaseParserRuleContext<'input, Conflict_clauseContextExt<'input>>;

#[derive(Clone)]
pub struct Conflict_clauseContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Conflict_clauseContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Conflict_clauseContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_conflict_clause(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_conflict_clause(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Conflict_clauseContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_conflict_clause
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_conflict_clause }
}
antlr_rust::tid! {Conflict_clauseContextExt<'a>}

impl<'input> Conflict_clauseContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Conflict_clauseContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Conflict_clauseContextExt { ph: PhantomData },
        ))
    }
}

pub trait Conflict_clauseContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Conflict_clauseContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token K_ON
    /// Returns `None` if there is no child corresponding to token K_ON
    fn K_ON(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ON, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_CONFLICT
    /// Returns `None` if there is no child corresponding to token K_CONFLICT
    fn K_CONFLICT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_CONFLICT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_ROLLBACK
    /// Returns `None` if there is no child corresponding to token K_ROLLBACK
    fn K_ROLLBACK(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ROLLBACK, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_ABORT
    /// Returns `None` if there is no child corresponding to token K_ABORT
    fn K_ABORT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ABORT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_FAIL
    /// Returns `None` if there is no child corresponding to token K_FAIL
    fn K_FAIL(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_FAIL, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_IGNORE
    /// Returns `None` if there is no child corresponding to token K_IGNORE
    fn K_IGNORE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_IGNORE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_REPLACE
    /// Returns `None` if there is no child corresponding to token K_REPLACE
    fn K_REPLACE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_REPLACE, 0)
    }
}

impl<'input> Conflict_clauseContextAttrs<'input> for Conflict_clauseContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn conflict_clause(&mut self) -> Result<Rc<Conflict_clauseContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Conflict_clauseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 76, RULE_conflict_clause);
        let mut _localctx: Rc<Conflict_clauseContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1069);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == K_ON {
                    {
                        recog.base.set_state(1066);
                        recog.base.match_token(K_ON, &mut recog.err_handler)?;

                        recog.base.set_state(1067);
                        recog.base.match_token(K_CONFLICT, &mut recog.err_handler)?;

                        recog.base.set_state(1068);
                        _la = recog.base.input.la(1);
                        if !(_la == K_ABORT
                            || _la == K_FAIL
                            || _la == K_IGNORE
                            || _la == K_REPLACE
                            || _la == K_ROLLBACK)
                        {
                            recog.err_handler.recover_inline(&mut recog.base)?;
                        } else {
                            if recog.base.input.la(1) == TOKEN_EOF {
                                recog.base.matched_eof = true
                            };
                            recog.err_handler.report_match(&mut recog.base);
                            recog.base.consume(&mut recog.err_handler);
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
//------------------- expr ----------------
pub type ExprContextAll<'input> = ExprContext<'input>;

pub type ExprContext<'input> = BaseParserRuleContext<'input, ExprContextExt<'input>>;

#[derive(Clone)]
pub struct ExprContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for ExprContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for ExprContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_expr(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_expr(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for ExprContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expr
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}
antlr_rust::tid! {ExprContextExt<'a>}

impl<'input> ExprContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ExprContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ExprContextExt { ph: PhantomData },
        ))
    }
}

pub trait ExprContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<ExprContextExt<'input>>
{
    fn literal_value(&self) -> Option<Rc<Literal_valueContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token BIND_PARAMETER
    /// Returns `None` if there is no child corresponding to token BIND_PARAMETER
    fn BIND_PARAMETER(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(BIND_PARAMETER, 0)
    }
    fn column_name(&self) -> Option<Rc<Column_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn table_name(&self) -> Option<Rc<Table_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token DOT in current rule
    fn DOT_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token DOT, starting from 0.
    /// Returns `None` if number of children corresponding to token DOT is less or equal than `i`.
    fn DOT(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DOT, i)
    }
    fn database_name(&self) -> Option<Rc<Database_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn unary_operator(&self) -> Option<Rc<Unary_operatorContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn expr_all(&self) -> Vec<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn function_name(&self) -> Option<Rc<Function_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token OPEN_PAR
    /// Returns `None` if there is no child corresponding to token OPEN_PAR
    fn OPEN_PAR(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPEN_PAR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token CLOSE_PAR
    /// Returns `None` if there is no child corresponding to token CLOSE_PAR
    fn CLOSE_PAR(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CLOSE_PAR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token STAR
    /// Returns `None` if there is no child corresponding to token STAR
    fn STAR(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(STAR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_DISTINCT
    /// Returns `None` if there is no child corresponding to token K_DISTINCT
    fn K_DISTINCT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_DISTINCT, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
    /// Retrieves first TerminalNode corresponding to token K_CAST
    /// Returns `None` if there is no child corresponding to token K_CAST
    fn K_CAST(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_CAST, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_AS
    /// Returns `None` if there is no child corresponding to token K_AS
    fn K_AS(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_AS, 0)
    }
    fn type_name(&self) -> Option<Rc<Type_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn select_stmt(&self) -> Option<Rc<Select_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_EXISTS
    /// Returns `None` if there is no child corresponding to token K_EXISTS
    fn K_EXISTS(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_EXISTS, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_NOT
    /// Returns `None` if there is no child corresponding to token K_NOT
    fn K_NOT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_NOT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_CASE
    /// Returns `None` if there is no child corresponding to token K_CASE
    fn K_CASE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_CASE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_END
    /// Returns `None` if there is no child corresponding to token K_END
    fn K_END(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_END, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token K_WHEN in current rule
    fn K_WHEN_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token K_WHEN, starting from 0.
    /// Returns `None` if number of children corresponding to token K_WHEN is less or equal than `i`.
    fn K_WHEN(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_WHEN, i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token K_THEN in current rule
    fn K_THEN_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token K_THEN, starting from 0.
    /// Returns `None` if number of children corresponding to token K_THEN is less or equal than `i`.
    fn K_THEN(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_THEN, i)
    }
    /// Retrieves first TerminalNode corresponding to token K_ELSE
    /// Returns `None` if there is no child corresponding to token K_ELSE
    fn K_ELSE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ELSE, 0)
    }
    fn raise_function(&self) -> Option<Rc<Raise_functionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token PIPE2
    /// Returns `None` if there is no child corresponding to token PIPE2
    fn PIPE2(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PIPE2, 0)
    }
    /// Retrieves first TerminalNode corresponding to token DIV
    /// Returns `None` if there is no child corresponding to token DIV
    fn DIV(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DIV, 0)
    }
    /// Retrieves first TerminalNode corresponding to token MOD
    /// Returns `None` if there is no child corresponding to token MOD
    fn MOD(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(MOD, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PLUS
    /// Returns `None` if there is no child corresponding to token PLUS
    fn PLUS(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PLUS, 0)
    }
    /// Retrieves first TerminalNode corresponding to token MINUS
    /// Returns `None` if there is no child corresponding to token MINUS
    fn MINUS(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(MINUS, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LT2
    /// Returns `None` if there is no child corresponding to token LT2
    fn LT2(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LT2, 0)
    }
    /// Retrieves first TerminalNode corresponding to token GT2
    /// Returns `None` if there is no child corresponding to token GT2
    fn GT2(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(GT2, 0)
    }
    /// Retrieves first TerminalNode corresponding to token AMP
    /// Returns `None` if there is no child corresponding to token AMP
    fn AMP(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(AMP, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PIPE
    /// Returns `None` if there is no child corresponding to token PIPE
    fn PIPE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PIPE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LT
    /// Returns `None` if there is no child corresponding to token LT
    fn LT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LT_EQ
    /// Returns `None` if there is no child corresponding to token LT_EQ
    fn LT_EQ(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LT_EQ, 0)
    }
    /// Retrieves first TerminalNode corresponding to token GT
    /// Returns `None` if there is no child corresponding to token GT
    fn GT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(GT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token GT_EQ
    /// Returns `None` if there is no child corresponding to token GT_EQ
    fn GT_EQ(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(GT_EQ, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ASSIGN
    /// Returns `None` if there is no child corresponding to token ASSIGN
    fn ASSIGN(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ASSIGN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token EQ
    /// Returns `None` if there is no child corresponding to token EQ
    fn EQ(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(EQ, 0)
    }
    /// Retrieves first TerminalNode corresponding to token NOT_EQ1
    /// Returns `None` if there is no child corresponding to token NOT_EQ1
    fn NOT_EQ1(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NOT_EQ1, 0)
    }
    /// Retrieves first TerminalNode corresponding to token NOT_EQ2
    /// Returns `None` if there is no child corresponding to token NOT_EQ2
    fn NOT_EQ2(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NOT_EQ2, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_AND
    /// Returns `None` if there is no child corresponding to token K_AND
    fn K_AND(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_AND, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_OR
    /// Returns `None` if there is no child corresponding to token K_OR
    fn K_OR(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_OR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_IS
    /// Returns `None` if there is no child corresponding to token K_IS
    fn K_IS(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_IS, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_BETWEEN
    /// Returns `None` if there is no child corresponding to token K_BETWEEN
    fn K_BETWEEN(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_BETWEEN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_IN
    /// Returns `None` if there is no child corresponding to token K_IN
    fn K_IN(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_IN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_COLLATE
    /// Returns `None` if there is no child corresponding to token K_COLLATE
    fn K_COLLATE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_COLLATE, 0)
    }
    fn collation_name(&self) -> Option<Rc<Collation_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_LIKE
    /// Returns `None` if there is no child corresponding to token K_LIKE
    fn K_LIKE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_LIKE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_GLOB
    /// Returns `None` if there is no child corresponding to token K_GLOB
    fn K_GLOB(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_GLOB, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_REGEXP
    /// Returns `None` if there is no child corresponding to token K_REGEXP
    fn K_REGEXP(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_REGEXP, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_MATCH
    /// Returns `None` if there is no child corresponding to token K_MATCH
    fn K_MATCH(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_MATCH, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_ESCAPE
    /// Returns `None` if there is no child corresponding to token K_ESCAPE
    fn K_ESCAPE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ESCAPE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_ISNULL
    /// Returns `None` if there is no child corresponding to token K_ISNULL
    fn K_ISNULL(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ISNULL, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_NOTNULL
    /// Returns `None` if there is no child corresponding to token K_NOTNULL
    fn K_NOTNULL(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_NOTNULL, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_NULL
    /// Returns `None` if there is no child corresponding to token K_NULL
    fn K_NULL(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_NULL, 0)
    }
}

impl<'input> ExprContextAttrs<'input> for ExprContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn expr(&mut self) -> Result<Rc<ExprContextAll<'input>>, ANTLRError> {
        self.expr_rec(0)
    }

    fn expr_rec(&mut self, _p: isize) -> Result<Rc<ExprContextAll<'input>>, ANTLRError> {
        let recog = self;
        let _parentctx = recog.ctx.take();
        let _parentState = recog.base.get_state();
        let mut _localctx = ExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_recursion_rule(_localctx.clone(), 78, RULE_expr, _p);
        let mut _localctx: Rc<ExprContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
        let _startState = 78;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1147);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(157, &mut recog.base)? {
                    1 => {
                        {
                            /*InvokeRule literal_value*/
                            recog.base.set_state(1072);
                            recog.literal_value()?;
                        }
                    }
                    2 => {
                        recog.base.set_state(1073);
                        recog
                            .base
                            .match_token(BIND_PARAMETER, &mut recog.err_handler)?;
                    }
                    3 => {
                        {
                            recog.base.set_state(1082);
                            recog.err_handler.sync(&mut recog.base)?;
                            match recog.interpreter.adaptive_predict(148, &mut recog.base)? {
                                x if x == 1 => {
                                    {
                                        recog.base.set_state(1077);
                                        recog.err_handler.sync(&mut recog.base)?;
                                        match recog
                                            .interpreter
                                            .adaptive_predict(147, &mut recog.base)?
                                        {
                                            x if x == 1 => {
                                                {
                                                    /*InvokeRule database_name*/
                                                    recog.base.set_state(1074);
                                                    recog.database_name()?;

                                                    recog.base.set_state(1075);
                                                    recog
                                                        .base
                                                        .match_token(DOT, &mut recog.err_handler)?;
                                                }
                                            }

                                            _ => {}
                                        }
                                        /*InvokeRule table_name*/
                                        recog.base.set_state(1079);
                                        recog.table_name()?;

                                        recog.base.set_state(1080);
                                        recog.base.match_token(DOT, &mut recog.err_handler)?;
                                    }
                                }

                                _ => {}
                            }
                            /*InvokeRule column_name*/
                            recog.base.set_state(1084);
                            recog.column_name()?;
                        }
                    }
                    4 => {
                        {
                            /*InvokeRule unary_operator*/
                            recog.base.set_state(1085);
                            recog.unary_operator()?;

                            /*InvokeRule expr*/
                            recog.base.set_state(1086);
                            recog.expr_rec(21)?;
                        }
                    }
                    5 => {
                        {
                            /*InvokeRule function_name*/
                            recog.base.set_state(1088);
                            recog.function_name()?;

                            recog.base.set_state(1089);
                            recog.base.match_token(OPEN_PAR, &mut recog.err_handler)?;

                            recog.base.set_state(1102);
                            recog.err_handler.sync(&mut recog.base)?;
                            match recog.base.input.la(1) {
                                OPEN_PAR | PLUS | MINUS | TILDE | K_ABORT | K_ACTION | K_ADD
                                | K_AFTER | K_ALL | K_ALTER | K_ANALYZE | K_AND | K_AS | K_ASC
                                | K_ATTACH | K_AUTOINCREMENT | K_BEFORE | K_BEGIN | K_BETWEEN
                                | K_BY | K_CASCADE | K_CASE | K_CAST | K_CHECK | K_COLLATE
                                | K_COLUMN | K_COMMIT | K_CONFLICT | K_CONSTRAINT | K_CREATE
                                | K_CROSS | K_CURRENT_DATE | K_CURRENT_TIME
                                | K_CURRENT_TIMESTAMP | K_DATABASE | K_DEFAULT | K_DEFERRABLE
                                | K_DEFERRED | K_DELETE | K_DESC | K_DETACH | K_DISTINCT
                                | K_DROP | K_EACH | K_ELSE | K_END | K_ESCAPE | K_EXCEPT
                                | K_EXCLUSIVE | K_EXISTS | K_EXPLAIN | K_FAIL | K_FOR
                                | K_FOREIGN | K_FROM | K_FULL | K_GLOB | K_GROUP | K_HAVING
                                | K_IF | K_IGNORE | K_IMMEDIATE | K_IN | K_INDEX | K_INDEXED
                                | K_INITIALLY | K_INNER | K_INSERT | K_INSTEAD | K_INTERSECT
                                | K_INTO | K_IS | K_ISNULL | K_JOIN | K_KEY | K_LEFT | K_LIKE
                                | K_LIMIT | K_MATCH | K_NATURAL | K_NO | K_NOT | K_NOTNULL
                                | K_NULL | K_OF | K_OFFSET | K_ON | K_OR | K_ORDER | K_OUTER
                                | K_PLAN | K_PRAGMA | K_PRIMARY | K_QUERY | K_RAISE
                                | K_RECURSIVE | K_REFERENCES | K_REGEXP | K_REINDEX | K_RELEASE
                                | K_RENAME | K_REPLACE | K_RESTRICT | K_RIGHT | K_ROLLBACK
                                | K_ROW | K_SAVEPOINT | K_SELECT | K_SET | K_TABLE | K_TEMP
                                | K_TEMPORARY | K_THEN | K_TO | K_TRANSACTION | K_TRIGGER
                                | K_UNION | K_UNIQUE | K_UPDATE | K_USING | K_VACUUM | K_VALUES
                                | K_VIEW | K_VIRTUAL | K_WHEN | K_WHERE | K_WITH | K_WITHOUT
                                | IDENTIFIER | NUMERIC_LITERAL | BIND_PARAMETER
                                | STRING_LITERAL | BLOB_LITERAL => {
                                    {
                                        recog.base.set_state(1091);
                                        recog.err_handler.sync(&mut recog.base)?;
                                        match recog
                                            .interpreter
                                            .adaptive_predict(149, &mut recog.base)?
                                        {
                                            x if x == 1 => {
                                                recog.base.set_state(1090);
                                                recog.base.match_token(
                                                    K_DISTINCT,
                                                    &mut recog.err_handler,
                                                )?;
                                            }

                                            _ => {}
                                        }
                                        /*InvokeRule expr*/
                                        recog.base.set_state(1093);
                                        recog.expr_rec(0)?;

                                        recog.base.set_state(1098);
                                        recog.err_handler.sync(&mut recog.base)?;
                                        _la = recog.base.input.la(1);
                                        while _la == COMMA {
                                            {
                                                {
                                                    recog.base.set_state(1094);
                                                    recog.base.match_token(
                                                        COMMA,
                                                        &mut recog.err_handler,
                                                    )?;

                                                    /*InvokeRule expr*/
                                                    recog.base.set_state(1095);
                                                    recog.expr_rec(0)?;
                                                }
                                            }
                                            recog.base.set_state(1100);
                                            recog.err_handler.sync(&mut recog.base)?;
                                            _la = recog.base.input.la(1);
                                        }
                                    }
                                }

                                STAR => {
                                    recog.base.set_state(1101);
                                    recog.base.match_token(STAR, &mut recog.err_handler)?;
                                }

                                CLOSE_PAR => {}

                                _ => {}
                            }
                            recog.base.set_state(1104);
                            recog.base.match_token(CLOSE_PAR, &mut recog.err_handler)?;
                        }
                    }
                    6 => {
                        {
                            recog.base.set_state(1106);
                            recog.base.match_token(OPEN_PAR, &mut recog.err_handler)?;

                            /*InvokeRule expr*/
                            recog.base.set_state(1107);
                            recog.expr_rec(0)?;

                            recog.base.set_state(1108);
                            recog.base.match_token(CLOSE_PAR, &mut recog.err_handler)?;
                        }
                    }
                    7 => {
                        {
                            recog.base.set_state(1110);
                            recog.base.match_token(K_CAST, &mut recog.err_handler)?;

                            recog.base.set_state(1111);
                            recog.base.match_token(OPEN_PAR, &mut recog.err_handler)?;

                            /*InvokeRule expr*/
                            recog.base.set_state(1112);
                            recog.expr_rec(0)?;

                            recog.base.set_state(1113);
                            recog.base.match_token(K_AS, &mut recog.err_handler)?;

                            /*InvokeRule type_name*/
                            recog.base.set_state(1114);
                            recog.type_name()?;

                            recog.base.set_state(1115);
                            recog.base.match_token(CLOSE_PAR, &mut recog.err_handler)?;
                        }
                    }
                    8 => {
                        {
                            recog.base.set_state(1121);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == K_EXISTS || _la == K_NOT {
                                {
                                    recog.base.set_state(1118);
                                    recog.err_handler.sync(&mut recog.base)?;
                                    _la = recog.base.input.la(1);
                                    if _la == K_NOT {
                                        {
                                            recog.base.set_state(1117);
                                            recog
                                                .base
                                                .match_token(K_NOT, &mut recog.err_handler)?;
                                        }
                                    }

                                    recog.base.set_state(1120);
                                    recog.base.match_token(K_EXISTS, &mut recog.err_handler)?;
                                }
                            }

                            recog.base.set_state(1123);
                            recog.base.match_token(OPEN_PAR, &mut recog.err_handler)?;

                            /*InvokeRule select_stmt*/
                            recog.base.set_state(1124);
                            recog.select_stmt()?;

                            recog.base.set_state(1125);
                            recog.base.match_token(CLOSE_PAR, &mut recog.err_handler)?;
                        }
                    }
                    9 => {
                        {
                            recog.base.set_state(1127);
                            recog.base.match_token(K_CASE, &mut recog.err_handler)?;

                            recog.base.set_state(1129);
                            recog.err_handler.sync(&mut recog.base)?;
                            match recog.interpreter.adaptive_predict(154, &mut recog.base)? {
                                x if x == 1 => {
                                    {
                                        /*InvokeRule expr*/
                                        recog.base.set_state(1128);
                                        recog.expr_rec(0)?;
                                    }
                                }

                                _ => {}
                            }
                            recog.base.set_state(1136);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            loop {
                                {
                                    {
                                        recog.base.set_state(1131);
                                        recog.base.match_token(K_WHEN, &mut recog.err_handler)?;

                                        /*InvokeRule expr*/
                                        recog.base.set_state(1132);
                                        recog.expr_rec(0)?;

                                        recog.base.set_state(1133);
                                        recog.base.match_token(K_THEN, &mut recog.err_handler)?;

                                        /*InvokeRule expr*/
                                        recog.base.set_state(1134);
                                        recog.expr_rec(0)?;
                                    }
                                }
                                recog.base.set_state(1138);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                                if _la != K_WHEN {
                                    break;
                                }
                            }
                            recog.base.set_state(1142);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == K_ELSE {
                                {
                                    recog.base.set_state(1140);
                                    recog.base.match_token(K_ELSE, &mut recog.err_handler)?;

                                    /*InvokeRule expr*/
                                    recog.base.set_state(1141);
                                    recog.expr_rec(0)?;
                                }
                            }

                            recog.base.set_state(1144);
                            recog.base.match_token(K_END, &mut recog.err_handler)?;
                        }
                    }
                    10 => {
                        {
                            /*InvokeRule raise_function*/
                            recog.base.set_state(1146);
                            recog.raise_function()?;
                        }
                    }

                    _ => {}
                }

                let tmp = recog.input.lt(-1).cloned();
                recog.ctx.as_ref().unwrap().set_stop(tmp);
                recog.base.set_state(1236);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(169, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        recog.trigger_exit_rule_event();
                        _prevctx = _localctx.clone();
                        {
                            recog.base.set_state(1234);
                            recog.err_handler.sync(&mut recog.base)?;
                            match recog.interpreter.adaptive_predict(168, &mut recog.base)? {
                                1 => {
                                    {
                                        /*recRuleAltStartAction*/
                                        let mut tmp =
                                            ExprContextExt::new(_parentctx.clone(), _parentState);
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expr,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(1149);
                                        if !({ recog.precpred(None, 20) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 20)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(1150);
                                        recog.base.match_token(PIPE2, &mut recog.err_handler)?;

                                        /*InvokeRule expr*/
                                        recog.base.set_state(1151);
                                        recog.expr_rec(21)?;
                                    }
                                }
                                2 => {
                                    {
                                        /*recRuleAltStartAction*/
                                        let mut tmp =
                                            ExprContextExt::new(_parentctx.clone(), _parentState);
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expr,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(1152);
                                        if !({ recog.precpred(None, 19) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 19)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(1153);
                                        _la = recog.base.input.la(1);
                                        if !(((_la) & !0x3f) == 0
                                            && ((1usize << _la)
                                                & ((1usize << STAR)
                                                    | (1usize << DIV)
                                                    | (1usize << MOD)))
                                                != 0)
                                        {
                                            recog.err_handler.recover_inline(&mut recog.base)?;
                                        } else {
                                            if recog.base.input.la(1) == TOKEN_EOF {
                                                recog.base.matched_eof = true
                                            };
                                            recog.err_handler.report_match(&mut recog.base);
                                            recog.base.consume(&mut recog.err_handler);
                                        }
                                        /*InvokeRule expr*/
                                        recog.base.set_state(1154);
                                        recog.expr_rec(20)?;
                                    }
                                }
                                3 => {
                                    {
                                        /*recRuleAltStartAction*/
                                        let mut tmp =
                                            ExprContextExt::new(_parentctx.clone(), _parentState);
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expr,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(1155);
                                        if !({ recog.precpred(None, 18) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 18)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(1156);
                                        _la = recog.base.input.la(1);
                                        if !(_la == PLUS || _la == MINUS) {
                                            recog.err_handler.recover_inline(&mut recog.base)?;
                                        } else {
                                            if recog.base.input.la(1) == TOKEN_EOF {
                                                recog.base.matched_eof = true
                                            };
                                            recog.err_handler.report_match(&mut recog.base);
                                            recog.base.consume(&mut recog.err_handler);
                                        }
                                        /*InvokeRule expr*/
                                        recog.base.set_state(1157);
                                        recog.expr_rec(19)?;
                                    }
                                }
                                4 => {
                                    {
                                        /*recRuleAltStartAction*/
                                        let mut tmp =
                                            ExprContextExt::new(_parentctx.clone(), _parentState);
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expr,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(1158);
                                        if !({ recog.precpred(None, 17) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 17)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(1159);
                                        _la = recog.base.input.la(1);
                                        if !(((_la) & !0x3f) == 0
                                            && ((1usize << _la)
                                                & ((1usize << LT2)
                                                    | (1usize << GT2)
                                                    | (1usize << AMP)
                                                    | (1usize << PIPE)))
                                                != 0)
                                        {
                                            recog.err_handler.recover_inline(&mut recog.base)?;
                                        } else {
                                            if recog.base.input.la(1) == TOKEN_EOF {
                                                recog.base.matched_eof = true
                                            };
                                            recog.err_handler.report_match(&mut recog.base);
                                            recog.base.consume(&mut recog.err_handler);
                                        }
                                        /*InvokeRule expr*/
                                        recog.base.set_state(1160);
                                        recog.expr_rec(18)?;
                                    }
                                }
                                5 => {
                                    {
                                        /*recRuleAltStartAction*/
                                        let mut tmp =
                                            ExprContextExt::new(_parentctx.clone(), _parentState);
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expr,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(1161);
                                        if !({ recog.precpred(None, 16) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 16)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(1162);
                                        _la = recog.base.input.la(1);
                                        if !(((_la) & !0x3f) == 0
                                            && ((1usize << _la)
                                                & ((1usize << LT)
                                                    | (1usize << LT_EQ)
                                                    | (1usize << GT)
                                                    | (1usize << GT_EQ)))
                                                != 0)
                                        {
                                            recog.err_handler.recover_inline(&mut recog.base)?;
                                        } else {
                                            if recog.base.input.la(1) == TOKEN_EOF {
                                                recog.base.matched_eof = true
                                            };
                                            recog.err_handler.report_match(&mut recog.base);
                                            recog.base.consume(&mut recog.err_handler);
                                        }
                                        /*InvokeRule expr*/
                                        recog.base.set_state(1163);
                                        recog.expr_rec(17)?;
                                    }
                                }
                                6 => {
                                    {
                                        /*recRuleAltStartAction*/
                                        let mut tmp =
                                            ExprContextExt::new(_parentctx.clone(), _parentState);
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expr,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(1164);
                                        if !({ recog.precpred(None, 15) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 15)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(1165);
                                        _la = recog.base.input.la(1);
                                        if !(((_la) & !0x3f) == 0
                                            && ((1usize << _la)
                                                & ((1usize << ASSIGN)
                                                    | (1usize << EQ)
                                                    | (1usize << NOT_EQ1)
                                                    | (1usize << NOT_EQ2)))
                                                != 0)
                                        {
                                            recog.err_handler.recover_inline(&mut recog.base)?;
                                        } else {
                                            if recog.base.input.la(1) == TOKEN_EOF {
                                                recog.base.matched_eof = true
                                            };
                                            recog.err_handler.report_match(&mut recog.base);
                                            recog.base.consume(&mut recog.err_handler);
                                        }
                                        /*InvokeRule expr*/
                                        recog.base.set_state(1166);
                                        recog.expr_rec(16)?;
                                    }
                                }
                                7 => {
                                    {
                                        /*recRuleAltStartAction*/
                                        let mut tmp =
                                            ExprContextExt::new(_parentctx.clone(), _parentState);
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expr,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(1167);
                                        if !({ recog.precpred(None, 13) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 13)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(1168);
                                        recog.base.match_token(K_AND, &mut recog.err_handler)?;

                                        /*InvokeRule expr*/
                                        recog.base.set_state(1169);
                                        recog.expr_rec(14)?;
                                    }
                                }
                                8 => {
                                    {
                                        /*recRuleAltStartAction*/
                                        let mut tmp =
                                            ExprContextExt::new(_parentctx.clone(), _parentState);
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expr,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(1170);
                                        if !({ recog.precpred(None, 12) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 12)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(1171);
                                        recog.base.match_token(K_OR, &mut recog.err_handler)?;

                                        /*InvokeRule expr*/
                                        recog.base.set_state(1172);
                                        recog.expr_rec(13)?;
                                    }
                                }
                                9 => {
                                    {
                                        /*recRuleAltStartAction*/
                                        let mut tmp =
                                            ExprContextExt::new(_parentctx.clone(), _parentState);
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expr,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(1173);
                                        if !({ recog.precpred(None, 5) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 5)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(1174);
                                        recog.base.match_token(K_IS, &mut recog.err_handler)?;

                                        recog.base.set_state(1176);
                                        recog.err_handler.sync(&mut recog.base)?;
                                        match recog
                                            .interpreter
                                            .adaptive_predict(158, &mut recog.base)?
                                        {
                                            x if x == 1 => {
                                                recog.base.set_state(1175);
                                                recog
                                                    .base
                                                    .match_token(K_NOT, &mut recog.err_handler)?;
                                            }

                                            _ => {}
                                        }
                                        /*InvokeRule expr*/
                                        recog.base.set_state(1178);
                                        recog.expr_rec(6)?;
                                    }
                                }
                                10 => {
                                    {
                                        /*recRuleAltStartAction*/
                                        let mut tmp =
                                            ExprContextExt::new(_parentctx.clone(), _parentState);
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expr,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(1179);
                                        if !({ recog.precpred(None, 4) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 4)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(1181);
                                        recog.err_handler.sync(&mut recog.base)?;
                                        _la = recog.base.input.la(1);
                                        if _la == K_NOT {
                                            {
                                                recog.base.set_state(1180);
                                                recog
                                                    .base
                                                    .match_token(K_NOT, &mut recog.err_handler)?;
                                            }
                                        }

                                        recog.base.set_state(1183);
                                        recog
                                            .base
                                            .match_token(K_BETWEEN, &mut recog.err_handler)?;

                                        /*InvokeRule expr*/
                                        recog.base.set_state(1184);
                                        recog.expr_rec(0)?;

                                        recog.base.set_state(1185);
                                        recog.base.match_token(K_AND, &mut recog.err_handler)?;

                                        /*InvokeRule expr*/
                                        recog.base.set_state(1186);
                                        recog.expr_rec(5)?;
                                    }
                                }
                                11 => {
                                    {
                                        /*recRuleAltStartAction*/
                                        let mut tmp =
                                            ExprContextExt::new(_parentctx.clone(), _parentState);
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expr,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(1188);
                                        if !({ recog.precpred(None, 14) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 14)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(1190);
                                        recog.err_handler.sync(&mut recog.base)?;
                                        _la = recog.base.input.la(1);
                                        if _la == K_NOT {
                                            {
                                                recog.base.set_state(1189);
                                                recog
                                                    .base
                                                    .match_token(K_NOT, &mut recog.err_handler)?;
                                            }
                                        }

                                        recog.base.set_state(1192);
                                        recog.base.match_token(K_IN, &mut recog.err_handler)?;

                                        recog.base.set_state(1212);
                                        recog.err_handler.sync(&mut recog.base)?;
                                        match recog
                                            .interpreter
                                            .adaptive_predict(164, &mut recog.base)?
                                        {
                                            1 => {
                                                {
                                                    recog.base.set_state(1193);
                                                    recog.base.match_token(
                                                        OPEN_PAR,
                                                        &mut recog.err_handler,
                                                    )?;

                                                    recog.base.set_state(1203);
                                                    recog.err_handler.sync(&mut recog.base)?;
                                                    match recog
                                                        .interpreter
                                                        .adaptive_predict(162, &mut recog.base)?
                                                    {
                                                        x if x == 1 => {
                                                            {
                                                                /*InvokeRule select_stmt*/
                                                                recog.base.set_state(1194);
                                                                recog.select_stmt()?;
                                                            }
                                                        }

                                                        x if x == 2 => {
                                                            {
                                                                /*InvokeRule expr*/
                                                                recog.base.set_state(1195);
                                                                recog.expr_rec(0)?;

                                                                recog.base.set_state(1200);
                                                                recog
                                                                    .err_handler
                                                                    .sync(&mut recog.base)?;
                                                                _la = recog.base.input.la(1);
                                                                while _la == COMMA {
                                                                    {
                                                                        {
                                                                            recog
                                                                                .base
                                                                                .set_state(1196);
                                                                            recog
                                                                                .base
                                                                                .match_token(
                                                                                COMMA,
                                                                                &mut recog
                                                                                    .err_handler,
                                                                            )?;

                                                                            /*InvokeRule expr*/
                                                                            recog
                                                                                .base
                                                                                .set_state(1197);
                                                                            recog.expr_rec(0)?;
                                                                        }
                                                                    }
                                                                    recog.base.set_state(1202);
                                                                    recog
                                                                        .err_handler
                                                                        .sync(&mut recog.base)?;
                                                                    _la = recog.base.input.la(1);
                                                                }
                                                            }
                                                        }

                                                        _ => {}
                                                    }
                                                    recog.base.set_state(1205);
                                                    recog.base.match_token(
                                                        CLOSE_PAR,
                                                        &mut recog.err_handler,
                                                    )?;
                                                }
                                            }
                                            2 => {
                                                {
                                                    recog.base.set_state(1209);
                                                    recog.err_handler.sync(&mut recog.base)?;
                                                    match recog
                                                        .interpreter
                                                        .adaptive_predict(163, &mut recog.base)?
                                                    {
                                                        x if x == 1 => {
                                                            {
                                                                /*InvokeRule database_name*/
                                                                recog.base.set_state(1206);
                                                                recog.database_name()?;

                                                                recog.base.set_state(1207);
                                                                recog.base.match_token(
                                                                    DOT,
                                                                    &mut recog.err_handler,
                                                                )?;
                                                            }
                                                        }

                                                        _ => {}
                                                    }
                                                    /*InvokeRule table_name*/
                                                    recog.base.set_state(1211);
                                                    recog.table_name()?;
                                                }
                                            }

                                            _ => {}
                                        }
                                    }
                                }
                                12 => {
                                    {
                                        /*recRuleAltStartAction*/
                                        let mut tmp =
                                            ExprContextExt::new(_parentctx.clone(), _parentState);
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expr,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(1214);
                                        if !({ recog.precpred(None, 8) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 8)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(1215);
                                        recog
                                            .base
                                            .match_token(K_COLLATE, &mut recog.err_handler)?;

                                        /*InvokeRule collation_name*/
                                        recog.base.set_state(1216);
                                        recog.collation_name()?;
                                    }
                                }
                                13 => {
                                    {
                                        /*recRuleAltStartAction*/
                                        let mut tmp =
                                            ExprContextExt::new(_parentctx.clone(), _parentState);
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expr,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(1217);
                                        if !({ recog.precpred(None, 7) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 7)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(1219);
                                        recog.err_handler.sync(&mut recog.base)?;
                                        _la = recog.base.input.la(1);
                                        if _la == K_NOT {
                                            {
                                                recog.base.set_state(1218);
                                                recog
                                                    .base
                                                    .match_token(K_NOT, &mut recog.err_handler)?;
                                            }
                                        }

                                        recog.base.set_state(1221);
                                        _la = recog.base.input.la(1);
                                        if !((((_la - 77) & !0x3f) == 0
                                            && ((1usize << (_la - 77))
                                                & ((1usize << (K_GLOB - 77))
                                                    | (1usize << (K_LIKE - 77))
                                                    | (1usize << (K_MATCH - 77))))
                                                != 0)
                                            || _la == K_REGEXP)
                                        {
                                            recog.err_handler.recover_inline(&mut recog.base)?;
                                        } else {
                                            if recog.base.input.la(1) == TOKEN_EOF {
                                                recog.base.matched_eof = true
                                            };
                                            recog.err_handler.report_match(&mut recog.base);
                                            recog.base.consume(&mut recog.err_handler);
                                        }
                                        /*InvokeRule expr*/
                                        recog.base.set_state(1222);
                                        recog.expr_rec(0)?;

                                        recog.base.set_state(1225);
                                        recog.err_handler.sync(&mut recog.base)?;
                                        match recog
                                            .interpreter
                                            .adaptive_predict(166, &mut recog.base)?
                                        {
                                            x if x == 1 => {
                                                {
                                                    recog.base.set_state(1223);
                                                    recog.base.match_token(
                                                        K_ESCAPE,
                                                        &mut recog.err_handler,
                                                    )?;

                                                    /*InvokeRule expr*/
                                                    recog.base.set_state(1224);
                                                    recog.expr_rec(0)?;
                                                }
                                            }

                                            _ => {}
                                        }
                                    }
                                }
                                14 => {
                                    {
                                        /*recRuleAltStartAction*/
                                        let mut tmp =
                                            ExprContextExt::new(_parentctx.clone(), _parentState);
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expr,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(1227);
                                        if !({ recog.precpred(None, 6) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 6)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(1232);
                                        recog.err_handler.sync(&mut recog.base)?;
                                        match recog.base.input.la(1) {
                                            K_ISNULL => {
                                                recog.base.set_state(1228);
                                                recog.base.match_token(
                                                    K_ISNULL,
                                                    &mut recog.err_handler,
                                                )?;
                                            }

                                            K_NOTNULL => {
                                                recog.base.set_state(1229);
                                                recog.base.match_token(
                                                    K_NOTNULL,
                                                    &mut recog.err_handler,
                                                )?;
                                            }

                                            K_NOT => {
                                                recog.base.set_state(1230);
                                                recog
                                                    .base
                                                    .match_token(K_NOT, &mut recog.err_handler)?;

                                                recog.base.set_state(1231);
                                                recog
                                                    .base
                                                    .match_token(K_NULL, &mut recog.err_handler)?;
                                            }

                                            _ => Err(ANTLRError::NoAltError(
                                                NoViableAltError::new(&mut recog.base),
                                            ))?,
                                        }
                                    }
                                }

                                _ => {}
                            }
                        }
                    }
                    recog.base.set_state(1238);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(169, &mut recog.base)?;
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
        recog.base.unroll_recursion_context(_parentctx);

        Ok(_localctx)
    }
}
//------------------- foreign_key_clause ----------------
pub type Foreign_key_clauseContextAll<'input> = Foreign_key_clauseContext<'input>;

pub type Foreign_key_clauseContext<'input> =
    BaseParserRuleContext<'input, Foreign_key_clauseContextExt<'input>>;

#[derive(Clone)]
pub struct Foreign_key_clauseContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Foreign_key_clauseContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Foreign_key_clauseContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_foreign_key_clause(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_foreign_key_clause(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Foreign_key_clauseContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_foreign_key_clause
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_foreign_key_clause }
}
antlr_rust::tid! {Foreign_key_clauseContextExt<'a>}

impl<'input> Foreign_key_clauseContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Foreign_key_clauseContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Foreign_key_clauseContextExt { ph: PhantomData },
        ))
    }
}

pub trait Foreign_key_clauseContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Foreign_key_clauseContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token K_REFERENCES
    /// Returns `None` if there is no child corresponding to token K_REFERENCES
    fn K_REFERENCES(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_REFERENCES, 0)
    }
    fn foreign_table(&self) -> Option<Rc<Foreign_tableContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token OPEN_PAR
    /// Returns `None` if there is no child corresponding to token OPEN_PAR
    fn OPEN_PAR(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPEN_PAR, 0)
    }
    fn column_name_all(&self) -> Vec<Rc<Column_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn column_name(&self, i: usize) -> Option<Rc<Column_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token CLOSE_PAR
    /// Returns `None` if there is no child corresponding to token CLOSE_PAR
    fn CLOSE_PAR(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CLOSE_PAR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_DEFERRABLE
    /// Returns `None` if there is no child corresponding to token K_DEFERRABLE
    fn K_DEFERRABLE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_DEFERRABLE, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token K_ON in current rule
    fn K_ON_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token K_ON, starting from 0.
    /// Returns `None` if number of children corresponding to token K_ON is less or equal than `i`.
    fn K_ON(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ON, i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token K_MATCH in current rule
    fn K_MATCH_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token K_MATCH, starting from 0.
    /// Returns `None` if number of children corresponding to token K_MATCH is less or equal than `i`.
    fn K_MATCH(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_MATCH, i)
    }
    fn name_all(&self) -> Vec<Rc<NameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn name(&self, i: usize) -> Option<Rc<NameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token K_DELETE in current rule
    fn K_DELETE_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token K_DELETE, starting from 0.
    /// Returns `None` if number of children corresponding to token K_DELETE is less or equal than `i`.
    fn K_DELETE(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_DELETE, i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token K_UPDATE in current rule
    fn K_UPDATE_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token K_UPDATE, starting from 0.
    /// Returns `None` if number of children corresponding to token K_UPDATE is less or equal than `i`.
    fn K_UPDATE(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_UPDATE, i)
    }
    /// Retrieves first TerminalNode corresponding to token K_NOT
    /// Returns `None` if there is no child corresponding to token K_NOT
    fn K_NOT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_NOT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_INITIALLY
    /// Returns `None` if there is no child corresponding to token K_INITIALLY
    fn K_INITIALLY(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_INITIALLY, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_DEFERRED
    /// Returns `None` if there is no child corresponding to token K_DEFERRED
    fn K_DEFERRED(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_DEFERRED, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_IMMEDIATE
    /// Returns `None` if there is no child corresponding to token K_IMMEDIATE
    fn K_IMMEDIATE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_IMMEDIATE, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token K_SET in current rule
    fn K_SET_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token K_SET, starting from 0.
    /// Returns `None` if number of children corresponding to token K_SET is less or equal than `i`.
    fn K_SET(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_SET, i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token K_NULL in current rule
    fn K_NULL_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token K_NULL, starting from 0.
    /// Returns `None` if number of children corresponding to token K_NULL is less or equal than `i`.
    fn K_NULL(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_NULL, i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token K_DEFAULT in current rule
    fn K_DEFAULT_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token K_DEFAULT, starting from 0.
    /// Returns `None` if number of children corresponding to token K_DEFAULT is less or equal than `i`.
    fn K_DEFAULT(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_DEFAULT, i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token K_CASCADE in current rule
    fn K_CASCADE_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token K_CASCADE, starting from 0.
    /// Returns `None` if number of children corresponding to token K_CASCADE is less or equal than `i`.
    fn K_CASCADE(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_CASCADE, i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token K_RESTRICT in current rule
    fn K_RESTRICT_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token K_RESTRICT, starting from 0.
    /// Returns `None` if number of children corresponding to token K_RESTRICT is less or equal than `i`.
    fn K_RESTRICT(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_RESTRICT, i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token K_NO in current rule
    fn K_NO_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token K_NO, starting from 0.
    /// Returns `None` if number of children corresponding to token K_NO is less or equal than `i`.
    fn K_NO(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_NO, i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token K_ACTION in current rule
    fn K_ACTION_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token K_ACTION, starting from 0.
    /// Returns `None` if number of children corresponding to token K_ACTION is less or equal than `i`.
    fn K_ACTION(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ACTION, i)
    }
}

impl<'input> Foreign_key_clauseContextAttrs<'input> for Foreign_key_clauseContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn foreign_key_clause(
        &mut self,
    ) -> Result<Rc<Foreign_key_clauseContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Foreign_key_clauseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 80, RULE_foreign_key_clause);
        let mut _localctx: Rc<Foreign_key_clauseContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1239);
                recog
                    .base
                    .match_token(K_REFERENCES, &mut recog.err_handler)?;

                /*InvokeRule foreign_table*/
                recog.base.set_state(1240);
                recog.foreign_table()?;

                recog.base.set_state(1252);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == OPEN_PAR {
                    {
                        recog.base.set_state(1241);
                        recog.base.match_token(OPEN_PAR, &mut recog.err_handler)?;

                        /*InvokeRule column_name*/
                        recog.base.set_state(1242);
                        recog.column_name()?;

                        recog.base.set_state(1247);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        while _la == COMMA {
                            {
                                {
                                    recog.base.set_state(1243);
                                    recog.base.match_token(COMMA, &mut recog.err_handler)?;

                                    /*InvokeRule column_name*/
                                    recog.base.set_state(1244);
                                    recog.column_name()?;
                                }
                            }
                            recog.base.set_state(1249);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                        }
                        recog.base.set_state(1250);
                        recog.base.match_token(CLOSE_PAR, &mut recog.err_handler)?;
                    }
                }

                recog.base.set_state(1272);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == K_MATCH || _la == K_ON {
                    {
                        {
                            recog.base.set_state(1268);
                            recog.err_handler.sync(&mut recog.base)?;
                            match recog.base.input.la(1) {
                                K_ON => {
                                    recog.base.set_state(1254);
                                    recog.base.match_token(K_ON, &mut recog.err_handler)?;

                                    recog.base.set_state(1255);
                                    _la = recog.base.input.la(1);
                                    if !(_la == K_DELETE || _la == K_UPDATE) {
                                        recog.err_handler.recover_inline(&mut recog.base)?;
                                    } else {
                                        if recog.base.input.la(1) == TOKEN_EOF {
                                            recog.base.matched_eof = true
                                        };
                                        recog.err_handler.report_match(&mut recog.base);
                                        recog.base.consume(&mut recog.err_handler);
                                    }
                                    recog.base.set_state(1264);
                                    recog.err_handler.sync(&mut recog.base)?;
                                    match recog
                                        .interpreter
                                        .adaptive_predict(172, &mut recog.base)?
                                    {
                                        1 => {
                                            recog.base.set_state(1256);
                                            recog
                                                .base
                                                .match_token(K_SET, &mut recog.err_handler)?;

                                            recog.base.set_state(1257);
                                            recog
                                                .base
                                                .match_token(K_NULL, &mut recog.err_handler)?;
                                        }
                                        2 => {
                                            recog.base.set_state(1258);
                                            recog
                                                .base
                                                .match_token(K_SET, &mut recog.err_handler)?;

                                            recog.base.set_state(1259);
                                            recog
                                                .base
                                                .match_token(K_DEFAULT, &mut recog.err_handler)?;
                                        }
                                        3 => {
                                            recog.base.set_state(1260);
                                            recog
                                                .base
                                                .match_token(K_CASCADE, &mut recog.err_handler)?;
                                        }
                                        4 => {
                                            recog.base.set_state(1261);
                                            recog
                                                .base
                                                .match_token(K_RESTRICT, &mut recog.err_handler)?;
                                        }
                                        5 => {
                                            recog.base.set_state(1262);
                                            recog.base.match_token(K_NO, &mut recog.err_handler)?;

                                            recog.base.set_state(1263);
                                            recog
                                                .base
                                                .match_token(K_ACTION, &mut recog.err_handler)?;
                                        }

                                        _ => {}
                                    }
                                }

                                K_MATCH => {
                                    {
                                        recog.base.set_state(1266);
                                        recog.base.match_token(K_MATCH, &mut recog.err_handler)?;

                                        /*InvokeRule name*/
                                        recog.base.set_state(1267);
                                        recog.name()?;
                                    }
                                }

                                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                                    &mut recog.base,
                                )))?,
                            }
                        }
                    }
                    recog.base.set_state(1274);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(1285);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(177, &mut recog.base)? {
                    x if x == 1 => {
                        recog.base.set_state(1276);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == K_NOT {
                            {
                                recog.base.set_state(1275);
                                recog.base.match_token(K_NOT, &mut recog.err_handler)?;
                            }
                        }

                        recog.base.set_state(1278);
                        recog
                            .base
                            .match_token(K_DEFERRABLE, &mut recog.err_handler)?;

                        recog.base.set_state(1283);
                        recog.err_handler.sync(&mut recog.base)?;
                        match recog.interpreter.adaptive_predict(176, &mut recog.base)? {
                            x if x == 1 => {
                                recog.base.set_state(1279);
                                recog
                                    .base
                                    .match_token(K_INITIALLY, &mut recog.err_handler)?;

                                recog.base.set_state(1280);
                                recog.base.match_token(K_DEFERRED, &mut recog.err_handler)?;
                            }

                            x if x == 2 => {
                                recog.base.set_state(1281);
                                recog
                                    .base
                                    .match_token(K_INITIALLY, &mut recog.err_handler)?;

                                recog.base.set_state(1282);
                                recog
                                    .base
                                    .match_token(K_IMMEDIATE, &mut recog.err_handler)?;
                            }

                            _ => {}
                        }
                    }

                    _ => {}
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
//------------------- raise_function ----------------
pub type Raise_functionContextAll<'input> = Raise_functionContext<'input>;

pub type Raise_functionContext<'input> =
    BaseParserRuleContext<'input, Raise_functionContextExt<'input>>;

#[derive(Clone)]
pub struct Raise_functionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Raise_functionContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Raise_functionContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_raise_function(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_raise_function(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Raise_functionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_raise_function
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_raise_function }
}
antlr_rust::tid! {Raise_functionContextExt<'a>}

impl<'input> Raise_functionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Raise_functionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Raise_functionContextExt { ph: PhantomData },
        ))
    }
}

pub trait Raise_functionContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Raise_functionContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token K_RAISE
    /// Returns `None` if there is no child corresponding to token K_RAISE
    fn K_RAISE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_RAISE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token OPEN_PAR
    /// Returns `None` if there is no child corresponding to token OPEN_PAR
    fn OPEN_PAR(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPEN_PAR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token CLOSE_PAR
    /// Returns `None` if there is no child corresponding to token CLOSE_PAR
    fn CLOSE_PAR(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CLOSE_PAR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_IGNORE
    /// Returns `None` if there is no child corresponding to token K_IGNORE
    fn K_IGNORE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_IGNORE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token COMMA
    /// Returns `None` if there is no child corresponding to token COMMA
    fn COMMA(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, 0)
    }
    fn error_message(&self) -> Option<Rc<Error_messageContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_ROLLBACK
    /// Returns `None` if there is no child corresponding to token K_ROLLBACK
    fn K_ROLLBACK(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ROLLBACK, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_ABORT
    /// Returns `None` if there is no child corresponding to token K_ABORT
    fn K_ABORT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ABORT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_FAIL
    /// Returns `None` if there is no child corresponding to token K_FAIL
    fn K_FAIL(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_FAIL, 0)
    }
}

impl<'input> Raise_functionContextAttrs<'input> for Raise_functionContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn raise_function(&mut self) -> Result<Rc<Raise_functionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Raise_functionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 82, RULE_raise_function);
        let mut _localctx: Rc<Raise_functionContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1287);
                recog.base.match_token(K_RAISE, &mut recog.err_handler)?;

                recog.base.set_state(1288);
                recog.base.match_token(OPEN_PAR, &mut recog.err_handler)?;

                recog.base.set_state(1293);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.base.input.la(1) {
                    K_IGNORE => {
                        recog.base.set_state(1289);
                        recog.base.match_token(K_IGNORE, &mut recog.err_handler)?;
                    }

                    K_ABORT | K_FAIL | K_ROLLBACK => {
                        {
                            recog.base.set_state(1290);
                            _la = recog.base.input.la(1);
                            if !(_la == K_ABORT || _la == K_FAIL || _la == K_ROLLBACK) {
                                recog.err_handler.recover_inline(&mut recog.base)?;
                            } else {
                                if recog.base.input.la(1) == TOKEN_EOF {
                                    recog.base.matched_eof = true
                                };
                                recog.err_handler.report_match(&mut recog.base);
                                recog.base.consume(&mut recog.err_handler);
                            }
                            recog.base.set_state(1291);
                            recog.base.match_token(COMMA, &mut recog.err_handler)?;

                            /*InvokeRule error_message*/
                            recog.base.set_state(1292);
                            recog.error_message()?;
                        }
                    }

                    _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                        &mut recog.base,
                    )))?,
                }
                recog.base.set_state(1295);
                recog.base.match_token(CLOSE_PAR, &mut recog.err_handler)?;
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
//------------------- indexed_column ----------------
pub type Indexed_columnContextAll<'input> = Indexed_columnContext<'input>;

pub type Indexed_columnContext<'input> =
    BaseParserRuleContext<'input, Indexed_columnContextExt<'input>>;

#[derive(Clone)]
pub struct Indexed_columnContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Indexed_columnContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Indexed_columnContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_indexed_column(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_indexed_column(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Indexed_columnContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_indexed_column
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_indexed_column }
}
antlr_rust::tid! {Indexed_columnContextExt<'a>}

impl<'input> Indexed_columnContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Indexed_columnContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Indexed_columnContextExt { ph: PhantomData },
        ))
    }
}

pub trait Indexed_columnContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Indexed_columnContextExt<'input>>
{
    fn column_name(&self) -> Option<Rc<Column_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_COLLATE
    /// Returns `None` if there is no child corresponding to token K_COLLATE
    fn K_COLLATE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_COLLATE, 0)
    }
    fn collation_name(&self) -> Option<Rc<Collation_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_ASC
    /// Returns `None` if there is no child corresponding to token K_ASC
    fn K_ASC(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ASC, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_DESC
    /// Returns `None` if there is no child corresponding to token K_DESC
    fn K_DESC(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_DESC, 0)
    }
}

impl<'input> Indexed_columnContextAttrs<'input> for Indexed_columnContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn indexed_column(&mut self) -> Result<Rc<Indexed_columnContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Indexed_columnContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 84, RULE_indexed_column);
        let mut _localctx: Rc<Indexed_columnContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule column_name*/
                recog.base.set_state(1297);
                recog.column_name()?;

                recog.base.set_state(1300);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == K_COLLATE {
                    {
                        recog.base.set_state(1298);
                        recog.base.match_token(K_COLLATE, &mut recog.err_handler)?;

                        /*InvokeRule collation_name*/
                        recog.base.set_state(1299);
                        recog.collation_name()?;
                    }
                }

                recog.base.set_state(1303);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == K_ASC || _la == K_DESC {
                    {
                        recog.base.set_state(1302);
                        _la = recog.base.input.la(1);
                        if !(_la == K_ASC || _la == K_DESC) {
                            recog.err_handler.recover_inline(&mut recog.base)?;
                        } else {
                            if recog.base.input.la(1) == TOKEN_EOF {
                                recog.base.matched_eof = true
                            };
                            recog.err_handler.report_match(&mut recog.base);
                            recog.base.consume(&mut recog.err_handler);
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
//------------------- table_constraint ----------------
pub type Table_constraintContextAll<'input> = Table_constraintContext<'input>;

pub type Table_constraintContext<'input> =
    BaseParserRuleContext<'input, Table_constraintContextExt<'input>>;

#[derive(Clone)]
pub struct Table_constraintContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Table_constraintContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Table_constraintContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_table_constraint(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_table_constraint(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Table_constraintContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_table_constraint
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_table_constraint }
}
antlr_rust::tid! {Table_constraintContextExt<'a>}

impl<'input> Table_constraintContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Table_constraintContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Table_constraintContextExt { ph: PhantomData },
        ))
    }
}

pub trait Table_constraintContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Table_constraintContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token OPEN_PAR
    /// Returns `None` if there is no child corresponding to token OPEN_PAR
    fn OPEN_PAR(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPEN_PAR, 0)
    }
    fn indexed_column_all(&self) -> Vec<Rc<Indexed_columnContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn indexed_column(&self, i: usize) -> Option<Rc<Indexed_columnContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token CLOSE_PAR
    /// Returns `None` if there is no child corresponding to token CLOSE_PAR
    fn CLOSE_PAR(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CLOSE_PAR, 0)
    }
    fn conflict_clause(&self) -> Option<Rc<Conflict_clauseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_CHECK
    /// Returns `None` if there is no child corresponding to token K_CHECK
    fn K_CHECK(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_CHECK, 0)
    }
    fn expr(&self) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_FOREIGN
    /// Returns `None` if there is no child corresponding to token K_FOREIGN
    fn K_FOREIGN(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_FOREIGN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_KEY
    /// Returns `None` if there is no child corresponding to token K_KEY
    fn K_KEY(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_KEY, 0)
    }
    fn column_name_all(&self) -> Vec<Rc<Column_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn column_name(&self, i: usize) -> Option<Rc<Column_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn foreign_key_clause(&self) -> Option<Rc<Foreign_key_clauseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_CONSTRAINT
    /// Returns `None` if there is no child corresponding to token K_CONSTRAINT
    fn K_CONSTRAINT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_CONSTRAINT, 0)
    }
    fn name(&self) -> Option<Rc<NameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_PRIMARY
    /// Returns `None` if there is no child corresponding to token K_PRIMARY
    fn K_PRIMARY(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_PRIMARY, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_UNIQUE
    /// Returns `None` if there is no child corresponding to token K_UNIQUE
    fn K_UNIQUE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_UNIQUE, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
}

impl<'input> Table_constraintContextAttrs<'input> for Table_constraintContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn table_constraint(
        &mut self,
    ) -> Result<Rc<Table_constraintContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Table_constraintContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 86, RULE_table_constraint);
        let mut _localctx: Rc<Table_constraintContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1307);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == K_CONSTRAINT {
                    {
                        recog.base.set_state(1305);
                        recog
                            .base
                            .match_token(K_CONSTRAINT, &mut recog.err_handler)?;

                        /*InvokeRule name*/
                        recog.base.set_state(1306);
                        recog.name()?;
                    }
                }

                recog.base.set_state(1345);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.base.input.la(1) {
                    K_PRIMARY | K_UNIQUE => {
                        {
                            recog.base.set_state(1312);
                            recog.err_handler.sync(&mut recog.base)?;
                            match recog.base.input.la(1) {
                                K_PRIMARY => {
                                    recog.base.set_state(1309);
                                    recog.base.match_token(K_PRIMARY, &mut recog.err_handler)?;

                                    recog.base.set_state(1310);
                                    recog.base.match_token(K_KEY, &mut recog.err_handler)?;
                                }

                                K_UNIQUE => {
                                    recog.base.set_state(1311);
                                    recog.base.match_token(K_UNIQUE, &mut recog.err_handler)?;
                                }

                                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                                    &mut recog.base,
                                )))?,
                            }
                            recog.base.set_state(1314);
                            recog.base.match_token(OPEN_PAR, &mut recog.err_handler)?;

                            /*InvokeRule indexed_column*/
                            recog.base.set_state(1315);
                            recog.indexed_column()?;

                            recog.base.set_state(1320);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            while _la == COMMA {
                                {
                                    {
                                        recog.base.set_state(1316);
                                        recog.base.match_token(COMMA, &mut recog.err_handler)?;

                                        /*InvokeRule indexed_column*/
                                        recog.base.set_state(1317);
                                        recog.indexed_column()?;
                                    }
                                }
                                recog.base.set_state(1322);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                            }
                            recog.base.set_state(1323);
                            recog.base.match_token(CLOSE_PAR, &mut recog.err_handler)?;

                            /*InvokeRule conflict_clause*/
                            recog.base.set_state(1324);
                            recog.conflict_clause()?;
                        }
                    }

                    K_CHECK => {
                        {
                            recog.base.set_state(1326);
                            recog.base.match_token(K_CHECK, &mut recog.err_handler)?;

                            recog.base.set_state(1327);
                            recog.base.match_token(OPEN_PAR, &mut recog.err_handler)?;

                            /*InvokeRule expr*/
                            recog.base.set_state(1328);
                            recog.expr_rec(0)?;

                            recog.base.set_state(1329);
                            recog.base.match_token(CLOSE_PAR, &mut recog.err_handler)?;
                        }
                    }

                    K_FOREIGN => {
                        {
                            recog.base.set_state(1331);
                            recog.base.match_token(K_FOREIGN, &mut recog.err_handler)?;

                            recog.base.set_state(1332);
                            recog.base.match_token(K_KEY, &mut recog.err_handler)?;

                            recog.base.set_state(1333);
                            recog.base.match_token(OPEN_PAR, &mut recog.err_handler)?;

                            /*InvokeRule column_name*/
                            recog.base.set_state(1334);
                            recog.column_name()?;

                            recog.base.set_state(1339);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            while _la == COMMA {
                                {
                                    {
                                        recog.base.set_state(1335);
                                        recog.base.match_token(COMMA, &mut recog.err_handler)?;

                                        /*InvokeRule column_name*/
                                        recog.base.set_state(1336);
                                        recog.column_name()?;
                                    }
                                }
                                recog.base.set_state(1341);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                            }
                            recog.base.set_state(1342);
                            recog.base.match_token(CLOSE_PAR, &mut recog.err_handler)?;

                            /*InvokeRule foreign_key_clause*/
                            recog.base.set_state(1343);
                            recog.foreign_key_clause()?;
                        }
                    }

                    _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                        &mut recog.base,
                    )))?,
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
//------------------- with_clause ----------------
pub type With_clauseContextAll<'input> = With_clauseContext<'input>;

pub type With_clauseContext<'input> = BaseParserRuleContext<'input, With_clauseContextExt<'input>>;

#[derive(Clone)]
pub struct With_clauseContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for With_clauseContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for With_clauseContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_with_clause(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_with_clause(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for With_clauseContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_with_clause
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_with_clause }
}
antlr_rust::tid! {With_clauseContextExt<'a>}

impl<'input> With_clauseContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<With_clauseContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            With_clauseContextExt { ph: PhantomData },
        ))
    }
}

pub trait With_clauseContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<With_clauseContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token K_WITH
    /// Returns `None` if there is no child corresponding to token K_WITH
    fn K_WITH(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_WITH, 0)
    }
    fn common_table_expression_all(&self) -> Vec<Rc<Common_table_expressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn common_table_expression(
        &self,
        i: usize,
    ) -> Option<Rc<Common_table_expressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token K_RECURSIVE
    /// Returns `None` if there is no child corresponding to token K_RECURSIVE
    fn K_RECURSIVE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_RECURSIVE, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
}

impl<'input> With_clauseContextAttrs<'input> for With_clauseContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn with_clause(&mut self) -> Result<Rc<With_clauseContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = With_clauseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 88, RULE_with_clause);
        let mut _localctx: Rc<With_clauseContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1347);
                recog.base.match_token(K_WITH, &mut recog.err_handler)?;

                recog.base.set_state(1349);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(186, &mut recog.base)? {
                    x if x == 1 => {
                        recog.base.set_state(1348);
                        recog
                            .base
                            .match_token(K_RECURSIVE, &mut recog.err_handler)?;
                    }

                    _ => {}
                }
                /*InvokeRule common_table_expression*/
                recog.base.set_state(1351);
                recog.common_table_expression()?;

                recog.base.set_state(1356);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == COMMA {
                    {
                        {
                            recog.base.set_state(1352);
                            recog.base.match_token(COMMA, &mut recog.err_handler)?;

                            /*InvokeRule common_table_expression*/
                            recog.base.set_state(1353);
                            recog.common_table_expression()?;
                        }
                    }
                    recog.base.set_state(1358);
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
//------------------- qualified_table_name ----------------
pub type Qualified_table_nameContextAll<'input> = Qualified_table_nameContext<'input>;

pub type Qualified_table_nameContext<'input> =
    BaseParserRuleContext<'input, Qualified_table_nameContextExt<'input>>;

#[derive(Clone)]
pub struct Qualified_table_nameContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Qualified_table_nameContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a>
    for Qualified_table_nameContext<'input>
{
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_qualified_table_name(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_qualified_table_name(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Qualified_table_nameContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_qualified_table_name
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_qualified_table_name }
}
antlr_rust::tid! {Qualified_table_nameContextExt<'a>}

impl<'input> Qualified_table_nameContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Qualified_table_nameContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Qualified_table_nameContextExt { ph: PhantomData },
        ))
    }
}

pub trait Qualified_table_nameContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Qualified_table_nameContextExt<'input>>
{
    fn table_name(&self) -> Option<Rc<Table_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn database_name(&self) -> Option<Rc<Database_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token DOT
    /// Returns `None` if there is no child corresponding to token DOT
    fn DOT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DOT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_INDEXED
    /// Returns `None` if there is no child corresponding to token K_INDEXED
    fn K_INDEXED(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_INDEXED, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_BY
    /// Returns `None` if there is no child corresponding to token K_BY
    fn K_BY(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_BY, 0)
    }
    fn index_name(&self) -> Option<Rc<Index_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_NOT
    /// Returns `None` if there is no child corresponding to token K_NOT
    fn K_NOT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_NOT, 0)
    }
}

impl<'input> Qualified_table_nameContextAttrs<'input> for Qualified_table_nameContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn qualified_table_name(
        &mut self,
    ) -> Result<Rc<Qualified_table_nameContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Qualified_table_nameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 90, RULE_qualified_table_name);
        let mut _localctx: Rc<Qualified_table_nameContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1362);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(188, &mut recog.base)? {
                    x if x == 1 => {
                        {
                            /*InvokeRule database_name*/
                            recog.base.set_state(1359);
                            recog.database_name()?;

                            recog.base.set_state(1360);
                            recog.base.match_token(DOT, &mut recog.err_handler)?;
                        }
                    }

                    _ => {}
                }
                /*InvokeRule table_name*/
                recog.base.set_state(1364);
                recog.table_name()?;

                recog.base.set_state(1370);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.base.input.la(1) {
                    K_INDEXED => {
                        {
                            recog.base.set_state(1365);
                            recog.base.match_token(K_INDEXED, &mut recog.err_handler)?;

                            recog.base.set_state(1366);
                            recog.base.match_token(K_BY, &mut recog.err_handler)?;

                            /*InvokeRule index_name*/
                            recog.base.set_state(1367);
                            recog.index_name()?;
                        }
                    }

                    K_NOT => {
                        recog.base.set_state(1368);
                        recog.base.match_token(K_NOT, &mut recog.err_handler)?;

                        recog.base.set_state(1369);
                        recog.base.match_token(K_INDEXED, &mut recog.err_handler)?;
                    }

                    EOF | SCOL | K_ALTER | K_ANALYZE | K_ATTACH | K_BEGIN | K_COMMIT | K_CREATE
                    | K_DELETE | K_DETACH | K_DROP | K_END | K_EXPLAIN | K_INSERT | K_LIMIT
                    | K_ORDER | K_PRAGMA | K_REINDEX | K_RELEASE | K_REPLACE | K_ROLLBACK
                    | K_SAVEPOINT | K_SELECT | K_SET | K_UPDATE | K_VACUUM | K_VALUES | K_WHERE
                    | K_WITH | UNEXPECTED_CHAR => {}

                    _ => {}
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
//------------------- ordering_term ----------------
pub type Ordering_termContextAll<'input> = Ordering_termContext<'input>;

pub type Ordering_termContext<'input> =
    BaseParserRuleContext<'input, Ordering_termContextExt<'input>>;

#[derive(Clone)]
pub struct Ordering_termContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Ordering_termContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Ordering_termContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_ordering_term(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_ordering_term(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Ordering_termContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_ordering_term
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_ordering_term }
}
antlr_rust::tid! {Ordering_termContextExt<'a>}

impl<'input> Ordering_termContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Ordering_termContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Ordering_termContextExt { ph: PhantomData },
        ))
    }
}

pub trait Ordering_termContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Ordering_termContextExt<'input>>
{
    fn expr(&self) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_COLLATE
    /// Returns `None` if there is no child corresponding to token K_COLLATE
    fn K_COLLATE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_COLLATE, 0)
    }
    fn collation_name(&self) -> Option<Rc<Collation_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_ASC
    /// Returns `None` if there is no child corresponding to token K_ASC
    fn K_ASC(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ASC, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_DESC
    /// Returns `None` if there is no child corresponding to token K_DESC
    fn K_DESC(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_DESC, 0)
    }
}

impl<'input> Ordering_termContextAttrs<'input> for Ordering_termContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn ordering_term(&mut self) -> Result<Rc<Ordering_termContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Ordering_termContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 92, RULE_ordering_term);
        let mut _localctx: Rc<Ordering_termContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule expr*/
                recog.base.set_state(1372);
                recog.expr_rec(0)?;

                recog.base.set_state(1375);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == K_COLLATE {
                    {
                        recog.base.set_state(1373);
                        recog.base.match_token(K_COLLATE, &mut recog.err_handler)?;

                        /*InvokeRule collation_name*/
                        recog.base.set_state(1374);
                        recog.collation_name()?;
                    }
                }

                recog.base.set_state(1378);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == K_ASC || _la == K_DESC {
                    {
                        recog.base.set_state(1377);
                        _la = recog.base.input.la(1);
                        if !(_la == K_ASC || _la == K_DESC) {
                            recog.err_handler.recover_inline(&mut recog.base)?;
                        } else {
                            if recog.base.input.la(1) == TOKEN_EOF {
                                recog.base.matched_eof = true
                            };
                            recog.err_handler.report_match(&mut recog.base);
                            recog.base.consume(&mut recog.err_handler);
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
//------------------- pragma_value ----------------
pub type Pragma_valueContextAll<'input> = Pragma_valueContext<'input>;

pub type Pragma_valueContext<'input> =
    BaseParserRuleContext<'input, Pragma_valueContextExt<'input>>;

#[derive(Clone)]
pub struct Pragma_valueContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Pragma_valueContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Pragma_valueContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_pragma_value(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_pragma_value(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Pragma_valueContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_pragma_value
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_pragma_value }
}
antlr_rust::tid! {Pragma_valueContextExt<'a>}

impl<'input> Pragma_valueContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Pragma_valueContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Pragma_valueContextExt { ph: PhantomData },
        ))
    }
}

pub trait Pragma_valueContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Pragma_valueContextExt<'input>>
{
    fn signed_number(&self) -> Option<Rc<Signed_numberContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn name(&self) -> Option<Rc<NameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token STRING_LITERAL
    /// Returns `None` if there is no child corresponding to token STRING_LITERAL
    fn STRING_LITERAL(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(STRING_LITERAL, 0)
    }
}

impl<'input> Pragma_valueContextAttrs<'input> for Pragma_valueContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn pragma_value(&mut self) -> Result<Rc<Pragma_valueContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Pragma_valueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 94, RULE_pragma_value);
        let mut _localctx: Rc<Pragma_valueContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(1383);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(192, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule signed_number*/
                        recog.base.set_state(1380);
                        recog.signed_number()?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule name*/
                        recog.base.set_state(1381);
                        recog.name()?;
                    }
                }
                3 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        recog.base.set_state(1382);
                        recog
                            .base
                            .match_token(STRING_LITERAL, &mut recog.err_handler)?;
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
//------------------- common_table_expression ----------------
pub type Common_table_expressionContextAll<'input> = Common_table_expressionContext<'input>;

pub type Common_table_expressionContext<'input> =
    BaseParserRuleContext<'input, Common_table_expressionContextExt<'input>>;

#[derive(Clone)]
pub struct Common_table_expressionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Common_table_expressionContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a>
    for Common_table_expressionContext<'input>
{
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_common_table_expression(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_common_table_expression(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Common_table_expressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_common_table_expression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_common_table_expression }
}
antlr_rust::tid! {Common_table_expressionContextExt<'a>}

impl<'input> Common_table_expressionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Common_table_expressionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Common_table_expressionContextExt { ph: PhantomData },
        ))
    }
}

pub trait Common_table_expressionContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Common_table_expressionContextExt<'input>>
{
    fn table_name(&self) -> Option<Rc<Table_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_AS
    /// Returns `None` if there is no child corresponding to token K_AS
    fn K_AS(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_AS, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token OPEN_PAR in current rule
    fn OPEN_PAR_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token OPEN_PAR, starting from 0.
    /// Returns `None` if number of children corresponding to token OPEN_PAR is less or equal than `i`.
    fn OPEN_PAR(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPEN_PAR, i)
    }
    fn select_stmt(&self) -> Option<Rc<Select_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token CLOSE_PAR in current rule
    fn CLOSE_PAR_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token CLOSE_PAR, starting from 0.
    /// Returns `None` if number of children corresponding to token CLOSE_PAR is less or equal than `i`.
    fn CLOSE_PAR(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CLOSE_PAR, i)
    }
    fn column_name_all(&self) -> Vec<Rc<Column_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn column_name(&self, i: usize) -> Option<Rc<Column_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
}

impl<'input> Common_table_expressionContextAttrs<'input>
    for Common_table_expressionContext<'input>
{
}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn common_table_expression(
        &mut self,
    ) -> Result<Rc<Common_table_expressionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Common_table_expressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 96, RULE_common_table_expression);
        let mut _localctx: Rc<Common_table_expressionContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule table_name*/
                recog.base.set_state(1385);
                recog.table_name()?;

                recog.base.set_state(1397);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == OPEN_PAR {
                    {
                        recog.base.set_state(1386);
                        recog.base.match_token(OPEN_PAR, &mut recog.err_handler)?;

                        /*InvokeRule column_name*/
                        recog.base.set_state(1387);
                        recog.column_name()?;

                        recog.base.set_state(1392);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        while _la == COMMA {
                            {
                                {
                                    recog.base.set_state(1388);
                                    recog.base.match_token(COMMA, &mut recog.err_handler)?;

                                    /*InvokeRule column_name*/
                                    recog.base.set_state(1389);
                                    recog.column_name()?;
                                }
                            }
                            recog.base.set_state(1394);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                        }
                        recog.base.set_state(1395);
                        recog.base.match_token(CLOSE_PAR, &mut recog.err_handler)?;
                    }
                }

                recog.base.set_state(1399);
                recog.base.match_token(K_AS, &mut recog.err_handler)?;

                recog.base.set_state(1400);
                recog.base.match_token(OPEN_PAR, &mut recog.err_handler)?;

                /*InvokeRule select_stmt*/
                recog.base.set_state(1401);
                recog.select_stmt()?;

                recog.base.set_state(1402);
                recog.base.match_token(CLOSE_PAR, &mut recog.err_handler)?;
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
//------------------- result_column ----------------
pub type Result_columnContextAll<'input> = Result_columnContext<'input>;

pub type Result_columnContext<'input> =
    BaseParserRuleContext<'input, Result_columnContextExt<'input>>;

#[derive(Clone)]
pub struct Result_columnContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Result_columnContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Result_columnContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_result_column(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_result_column(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Result_columnContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_result_column
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_result_column }
}
antlr_rust::tid! {Result_columnContextExt<'a>}

impl<'input> Result_columnContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Result_columnContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Result_columnContextExt { ph: PhantomData },
        ))
    }
}

pub trait Result_columnContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Result_columnContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token STAR
    /// Returns `None` if there is no child corresponding to token STAR
    fn STAR(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(STAR, 0)
    }
    fn table_name(&self) -> Option<Rc<Table_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token DOT
    /// Returns `None` if there is no child corresponding to token DOT
    fn DOT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DOT, 0)
    }
    fn expr(&self) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn column_alias(&self) -> Option<Rc<Column_aliasContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_AS
    /// Returns `None` if there is no child corresponding to token K_AS
    fn K_AS(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_AS, 0)
    }
}

impl<'input> Result_columnContextAttrs<'input> for Result_columnContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn result_column(&mut self) -> Result<Rc<Result_columnContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Result_columnContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 98, RULE_result_column);
        let mut _localctx: Rc<Result_columnContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(1416);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(197, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        recog.base.set_state(1404);
                        recog.base.match_token(STAR, &mut recog.err_handler)?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule table_name*/
                        recog.base.set_state(1405);
                        recog.table_name()?;

                        recog.base.set_state(1406);
                        recog.base.match_token(DOT, &mut recog.err_handler)?;

                        recog.base.set_state(1407);
                        recog.base.match_token(STAR, &mut recog.err_handler)?;
                    }
                }
                3 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        /*InvokeRule expr*/
                        recog.base.set_state(1409);
                        recog.expr_rec(0)?;

                        recog.base.set_state(1414);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == K_AS || _la == IDENTIFIER || _la == STRING_LITERAL {
                            {
                                recog.base.set_state(1411);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                                if _la == K_AS {
                                    {
                                        recog.base.set_state(1410);
                                        recog.base.match_token(K_AS, &mut recog.err_handler)?;
                                    }
                                }

                                /*InvokeRule column_alias*/
                                recog.base.set_state(1413);
                                recog.column_alias()?;
                            }
                        }
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
//------------------- table_or_subquery ----------------
pub type Table_or_subqueryContextAll<'input> = Table_or_subqueryContext<'input>;

pub type Table_or_subqueryContext<'input> =
    BaseParserRuleContext<'input, Table_or_subqueryContextExt<'input>>;

#[derive(Clone)]
pub struct Table_or_subqueryContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Table_or_subqueryContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Table_or_subqueryContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_table_or_subquery(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_table_or_subquery(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Table_or_subqueryContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_table_or_subquery
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_table_or_subquery }
}
antlr_rust::tid! {Table_or_subqueryContextExt<'a>}

impl<'input> Table_or_subqueryContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Table_or_subqueryContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Table_or_subqueryContextExt { ph: PhantomData },
        ))
    }
}

pub trait Table_or_subqueryContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Table_or_subqueryContextExt<'input>>
{
    fn table_name(&self) -> Option<Rc<Table_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn schema_name(&self) -> Option<Rc<Schema_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token DOT
    /// Returns `None` if there is no child corresponding to token DOT
    fn DOT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DOT, 0)
    }
    fn table_alias(&self) -> Option<Rc<Table_aliasContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_INDEXED
    /// Returns `None` if there is no child corresponding to token K_INDEXED
    fn K_INDEXED(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_INDEXED, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_BY
    /// Returns `None` if there is no child corresponding to token K_BY
    fn K_BY(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_BY, 0)
    }
    fn index_name(&self) -> Option<Rc<Index_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_NOT
    /// Returns `None` if there is no child corresponding to token K_NOT
    fn K_NOT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_NOT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_AS
    /// Returns `None` if there is no child corresponding to token K_AS
    fn K_AS(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_AS, 0)
    }
    fn table_function_name(&self) -> Option<Rc<Table_function_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token OPEN_PAR
    /// Returns `None` if there is no child corresponding to token OPEN_PAR
    fn OPEN_PAR(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPEN_PAR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token CLOSE_PAR
    /// Returns `None` if there is no child corresponding to token CLOSE_PAR
    fn CLOSE_PAR(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CLOSE_PAR, 0)
    }
    fn expr_all(&self) -> Vec<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
    fn table_or_subquery_all(&self) -> Vec<Rc<Table_or_subqueryContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn table_or_subquery(&self, i: usize) -> Option<Rc<Table_or_subqueryContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn join_clause(&self) -> Option<Rc<Join_clauseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn select_stmt(&self) -> Option<Rc<Select_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Table_or_subqueryContextAttrs<'input> for Table_or_subqueryContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn table_or_subquery(
        &mut self,
    ) -> Result<Rc<Table_or_subqueryContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Table_or_subqueryContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 100, RULE_table_or_subquery);
        let mut _localctx: Rc<Table_or_subqueryContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(1484);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(211, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        recog.base.set_state(1421);
                        recog.err_handler.sync(&mut recog.base)?;
                        match recog.interpreter.adaptive_predict(198, &mut recog.base)? {
                            x if x == 1 => {
                                {
                                    /*InvokeRule schema_name*/
                                    recog.base.set_state(1418);
                                    recog.schema_name()?;

                                    recog.base.set_state(1419);
                                    recog.base.match_token(DOT, &mut recog.err_handler)?;
                                }
                            }

                            _ => {}
                        }
                        /*InvokeRule table_name*/
                        recog.base.set_state(1423);
                        recog.table_name()?;

                        recog.base.set_state(1428);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == OPEN_PAR
                            || _la == K_AS
                            || _la == IDENTIFIER
                            || _la == STRING_LITERAL
                        {
                            {
                                recog.base.set_state(1425);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                                if _la == K_AS {
                                    {
                                        recog.base.set_state(1424);
                                        recog.base.match_token(K_AS, &mut recog.err_handler)?;
                                    }
                                }

                                /*InvokeRule table_alias*/
                                recog.base.set_state(1427);
                                recog.table_alias()?;
                            }
                        }

                        recog.base.set_state(1435);
                        recog.err_handler.sync(&mut recog.base)?;
                        match recog.base.input.la(1) {
                            K_INDEXED => {
                                {
                                    recog.base.set_state(1430);
                                    recog.base.match_token(K_INDEXED, &mut recog.err_handler)?;

                                    recog.base.set_state(1431);
                                    recog.base.match_token(K_BY, &mut recog.err_handler)?;

                                    /*InvokeRule index_name*/
                                    recog.base.set_state(1432);
                                    recog.index_name()?;
                                }
                            }

                            K_NOT => {
                                recog.base.set_state(1433);
                                recog.base.match_token(K_NOT, &mut recog.err_handler)?;

                                recog.base.set_state(1434);
                                recog.base.match_token(K_INDEXED, &mut recog.err_handler)?;
                            }

                            EOF | SCOL | CLOSE_PAR | COMMA | K_ALTER | K_ANALYZE | K_ATTACH
                            | K_BEGIN | K_COMMIT | K_CREATE | K_CROSS | K_DELETE | K_DETACH
                            | K_DROP | K_END | K_EXCEPT | K_EXPLAIN | K_GROUP | K_INNER
                            | K_INSERT | K_INTERSECT | K_JOIN | K_LEFT | K_LIMIT | K_NATURAL
                            | K_ON | K_ORDER | K_PRAGMA | K_REINDEX | K_RELEASE | K_REPLACE
                            | K_ROLLBACK | K_SAVEPOINT | K_SELECT | K_UNION | K_UPDATE
                            | K_USING | K_VACUUM | K_VALUES | K_WHERE | K_WITH
                            | UNEXPECTED_CHAR => {}

                            _ => {}
                        }
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        recog.base.set_state(1440);
                        recog.err_handler.sync(&mut recog.base)?;
                        match recog.interpreter.adaptive_predict(202, &mut recog.base)? {
                            x if x == 1 => {
                                {
                                    /*InvokeRule schema_name*/
                                    recog.base.set_state(1437);
                                    recog.schema_name()?;

                                    recog.base.set_state(1438);
                                    recog.base.match_token(DOT, &mut recog.err_handler)?;
                                }
                            }

                            _ => {}
                        }
                        /*InvokeRule table_function_name*/
                        recog.base.set_state(1442);
                        recog.table_function_name()?;

                        recog.base.set_state(1443);
                        recog.base.match_token(OPEN_PAR, &mut recog.err_handler)?;

                        recog.base.set_state(1452);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if (((_la) & !0x3f) == 0
                            && ((1usize << _la)
                                & ((1usize << OPEN_PAR)
                                    | (1usize << PLUS)
                                    | (1usize << MINUS)
                                    | (1usize << TILDE)
                                    | (1usize << K_ABORT)
                                    | (1usize << K_ACTION)
                                    | (1usize << K_ADD)
                                    | (1usize << K_AFTER)
                                    | (1usize << K_ALL)
                                    | (1usize << K_ALTER)
                                    | (1usize << K_ANALYZE)))
                                != 0)
                            || (((_la - 32) & !0x3f) == 0
                                && ((1usize << (_la - 32))
                                    & ((1usize << (K_AND - 32))
                                        | (1usize << (K_AS - 32))
                                        | (1usize << (K_ASC - 32))
                                        | (1usize << (K_ATTACH - 32))
                                        | (1usize << (K_AUTOINCREMENT - 32))
                                        | (1usize << (K_BEFORE - 32))
                                        | (1usize << (K_BEGIN - 32))
                                        | (1usize << (K_BETWEEN - 32))
                                        | (1usize << (K_BY - 32))
                                        | (1usize << (K_CASCADE - 32))
                                        | (1usize << (K_CASE - 32))
                                        | (1usize << (K_CAST - 32))
                                        | (1usize << (K_CHECK - 32))
                                        | (1usize << (K_COLLATE - 32))
                                        | (1usize << (K_COLUMN - 32))
                                        | (1usize << (K_COMMIT - 32))
                                        | (1usize << (K_CONFLICT - 32))
                                        | (1usize << (K_CONSTRAINT - 32))
                                        | (1usize << (K_CREATE - 32))
                                        | (1usize << (K_CROSS - 32))
                                        | (1usize << (K_CURRENT_DATE - 32))
                                        | (1usize << (K_CURRENT_TIME - 32))
                                        | (1usize << (K_CURRENT_TIMESTAMP - 32))
                                        | (1usize << (K_DATABASE - 32))
                                        | (1usize << (K_DEFAULT - 32))
                                        | (1usize << (K_DEFERRABLE - 32))
                                        | (1usize << (K_DEFERRED - 32))
                                        | (1usize << (K_DELETE - 32))
                                        | (1usize << (K_DESC - 32))
                                        | (1usize << (K_DETACH - 32))
                                        | (1usize << (K_DISTINCT - 32))
                                        | (1usize << (K_DROP - 32))))
                                    != 0)
                            || (((_la - 64) & !0x3f) == 0
                                && ((1usize << (_la - 64))
                                    & ((1usize << (K_EACH - 64))
                                        | (1usize << (K_ELSE - 64))
                                        | (1usize << (K_END - 64))
                                        | (1usize << (K_ESCAPE - 64))
                                        | (1usize << (K_EXCEPT - 64))
                                        | (1usize << (K_EXCLUSIVE - 64))
                                        | (1usize << (K_EXISTS - 64))
                                        | (1usize << (K_EXPLAIN - 64))
                                        | (1usize << (K_FAIL - 64))
                                        | (1usize << (K_FOR - 64))
                                        | (1usize << (K_FOREIGN - 64))
                                        | (1usize << (K_FROM - 64))
                                        | (1usize << (K_FULL - 64))
                                        | (1usize << (K_GLOB - 64))
                                        | (1usize << (K_GROUP - 64))
                                        | (1usize << (K_HAVING - 64))
                                        | (1usize << (K_IF - 64))
                                        | (1usize << (K_IGNORE - 64))
                                        | (1usize << (K_IMMEDIATE - 64))
                                        | (1usize << (K_IN - 64))
                                        | (1usize << (K_INDEX - 64))
                                        | (1usize << (K_INDEXED - 64))
                                        | (1usize << (K_INITIALLY - 64))
                                        | (1usize << (K_INNER - 64))
                                        | (1usize << (K_INSERT - 64))
                                        | (1usize << (K_INSTEAD - 64))
                                        | (1usize << (K_INTERSECT - 64))
                                        | (1usize << (K_INTO - 64))
                                        | (1usize << (K_IS - 64))
                                        | (1usize << (K_ISNULL - 64))
                                        | (1usize << (K_JOIN - 64))
                                        | (1usize << (K_KEY - 64))))
                                    != 0)
                            || (((_la - 96) & !0x3f) == 0
                                && ((1usize << (_la - 96))
                                    & ((1usize << (K_LEFT - 96))
                                        | (1usize << (K_LIKE - 96))
                                        | (1usize << (K_LIMIT - 96))
                                        | (1usize << (K_MATCH - 96))
                                        | (1usize << (K_NATURAL - 96))
                                        | (1usize << (K_NO - 96))
                                        | (1usize << (K_NOT - 96))
                                        | (1usize << (K_NOTNULL - 96))
                                        | (1usize << (K_NULL - 96))
                                        | (1usize << (K_OF - 96))
                                        | (1usize << (K_OFFSET - 96))
                                        | (1usize << (K_ON - 96))
                                        | (1usize << (K_OR - 96))
                                        | (1usize << (K_ORDER - 96))
                                        | (1usize << (K_OUTER - 96))
                                        | (1usize << (K_PLAN - 96))
                                        | (1usize << (K_PRAGMA - 96))
                                        | (1usize << (K_PRIMARY - 96))
                                        | (1usize << (K_QUERY - 96))
                                        | (1usize << (K_RAISE - 96))
                                        | (1usize << (K_RECURSIVE - 96))
                                        | (1usize << (K_REFERENCES - 96))
                                        | (1usize << (K_REGEXP - 96))
                                        | (1usize << (K_REINDEX - 96))
                                        | (1usize << (K_RELEASE - 96))
                                        | (1usize << (K_RENAME - 96))
                                        | (1usize << (K_REPLACE - 96))
                                        | (1usize << (K_RESTRICT - 96))
                                        | (1usize << (K_RIGHT - 96))
                                        | (1usize << (K_ROLLBACK - 96))
                                        | (1usize << (K_ROW - 96))
                                        | (1usize << (K_SAVEPOINT - 96))))
                                    != 0)
                            || (((_la - 128) & !0x3f) == 0
                                && ((1usize << (_la - 128))
                                    & ((1usize << (K_SELECT - 128))
                                        | (1usize << (K_SET - 128))
                                        | (1usize << (K_TABLE - 128))
                                        | (1usize << (K_TEMP - 128))
                                        | (1usize << (K_TEMPORARY - 128))
                                        | (1usize << (K_THEN - 128))
                                        | (1usize << (K_TO - 128))
                                        | (1usize << (K_TRANSACTION - 128))
                                        | (1usize << (K_TRIGGER - 128))
                                        | (1usize << (K_UNION - 128))
                                        | (1usize << (K_UNIQUE - 128))
                                        | (1usize << (K_UPDATE - 128))
                                        | (1usize << (K_USING - 128))
                                        | (1usize << (K_VACUUM - 128))
                                        | (1usize << (K_VALUES - 128))
                                        | (1usize << (K_VIEW - 128))
                                        | (1usize << (K_VIRTUAL - 128))
                                        | (1usize << (K_WHEN - 128))
                                        | (1usize << (K_WHERE - 128))
                                        | (1usize << (K_WITH - 128))
                                        | (1usize << (K_WITHOUT - 128))
                                        | (1usize << (IDENTIFIER - 128))
                                        | (1usize << (NUMERIC_LITERAL - 128))
                                        | (1usize << (BIND_PARAMETER - 128))
                                        | (1usize << (STRING_LITERAL - 128))
                                        | (1usize << (BLOB_LITERAL - 128))))
                                    != 0)
                        {
                            {
                                /*InvokeRule expr*/
                                recog.base.set_state(1444);
                                recog.expr_rec(0)?;

                                recog.base.set_state(1449);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                                while _la == COMMA {
                                    {
                                        {
                                            recog.base.set_state(1445);
                                            recog
                                                .base
                                                .match_token(COMMA, &mut recog.err_handler)?;

                                            /*InvokeRule expr*/
                                            recog.base.set_state(1446);
                                            recog.expr_rec(0)?;
                                        }
                                    }
                                    recog.base.set_state(1451);
                                    recog.err_handler.sync(&mut recog.base)?;
                                    _la = recog.base.input.la(1);
                                }
                            }
                        }

                        recog.base.set_state(1454);
                        recog.base.match_token(CLOSE_PAR, &mut recog.err_handler)?;

                        recog.base.set_state(1459);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == OPEN_PAR
                            || _la == K_AS
                            || _la == IDENTIFIER
                            || _la == STRING_LITERAL
                        {
                            {
                                recog.base.set_state(1456);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                                if _la == K_AS {
                                    {
                                        recog.base.set_state(1455);
                                        recog.base.match_token(K_AS, &mut recog.err_handler)?;
                                    }
                                }

                                /*InvokeRule table_alias*/
                                recog.base.set_state(1458);
                                recog.table_alias()?;
                            }
                        }
                    }
                }
                3 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        recog.base.set_state(1461);
                        recog.base.match_token(OPEN_PAR, &mut recog.err_handler)?;

                        recog.base.set_state(1471);
                        recog.err_handler.sync(&mut recog.base)?;
                        match recog.interpreter.adaptive_predict(208, &mut recog.base)? {
                            1 => {
                                {
                                    /*InvokeRule table_or_subquery*/
                                    recog.base.set_state(1462);
                                    recog.table_or_subquery()?;

                                    recog.base.set_state(1467);
                                    recog.err_handler.sync(&mut recog.base)?;
                                    _la = recog.base.input.la(1);
                                    while _la == COMMA {
                                        {
                                            {
                                                recog.base.set_state(1463);
                                                recog
                                                    .base
                                                    .match_token(COMMA, &mut recog.err_handler)?;

                                                /*InvokeRule table_or_subquery*/
                                                recog.base.set_state(1464);
                                                recog.table_or_subquery()?;
                                            }
                                        }
                                        recog.base.set_state(1469);
                                        recog.err_handler.sync(&mut recog.base)?;
                                        _la = recog.base.input.la(1);
                                    }
                                }
                            }
                            2 => {
                                {
                                    /*InvokeRule join_clause*/
                                    recog.base.set_state(1470);
                                    recog.join_clause()?;
                                }
                            }

                            _ => {}
                        }
                        recog.base.set_state(1473);
                        recog.base.match_token(CLOSE_PAR, &mut recog.err_handler)?;
                    }
                }
                4 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 4);
                    recog.base.enter_outer_alt(None, 4);
                    {
                        recog.base.set_state(1475);
                        recog.base.match_token(OPEN_PAR, &mut recog.err_handler)?;

                        /*InvokeRule select_stmt*/
                        recog.base.set_state(1476);
                        recog.select_stmt()?;

                        recog.base.set_state(1477);
                        recog.base.match_token(CLOSE_PAR, &mut recog.err_handler)?;

                        recog.base.set_state(1482);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == OPEN_PAR
                            || _la == K_AS
                            || _la == IDENTIFIER
                            || _la == STRING_LITERAL
                        {
                            {
                                recog.base.set_state(1479);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                                if _la == K_AS {
                                    {
                                        recog.base.set_state(1478);
                                        recog.base.match_token(K_AS, &mut recog.err_handler)?;
                                    }
                                }

                                /*InvokeRule table_alias*/
                                recog.base.set_state(1481);
                                recog.table_alias()?;
                            }
                        }
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
//------------------- join_clause ----------------
pub type Join_clauseContextAll<'input> = Join_clauseContext<'input>;

pub type Join_clauseContext<'input> = BaseParserRuleContext<'input, Join_clauseContextExt<'input>>;

#[derive(Clone)]
pub struct Join_clauseContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Join_clauseContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Join_clauseContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_join_clause(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_join_clause(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Join_clauseContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_join_clause
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_join_clause }
}
antlr_rust::tid! {Join_clauseContextExt<'a>}

impl<'input> Join_clauseContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Join_clauseContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Join_clauseContextExt { ph: PhantomData },
        ))
    }
}

pub trait Join_clauseContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Join_clauseContextExt<'input>>
{
    fn table_or_subquery_all(&self) -> Vec<Rc<Table_or_subqueryContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn table_or_subquery(&self, i: usize) -> Option<Rc<Table_or_subqueryContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn join_operator_all(&self) -> Vec<Rc<Join_operatorContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn join_operator(&self, i: usize) -> Option<Rc<Join_operatorContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn join_constraint_all(&self) -> Vec<Rc<Join_constraintContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn join_constraint(&self, i: usize) -> Option<Rc<Join_constraintContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Join_clauseContextAttrs<'input> for Join_clauseContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn join_clause(&mut self) -> Result<Rc<Join_clauseContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Join_clauseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 102, RULE_join_clause);
        let mut _localctx: Rc<Join_clauseContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule table_or_subquery*/
                recog.base.set_state(1486);
                recog.table_or_subquery()?;

                recog.base.set_state(1493);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == COMMA
                    || _la == K_CROSS
                    || (((_la - 87) & !0x3f) == 0
                        && ((1usize << (_la - 87))
                            & ((1usize << (K_INNER - 87))
                                | (1usize << (K_JOIN - 87))
                                | (1usize << (K_LEFT - 87))
                                | (1usize << (K_NATURAL - 87))))
                            != 0)
                {
                    {
                        {
                            /*InvokeRule join_operator*/
                            recog.base.set_state(1487);
                            recog.join_operator()?;

                            /*InvokeRule table_or_subquery*/
                            recog.base.set_state(1488);
                            recog.table_or_subquery()?;

                            /*InvokeRule join_constraint*/
                            recog.base.set_state(1489);
                            recog.join_constraint()?;
                        }
                    }
                    recog.base.set_state(1495);
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
//------------------- join_operator ----------------
pub type Join_operatorContextAll<'input> = Join_operatorContext<'input>;

pub type Join_operatorContext<'input> =
    BaseParserRuleContext<'input, Join_operatorContextExt<'input>>;

#[derive(Clone)]
pub struct Join_operatorContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Join_operatorContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Join_operatorContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_join_operator(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_join_operator(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Join_operatorContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_join_operator
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_join_operator }
}
antlr_rust::tid! {Join_operatorContextExt<'a>}

impl<'input> Join_operatorContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Join_operatorContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Join_operatorContextExt { ph: PhantomData },
        ))
    }
}

pub trait Join_operatorContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Join_operatorContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token COMMA
    /// Returns `None` if there is no child corresponding to token COMMA
    fn COMMA(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_JOIN
    /// Returns `None` if there is no child corresponding to token K_JOIN
    fn K_JOIN(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_JOIN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_NATURAL
    /// Returns `None` if there is no child corresponding to token K_NATURAL
    fn K_NATURAL(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_NATURAL, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_LEFT
    /// Returns `None` if there is no child corresponding to token K_LEFT
    fn K_LEFT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_LEFT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_INNER
    /// Returns `None` if there is no child corresponding to token K_INNER
    fn K_INNER(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_INNER, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_CROSS
    /// Returns `None` if there is no child corresponding to token K_CROSS
    fn K_CROSS(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_CROSS, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_OUTER
    /// Returns `None` if there is no child corresponding to token K_OUTER
    fn K_OUTER(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_OUTER, 0)
    }
}

impl<'input> Join_operatorContextAttrs<'input> for Join_operatorContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn join_operator(&mut self) -> Result<Rc<Join_operatorContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Join_operatorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 104, RULE_join_operator);
        let mut _localctx: Rc<Join_operatorContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(1509);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                COMMA => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        recog.base.set_state(1496);
                        recog.base.match_token(COMMA, &mut recog.err_handler)?;
                    }
                }

                K_CROSS | K_INNER | K_JOIN | K_LEFT | K_NATURAL => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        recog.base.set_state(1498);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == K_NATURAL {
                            {
                                recog.base.set_state(1497);
                                recog.base.match_token(K_NATURAL, &mut recog.err_handler)?;
                            }
                        }

                        recog.base.set_state(1506);
                        recog.err_handler.sync(&mut recog.base)?;
                        match recog.base.input.la(1) {
                            K_LEFT => {
                                recog.base.set_state(1500);
                                recog.base.match_token(K_LEFT, &mut recog.err_handler)?;

                                recog.base.set_state(1502);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                                if _la == K_OUTER {
                                    {
                                        recog.base.set_state(1501);
                                        recog.base.match_token(K_OUTER, &mut recog.err_handler)?;
                                    }
                                }
                            }

                            K_INNER => {
                                recog.base.set_state(1504);
                                recog.base.match_token(K_INNER, &mut recog.err_handler)?;
                            }

                            K_CROSS => {
                                recog.base.set_state(1505);
                                recog.base.match_token(K_CROSS, &mut recog.err_handler)?;
                            }

                            K_JOIN => {}

                            _ => {}
                        }
                        recog.base.set_state(1508);
                        recog.base.match_token(K_JOIN, &mut recog.err_handler)?;
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
//------------------- join_constraint ----------------
pub type Join_constraintContextAll<'input> = Join_constraintContext<'input>;

pub type Join_constraintContext<'input> =
    BaseParserRuleContext<'input, Join_constraintContextExt<'input>>;

#[derive(Clone)]
pub struct Join_constraintContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Join_constraintContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Join_constraintContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_join_constraint(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_join_constraint(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Join_constraintContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_join_constraint
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_join_constraint }
}
antlr_rust::tid! {Join_constraintContextExt<'a>}

impl<'input> Join_constraintContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Join_constraintContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Join_constraintContextExt { ph: PhantomData },
        ))
    }
}

pub trait Join_constraintContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Join_constraintContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token K_ON
    /// Returns `None` if there is no child corresponding to token K_ON
    fn K_ON(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ON, 0)
    }
    fn expr(&self) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_USING
    /// Returns `None` if there is no child corresponding to token K_USING
    fn K_USING(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_USING, 0)
    }
    /// Retrieves first TerminalNode corresponding to token OPEN_PAR
    /// Returns `None` if there is no child corresponding to token OPEN_PAR
    fn OPEN_PAR(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPEN_PAR, 0)
    }
    fn column_name_all(&self) -> Vec<Rc<Column_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn column_name(&self, i: usize) -> Option<Rc<Column_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token CLOSE_PAR
    /// Returns `None` if there is no child corresponding to token CLOSE_PAR
    fn CLOSE_PAR(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CLOSE_PAR, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
}

impl<'input> Join_constraintContextAttrs<'input> for Join_constraintContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn join_constraint(&mut self) -> Result<Rc<Join_constraintContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Join_constraintContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 106, RULE_join_constraint);
        let mut _localctx: Rc<Join_constraintContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1525);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.base.input.la(1) {
                    K_ON => {
                        {
                            recog.base.set_state(1511);
                            recog.base.match_token(K_ON, &mut recog.err_handler)?;

                            /*InvokeRule expr*/
                            recog.base.set_state(1512);
                            recog.expr_rec(0)?;
                        }
                    }

                    K_USING => {
                        {
                            recog.base.set_state(1513);
                            recog.base.match_token(K_USING, &mut recog.err_handler)?;

                            recog.base.set_state(1514);
                            recog.base.match_token(OPEN_PAR, &mut recog.err_handler)?;

                            /*InvokeRule column_name*/
                            recog.base.set_state(1515);
                            recog.column_name()?;

                            recog.base.set_state(1520);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            while _la == COMMA {
                                {
                                    {
                                        recog.base.set_state(1516);
                                        recog.base.match_token(COMMA, &mut recog.err_handler)?;

                                        /*InvokeRule column_name*/
                                        recog.base.set_state(1517);
                                        recog.column_name()?;
                                    }
                                }
                                recog.base.set_state(1522);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                            }
                            recog.base.set_state(1523);
                            recog.base.match_token(CLOSE_PAR, &mut recog.err_handler)?;
                        }
                    }

                    EOF | SCOL | CLOSE_PAR | COMMA | K_ALTER | K_ANALYZE | K_ATTACH | K_BEGIN
                    | K_COMMIT | K_CREATE | K_CROSS | K_DELETE | K_DETACH | K_DROP | K_END
                    | K_EXCEPT | K_EXPLAIN | K_GROUP | K_INNER | K_INSERT | K_INTERSECT
                    | K_JOIN | K_LEFT | K_LIMIT | K_NATURAL | K_ORDER | K_PRAGMA | K_REINDEX
                    | K_RELEASE | K_REPLACE | K_ROLLBACK | K_SAVEPOINT | K_SELECT | K_UNION
                    | K_UPDATE | K_VACUUM | K_VALUES | K_WHERE | K_WITH | UNEXPECTED_CHAR => {}

                    _ => {}
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
//------------------- select_core ----------------
pub type Select_coreContextAll<'input> = Select_coreContext<'input>;

pub type Select_coreContext<'input> = BaseParserRuleContext<'input, Select_coreContextExt<'input>>;

#[derive(Clone)]
pub struct Select_coreContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Select_coreContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Select_coreContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_select_core(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_select_core(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Select_coreContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_select_core
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_select_core }
}
antlr_rust::tid! {Select_coreContextExt<'a>}

impl<'input> Select_coreContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Select_coreContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Select_coreContextExt { ph: PhantomData },
        ))
    }
}

pub trait Select_coreContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Select_coreContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token K_SELECT
    /// Returns `None` if there is no child corresponding to token K_SELECT
    fn K_SELECT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_SELECT, 0)
    }
    fn result_column_all(&self) -> Vec<Rc<Result_columnContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn result_column(&self, i: usize) -> Option<Rc<Result_columnContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
    /// Retrieves first TerminalNode corresponding to token K_FROM
    /// Returns `None` if there is no child corresponding to token K_FROM
    fn K_FROM(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_FROM, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_WHERE
    /// Returns `None` if there is no child corresponding to token K_WHERE
    fn K_WHERE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_WHERE, 0)
    }
    fn expr_all(&self) -> Vec<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token K_GROUP
    /// Returns `None` if there is no child corresponding to token K_GROUP
    fn K_GROUP(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_GROUP, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_BY
    /// Returns `None` if there is no child corresponding to token K_BY
    fn K_BY(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_BY, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_DISTINCT
    /// Returns `None` if there is no child corresponding to token K_DISTINCT
    fn K_DISTINCT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_DISTINCT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_ALL
    /// Returns `None` if there is no child corresponding to token K_ALL
    fn K_ALL(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ALL, 0)
    }
    fn table_or_subquery_all(&self) -> Vec<Rc<Table_or_subqueryContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn table_or_subquery(&self, i: usize) -> Option<Rc<Table_or_subqueryContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn join_clause(&self) -> Option<Rc<Join_clauseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token K_HAVING
    /// Returns `None` if there is no child corresponding to token K_HAVING
    fn K_HAVING(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_HAVING, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_VALUES
    /// Returns `None` if there is no child corresponding to token K_VALUES
    fn K_VALUES(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_VALUES, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token OPEN_PAR in current rule
    fn OPEN_PAR_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token OPEN_PAR, starting from 0.
    /// Returns `None` if number of children corresponding to token OPEN_PAR is less or equal than `i`.
    fn OPEN_PAR(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPEN_PAR, i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token CLOSE_PAR in current rule
    fn CLOSE_PAR_all(&self) -> Vec<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token CLOSE_PAR, starting from 0.
    /// Returns `None` if number of children corresponding to token CLOSE_PAR is less or equal than `i`.
    fn CLOSE_PAR(&self, i: usize) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CLOSE_PAR, i)
    }
}

impl<'input> Select_coreContextAttrs<'input> for Select_coreContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn select_core(&mut self) -> Result<Rc<Select_coreContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Select_coreContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 108, RULE_select_core);
        let mut _localctx: Rc<Select_coreContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(1601);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                K_SELECT => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        recog.base.set_state(1527);
                        recog.base.match_token(K_SELECT, &mut recog.err_handler)?;

                        recog.base.set_state(1529);
                        recog.err_handler.sync(&mut recog.base)?;
                        match recog.interpreter.adaptive_predict(219, &mut recog.base)? {
                            x if x == 1 => {
                                recog.base.set_state(1528);
                                _la = recog.base.input.la(1);
                                if !(_la == K_ALL || _la == K_DISTINCT) {
                                    recog.err_handler.recover_inline(&mut recog.base)?;
                                } else {
                                    if recog.base.input.la(1) == TOKEN_EOF {
                                        recog.base.matched_eof = true
                                    };
                                    recog.err_handler.report_match(&mut recog.base);
                                    recog.base.consume(&mut recog.err_handler);
                                }
                            }

                            _ => {}
                        }
                        /*InvokeRule result_column*/
                        recog.base.set_state(1531);
                        recog.result_column()?;

                        recog.base.set_state(1536);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        while _la == COMMA {
                            {
                                {
                                    recog.base.set_state(1532);
                                    recog.base.match_token(COMMA, &mut recog.err_handler)?;

                                    /*InvokeRule result_column*/
                                    recog.base.set_state(1533);
                                    recog.result_column()?;
                                }
                            }
                            recog.base.set_state(1538);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                        }
                        recog.base.set_state(1551);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == K_FROM {
                            {
                                recog.base.set_state(1539);
                                recog.base.match_token(K_FROM, &mut recog.err_handler)?;

                                recog.base.set_state(1549);
                                recog.err_handler.sync(&mut recog.base)?;
                                match recog.interpreter.adaptive_predict(222, &mut recog.base)? {
                                    1 => {
                                        {
                                            /*InvokeRule table_or_subquery*/
                                            recog.base.set_state(1540);
                                            recog.table_or_subquery()?;

                                            recog.base.set_state(1545);
                                            recog.err_handler.sync(&mut recog.base)?;
                                            _la = recog.base.input.la(1);
                                            while _la == COMMA {
                                                {
                                                    {
                                                        recog.base.set_state(1541);
                                                        recog.base.match_token(
                                                            COMMA,
                                                            &mut recog.err_handler,
                                                        )?;

                                                        /*InvokeRule table_or_subquery*/
                                                        recog.base.set_state(1542);
                                                        recog.table_or_subquery()?;
                                                    }
                                                }
                                                recog.base.set_state(1547);
                                                recog.err_handler.sync(&mut recog.base)?;
                                                _la = recog.base.input.la(1);
                                            }
                                        }
                                    }
                                    2 => {
                                        {
                                            /*InvokeRule join_clause*/
                                            recog.base.set_state(1548);
                                            recog.join_clause()?;
                                        }
                                    }

                                    _ => {}
                                }
                            }
                        }

                        recog.base.set_state(1555);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == K_WHERE {
                            {
                                recog.base.set_state(1553);
                                recog.base.match_token(K_WHERE, &mut recog.err_handler)?;

                                /*InvokeRule expr*/
                                recog.base.set_state(1554);
                                recog.expr_rec(0)?;
                            }
                        }

                        recog.base.set_state(1571);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == K_GROUP {
                            {
                                recog.base.set_state(1557);
                                recog.base.match_token(K_GROUP, &mut recog.err_handler)?;

                                recog.base.set_state(1558);
                                recog.base.match_token(K_BY, &mut recog.err_handler)?;

                                /*InvokeRule expr*/
                                recog.base.set_state(1559);
                                recog.expr_rec(0)?;

                                recog.base.set_state(1564);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                                while _la == COMMA {
                                    {
                                        {
                                            recog.base.set_state(1560);
                                            recog
                                                .base
                                                .match_token(COMMA, &mut recog.err_handler)?;

                                            /*InvokeRule expr*/
                                            recog.base.set_state(1561);
                                            recog.expr_rec(0)?;
                                        }
                                    }
                                    recog.base.set_state(1566);
                                    recog.err_handler.sync(&mut recog.base)?;
                                    _la = recog.base.input.la(1);
                                }
                                recog.base.set_state(1569);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                                if _la == K_HAVING {
                                    {
                                        recog.base.set_state(1567);
                                        recog.base.match_token(K_HAVING, &mut recog.err_handler)?;

                                        /*InvokeRule expr*/
                                        recog.base.set_state(1568);
                                        recog.expr_rec(0)?;
                                    }
                                }
                            }
                        }
                    }
                }

                K_VALUES => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        recog.base.set_state(1573);
                        recog.base.match_token(K_VALUES, &mut recog.err_handler)?;

                        recog.base.set_state(1574);
                        recog.base.match_token(OPEN_PAR, &mut recog.err_handler)?;

                        /*InvokeRule expr*/
                        recog.base.set_state(1575);
                        recog.expr_rec(0)?;

                        recog.base.set_state(1580);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        while _la == COMMA {
                            {
                                {
                                    recog.base.set_state(1576);
                                    recog.base.match_token(COMMA, &mut recog.err_handler)?;

                                    /*InvokeRule expr*/
                                    recog.base.set_state(1577);
                                    recog.expr_rec(0)?;
                                }
                            }
                            recog.base.set_state(1582);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                        }
                        recog.base.set_state(1583);
                        recog.base.match_token(CLOSE_PAR, &mut recog.err_handler)?;

                        recog.base.set_state(1598);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        while _la == COMMA {
                            {
                                {
                                    recog.base.set_state(1584);
                                    recog.base.match_token(COMMA, &mut recog.err_handler)?;

                                    recog.base.set_state(1585);
                                    recog.base.match_token(OPEN_PAR, &mut recog.err_handler)?;

                                    /*InvokeRule expr*/
                                    recog.base.set_state(1586);
                                    recog.expr_rec(0)?;

                                    recog.base.set_state(1591);
                                    recog.err_handler.sync(&mut recog.base)?;
                                    _la = recog.base.input.la(1);
                                    while _la == COMMA {
                                        {
                                            {
                                                recog.base.set_state(1587);
                                                recog
                                                    .base
                                                    .match_token(COMMA, &mut recog.err_handler)?;

                                                /*InvokeRule expr*/
                                                recog.base.set_state(1588);
                                                recog.expr_rec(0)?;
                                            }
                                        }
                                        recog.base.set_state(1593);
                                        recog.err_handler.sync(&mut recog.base)?;
                                        _la = recog.base.input.la(1);
                                    }
                                    recog.base.set_state(1594);
                                    recog.base.match_token(CLOSE_PAR, &mut recog.err_handler)?;
                                }
                            }
                            recog.base.set_state(1600);
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
//------------------- compound_operator ----------------
pub type Compound_operatorContextAll<'input> = Compound_operatorContext<'input>;

pub type Compound_operatorContext<'input> =
    BaseParserRuleContext<'input, Compound_operatorContextExt<'input>>;

#[derive(Clone)]
pub struct Compound_operatorContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Compound_operatorContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Compound_operatorContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_compound_operator(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_compound_operator(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Compound_operatorContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_compound_operator
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_compound_operator }
}
antlr_rust::tid! {Compound_operatorContextExt<'a>}

impl<'input> Compound_operatorContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Compound_operatorContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Compound_operatorContextExt { ph: PhantomData },
        ))
    }
}

pub trait Compound_operatorContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Compound_operatorContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token K_UNION
    /// Returns `None` if there is no child corresponding to token K_UNION
    fn K_UNION(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_UNION, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_ALL
    /// Returns `None` if there is no child corresponding to token K_ALL
    fn K_ALL(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ALL, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_INTERSECT
    /// Returns `None` if there is no child corresponding to token K_INTERSECT
    fn K_INTERSECT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_INTERSECT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_EXCEPT
    /// Returns `None` if there is no child corresponding to token K_EXCEPT
    fn K_EXCEPT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_EXCEPT, 0)
    }
}

impl<'input> Compound_operatorContextAttrs<'input> for Compound_operatorContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn compound_operator(
        &mut self,
    ) -> Result<Rc<Compound_operatorContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Compound_operatorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 110, RULE_compound_operator);
        let mut _localctx: Rc<Compound_operatorContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(1608);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(232, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        recog.base.set_state(1603);
                        recog.base.match_token(K_UNION, &mut recog.err_handler)?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        recog.base.set_state(1604);
                        recog.base.match_token(K_UNION, &mut recog.err_handler)?;

                        recog.base.set_state(1605);
                        recog.base.match_token(K_ALL, &mut recog.err_handler)?;
                    }
                }
                3 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        recog.base.set_state(1606);
                        recog
                            .base
                            .match_token(K_INTERSECT, &mut recog.err_handler)?;
                    }
                }
                4 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 4);
                    recog.base.enter_outer_alt(None, 4);
                    {
                        recog.base.set_state(1607);
                        recog.base.match_token(K_EXCEPT, &mut recog.err_handler)?;
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
//------------------- signed_number ----------------
pub type Signed_numberContextAll<'input> = Signed_numberContext<'input>;

pub type Signed_numberContext<'input> =
    BaseParserRuleContext<'input, Signed_numberContextExt<'input>>;

#[derive(Clone)]
pub struct Signed_numberContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Signed_numberContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Signed_numberContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_signed_number(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_signed_number(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Signed_numberContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_signed_number
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_signed_number }
}
antlr_rust::tid! {Signed_numberContextExt<'a>}

impl<'input> Signed_numberContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Signed_numberContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Signed_numberContextExt { ph: PhantomData },
        ))
    }
}

pub trait Signed_numberContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Signed_numberContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token NUMERIC_LITERAL
    /// Returns `None` if there is no child corresponding to token NUMERIC_LITERAL
    fn NUMERIC_LITERAL(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NUMERIC_LITERAL, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PLUS
    /// Returns `None` if there is no child corresponding to token PLUS
    fn PLUS(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PLUS, 0)
    }
    /// Retrieves first TerminalNode corresponding to token MINUS
    /// Returns `None` if there is no child corresponding to token MINUS
    fn MINUS(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(MINUS, 0)
    }
}

impl<'input> Signed_numberContextAttrs<'input> for Signed_numberContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn signed_number(&mut self) -> Result<Rc<Signed_numberContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Signed_numberContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 112, RULE_signed_number);
        let mut _localctx: Rc<Signed_numberContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1611);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == PLUS || _la == MINUS {
                    {
                        recog.base.set_state(1610);
                        _la = recog.base.input.la(1);
                        if !(_la == PLUS || _la == MINUS) {
                            recog.err_handler.recover_inline(&mut recog.base)?;
                        } else {
                            if recog.base.input.la(1) == TOKEN_EOF {
                                recog.base.matched_eof = true
                            };
                            recog.err_handler.report_match(&mut recog.base);
                            recog.base.consume(&mut recog.err_handler);
                        }
                    }
                }

                recog.base.set_state(1613);
                recog
                    .base
                    .match_token(NUMERIC_LITERAL, &mut recog.err_handler)?;
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
//------------------- literal_value ----------------
pub type Literal_valueContextAll<'input> = Literal_valueContext<'input>;

pub type Literal_valueContext<'input> =
    BaseParserRuleContext<'input, Literal_valueContextExt<'input>>;

#[derive(Clone)]
pub struct Literal_valueContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Literal_valueContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Literal_valueContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_literal_value(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_literal_value(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Literal_valueContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_literal_value
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_literal_value }
}
antlr_rust::tid! {Literal_valueContextExt<'a>}

impl<'input> Literal_valueContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Literal_valueContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Literal_valueContextExt { ph: PhantomData },
        ))
    }
}

pub trait Literal_valueContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Literal_valueContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token NUMERIC_LITERAL
    /// Returns `None` if there is no child corresponding to token NUMERIC_LITERAL
    fn NUMERIC_LITERAL(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NUMERIC_LITERAL, 0)
    }
    /// Retrieves first TerminalNode corresponding to token STRING_LITERAL
    /// Returns `None` if there is no child corresponding to token STRING_LITERAL
    fn STRING_LITERAL(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(STRING_LITERAL, 0)
    }
    /// Retrieves first TerminalNode corresponding to token BLOB_LITERAL
    /// Returns `None` if there is no child corresponding to token BLOB_LITERAL
    fn BLOB_LITERAL(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(BLOB_LITERAL, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_NULL
    /// Returns `None` if there is no child corresponding to token K_NULL
    fn K_NULL(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_NULL, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_CURRENT_TIME
    /// Returns `None` if there is no child corresponding to token K_CURRENT_TIME
    fn K_CURRENT_TIME(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_CURRENT_TIME, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_CURRENT_DATE
    /// Returns `None` if there is no child corresponding to token K_CURRENT_DATE
    fn K_CURRENT_DATE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_CURRENT_DATE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_CURRENT_TIMESTAMP
    /// Returns `None` if there is no child corresponding to token K_CURRENT_TIMESTAMP
    fn K_CURRENT_TIMESTAMP(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_CURRENT_TIMESTAMP, 0)
    }
}

impl<'input> Literal_valueContextAttrs<'input> for Literal_valueContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn literal_value(&mut self) -> Result<Rc<Literal_valueContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Literal_valueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 114, RULE_literal_value);
        let mut _localctx: Rc<Literal_valueContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1615);
                _la = recog.base.input.la(1);
                if !((((_la - 52) & !0x3f) == 0
                    && ((1usize << (_la - 52))
                        & ((1usize << (K_CURRENT_DATE - 52))
                            | (1usize << (K_CURRENT_TIME - 52))
                            | (1usize << (K_CURRENT_TIMESTAMP - 52))))
                        != 0)
                    || _la == K_NULL
                    || (((_la - 150) & !0x3f) == 0
                        && ((1usize << (_la - 150))
                            & ((1usize << (NUMERIC_LITERAL - 150))
                                | (1usize << (STRING_LITERAL - 150))
                                | (1usize << (BLOB_LITERAL - 150))))
                            != 0))
                {
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
//------------------- unary_operator ----------------
pub type Unary_operatorContextAll<'input> = Unary_operatorContext<'input>;

pub type Unary_operatorContext<'input> =
    BaseParserRuleContext<'input, Unary_operatorContextExt<'input>>;

#[derive(Clone)]
pub struct Unary_operatorContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Unary_operatorContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Unary_operatorContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_unary_operator(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_unary_operator(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Unary_operatorContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_unary_operator
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_unary_operator }
}
antlr_rust::tid! {Unary_operatorContextExt<'a>}

impl<'input> Unary_operatorContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Unary_operatorContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Unary_operatorContextExt { ph: PhantomData },
        ))
    }
}

pub trait Unary_operatorContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Unary_operatorContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token MINUS
    /// Returns `None` if there is no child corresponding to token MINUS
    fn MINUS(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(MINUS, 0)
    }
    /// Retrieves first TerminalNode corresponding to token PLUS
    /// Returns `None` if there is no child corresponding to token PLUS
    fn PLUS(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PLUS, 0)
    }
    /// Retrieves first TerminalNode corresponding to token TILDE
    /// Returns `None` if there is no child corresponding to token TILDE
    fn TILDE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(TILDE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_NOT
    /// Returns `None` if there is no child corresponding to token K_NOT
    fn K_NOT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_NOT, 0)
    }
}

impl<'input> Unary_operatorContextAttrs<'input> for Unary_operatorContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn unary_operator(&mut self) -> Result<Rc<Unary_operatorContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Unary_operatorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 116, RULE_unary_operator);
        let mut _localctx: Rc<Unary_operatorContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1617);
                _la = recog.base.input.la(1);
                if !((((_la) & !0x3f) == 0
                    && ((1usize << _la)
                        & ((1usize << PLUS) | (1usize << MINUS) | (1usize << TILDE)))
                        != 0)
                    || _la == K_NOT)
                {
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
//------------------- error_message ----------------
pub type Error_messageContextAll<'input> = Error_messageContext<'input>;

pub type Error_messageContext<'input> =
    BaseParserRuleContext<'input, Error_messageContextExt<'input>>;

#[derive(Clone)]
pub struct Error_messageContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Error_messageContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Error_messageContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_error_message(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_error_message(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Error_messageContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_error_message
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_error_message }
}
antlr_rust::tid! {Error_messageContextExt<'a>}

impl<'input> Error_messageContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Error_messageContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Error_messageContextExt { ph: PhantomData },
        ))
    }
}

pub trait Error_messageContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Error_messageContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token STRING_LITERAL
    /// Returns `None` if there is no child corresponding to token STRING_LITERAL
    fn STRING_LITERAL(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(STRING_LITERAL, 0)
    }
}

impl<'input> Error_messageContextAttrs<'input> for Error_messageContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn error_message(&mut self) -> Result<Rc<Error_messageContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Error_messageContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 118, RULE_error_message);
        let mut _localctx: Rc<Error_messageContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1619);
                recog
                    .base
                    .match_token(STRING_LITERAL, &mut recog.err_handler)?;
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
//------------------- module_argument ----------------
pub type Module_argumentContextAll<'input> = Module_argumentContext<'input>;

pub type Module_argumentContext<'input> =
    BaseParserRuleContext<'input, Module_argumentContextExt<'input>>;

#[derive(Clone)]
pub struct Module_argumentContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Module_argumentContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Module_argumentContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_module_argument(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_module_argument(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Module_argumentContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_module_argument
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_module_argument }
}
antlr_rust::tid! {Module_argumentContextExt<'a>}

impl<'input> Module_argumentContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Module_argumentContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Module_argumentContextExt { ph: PhantomData },
        ))
    }
}

pub trait Module_argumentContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Module_argumentContextExt<'input>>
{
    fn expr(&self) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn column_def(&self) -> Option<Rc<Column_defContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Module_argumentContextAttrs<'input> for Module_argumentContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn module_argument(&mut self) -> Result<Rc<Module_argumentContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Module_argumentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 120, RULE_module_argument);
        let mut _localctx: Rc<Module_argumentContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(1623);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(234, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule expr*/
                        recog.base.set_state(1621);
                        recog.expr_rec(0)?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule column_def*/
                        recog.base.set_state(1622);
                        recog.column_def()?;
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
//------------------- column_alias ----------------
pub type Column_aliasContextAll<'input> = Column_aliasContext<'input>;

pub type Column_aliasContext<'input> =
    BaseParserRuleContext<'input, Column_aliasContextExt<'input>>;

#[derive(Clone)]
pub struct Column_aliasContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Column_aliasContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Column_aliasContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_column_alias(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_column_alias(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Column_aliasContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_column_alias
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_column_alias }
}
antlr_rust::tid! {Column_aliasContextExt<'a>}

impl<'input> Column_aliasContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Column_aliasContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Column_aliasContextExt { ph: PhantomData },
        ))
    }
}

pub trait Column_aliasContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Column_aliasContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token IDENTIFIER
    /// Returns `None` if there is no child corresponding to token IDENTIFIER
    fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(IDENTIFIER, 0)
    }
    /// Retrieves first TerminalNode corresponding to token STRING_LITERAL
    /// Returns `None` if there is no child corresponding to token STRING_LITERAL
    fn STRING_LITERAL(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(STRING_LITERAL, 0)
    }
}

impl<'input> Column_aliasContextAttrs<'input> for Column_aliasContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn column_alias(&mut self) -> Result<Rc<Column_aliasContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Column_aliasContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 122, RULE_column_alias);
        let mut _localctx: Rc<Column_aliasContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1625);
                _la = recog.base.input.la(1);
                if !(_la == IDENTIFIER || _la == STRING_LITERAL) {
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
//------------------- keyword ----------------
pub type KeywordContextAll<'input> = KeywordContext<'input>;

pub type KeywordContext<'input> = BaseParserRuleContext<'input, KeywordContextExt<'input>>;

#[derive(Clone)]
pub struct KeywordContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for KeywordContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for KeywordContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_keyword(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_keyword(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for KeywordContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_keyword
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_keyword }
}
antlr_rust::tid! {KeywordContextExt<'a>}

impl<'input> KeywordContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<KeywordContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            KeywordContextExt { ph: PhantomData },
        ))
    }
}

pub trait KeywordContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<KeywordContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token K_ABORT
    /// Returns `None` if there is no child corresponding to token K_ABORT
    fn K_ABORT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ABORT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_ACTION
    /// Returns `None` if there is no child corresponding to token K_ACTION
    fn K_ACTION(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ACTION, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_ADD
    /// Returns `None` if there is no child corresponding to token K_ADD
    fn K_ADD(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ADD, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_AFTER
    /// Returns `None` if there is no child corresponding to token K_AFTER
    fn K_AFTER(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_AFTER, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_ALL
    /// Returns `None` if there is no child corresponding to token K_ALL
    fn K_ALL(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ALL, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_ALTER
    /// Returns `None` if there is no child corresponding to token K_ALTER
    fn K_ALTER(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ALTER, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_ANALYZE
    /// Returns `None` if there is no child corresponding to token K_ANALYZE
    fn K_ANALYZE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ANALYZE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_AND
    /// Returns `None` if there is no child corresponding to token K_AND
    fn K_AND(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_AND, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_AS
    /// Returns `None` if there is no child corresponding to token K_AS
    fn K_AS(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_AS, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_ASC
    /// Returns `None` if there is no child corresponding to token K_ASC
    fn K_ASC(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ASC, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_ATTACH
    /// Returns `None` if there is no child corresponding to token K_ATTACH
    fn K_ATTACH(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ATTACH, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_AUTOINCREMENT
    /// Returns `None` if there is no child corresponding to token K_AUTOINCREMENT
    fn K_AUTOINCREMENT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_AUTOINCREMENT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_BEFORE
    /// Returns `None` if there is no child corresponding to token K_BEFORE
    fn K_BEFORE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_BEFORE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_BEGIN
    /// Returns `None` if there is no child corresponding to token K_BEGIN
    fn K_BEGIN(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_BEGIN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_BETWEEN
    /// Returns `None` if there is no child corresponding to token K_BETWEEN
    fn K_BETWEEN(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_BETWEEN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_BY
    /// Returns `None` if there is no child corresponding to token K_BY
    fn K_BY(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_BY, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_CASCADE
    /// Returns `None` if there is no child corresponding to token K_CASCADE
    fn K_CASCADE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_CASCADE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_CASE
    /// Returns `None` if there is no child corresponding to token K_CASE
    fn K_CASE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_CASE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_CAST
    /// Returns `None` if there is no child corresponding to token K_CAST
    fn K_CAST(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_CAST, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_CHECK
    /// Returns `None` if there is no child corresponding to token K_CHECK
    fn K_CHECK(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_CHECK, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_COLLATE
    /// Returns `None` if there is no child corresponding to token K_COLLATE
    fn K_COLLATE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_COLLATE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_COLUMN
    /// Returns `None` if there is no child corresponding to token K_COLUMN
    fn K_COLUMN(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_COLUMN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_COMMIT
    /// Returns `None` if there is no child corresponding to token K_COMMIT
    fn K_COMMIT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_COMMIT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_CONFLICT
    /// Returns `None` if there is no child corresponding to token K_CONFLICT
    fn K_CONFLICT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_CONFLICT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_CONSTRAINT
    /// Returns `None` if there is no child corresponding to token K_CONSTRAINT
    fn K_CONSTRAINT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_CONSTRAINT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_CREATE
    /// Returns `None` if there is no child corresponding to token K_CREATE
    fn K_CREATE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_CREATE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_CROSS
    /// Returns `None` if there is no child corresponding to token K_CROSS
    fn K_CROSS(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_CROSS, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_CURRENT_DATE
    /// Returns `None` if there is no child corresponding to token K_CURRENT_DATE
    fn K_CURRENT_DATE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_CURRENT_DATE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_CURRENT_TIME
    /// Returns `None` if there is no child corresponding to token K_CURRENT_TIME
    fn K_CURRENT_TIME(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_CURRENT_TIME, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_CURRENT_TIMESTAMP
    /// Returns `None` if there is no child corresponding to token K_CURRENT_TIMESTAMP
    fn K_CURRENT_TIMESTAMP(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_CURRENT_TIMESTAMP, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_DATABASE
    /// Returns `None` if there is no child corresponding to token K_DATABASE
    fn K_DATABASE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_DATABASE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_DEFAULT
    /// Returns `None` if there is no child corresponding to token K_DEFAULT
    fn K_DEFAULT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_DEFAULT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_DEFERRABLE
    /// Returns `None` if there is no child corresponding to token K_DEFERRABLE
    fn K_DEFERRABLE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_DEFERRABLE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_DEFERRED
    /// Returns `None` if there is no child corresponding to token K_DEFERRED
    fn K_DEFERRED(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_DEFERRED, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_DELETE
    /// Returns `None` if there is no child corresponding to token K_DELETE
    fn K_DELETE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_DELETE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_DESC
    /// Returns `None` if there is no child corresponding to token K_DESC
    fn K_DESC(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_DESC, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_DETACH
    /// Returns `None` if there is no child corresponding to token K_DETACH
    fn K_DETACH(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_DETACH, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_DISTINCT
    /// Returns `None` if there is no child corresponding to token K_DISTINCT
    fn K_DISTINCT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_DISTINCT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_DROP
    /// Returns `None` if there is no child corresponding to token K_DROP
    fn K_DROP(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_DROP, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_EACH
    /// Returns `None` if there is no child corresponding to token K_EACH
    fn K_EACH(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_EACH, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_ELSE
    /// Returns `None` if there is no child corresponding to token K_ELSE
    fn K_ELSE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ELSE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_END
    /// Returns `None` if there is no child corresponding to token K_END
    fn K_END(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_END, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_ESCAPE
    /// Returns `None` if there is no child corresponding to token K_ESCAPE
    fn K_ESCAPE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ESCAPE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_EXCEPT
    /// Returns `None` if there is no child corresponding to token K_EXCEPT
    fn K_EXCEPT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_EXCEPT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_EXCLUSIVE
    /// Returns `None` if there is no child corresponding to token K_EXCLUSIVE
    fn K_EXCLUSIVE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_EXCLUSIVE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_EXISTS
    /// Returns `None` if there is no child corresponding to token K_EXISTS
    fn K_EXISTS(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_EXISTS, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_EXPLAIN
    /// Returns `None` if there is no child corresponding to token K_EXPLAIN
    fn K_EXPLAIN(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_EXPLAIN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_FAIL
    /// Returns `None` if there is no child corresponding to token K_FAIL
    fn K_FAIL(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_FAIL, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_FOR
    /// Returns `None` if there is no child corresponding to token K_FOR
    fn K_FOR(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_FOR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_FOREIGN
    /// Returns `None` if there is no child corresponding to token K_FOREIGN
    fn K_FOREIGN(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_FOREIGN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_FROM
    /// Returns `None` if there is no child corresponding to token K_FROM
    fn K_FROM(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_FROM, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_FULL
    /// Returns `None` if there is no child corresponding to token K_FULL
    fn K_FULL(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_FULL, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_GLOB
    /// Returns `None` if there is no child corresponding to token K_GLOB
    fn K_GLOB(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_GLOB, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_GROUP
    /// Returns `None` if there is no child corresponding to token K_GROUP
    fn K_GROUP(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_GROUP, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_HAVING
    /// Returns `None` if there is no child corresponding to token K_HAVING
    fn K_HAVING(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_HAVING, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_IF
    /// Returns `None` if there is no child corresponding to token K_IF
    fn K_IF(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_IF, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_IGNORE
    /// Returns `None` if there is no child corresponding to token K_IGNORE
    fn K_IGNORE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_IGNORE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_IMMEDIATE
    /// Returns `None` if there is no child corresponding to token K_IMMEDIATE
    fn K_IMMEDIATE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_IMMEDIATE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_IN
    /// Returns `None` if there is no child corresponding to token K_IN
    fn K_IN(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_IN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_INDEX
    /// Returns `None` if there is no child corresponding to token K_INDEX
    fn K_INDEX(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_INDEX, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_INDEXED
    /// Returns `None` if there is no child corresponding to token K_INDEXED
    fn K_INDEXED(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_INDEXED, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_INITIALLY
    /// Returns `None` if there is no child corresponding to token K_INITIALLY
    fn K_INITIALLY(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_INITIALLY, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_INNER
    /// Returns `None` if there is no child corresponding to token K_INNER
    fn K_INNER(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_INNER, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_INSERT
    /// Returns `None` if there is no child corresponding to token K_INSERT
    fn K_INSERT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_INSERT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_INSTEAD
    /// Returns `None` if there is no child corresponding to token K_INSTEAD
    fn K_INSTEAD(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_INSTEAD, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_INTERSECT
    /// Returns `None` if there is no child corresponding to token K_INTERSECT
    fn K_INTERSECT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_INTERSECT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_INTO
    /// Returns `None` if there is no child corresponding to token K_INTO
    fn K_INTO(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_INTO, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_IS
    /// Returns `None` if there is no child corresponding to token K_IS
    fn K_IS(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_IS, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_ISNULL
    /// Returns `None` if there is no child corresponding to token K_ISNULL
    fn K_ISNULL(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ISNULL, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_JOIN
    /// Returns `None` if there is no child corresponding to token K_JOIN
    fn K_JOIN(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_JOIN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_KEY
    /// Returns `None` if there is no child corresponding to token K_KEY
    fn K_KEY(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_KEY, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_LEFT
    /// Returns `None` if there is no child corresponding to token K_LEFT
    fn K_LEFT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_LEFT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_LIKE
    /// Returns `None` if there is no child corresponding to token K_LIKE
    fn K_LIKE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_LIKE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_LIMIT
    /// Returns `None` if there is no child corresponding to token K_LIMIT
    fn K_LIMIT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_LIMIT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_MATCH
    /// Returns `None` if there is no child corresponding to token K_MATCH
    fn K_MATCH(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_MATCH, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_NATURAL
    /// Returns `None` if there is no child corresponding to token K_NATURAL
    fn K_NATURAL(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_NATURAL, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_NO
    /// Returns `None` if there is no child corresponding to token K_NO
    fn K_NO(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_NO, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_NOT
    /// Returns `None` if there is no child corresponding to token K_NOT
    fn K_NOT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_NOT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_NOTNULL
    /// Returns `None` if there is no child corresponding to token K_NOTNULL
    fn K_NOTNULL(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_NOTNULL, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_NULL
    /// Returns `None` if there is no child corresponding to token K_NULL
    fn K_NULL(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_NULL, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_OF
    /// Returns `None` if there is no child corresponding to token K_OF
    fn K_OF(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_OF, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_OFFSET
    /// Returns `None` if there is no child corresponding to token K_OFFSET
    fn K_OFFSET(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_OFFSET, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_ON
    /// Returns `None` if there is no child corresponding to token K_ON
    fn K_ON(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ON, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_OR
    /// Returns `None` if there is no child corresponding to token K_OR
    fn K_OR(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_OR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_ORDER
    /// Returns `None` if there is no child corresponding to token K_ORDER
    fn K_ORDER(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ORDER, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_OUTER
    /// Returns `None` if there is no child corresponding to token K_OUTER
    fn K_OUTER(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_OUTER, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_PLAN
    /// Returns `None` if there is no child corresponding to token K_PLAN
    fn K_PLAN(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_PLAN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_PRAGMA
    /// Returns `None` if there is no child corresponding to token K_PRAGMA
    fn K_PRAGMA(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_PRAGMA, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_PRIMARY
    /// Returns `None` if there is no child corresponding to token K_PRIMARY
    fn K_PRIMARY(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_PRIMARY, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_QUERY
    /// Returns `None` if there is no child corresponding to token K_QUERY
    fn K_QUERY(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_QUERY, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_RAISE
    /// Returns `None` if there is no child corresponding to token K_RAISE
    fn K_RAISE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_RAISE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_RECURSIVE
    /// Returns `None` if there is no child corresponding to token K_RECURSIVE
    fn K_RECURSIVE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_RECURSIVE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_REFERENCES
    /// Returns `None` if there is no child corresponding to token K_REFERENCES
    fn K_REFERENCES(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_REFERENCES, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_REGEXP
    /// Returns `None` if there is no child corresponding to token K_REGEXP
    fn K_REGEXP(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_REGEXP, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_REINDEX
    /// Returns `None` if there is no child corresponding to token K_REINDEX
    fn K_REINDEX(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_REINDEX, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_RELEASE
    /// Returns `None` if there is no child corresponding to token K_RELEASE
    fn K_RELEASE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_RELEASE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_RENAME
    /// Returns `None` if there is no child corresponding to token K_RENAME
    fn K_RENAME(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_RENAME, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_REPLACE
    /// Returns `None` if there is no child corresponding to token K_REPLACE
    fn K_REPLACE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_REPLACE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_RESTRICT
    /// Returns `None` if there is no child corresponding to token K_RESTRICT
    fn K_RESTRICT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_RESTRICT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_RIGHT
    /// Returns `None` if there is no child corresponding to token K_RIGHT
    fn K_RIGHT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_RIGHT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_ROLLBACK
    /// Returns `None` if there is no child corresponding to token K_ROLLBACK
    fn K_ROLLBACK(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ROLLBACK, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_ROW
    /// Returns `None` if there is no child corresponding to token K_ROW
    fn K_ROW(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_ROW, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_SAVEPOINT
    /// Returns `None` if there is no child corresponding to token K_SAVEPOINT
    fn K_SAVEPOINT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_SAVEPOINT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_SELECT
    /// Returns `None` if there is no child corresponding to token K_SELECT
    fn K_SELECT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_SELECT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_SET
    /// Returns `None` if there is no child corresponding to token K_SET
    fn K_SET(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_SET, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_TABLE
    /// Returns `None` if there is no child corresponding to token K_TABLE
    fn K_TABLE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_TABLE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_TEMP
    /// Returns `None` if there is no child corresponding to token K_TEMP
    fn K_TEMP(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_TEMP, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_TEMPORARY
    /// Returns `None` if there is no child corresponding to token K_TEMPORARY
    fn K_TEMPORARY(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_TEMPORARY, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_THEN
    /// Returns `None` if there is no child corresponding to token K_THEN
    fn K_THEN(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_THEN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_TO
    /// Returns `None` if there is no child corresponding to token K_TO
    fn K_TO(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_TO, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_TRANSACTION
    /// Returns `None` if there is no child corresponding to token K_TRANSACTION
    fn K_TRANSACTION(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_TRANSACTION, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_TRIGGER
    /// Returns `None` if there is no child corresponding to token K_TRIGGER
    fn K_TRIGGER(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_TRIGGER, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_UNION
    /// Returns `None` if there is no child corresponding to token K_UNION
    fn K_UNION(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_UNION, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_UNIQUE
    /// Returns `None` if there is no child corresponding to token K_UNIQUE
    fn K_UNIQUE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_UNIQUE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_UPDATE
    /// Returns `None` if there is no child corresponding to token K_UPDATE
    fn K_UPDATE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_UPDATE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_USING
    /// Returns `None` if there is no child corresponding to token K_USING
    fn K_USING(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_USING, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_VACUUM
    /// Returns `None` if there is no child corresponding to token K_VACUUM
    fn K_VACUUM(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_VACUUM, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_VALUES
    /// Returns `None` if there is no child corresponding to token K_VALUES
    fn K_VALUES(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_VALUES, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_VIEW
    /// Returns `None` if there is no child corresponding to token K_VIEW
    fn K_VIEW(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_VIEW, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_VIRTUAL
    /// Returns `None` if there is no child corresponding to token K_VIRTUAL
    fn K_VIRTUAL(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_VIRTUAL, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_WHEN
    /// Returns `None` if there is no child corresponding to token K_WHEN
    fn K_WHEN(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_WHEN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_WHERE
    /// Returns `None` if there is no child corresponding to token K_WHERE
    fn K_WHERE(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_WHERE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_WITH
    /// Returns `None` if there is no child corresponding to token K_WITH
    fn K_WITH(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_WITH, 0)
    }
    /// Retrieves first TerminalNode corresponding to token K_WITHOUT
    /// Returns `None` if there is no child corresponding to token K_WITHOUT
    fn K_WITHOUT(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(K_WITHOUT, 0)
    }
}

impl<'input> KeywordContextAttrs<'input> for KeywordContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn keyword(&mut self) -> Result<Rc<KeywordContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = KeywordContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 124, RULE_keyword);
        let mut _localctx: Rc<KeywordContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1627);
                _la = recog.base.input.la(1);
                if !((((_la - 25) & !0x3f) == 0
                    && ((1usize << (_la - 25))
                        & ((1usize << (K_ABORT - 25))
                            | (1usize << (K_ACTION - 25))
                            | (1usize << (K_ADD - 25))
                            | (1usize << (K_AFTER - 25))
                            | (1usize << (K_ALL - 25))
                            | (1usize << (K_ALTER - 25))
                            | (1usize << (K_ANALYZE - 25))
                            | (1usize << (K_AND - 25))
                            | (1usize << (K_AS - 25))
                            | (1usize << (K_ASC - 25))
                            | (1usize << (K_ATTACH - 25))
                            | (1usize << (K_AUTOINCREMENT - 25))
                            | (1usize << (K_BEFORE - 25))
                            | (1usize << (K_BEGIN - 25))
                            | (1usize << (K_BETWEEN - 25))
                            | (1usize << (K_BY - 25))
                            | (1usize << (K_CASCADE - 25))
                            | (1usize << (K_CASE - 25))
                            | (1usize << (K_CAST - 25))
                            | (1usize << (K_CHECK - 25))
                            | (1usize << (K_COLLATE - 25))
                            | (1usize << (K_COLUMN - 25))
                            | (1usize << (K_COMMIT - 25))
                            | (1usize << (K_CONFLICT - 25))
                            | (1usize << (K_CONSTRAINT - 25))
                            | (1usize << (K_CREATE - 25))
                            | (1usize << (K_CROSS - 25))
                            | (1usize << (K_CURRENT_DATE - 25))
                            | (1usize << (K_CURRENT_TIME - 25))
                            | (1usize << (K_CURRENT_TIMESTAMP - 25))
                            | (1usize << (K_DATABASE - 25))
                            | (1usize << (K_DEFAULT - 25))))
                        != 0)
                    || (((_la - 57) & !0x3f) == 0
                        && ((1usize << (_la - 57))
                            & ((1usize << (K_DEFERRABLE - 57))
                                | (1usize << (K_DEFERRED - 57))
                                | (1usize << (K_DELETE - 57))
                                | (1usize << (K_DESC - 57))
                                | (1usize << (K_DETACH - 57))
                                | (1usize << (K_DISTINCT - 57))
                                | (1usize << (K_DROP - 57))
                                | (1usize << (K_EACH - 57))
                                | (1usize << (K_ELSE - 57))
                                | (1usize << (K_END - 57))
                                | (1usize << (K_ESCAPE - 57))
                                | (1usize << (K_EXCEPT - 57))
                                | (1usize << (K_EXCLUSIVE - 57))
                                | (1usize << (K_EXISTS - 57))
                                | (1usize << (K_EXPLAIN - 57))
                                | (1usize << (K_FAIL - 57))
                                | (1usize << (K_FOR - 57))
                                | (1usize << (K_FOREIGN - 57))
                                | (1usize << (K_FROM - 57))
                                | (1usize << (K_FULL - 57))
                                | (1usize << (K_GLOB - 57))
                                | (1usize << (K_GROUP - 57))
                                | (1usize << (K_HAVING - 57))
                                | (1usize << (K_IF - 57))
                                | (1usize << (K_IGNORE - 57))
                                | (1usize << (K_IMMEDIATE - 57))
                                | (1usize << (K_IN - 57))
                                | (1usize << (K_INDEX - 57))
                                | (1usize << (K_INDEXED - 57))
                                | (1usize << (K_INITIALLY - 57))
                                | (1usize << (K_INNER - 57))
                                | (1usize << (K_INSERT - 57))))
                            != 0)
                    || (((_la - 89) & !0x3f) == 0
                        && ((1usize << (_la - 89))
                            & ((1usize << (K_INSTEAD - 89))
                                | (1usize << (K_INTERSECT - 89))
                                | (1usize << (K_INTO - 89))
                                | (1usize << (K_IS - 89))
                                | (1usize << (K_ISNULL - 89))
                                | (1usize << (K_JOIN - 89))
                                | (1usize << (K_KEY - 89))
                                | (1usize << (K_LEFT - 89))
                                | (1usize << (K_LIKE - 89))
                                | (1usize << (K_LIMIT - 89))
                                | (1usize << (K_MATCH - 89))
                                | (1usize << (K_NATURAL - 89))
                                | (1usize << (K_NO - 89))
                                | (1usize << (K_NOT - 89))
                                | (1usize << (K_NOTNULL - 89))
                                | (1usize << (K_NULL - 89))
                                | (1usize << (K_OF - 89))
                                | (1usize << (K_OFFSET - 89))
                                | (1usize << (K_ON - 89))
                                | (1usize << (K_OR - 89))
                                | (1usize << (K_ORDER - 89))
                                | (1usize << (K_OUTER - 89))
                                | (1usize << (K_PLAN - 89))
                                | (1usize << (K_PRAGMA - 89))
                                | (1usize << (K_PRIMARY - 89))
                                | (1usize << (K_QUERY - 89))
                                | (1usize << (K_RAISE - 89))
                                | (1usize << (K_RECURSIVE - 89))
                                | (1usize << (K_REFERENCES - 89))
                                | (1usize << (K_REGEXP - 89))
                                | (1usize << (K_REINDEX - 89))
                                | (1usize << (K_RELEASE - 89))))
                            != 0)
                    || (((_la - 121) & !0x3f) == 0
                        && ((1usize << (_la - 121))
                            & ((1usize << (K_RENAME - 121))
                                | (1usize << (K_REPLACE - 121))
                                | (1usize << (K_RESTRICT - 121))
                                | (1usize << (K_RIGHT - 121))
                                | (1usize << (K_ROLLBACK - 121))
                                | (1usize << (K_ROW - 121))
                                | (1usize << (K_SAVEPOINT - 121))
                                | (1usize << (K_SELECT - 121))
                                | (1usize << (K_SET - 121))
                                | (1usize << (K_TABLE - 121))
                                | (1usize << (K_TEMP - 121))
                                | (1usize << (K_TEMPORARY - 121))
                                | (1usize << (K_THEN - 121))
                                | (1usize << (K_TO - 121))
                                | (1usize << (K_TRANSACTION - 121))
                                | (1usize << (K_TRIGGER - 121))
                                | (1usize << (K_UNION - 121))
                                | (1usize << (K_UNIQUE - 121))
                                | (1usize << (K_UPDATE - 121))
                                | (1usize << (K_USING - 121))
                                | (1usize << (K_VACUUM - 121))
                                | (1usize << (K_VALUES - 121))
                                | (1usize << (K_VIEW - 121))
                                | (1usize << (K_VIRTUAL - 121))
                                | (1usize << (K_WHEN - 121))
                                | (1usize << (K_WHERE - 121))
                                | (1usize << (K_WITH - 121))
                                | (1usize << (K_WITHOUT - 121))))
                            != 0))
                {
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
//------------------- name ----------------
pub type NameContextAll<'input> = NameContext<'input>;

pub type NameContext<'input> = BaseParserRuleContext<'input, NameContextExt<'input>>;

#[derive(Clone)]
pub struct NameContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for NameContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for NameContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_name(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_name(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for NameContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_name
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_name }
}
antlr_rust::tid! {NameContextExt<'a>}

impl<'input> NameContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<NameContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            NameContextExt { ph: PhantomData },
        ))
    }
}

pub trait NameContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<NameContextExt<'input>>
{
    fn any_name(&self) -> Option<Rc<Any_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> NameContextAttrs<'input> for NameContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn name(&mut self) -> Result<Rc<NameContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = NameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 126, RULE_name);
        let mut _localctx: Rc<NameContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule any_name*/
                recog.base.set_state(1629);
                recog.any_name()?;
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
//------------------- function_name ----------------
pub type Function_nameContextAll<'input> = Function_nameContext<'input>;

pub type Function_nameContext<'input> =
    BaseParserRuleContext<'input, Function_nameContextExt<'input>>;

#[derive(Clone)]
pub struct Function_nameContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Function_nameContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Function_nameContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_function_name(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_function_name(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Function_nameContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_function_name
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_function_name }
}
antlr_rust::tid! {Function_nameContextExt<'a>}

impl<'input> Function_nameContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Function_nameContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Function_nameContextExt { ph: PhantomData },
        ))
    }
}

pub trait Function_nameContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Function_nameContextExt<'input>>
{
    fn any_name(&self) -> Option<Rc<Any_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Function_nameContextAttrs<'input> for Function_nameContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn function_name(&mut self) -> Result<Rc<Function_nameContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Function_nameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 128, RULE_function_name);
        let mut _localctx: Rc<Function_nameContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule any_name*/
                recog.base.set_state(1631);
                recog.any_name()?;
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
//------------------- database_name ----------------
pub type Database_nameContextAll<'input> = Database_nameContext<'input>;

pub type Database_nameContext<'input> =
    BaseParserRuleContext<'input, Database_nameContextExt<'input>>;

#[derive(Clone)]
pub struct Database_nameContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Database_nameContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Database_nameContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_database_name(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_database_name(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Database_nameContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_database_name
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_database_name }
}
antlr_rust::tid! {Database_nameContextExt<'a>}

impl<'input> Database_nameContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Database_nameContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Database_nameContextExt { ph: PhantomData },
        ))
    }
}

pub trait Database_nameContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Database_nameContextExt<'input>>
{
    fn any_name(&self) -> Option<Rc<Any_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Database_nameContextAttrs<'input> for Database_nameContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn database_name(&mut self) -> Result<Rc<Database_nameContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Database_nameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 130, RULE_database_name);
        let mut _localctx: Rc<Database_nameContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule any_name*/
                recog.base.set_state(1633);
                recog.any_name()?;
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
//------------------- schema_name ----------------
pub type Schema_nameContextAll<'input> = Schema_nameContext<'input>;

pub type Schema_nameContext<'input> = BaseParserRuleContext<'input, Schema_nameContextExt<'input>>;

#[derive(Clone)]
pub struct Schema_nameContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Schema_nameContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Schema_nameContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_schema_name(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_schema_name(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Schema_nameContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_schema_name
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_schema_name }
}
antlr_rust::tid! {Schema_nameContextExt<'a>}

impl<'input> Schema_nameContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Schema_nameContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Schema_nameContextExt { ph: PhantomData },
        ))
    }
}

pub trait Schema_nameContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Schema_nameContextExt<'input>>
{
    fn any_name(&self) -> Option<Rc<Any_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Schema_nameContextAttrs<'input> for Schema_nameContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn schema_name(&mut self) -> Result<Rc<Schema_nameContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Schema_nameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 132, RULE_schema_name);
        let mut _localctx: Rc<Schema_nameContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule any_name*/
                recog.base.set_state(1635);
                recog.any_name()?;
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
//------------------- table_function_name ----------------
pub type Table_function_nameContextAll<'input> = Table_function_nameContext<'input>;

pub type Table_function_nameContext<'input> =
    BaseParserRuleContext<'input, Table_function_nameContextExt<'input>>;

#[derive(Clone)]
pub struct Table_function_nameContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Table_function_nameContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a>
    for Table_function_nameContext<'input>
{
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_table_function_name(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_table_function_name(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Table_function_nameContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_table_function_name
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_table_function_name }
}
antlr_rust::tid! {Table_function_nameContextExt<'a>}

impl<'input> Table_function_nameContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Table_function_nameContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Table_function_nameContextExt { ph: PhantomData },
        ))
    }
}

pub trait Table_function_nameContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Table_function_nameContextExt<'input>>
{
    fn any_name(&self) -> Option<Rc<Any_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Table_function_nameContextAttrs<'input> for Table_function_nameContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn table_function_name(
        &mut self,
    ) -> Result<Rc<Table_function_nameContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Table_function_nameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 134, RULE_table_function_name);
        let mut _localctx: Rc<Table_function_nameContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule any_name*/
                recog.base.set_state(1637);
                recog.any_name()?;
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
//------------------- table_name ----------------
pub type Table_nameContextAll<'input> = Table_nameContext<'input>;

pub type Table_nameContext<'input> = BaseParserRuleContext<'input, Table_nameContextExt<'input>>;

#[derive(Clone)]
pub struct Table_nameContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Table_nameContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Table_nameContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_table_name(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_table_name(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Table_nameContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_table_name
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_table_name }
}
antlr_rust::tid! {Table_nameContextExt<'a>}

impl<'input> Table_nameContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Table_nameContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Table_nameContextExt { ph: PhantomData },
        ))
    }
}

pub trait Table_nameContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Table_nameContextExt<'input>>
{
    fn any_name(&self) -> Option<Rc<Any_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Table_nameContextAttrs<'input> for Table_nameContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn table_name(&mut self) -> Result<Rc<Table_nameContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Table_nameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 136, RULE_table_name);
        let mut _localctx: Rc<Table_nameContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule any_name*/
                recog.base.set_state(1639);
                recog.any_name()?;
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
//------------------- table_or_index_name ----------------
pub type Table_or_index_nameContextAll<'input> = Table_or_index_nameContext<'input>;

pub type Table_or_index_nameContext<'input> =
    BaseParserRuleContext<'input, Table_or_index_nameContextExt<'input>>;

#[derive(Clone)]
pub struct Table_or_index_nameContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Table_or_index_nameContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a>
    for Table_or_index_nameContext<'input>
{
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_table_or_index_name(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_table_or_index_name(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Table_or_index_nameContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_table_or_index_name
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_table_or_index_name }
}
antlr_rust::tid! {Table_or_index_nameContextExt<'a>}

impl<'input> Table_or_index_nameContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Table_or_index_nameContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Table_or_index_nameContextExt { ph: PhantomData },
        ))
    }
}

pub trait Table_or_index_nameContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Table_or_index_nameContextExt<'input>>
{
    fn any_name(&self) -> Option<Rc<Any_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Table_or_index_nameContextAttrs<'input> for Table_or_index_nameContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn table_or_index_name(
        &mut self,
    ) -> Result<Rc<Table_or_index_nameContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Table_or_index_nameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 138, RULE_table_or_index_name);
        let mut _localctx: Rc<Table_or_index_nameContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule any_name*/
                recog.base.set_state(1641);
                recog.any_name()?;
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
//------------------- new_table_name ----------------
pub type New_table_nameContextAll<'input> = New_table_nameContext<'input>;

pub type New_table_nameContext<'input> =
    BaseParserRuleContext<'input, New_table_nameContextExt<'input>>;

#[derive(Clone)]
pub struct New_table_nameContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for New_table_nameContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for New_table_nameContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_new_table_name(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_new_table_name(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for New_table_nameContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_new_table_name
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_new_table_name }
}
antlr_rust::tid! {New_table_nameContextExt<'a>}

impl<'input> New_table_nameContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<New_table_nameContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            New_table_nameContextExt { ph: PhantomData },
        ))
    }
}

pub trait New_table_nameContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<New_table_nameContextExt<'input>>
{
    fn any_name(&self) -> Option<Rc<Any_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> New_table_nameContextAttrs<'input> for New_table_nameContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn new_table_name(&mut self) -> Result<Rc<New_table_nameContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            New_table_nameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 140, RULE_new_table_name);
        let mut _localctx: Rc<New_table_nameContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule any_name*/
                recog.base.set_state(1643);
                recog.any_name()?;
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
//------------------- column_name ----------------
pub type Column_nameContextAll<'input> = Column_nameContext<'input>;

pub type Column_nameContext<'input> = BaseParserRuleContext<'input, Column_nameContextExt<'input>>;

#[derive(Clone)]
pub struct Column_nameContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Column_nameContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Column_nameContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_column_name(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_column_name(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Column_nameContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_column_name
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_column_name }
}
antlr_rust::tid! {Column_nameContextExt<'a>}

impl<'input> Column_nameContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Column_nameContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Column_nameContextExt { ph: PhantomData },
        ))
    }
}

pub trait Column_nameContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Column_nameContextExt<'input>>
{
    fn any_name(&self) -> Option<Rc<Any_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Column_nameContextAttrs<'input> for Column_nameContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn column_name(&mut self) -> Result<Rc<Column_nameContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Column_nameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 142, RULE_column_name);
        let mut _localctx: Rc<Column_nameContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule any_name*/
                recog.base.set_state(1645);
                recog.any_name()?;
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
//------------------- collation_name ----------------
pub type Collation_nameContextAll<'input> = Collation_nameContext<'input>;

pub type Collation_nameContext<'input> =
    BaseParserRuleContext<'input, Collation_nameContextExt<'input>>;

#[derive(Clone)]
pub struct Collation_nameContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Collation_nameContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Collation_nameContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_collation_name(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_collation_name(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Collation_nameContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_collation_name
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_collation_name }
}
antlr_rust::tid! {Collation_nameContextExt<'a>}

impl<'input> Collation_nameContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Collation_nameContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Collation_nameContextExt { ph: PhantomData },
        ))
    }
}

pub trait Collation_nameContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Collation_nameContextExt<'input>>
{
    fn any_name(&self) -> Option<Rc<Any_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Collation_nameContextAttrs<'input> for Collation_nameContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn collation_name(&mut self) -> Result<Rc<Collation_nameContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Collation_nameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 144, RULE_collation_name);
        let mut _localctx: Rc<Collation_nameContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule any_name*/
                recog.base.set_state(1647);
                recog.any_name()?;
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
//------------------- foreign_table ----------------
pub type Foreign_tableContextAll<'input> = Foreign_tableContext<'input>;

pub type Foreign_tableContext<'input> =
    BaseParserRuleContext<'input, Foreign_tableContextExt<'input>>;

#[derive(Clone)]
pub struct Foreign_tableContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Foreign_tableContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Foreign_tableContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_foreign_table(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_foreign_table(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Foreign_tableContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_foreign_table
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_foreign_table }
}
antlr_rust::tid! {Foreign_tableContextExt<'a>}

impl<'input> Foreign_tableContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Foreign_tableContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Foreign_tableContextExt { ph: PhantomData },
        ))
    }
}

pub trait Foreign_tableContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Foreign_tableContextExt<'input>>
{
    fn any_name(&self) -> Option<Rc<Any_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Foreign_tableContextAttrs<'input> for Foreign_tableContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn foreign_table(&mut self) -> Result<Rc<Foreign_tableContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Foreign_tableContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 146, RULE_foreign_table);
        let mut _localctx: Rc<Foreign_tableContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule any_name*/
                recog.base.set_state(1649);
                recog.any_name()?;
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
//------------------- index_name ----------------
pub type Index_nameContextAll<'input> = Index_nameContext<'input>;

pub type Index_nameContext<'input> = BaseParserRuleContext<'input, Index_nameContextExt<'input>>;

#[derive(Clone)]
pub struct Index_nameContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Index_nameContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Index_nameContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_index_name(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_index_name(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Index_nameContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_index_name
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_index_name }
}
antlr_rust::tid! {Index_nameContextExt<'a>}

impl<'input> Index_nameContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Index_nameContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Index_nameContextExt { ph: PhantomData },
        ))
    }
}

pub trait Index_nameContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Index_nameContextExt<'input>>
{
    fn any_name(&self) -> Option<Rc<Any_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Index_nameContextAttrs<'input> for Index_nameContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn index_name(&mut self) -> Result<Rc<Index_nameContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Index_nameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 148, RULE_index_name);
        let mut _localctx: Rc<Index_nameContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule any_name*/
                recog.base.set_state(1651);
                recog.any_name()?;
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
//------------------- trigger_name ----------------
pub type Trigger_nameContextAll<'input> = Trigger_nameContext<'input>;

pub type Trigger_nameContext<'input> =
    BaseParserRuleContext<'input, Trigger_nameContextExt<'input>>;

#[derive(Clone)]
pub struct Trigger_nameContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Trigger_nameContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Trigger_nameContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_trigger_name(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_trigger_name(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Trigger_nameContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_trigger_name
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_trigger_name }
}
antlr_rust::tid! {Trigger_nameContextExt<'a>}

impl<'input> Trigger_nameContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Trigger_nameContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Trigger_nameContextExt { ph: PhantomData },
        ))
    }
}

pub trait Trigger_nameContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Trigger_nameContextExt<'input>>
{
    fn any_name(&self) -> Option<Rc<Any_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Trigger_nameContextAttrs<'input> for Trigger_nameContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    #![allow(unknown_lints)]
    #![allow(clippy::all)]
    pub fn trigger_name(&mut self) -> Result<Rc<Trigger_nameContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Trigger_nameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 150, RULE_trigger_name);
        let mut _localctx: Rc<Trigger_nameContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule any_name*/
                recog.base.set_state(1653);
                recog.any_name()?;
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
//------------------- view_name ----------------
pub type View_nameContextAll<'input> = View_nameContext<'input>;

pub type View_nameContext<'input> = BaseParserRuleContext<'input, View_nameContextExt<'input>>;

#[derive(Clone)]
pub struct View_nameContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for View_nameContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for View_nameContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_view_name(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_view_name(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for View_nameContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_view_name
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_view_name }
}
antlr_rust::tid! {View_nameContextExt<'a>}

impl<'input> View_nameContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<View_nameContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            View_nameContextExt { ph: PhantomData },
        ))
    }
}

pub trait View_nameContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<View_nameContextExt<'input>>
{
    fn any_name(&self) -> Option<Rc<Any_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> View_nameContextAttrs<'input> for View_nameContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    #![allow(unknown_lints)]
    #![allow(clippy::all)]
    pub fn view_name(&mut self) -> Result<Rc<View_nameContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = View_nameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 152, RULE_view_name);
        let mut _localctx: Rc<View_nameContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule any_name*/
                recog.base.set_state(1655);
                recog.any_name()?;
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
//------------------- module_name ----------------
pub type Module_nameContextAll<'input> = Module_nameContext<'input>;

pub type Module_nameContext<'input> = BaseParserRuleContext<'input, Module_nameContextExt<'input>>;

#[derive(Clone)]
pub struct Module_nameContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Module_nameContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Module_nameContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_module_name(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_module_name(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Module_nameContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_module_name
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_module_name }
}
antlr_rust::tid! {Module_nameContextExt<'a>}

impl<'input> Module_nameContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Module_nameContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Module_nameContextExt { ph: PhantomData },
        ))
    }
}

pub trait Module_nameContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Module_nameContextExt<'input>>
{
    fn any_name(&self) -> Option<Rc<Any_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Module_nameContextAttrs<'input> for Module_nameContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    #![allow(unknown_lints)]
    #![allow(clippy::all)]
    pub fn module_name(&mut self) -> Result<Rc<Module_nameContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Module_nameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 154, RULE_module_name);
        let mut _localctx: Rc<Module_nameContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule any_name*/
                recog.base.set_state(1657);
                recog.any_name()?;
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
//------------------- pragma_name ----------------
pub type Pragma_nameContextAll<'input> = Pragma_nameContext<'input>;

pub type Pragma_nameContext<'input> = BaseParserRuleContext<'input, Pragma_nameContextExt<'input>>;

#[derive(Clone)]
pub struct Pragma_nameContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Pragma_nameContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Pragma_nameContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_pragma_name(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_pragma_name(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Pragma_nameContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_pragma_name
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_pragma_name }
}
antlr_rust::tid! {Pragma_nameContextExt<'a>}

impl<'input> Pragma_nameContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Pragma_nameContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Pragma_nameContextExt { ph: PhantomData },
        ))
    }
}

pub trait Pragma_nameContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Pragma_nameContextExt<'input>>
{
    fn any_name(&self) -> Option<Rc<Any_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Pragma_nameContextAttrs<'input> for Pragma_nameContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn pragma_name(&mut self) -> Result<Rc<Pragma_nameContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Pragma_nameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 156, RULE_pragma_name);
        let mut _localctx: Rc<Pragma_nameContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule any_name*/
                recog.base.set_state(1659);
                recog.any_name()?;
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
//------------------- savepoint_name ----------------
pub type Savepoint_nameContextAll<'input> = Savepoint_nameContext<'input>;

pub type Savepoint_nameContext<'input> =
    BaseParserRuleContext<'input, Savepoint_nameContextExt<'input>>;

#[derive(Clone)]
pub struct Savepoint_nameContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Savepoint_nameContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Savepoint_nameContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_savepoint_name(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_savepoint_name(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Savepoint_nameContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_savepoint_name
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_savepoint_name }
}
antlr_rust::tid! {Savepoint_nameContextExt<'a>}

impl<'input> Savepoint_nameContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Savepoint_nameContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Savepoint_nameContextExt { ph: PhantomData },
        ))
    }
}

pub trait Savepoint_nameContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Savepoint_nameContextExt<'input>>
{
    fn any_name(&self) -> Option<Rc<Any_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Savepoint_nameContextAttrs<'input> for Savepoint_nameContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn savepoint_name(&mut self) -> Result<Rc<Savepoint_nameContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Savepoint_nameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 158, RULE_savepoint_name);
        let mut _localctx: Rc<Savepoint_nameContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule any_name*/
                recog.base.set_state(1661);
                recog.any_name()?;
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
//------------------- table_alias ----------------
pub type Table_aliasContextAll<'input> = Table_aliasContext<'input>;

pub type Table_aliasContext<'input> = BaseParserRuleContext<'input, Table_aliasContextExt<'input>>;

#[derive(Clone)]
pub struct Table_aliasContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Table_aliasContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Table_aliasContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_table_alias(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_table_alias(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Table_aliasContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_table_alias
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_table_alias }
}
antlr_rust::tid! {Table_aliasContextExt<'a>}

impl<'input> Table_aliasContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Table_aliasContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Table_aliasContextExt { ph: PhantomData },
        ))
    }
}

pub trait Table_aliasContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Table_aliasContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token IDENTIFIER
    /// Returns `None` if there is no child corresponding to token IDENTIFIER
    fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(IDENTIFIER, 0)
    }
    /// Retrieves first TerminalNode corresponding to token STRING_LITERAL
    /// Returns `None` if there is no child corresponding to token STRING_LITERAL
    fn STRING_LITERAL(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(STRING_LITERAL, 0)
    }
    /// Retrieves first TerminalNode corresponding to token OPEN_PAR
    /// Returns `None` if there is no child corresponding to token OPEN_PAR
    fn OPEN_PAR(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPEN_PAR, 0)
    }
    fn table_alias(&self) -> Option<Rc<Table_aliasContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token CLOSE_PAR
    /// Returns `None` if there is no child corresponding to token CLOSE_PAR
    fn CLOSE_PAR(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CLOSE_PAR, 0)
    }
}

impl<'input> Table_aliasContextAttrs<'input> for Table_aliasContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn table_alias(&mut self) -> Result<Rc<Table_aliasContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Table_aliasContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 160, RULE_table_alias);
        let mut _localctx: Rc<Table_aliasContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(1669);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                IDENTIFIER => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        recog.base.set_state(1663);
                        recog.base.match_token(IDENTIFIER, &mut recog.err_handler)?;
                    }
                }

                STRING_LITERAL => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        recog.base.set_state(1664);
                        recog
                            .base
                            .match_token(STRING_LITERAL, &mut recog.err_handler)?;
                    }
                }

                OPEN_PAR => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        recog.base.set_state(1665);
                        recog.base.match_token(OPEN_PAR, &mut recog.err_handler)?;

                        /*InvokeRule table_alias*/
                        recog.base.set_state(1666);
                        recog.table_alias()?;

                        recog.base.set_state(1667);
                        recog.base.match_token(CLOSE_PAR, &mut recog.err_handler)?;
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
//------------------- transaction_name ----------------
pub type Transaction_nameContextAll<'input> = Transaction_nameContext<'input>;

pub type Transaction_nameContext<'input> =
    BaseParserRuleContext<'input, Transaction_nameContextExt<'input>>;

#[derive(Clone)]
pub struct Transaction_nameContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Transaction_nameContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Transaction_nameContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_transaction_name(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_transaction_name(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Transaction_nameContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_transaction_name
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_transaction_name }
}
antlr_rust::tid! {Transaction_nameContextExt<'a>}

impl<'input> Transaction_nameContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Transaction_nameContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Transaction_nameContextExt { ph: PhantomData },
        ))
    }
}

pub trait Transaction_nameContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Transaction_nameContextExt<'input>>
{
    fn any_name(&self) -> Option<Rc<Any_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Transaction_nameContextAttrs<'input> for Transaction_nameContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn transaction_name(
        &mut self,
    ) -> Result<Rc<Transaction_nameContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Transaction_nameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 162, RULE_transaction_name);
        let mut _localctx: Rc<Transaction_nameContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule any_name*/
                recog.base.set_state(1671);
                recog.any_name()?;
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
//------------------- any_name ----------------
pub type Any_nameContextAll<'input> = Any_nameContext<'input>;

pub type Any_nameContext<'input> = BaseParserRuleContext<'input, Any_nameContextExt<'input>>;

#[derive(Clone)]
pub struct Any_nameContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> SQLiteParserContext<'input> for Any_nameContext<'input> {}

impl<'input, 'a> Listenable<dyn SQLiteListener<'input> + 'a> for Any_nameContext<'input> {
    fn enter(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_any_name(self);
    }
    fn exit(&self, listener: &mut (dyn SQLiteListener<'input> + 'a)) {
        listener.exit_any_name(self);
        listener.exit_every_rule(self);
    }
}

impl<'input> CustomRuleContext<'input> for Any_nameContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = SQLiteParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_any_name
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_any_name }
}
antlr_rust::tid! {Any_nameContextExt<'a>}

impl<'input> Any_nameContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn SQLiteParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Any_nameContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Any_nameContextExt { ph: PhantomData },
        ))
    }
}

pub trait Any_nameContextAttrs<'input>:
    SQLiteParserContext<'input> + BorrowMut<Any_nameContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token IDENTIFIER
    /// Returns `None` if there is no child corresponding to token IDENTIFIER
    fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(IDENTIFIER, 0)
    }
    fn keyword(&self) -> Option<Rc<KeywordContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token STRING_LITERAL
    /// Returns `None` if there is no child corresponding to token STRING_LITERAL
    fn STRING_LITERAL(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(STRING_LITERAL, 0)
    }
    /// Retrieves first TerminalNode corresponding to token OPEN_PAR
    /// Returns `None` if there is no child corresponding to token OPEN_PAR
    fn OPEN_PAR(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPEN_PAR, 0)
    }
    fn any_name(&self) -> Option<Rc<Any_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token CLOSE_PAR
    /// Returns `None` if there is no child corresponding to token CLOSE_PAR
    fn CLOSE_PAR(&self) -> Option<Rc<TerminalNode<'input, SQLiteParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CLOSE_PAR, 0)
    }
}

impl<'input> Any_nameContextAttrs<'input> for Any_nameContext<'input> {}

impl<'input, I, H> SQLiteParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn any_name(&mut self) -> Result<Rc<Any_nameContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Any_nameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 164, RULE_any_name);
        let mut _localctx: Rc<Any_nameContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(1680);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                IDENTIFIER => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        recog.base.set_state(1673);
                        recog.base.match_token(IDENTIFIER, &mut recog.err_handler)?;
                    }
                }

                K_ABORT | K_ACTION | K_ADD | K_AFTER | K_ALL | K_ALTER | K_ANALYZE | K_AND
                | K_AS | K_ASC | K_ATTACH | K_AUTOINCREMENT | K_BEFORE | K_BEGIN | K_BETWEEN
                | K_BY | K_CASCADE | K_CASE | K_CAST | K_CHECK | K_COLLATE | K_COLUMN
                | K_COMMIT | K_CONFLICT | K_CONSTRAINT | K_CREATE | K_CROSS | K_CURRENT_DATE
                | K_CURRENT_TIME | K_CURRENT_TIMESTAMP | K_DATABASE | K_DEFAULT | K_DEFERRABLE
                | K_DEFERRED | K_DELETE | K_DESC | K_DETACH | K_DISTINCT | K_DROP | K_EACH
                | K_ELSE | K_END | K_ESCAPE | K_EXCEPT | K_EXCLUSIVE | K_EXISTS | K_EXPLAIN
                | K_FAIL | K_FOR | K_FOREIGN | K_FROM | K_FULL | K_GLOB | K_GROUP | K_HAVING
                | K_IF | K_IGNORE | K_IMMEDIATE | K_IN | K_INDEX | K_INDEXED | K_INITIALLY
                | K_INNER | K_INSERT | K_INSTEAD | K_INTERSECT | K_INTO | K_IS | K_ISNULL
                | K_JOIN | K_KEY | K_LEFT | K_LIKE | K_LIMIT | K_MATCH | K_NATURAL | K_NO
                | K_NOT | K_NOTNULL | K_NULL | K_OF | K_OFFSET | K_ON | K_OR | K_ORDER
                | K_OUTER | K_PLAN | K_PRAGMA | K_PRIMARY | K_QUERY | K_RAISE | K_RECURSIVE
                | K_REFERENCES | K_REGEXP | K_REINDEX | K_RELEASE | K_RENAME | K_REPLACE
                | K_RESTRICT | K_RIGHT | K_ROLLBACK | K_ROW | K_SAVEPOINT | K_SELECT | K_SET
                | K_TABLE | K_TEMP | K_TEMPORARY | K_THEN | K_TO | K_TRANSACTION | K_TRIGGER
                | K_UNION | K_UNIQUE | K_UPDATE | K_USING | K_VACUUM | K_VALUES | K_VIEW
                | K_VIRTUAL | K_WHEN | K_WHERE | K_WITH | K_WITHOUT => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule keyword*/
                        recog.base.set_state(1674);
                        recog.keyword()?;
                    }
                }

                STRING_LITERAL => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        recog.base.set_state(1675);
                        recog
                            .base
                            .match_token(STRING_LITERAL, &mut recog.err_handler)?;
                    }
                }

                OPEN_PAR => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 4);
                    recog.base.enter_outer_alt(None, 4);
                    {
                        recog.base.set_state(1676);
                        recog.base.match_token(OPEN_PAR, &mut recog.err_handler)?;

                        /*InvokeRule any_name*/
                        recog.base.set_state(1677);
                        recog.any_name()?;

                        recog.base.set_state(1678);
                        recog.base.match_token(CLOSE_PAR, &mut recog.err_handler)?;
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

const _serializedATN: &str =
    "\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\u{9f}\u{695}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\
	\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\
	\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\
	\x0e\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x04\
	\x13\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x04\x16\x09\x16\x04\x17\x09\
	\x17\x04\x18\x09\x18\x04\x19\x09\x19\x04\x1a\x09\x1a\x04\x1b\x09\x1b\x04\
	\x1c\x09\x1c\x04\x1d\x09\x1d\x04\x1e\x09\x1e\x04\x1f\x09\x1f\x04\x20\x09\
	\x20\x04\x21\x09\x21\x04\x22\x09\x22\x04\x23\x09\x23\x04\x24\x09\x24\x04\
	\x25\x09\x25\x04\x26\x09\x26\x04\x27\x09\x27\x04\x28\x09\x28\x04\x29\x09\
	\x29\x04\x2a\x09\x2a\x04\x2b\x09\x2b\x04\x2c\x09\x2c\x04\x2d\x09\x2d\x04\
	\x2e\x09\x2e\x04\x2f\x09\x2f\x04\x30\x09\x30\x04\x31\x09\x31\x04\x32\x09\
	\x32\x04\x33\x09\x33\x04\x34\x09\x34\x04\x35\x09\x35\x04\x36\x09\x36\x04\
	\x37\x09\x37\x04\x38\x09\x38\x04\x39\x09\x39\x04\x3a\x09\x3a\x04\x3b\x09\
	\x3b\x04\x3c\x09\x3c\x04\x3d\x09\x3d\x04\x3e\x09\x3e\x04\x3f\x09\x3f\x04\
	\x40\x09\x40\x04\x41\x09\x41\x04\x42\x09\x42\x04\x43\x09\x43\x04\x44\x09\
	\x44\x04\x45\x09\x45\x04\x46\x09\x46\x04\x47\x09\x47\x04\x48\x09\x48\x04\
	\x49\x09\x49\x04\x4a\x09\x4a\x04\x4b\x09\x4b\x04\x4c\x09\x4c\x04\x4d\x09\
	\x4d\x04\x4e\x09\x4e\x04\x4f\x09\x4f\x04\x50\x09\x50\x04\x51\x09\x51\x04\
	\x52\x09\x52\x04\x53\x09\x53\x04\x54\x09\x54\x03\x02\x03\x02\x07\x02\u{ab}\
	\x0a\x02\x0c\x02\x0e\x02\u{ae}\x0b\x02\x03\x02\x03\x02\x03\x03\x03\x03\x03\
	\x03\x03\x04\x07\x04\u{b6}\x0a\x04\x0c\x04\x0e\x04\u{b9}\x0b\x04\x03\x04\
	\x03\x04\x06\x04\u{bd}\x0a\x04\x0d\x04\x0e\x04\u{be}\x03\x04\x07\x04\u{c2}\
	\x0a\x04\x0c\x04\x0e\x04\u{c5}\x0b\x04\x03\x04\x07\x04\u{c8}\x0a\x04\x0c\
	\x04\x0e\x04\u{cb}\x0b\x04\x03\x05\x03\x05\x03\x05\x05\x05\u{d0}\x0a\x05\
	\x05\x05\u{d2}\x0a\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\
	\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\
	\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\
	\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x05\x05\u{f2}\x0a\x05\x03\x06\
	\x03\x06\x03\x06\x03\x06\x03\x06\x05\x06\u{f9}\x0a\x06\x03\x06\x03\x06\x03\
	\x06\x03\x06\x03\x06\x03\x06\x05\x06\u{101}\x0a\x06\x03\x06\x05\x06\u{104}\
	\x0a\x06\x03\x07\x03\x07\x03\x07\x03\x07\x03\x07\x03\x07\x03\x07\x05\x07\
	\u{10d}\x0a\x07\x03\x08\x03\x08\x05\x08\u{111}\x0a\x08\x03\x08\x03\x08\x03\
	\x08\x03\x08\x03\x09\x03\x09\x05\x09\u{119}\x0a\x09\x03\x09\x03\x09\x05\
	\x09\u{11d}\x0a\x09\x05\x09\u{11f}\x0a\x09\x03\x0a\x03\x0a\x03\x0a\x05\x0a\
	\u{124}\x0a\x0a\x05\x0a\u{126}\x0a\x0a\x03\x0b\x05\x0b\u{129}\x0a\x0b\x03\
	\x0b\x03\x0b\x03\x0b\x05\x0b\u{12e}\x0a\x0b\x03\x0b\x03\x0b\x05\x0b\u{132}\
	\x0a\x0b\x03\x0b\x06\x0b\u{135}\x0a\x0b\x0d\x0b\x0e\x0b\u{136}\x03\x0b\x03\
	\x0b\x03\x0b\x03\x0b\x03\x0b\x07\x0b\u{13e}\x0a\x0b\x0c\x0b\x0e\x0b\u{141}\
	\x0b\x0b\x05\x0b\u{143}\x0a\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x05\x0b\
	\u{149}\x0a\x0b\x05\x0b\u{14b}\x0a\x0b\x03\x0c\x03\x0c\x05\x0c\u{14f}\x0a\
	\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x05\x0c\u{155}\x0a\x0c\x03\x0c\x03\
	\x0c\x03\x0c\x05\x0c\u{15a}\x0a\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\
	\x0c\x03\x0c\x03\x0c\x07\x0c\u{163}\x0a\x0c\x0c\x0c\x0e\x0c\u{166}\x0b\x0c\
	\x03\x0c\x03\x0c\x03\x0c\x05\x0c\u{16b}\x0a\x0c\x03\x0d\x03\x0d\x05\x0d\
	\u{16f}\x0a\x0d\x03\x0d\x03\x0d\x03\x0d\x03\x0d\x05\x0d\u{175}\x0a\x0d\x03\
	\x0d\x03\x0d\x03\x0d\x05\x0d\u{17a}\x0a\x0d\x03\x0d\x03\x0d\x03\x0d\x03\
	\x0d\x03\x0d\x07\x0d\u{181}\x0a\x0d\x0c\x0d\x0e\x0d\u{184}\x0b\x0d\x03\x0d\
	\x03\x0d\x07\x0d\u{188}\x0a\x0d\x0c\x0d\x0e\x0d\u{18b}\x0b\x0d\x03\x0d\x03\
	\x0d\x03\x0d\x05\x0d\u{190}\x0a\x0d\x03\x0d\x03\x0d\x05\x0d\u{194}\x0a\x0d\
	\x03\x0e\x03\x0e\x05\x0e\u{198}\x0a\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\
	\x05\x0e\u{19e}\x0a\x0e\x03\x0e\x03\x0e\x03\x0e\x05\x0e\u{1a3}\x0a\x0e\x03\
	\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x05\x0e\u{1aa}\x0a\x0e\x03\x0e\x03\
	\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x07\x0e\u{1b3}\x0a\x0e\x0c\
	\x0e\x0e\x0e\u{1b6}\x0b\x0e\x05\x0e\u{1b8}\x0a\x0e\x05\x0e\u{1ba}\x0a\x0e\
	\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x05\x0e\u{1c0}\x0a\x0e\x03\x0e\x03\x0e\
	\x03\x0e\x03\x0e\x05\x0e\u{1c6}\x0a\x0e\x03\x0e\x03\x0e\x05\x0e\u{1ca}\x0a\
	\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x05\x0e\u{1d1}\x0a\x0e\x03\
	\x0e\x03\x0e\x06\x0e\u{1d5}\x0a\x0e\x0d\x0e\x0e\x0e\u{1d6}\x03\x0e\x03\x0e\
	\x03\x0f\x03\x0f\x05\x0f\u{1dd}\x0a\x0f\x03\x0f\x03\x0f\x03\x0f\x03\x0f\
	\x05\x0f\u{1e3}\x0a\x0f\x03\x0f\x03\x0f\x03\x0f\x05\x0f\u{1e8}\x0a\x0f\x03\
	\x0f\x03\x0f\x03\x0f\x03\x0f\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\x03\
	\x10\x05\x10\u{1f4}\x0a\x10\x03\x10\x03\x10\x03\x10\x05\x10\u{1f9}\x0a\x10\
	\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\x07\x10\u{202}\
	\x0a\x10\x0c\x10\x0e\x10\u{205}\x0b\x10\x03\x10\x03\x10\x05\x10\u{209}\x0a\
	\x10\x03\x11\x05\x11\u{20c}\x0a\x11\x03\x11\x03\x11\x03\x11\x03\x11\x03\
	\x11\x05\x11\u{213}\x0a\x11\x03\x12\x05\x12\u{216}\x0a\x12\x03\x12\x03\x12\
	\x03\x12\x03\x12\x03\x12\x05\x12\u{21d}\x0a\x12\x03\x12\x03\x12\x03\x12\
	\x03\x12\x03\x12\x07\x12\u{224}\x0a\x12\x0c\x12\x0e\x12\u{227}\x0b\x12\x05\
	\x12\u{229}\x0a\x12\x03\x12\x03\x12\x03\x12\x03\x12\x05\x12\u{22f}\x0a\x12\
	\x05\x12\u{231}\x0a\x12\x03\x13\x03\x13\x05\x13\u{235}\x0a\x13\x03\x13\x03\
	\x13\x03\x14\x03\x14\x03\x14\x03\x14\x05\x14\u{23d}\x0a\x14\x03\x14\x03\
	\x14\x03\x14\x05\x14\u{242}\x0a\x14\x03\x14\x03\x14\x03\x15\x03\x15\x03\
	\x15\x03\x15\x05\x15\u{24a}\x0a\x15\x03\x15\x03\x15\x03\x15\x05\x15\u{24f}\
	\x0a\x15\x03\x15\x03\x15\x03\x16\x03\x16\x03\x16\x03\x16\x05\x16\u{257}\
	\x0a\x16\x03\x16\x03\x16\x03\x16\x05\x16\u{25c}\x0a\x16\x03\x16\x03\x16\
	\x03\x17\x03\x17\x03\x17\x03\x17\x05\x17\u{264}\x0a\x17\x03\x17\x03\x17\
	\x03\x17\x05\x17\u{269}\x0a\x17\x03\x17\x03\x17\x03\x18\x05\x18\u{26e}\x0a\
	\x18\x03\x18\x03\x18\x03\x18\x03\x18\x07\x18\u{274}\x0a\x18\x0c\x18\x0e\
	\x18\u{277}\x0b\x18\x03\x18\x03\x18\x03\x18\x03\x18\x03\x18\x07\x18\u{27e}\
	\x0a\x18\x0c\x18\x0e\x18\u{281}\x0b\x18\x05\x18\u{283}\x0a\x18\x03\x18\x03\
	\x18\x03\x18\x03\x18\x05\x18\u{289}\x0a\x18\x05\x18\u{28b}\x0a\x18\x03\x19\
	\x05\x19\u{28e}\x0a\x19\x03\x19\x03\x19\x03\x19\x03\x19\x03\x19\x03\x19\
	\x03\x19\x03\x19\x03\x19\x03\x19\x03\x19\x03\x19\x03\x19\x03\x19\x03\x19\
	\x03\x19\x03\x19\x05\x19\u{2a1}\x0a\x19\x03\x19\x03\x19\x03\x19\x03\x19\
	\x05\x19\u{2a7}\x0a\x19\x03\x19\x03\x19\x03\x19\x03\x19\x03\x19\x07\x19\
	\u{2ae}\x0a\x19\x0c\x19\x0e\x19\u{2b1}\x0b\x19\x03\x19\x03\x19\x05\x19\u{2b5}\
	\x0a\x19\x03\x19\x03\x19\x03\x19\x03\x19\x03\x19\x07\x19\u{2bc}\x0a\x19\
	\x0c\x19\x0e\x19\u{2bf}\x0b\x19\x03\x19\x03\x19\x03\x19\x03\x19\x03\x19\
	\x03\x19\x07\x19\u{2c7}\x0a\x19\x0c\x19\x0e\x19\u{2ca}\x0b\x19\x03\x19\x03\
	\x19\x07\x19\u{2ce}\x0a\x19\x0c\x19\x0e\x19\u{2d1}\x0b\x19\x03\x19\x03\x19\
	\x03\x19\x05\x19\u{2d6}\x0a\x19\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x05\x1a\
	\u{2dc}\x0a\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1a\
	\x05\x1a\u{2e5}\x0a\x1a\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x05\x1b\
	\u{2ec}\x0a\x1b\x03\x1b\x03\x1b\x05\x1b\u{2f0}\x0a\x1b\x05\x1b\u{2f2}\x0a\
	\x1b\x03\x1c\x03\x1c\x05\x1c\u{2f6}\x0a\x1c\x03\x1c\x03\x1c\x03\x1d\x03\
	\x1d\x03\x1d\x05\x1d\u{2fd}\x0a\x1d\x05\x1d\u{2ff}\x0a\x1d\x03\x1d\x03\x1d\
	\x05\x1d\u{303}\x0a\x1d\x03\x1d\x05\x1d\u{306}\x0a\x1d\x03\x1e\x03\x1e\x03\
	\x1e\x03\x1f\x05\x1f\u{30c}\x0a\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\
	\x1f\x03\x1f\x07\x1f\u{314}\x0a\x1f\x0c\x1f\x0e\x1f\u{317}\x0b\x1f\x05\x1f\
	\u{319}\x0a\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x05\x1f\u{31f}\x0a\x1f\x05\
	\x1f\u{321}\x0a\x1f\x03\x20\x05\x20\u{324}\x0a\x20\x03\x20\x03\x20\x03\x20\
	\x03\x20\x07\x20\u{32a}\x0a\x20\x0c\x20\x0e\x20\u{32d}\x0b\x20\x03\x20\x03\
	\x20\x03\x20\x03\x20\x03\x20\x07\x20\u{334}\x0a\x20\x0c\x20\x0e\x20\u{337}\
	\x0b\x20\x05\x20\u{339}\x0a\x20\x03\x20\x03\x20\x03\x20\x03\x20\x05\x20\
	\u{33f}\x0a\x20\x05\x20\u{341}\x0a\x20\x03\x21\x03\x21\x05\x21\u{345}\x0a\
	\x21\x03\x21\x03\x21\x03\x21\x07\x21\u{34a}\x0a\x21\x0c\x21\x0e\x21\u{34d}\
	\x0b\x21\x03\x21\x03\x21\x03\x21\x03\x21\x07\x21\u{353}\x0a\x21\x0c\x21\
	\x0e\x21\u{356}\x0b\x21\x03\x21\x05\x21\u{359}\x0a\x21\x05\x21\u{35b}\x0a\
	\x21\x03\x21\x03\x21\x05\x21\u{35f}\x0a\x21\x03\x21\x03\x21\x03\x21\x03\
	\x21\x03\x21\x07\x21\u{366}\x0a\x21\x0c\x21\x0e\x21\u{369}\x0b\x21\x03\x21\
	\x03\x21\x05\x21\u{36d}\x0a\x21\x05\x21\u{36f}\x0a\x21\x03\x21\x03\x21\x03\
	\x21\x03\x21\x03\x21\x07\x21\u{376}\x0a\x21\x0c\x21\x0e\x21\u{379}\x0b\x21\
	\x03\x21\x03\x21\x03\x21\x03\x21\x03\x21\x03\x21\x07\x21\u{381}\x0a\x21\
	\x0c\x21\x0e\x21\u{384}\x0b\x21\x03\x21\x03\x21\x07\x21\u{388}\x0a\x21\x0c\
	\x21\x0e\x21\u{38b}\x0b\x21\x05\x21\u{38d}\x0a\x21\x03\x22\x05\x22\u{390}\
	\x0a\x22\x03\x22\x03\x22\x03\x22\x03\x22\x03\x22\x03\x22\x03\x22\x03\x22\
	\x03\x22\x03\x22\x03\x22\x05\x22\u{39d}\x0a\x22\x03\x22\x03\x22\x03\x22\
	\x03\x22\x03\x22\x03\x22\x03\x22\x03\x22\x03\x22\x03\x22\x07\x22\u{3a9}\
	\x0a\x22\x0c\x22\x0e\x22\u{3ac}\x0b\x22\x03\x22\x03\x22\x05\x22\u{3b0}\x0a\
	\x22\x03\x23\x05\x23\u{3b3}\x0a\x23\x03\x23\x03\x23\x03\x23\x03\x23\x03\
	\x23\x03\x23\x03\x23\x03\x23\x03\x23\x03\x23\x03\x23\x05\x23\u{3c0}\x0a\
	\x23\x03\x23\x03\x23\x03\x23\x03\x23\x03\x23\x03\x23\x03\x23\x03\x23\x03\
	\x23\x03\x23\x07\x23\u{3cc}\x0a\x23\x0c\x23\x0e\x23\u{3cf}\x0b\x23\x03\x23\
	\x03\x23\x05\x23\u{3d3}\x0a\x23\x03\x23\x03\x23\x03\x23\x03\x23\x03\x23\
	\x07\x23\u{3da}\x0a\x23\x0c\x23\x0e\x23\u{3dd}\x0b\x23\x05\x23\u{3df}\x0a\
	\x23\x03\x23\x03\x23\x03\x23\x03\x23\x05\x23\u{3e5}\x0a\x23\x05\x23\u{3e7}\
	\x0a\x23\x03\x24\x03\x24\x03\x25\x03\x25\x05\x25\u{3ed}\x0a\x25\x03\x25\
	\x07\x25\u{3f0}\x0a\x25\x0c\x25\x0e\x25\u{3f3}\x0b\x25\x03\x26\x06\x26\u{3f6}\
	\x0a\x26\x0d\x26\x0e\x26\u{3f7}\x03\x26\x03\x26\x03\x26\x03\x26\x03\x26\
	\x03\x26\x03\x26\x03\x26\x03\x26\x03\x26\x05\x26\u{404}\x0a\x26\x03\x27\
	\x03\x27\x05\x27\u{408}\x0a\x27\x03\x27\x03\x27\x03\x27\x05\x27\u{40d}\x0a\
	\x27\x03\x27\x03\x27\x05\x27\u{411}\x0a\x27\x03\x27\x05\x27\u{414}\x0a\x27\
	\x03\x27\x03\x27\x03\x27\x03\x27\x03\x27\x03\x27\x03\x27\x03\x27\x03\x27\
	\x03\x27\x03\x27\x03\x27\x03\x27\x03\x27\x03\x27\x03\x27\x05\x27\u{426}\
	\x0a\x27\x03\x27\x03\x27\x03\x27\x05\x27\u{42b}\x0a\x27\x03\x28\x03\x28\
	\x03\x28\x05\x28\u{430}\x0a\x28\x03\x29\x03\x29\x03\x29\x03\x29\x03\x29\
	\x03\x29\x05\x29\u{438}\x0a\x29\x03\x29\x03\x29\x03\x29\x05\x29\u{43d}\x0a\
	\x29\x03\x29\x03\x29\x03\x29\x03\x29\x03\x29\x03\x29\x03\x29\x05\x29\u{446}\
	\x0a\x29\x03\x29\x03\x29\x03\x29\x07\x29\u{44b}\x0a\x29\x0c\x29\x0e\x29\
	\u{44e}\x0b\x29\x03\x29\x05\x29\u{451}\x0a\x29\x03\x29\x03\x29\x03\x29\x03\
	\x29\x03\x29\x03\x29\x03\x29\x03\x29\x03\x29\x03\x29\x03\x29\x03\x29\x03\
	\x29\x03\x29\x05\x29\u{461}\x0a\x29\x03\x29\x05\x29\u{464}\x0a\x29\x03\x29\
	\x03\x29\x03\x29\x03\x29\x03\x29\x03\x29\x05\x29\u{46c}\x0a\x29\x03\x29\
	\x03\x29\x03\x29\x03\x29\x03\x29\x06\x29\u{473}\x0a\x29\x0d\x29\x0e\x29\
	\u{474}\x03\x29\x03\x29\x05\x29\u{479}\x0a\x29\x03\x29\x03\x29\x03\x29\x05\
	\x29\u{47e}\x0a\x29\x03\x29\x03\x29\x03\x29\x03\x29\x03\x29\x03\x29\x03\
	\x29\x03\x29\x03\x29\x03\x29\x03\x29\x03\x29\x03\x29\x03\x29\x03\x29\x03\
	\x29\x03\x29\x03\x29\x03\x29\x03\x29\x03\x29\x03\x29\x03\x29\x03\x29\x03\
	\x29\x03\x29\x03\x29\x05\x29\u{49b}\x0a\x29\x03\x29\x03\x29\x03\x29\x05\
	\x29\u{4a0}\x0a\x29\x03\x29\x03\x29\x03\x29\x03\x29\x03\x29\x03\x29\x03\
	\x29\x05\x29\u{4a9}\x0a\x29\x03\x29\x03\x29\x03\x29\x03\x29\x03\x29\x03\
	\x29\x07\x29\u{4b1}\x0a\x29\x0c\x29\x0e\x29\u{4b4}\x0b\x29\x05\x29\u{4b6}\
	\x0a\x29\x03\x29\x03\x29\x03\x29\x03\x29\x05\x29\u{4bc}\x0a\x29\x03\x29\
	\x05\x29\u{4bf}\x0a\x29\x03\x29\x03\x29\x03\x29\x03\x29\x03\x29\x05\x29\
	\u{4c6}\x0a\x29\x03\x29\x03\x29\x03\x29\x03\x29\x05\x29\u{4cc}\x0a\x29\x03\
	\x29\x03\x29\x03\x29\x03\x29\x03\x29\x05\x29\u{4d3}\x0a\x29\x07\x29\u{4d5}\
	\x0a\x29\x0c\x29\x0e\x29\u{4d8}\x0b\x29\x03\x2a\x03\x2a\x03\x2a\x03\x2a\
	\x03\x2a\x03\x2a\x07\x2a\u{4e0}\x0a\x2a\x0c\x2a\x0e\x2a\u{4e3}\x0b\x2a\x03\
	\x2a\x03\x2a\x05\x2a\u{4e7}\x0a\x2a\x03\x2a\x03\x2a\x03\x2a\x03\x2a\x03\
	\x2a\x03\x2a\x03\x2a\x03\x2a\x03\x2a\x03\x2a\x05\x2a\u{4f3}\x0a\x2a\x03\
	\x2a\x03\x2a\x05\x2a\u{4f7}\x0a\x2a\x07\x2a\u{4f9}\x0a\x2a\x0c\x2a\x0e\x2a\
	\u{4fc}\x0b\x2a\x03\x2a\x05\x2a\u{4ff}\x0a\x2a\x03\x2a\x03\x2a\x03\x2a\x03\
	\x2a\x03\x2a\x05\x2a\u{506}\x0a\x2a\x05\x2a\u{508}\x0a\x2a\x03\x2b\x03\x2b\
	\x03\x2b\x03\x2b\x03\x2b\x03\x2b\x05\x2b\u{510}\x0a\x2b\x03\x2b\x03\x2b\
	\x03\x2c\x03\x2c\x03\x2c\x05\x2c\u{517}\x0a\x2c\x03\x2c\x05\x2c\u{51a}\x0a\
	\x2c\x03\x2d\x03\x2d\x05\x2d\u{51e}\x0a\x2d\x03\x2d\x03\x2d\x03\x2d\x05\
	\x2d\u{523}\x0a\x2d\x03\x2d\x03\x2d\x03\x2d\x03\x2d\x07\x2d\u{529}\x0a\x2d\
	\x0c\x2d\x0e\x2d\u{52c}\x0b\x2d\x03\x2d\x03\x2d\x03\x2d\x03\x2d\x03\x2d\
	\x03\x2d\x03\x2d\x03\x2d\x03\x2d\x03\x2d\x03\x2d\x03\x2d\x03\x2d\x03\x2d\
	\x07\x2d\u{53c}\x0a\x2d\x0c\x2d\x0e\x2d\u{53f}\x0b\x2d\x03\x2d\x03\x2d\x03\
	\x2d\x05\x2d\u{544}\x0a\x2d\x03\x2e\x03\x2e\x05\x2e\u{548}\x0a\x2e\x03\x2e\
	\x03\x2e\x03\x2e\x07\x2e\u{54d}\x0a\x2e\x0c\x2e\x0e\x2e\u{550}\x0b\x2e\x03\
	\x2f\x03\x2f\x03\x2f\x05\x2f\u{555}\x0a\x2f\x03\x2f\x03\x2f\x03\x2f\x03\
	\x2f\x03\x2f\x03\x2f\x05\x2f\u{55d}\x0a\x2f\x03\x30\x03\x30\x03\x30\x05\
	\x30\u{562}\x0a\x30\x03\x30\x05\x30\u{565}\x0a\x30\x03\x31\x03\x31\x03\x31\
	\x05\x31\u{56a}\x0a\x31\x03\x32\x03\x32\x03\x32\x03\x32\x03\x32\x07\x32\
	\u{571}\x0a\x32\x0c\x32\x0e\x32\u{574}\x0b\x32\x03\x32\x03\x32\x05\x32\u{578}\
	\x0a\x32\x03\x32\x03\x32\x03\x32\x03\x32\x03\x32\x03\x33\x03\x33\x03\x33\
	\x03\x33\x03\x33\x03\x33\x03\x33\x05\x33\u{586}\x0a\x33\x03\x33\x05\x33\
	\u{589}\x0a\x33\x05\x33\u{58b}\x0a\x33\x03\x34\x03\x34\x03\x34\x05\x34\u{590}\
	\x0a\x34\x03\x34\x03\x34\x05\x34\u{594}\x0a\x34\x03\x34\x05\x34\u{597}\x0a\
	\x34\x03\x34\x03\x34\x03\x34\x03\x34\x03\x34\x05\x34\u{59e}\x0a\x34\x03\
	\x34\x03\x34\x03\x34\x05\x34\u{5a3}\x0a\x34\x03\x34\x03\x34\x03\x34\x03\
	\x34\x03\x34\x07\x34\u{5aa}\x0a\x34\x0c\x34\x0e\x34\u{5ad}\x0b\x34\x05\x34\
	\u{5af}\x0a\x34\x03\x34\x03\x34\x05\x34\u{5b3}\x0a\x34\x03\x34\x05\x34\u{5b6}\
	\x0a\x34\x03\x34\x03\x34\x03\x34\x03\x34\x07\x34\u{5bc}\x0a\x34\x0c\x34\
	\x0e\x34\u{5bf}\x0b\x34\x03\x34\x05\x34\u{5c2}\x0a\x34\x03\x34\x03\x34\x03\
	\x34\x03\x34\x03\x34\x03\x34\x05\x34\u{5ca}\x0a\x34\x03\x34\x05\x34\u{5cd}\
	\x0a\x34\x05\x34\u{5cf}\x0a\x34\x03\x35\x03\x35\x03\x35\x03\x35\x03\x35\
	\x07\x35\u{5d6}\x0a\x35\x0c\x35\x0e\x35\u{5d9}\x0b\x35\x03\x36\x03\x36\x05\
	\x36\u{5dd}\x0a\x36\x03\x36\x03\x36\x05\x36\u{5e1}\x0a\x36\x03\x36\x03\x36\
	\x05\x36\u{5e5}\x0a\x36\x03\x36\x05\x36\u{5e8}\x0a\x36\x03\x37\x03\x37\x03\
	\x37\x03\x37\x03\x37\x03\x37\x03\x37\x07\x37\u{5f1}\x0a\x37\x0c\x37\x0e\
	\x37\u{5f4}\x0b\x37\x03\x37\x03\x37\x05\x37\u{5f8}\x0a\x37\x03\x38\x03\x38\
	\x05\x38\u{5fc}\x0a\x38\x03\x38\x03\x38\x03\x38\x07\x38\u{601}\x0a\x38\x0c\
	\x38\x0e\x38\u{604}\x0b\x38\x03\x38\x03\x38\x03\x38\x03\x38\x07\x38\u{60a}\
	\x0a\x38\x0c\x38\x0e\x38\u{60d}\x0b\x38\x03\x38\x05\x38\u{610}\x0a\x38\x05\
	\x38\u{612}\x0a\x38\x03\x38\x03\x38\x05\x38\u{616}\x0a\x38\x03\x38\x03\x38\
	\x03\x38\x03\x38\x03\x38\x07\x38\u{61d}\x0a\x38\x0c\x38\x0e\x38\u{620}\x0b\
	\x38\x03\x38\x03\x38\x05\x38\u{624}\x0a\x38\x05\x38\u{626}\x0a\x38\x03\x38\
	\x03\x38\x03\x38\x03\x38\x03\x38\x07\x38\u{62d}\x0a\x38\x0c\x38\x0e\x38\
	\u{630}\x0b\x38\x03\x38\x03\x38\x03\x38\x03\x38\x03\x38\x03\x38\x07\x38\
	\u{638}\x0a\x38\x0c\x38\x0e\x38\u{63b}\x0b\x38\x03\x38\x03\x38\x07\x38\u{63f}\
	\x0a\x38\x0c\x38\x0e\x38\u{642}\x0b\x38\x05\x38\u{644}\x0a\x38\x03\x39\x03\
	\x39\x03\x39\x03\x39\x03\x39\x05\x39\u{64b}\x0a\x39\x03\x3a\x05\x3a\u{64e}\
	\x0a\x3a\x03\x3a\x03\x3a\x03\x3b\x03\x3b\x03\x3c\x03\x3c\x03\x3d\x03\x3d\
	\x03\x3e\x03\x3e\x05\x3e\u{65a}\x0a\x3e\x03\x3f\x03\x3f\x03\x40\x03\x40\
	\x03\x41\x03\x41\x03\x42\x03\x42\x03\x43\x03\x43\x03\x44\x03\x44\x03\x45\
	\x03\x45\x03\x46\x03\x46\x03\x47\x03\x47\x03\x48\x03\x48\x03\x49\x03\x49\
	\x03\x4a\x03\x4a\x03\x4b\x03\x4b\x03\x4c\x03\x4c\x03\x4d\x03\x4d\x03\x4e\
	\x03\x4e\x03\x4f\x03\x4f\x03\x50\x03\x50\x03\x51\x03\x51\x03\x52\x03\x52\
	\x03\x52\x03\x52\x03\x52\x03\x52\x05\x52\u{688}\x0a\x52\x03\x53\x03\x53\
	\x03\x54\x03\x54\x03\x54\x03\x54\x03\x54\x03\x54\x03\x54\x05\x54\u{693}\
	\x0a\x54\x03\x54\x04\u{182}\u{3f7}\x03\x50\x55\x02\x04\x06\x08\x0a\x0c\x0e\
	\x10\x12\x14\x16\x18\x1a\x1c\x1e\x20\x22\x24\x26\x28\x2a\x2c\x2e\x30\x32\
	\x34\x36\x38\x3a\x3c\x3e\x40\x42\x44\x46\x48\x4a\x4c\x4e\x50\x52\x54\x56\
	\x58\x5a\x5c\x5e\x60\x62\x64\x66\x68\x6a\x6c\x6e\x70\x72\x74\x76\x78\x7a\
	\x7c\x7e\u{80}\u{82}\u{84}\u{86}\u{88}\u{8a}\u{8c}\u{8e}\u{90}\u{92}\u{94}\
	\u{96}\u{98}\u{9a}\u{9c}\u{9e}\u{a0}\u{a2}\u{a4}\u{a6}\x02\x15\x05\x02\x3c\
	\x3c\x47\x47\x54\x54\x04\x02\x31\x31\x44\x44\x04\x02\x07\x07\x6c\x6c\x03\
	\x02\u{85}\u{86}\x04\x02\x1f\x1f\x40\x40\x04\x02\x24\x24\x3e\x3e\x07\x02\
	\x1b\x1b\x4a\x4a\x53\x53\x7c\x7c\x7f\x7f\x04\x02\x09\x09\x0e\x0f\x03\x02\
	\x0a\x0b\x03\x02\x10\x13\x03\x02\x14\x17\x04\x02\x08\x08\x18\x1a\x06\x02\
	\x4f\x4f\x63\x63\x65\x65\x78\x78\x04\x02\x3d\x3d\u{8d}\u{8d}\x05\x02\x1b\
	\x1b\x4a\x4a\x7f\x7f\x06\x02\x36\x38\x6a\x6a\u{98}\u{98}\u{9a}\u{9b}\x04\
	\x02\x0a\x0c\x68\x68\x04\x02\u{97}\u{97}\u{9a}\u{9a}\x03\x02\x1b\u{96}\x02\
	\u{793}\x02\u{ac}\x03\x02\x02\x02\x04\u{b1}\x03\x02\x02\x02\x06\u{b7}\x03\
	\x02\x02\x02\x08\u{d1}\x03\x02\x02\x02\x0a\u{f3}\x03\x02\x02\x02\x0c\u{105}\
	\x03\x02\x02\x02\x0e\u{10e}\x03\x02\x02\x02\x10\u{116}\x03\x02\x02\x02\x12\
	\u{120}\x03\x02\x02\x02\x14\u{128}\x03\x02\x02\x02\x16\u{14c}\x03\x02\x02\
	\x02\x18\u{16c}\x03\x02\x02\x02\x1a\u{195}\x03\x02\x02\x02\x1c\u{1da}\x03\
	\x02\x02\x02\x1e\u{1ed}\x03\x02\x02\x02\x20\u{20b}\x03\x02\x02\x02\x22\u{215}\
	\x03\x02\x02\x02\x24\u{232}\x03\x02\x02\x02\x26\u{238}\x03\x02\x02\x02\x28\
	\u{245}\x03\x02\x02\x02\x2a\u{252}\x03\x02\x02\x02\x2c\u{25f}\x03\x02\x02\
	\x02\x2e\u{26d}\x03\x02\x02\x02\x30\u{28d}\x03\x02\x02\x02\x32\u{2d7}\x03\
	\x02\x02\x02\x34\u{2e6}\x03\x02\x02\x02\x36\u{2f3}\x03\x02\x02\x02\x38\u{2f9}\
	\x03\x02\x02\x02\x3a\u{307}\x03\x02\x02\x02\x3c\u{30b}\x03\x02\x02\x02\x3e\
	\u{323}\x03\x02\x02\x02\x40\u{38c}\x03\x02\x02\x02\x42\u{38f}\x03\x02\x02\
	\x02\x44\u{3b2}\x03\x02\x02\x02\x46\u{3e8}\x03\x02\x02\x02\x48\u{3ea}\x03\
	\x02\x02\x02\x4a\u{3f5}\x03\x02\x02\x02\x4c\u{407}\x03\x02\x02\x02\x4e\u{42f}\
	\x03\x02\x02\x02\x50\u{47d}\x03\x02\x02\x02\x52\u{4d9}\x03\x02\x02\x02\x54\
	\u{509}\x03\x02\x02\x02\x56\u{513}\x03\x02\x02\x02\x58\u{51d}\x03\x02\x02\
	\x02\x5a\u{545}\x03\x02\x02\x02\x5c\u{554}\x03\x02\x02\x02\x5e\u{55e}\x03\
	\x02\x02\x02\x60\u{569}\x03\x02\x02\x02\x62\u{56b}\x03\x02\x02\x02\x64\u{58a}\
	\x03\x02\x02\x02\x66\u{5ce}\x03\x02\x02\x02\x68\u{5d0}\x03\x02\x02\x02\x6a\
	\u{5e7}\x03\x02\x02\x02\x6c\u{5f7}\x03\x02\x02\x02\x6e\u{643}\x03\x02\x02\
	\x02\x70\u{64a}\x03\x02\x02\x02\x72\u{64d}\x03\x02\x02\x02\x74\u{651}\x03\
	\x02\x02\x02\x76\u{653}\x03\x02\x02\x02\x78\u{655}\x03\x02\x02\x02\x7a\u{659}\
	\x03\x02\x02\x02\x7c\u{65b}\x03\x02\x02\x02\x7e\u{65d}\x03\x02\x02\x02\u{80}\
	\u{65f}\x03\x02\x02\x02\u{82}\u{661}\x03\x02\x02\x02\u{84}\u{663}\x03\x02\
	\x02\x02\u{86}\u{665}\x03\x02\x02\x02\u{88}\u{667}\x03\x02\x02\x02\u{8a}\
	\u{669}\x03\x02\x02\x02\u{8c}\u{66b}\x03\x02\x02\x02\u{8e}\u{66d}\x03\x02\
	\x02\x02\u{90}\u{66f}\x03\x02\x02\x02\u{92}\u{671}\x03\x02\x02\x02\u{94}\
	\u{673}\x03\x02\x02\x02\u{96}\u{675}\x03\x02\x02\x02\u{98}\u{677}\x03\x02\
	\x02\x02\u{9a}\u{679}\x03\x02\x02\x02\u{9c}\u{67b}\x03\x02\x02\x02\u{9e}\
	\u{67d}\x03\x02\x02\x02\u{a0}\u{67f}\x03\x02\x02\x02\u{a2}\u{687}\x03\x02\
	\x02\x02\u{a4}\u{689}\x03\x02\x02\x02\u{a6}\u{692}\x03\x02\x02\x02\u{a8}\
	\u{ab}\x05\x06\x04\x02\u{a9}\u{ab}\x05\x04\x03\x02\u{aa}\u{a8}\x03\x02\x02\
	\x02\u{aa}\u{a9}\x03\x02\x02\x02\u{ab}\u{ae}\x03\x02\x02\x02\u{ac}\u{aa}\
	\x03\x02\x02\x02\u{ac}\u{ad}\x03\x02\x02\x02\u{ad}\u{af}\x03\x02\x02\x02\
	\u{ae}\u{ac}\x03\x02\x02\x02\u{af}\u{b0}\x07\x02\x02\x03\u{b0}\x03\x03\x02\
	\x02\x02\u{b1}\u{b2}\x07\u{9f}\x02\x02\u{b2}\u{b3}\x08\x03\x01\x02\u{b3}\
	\x05\x03\x02\x02\x02\u{b4}\u{b6}\x07\x03\x02\x02\u{b5}\u{b4}\x03\x02\x02\
	\x02\u{b6}\u{b9}\x03\x02\x02\x02\u{b7}\u{b5}\x03\x02\x02\x02\u{b7}\u{b8}\
	\x03\x02\x02\x02\u{b8}\u{ba}\x03\x02\x02\x02\u{b9}\u{b7}\x03\x02\x02\x02\
	\u{ba}\u{c3}\x05\x08\x05\x02\u{bb}\u{bd}\x07\x03\x02\x02\u{bc}\u{bb}\x03\
	\x02\x02\x02\u{bd}\u{be}\x03\x02\x02\x02\u{be}\u{bc}\x03\x02\x02\x02\u{be}\
	\u{bf}\x03\x02\x02\x02\u{bf}\u{c0}\x03\x02\x02\x02\u{c0}\u{c2}\x05\x08\x05\
	\x02\u{c1}\u{bc}\x03\x02\x02\x02\u{c2}\u{c5}\x03\x02\x02\x02\u{c3}\u{c1}\
	\x03\x02\x02\x02\u{c3}\u{c4}\x03\x02\x02\x02\u{c4}\u{c9}\x03\x02\x02\x02\
	\u{c5}\u{c3}\x03\x02\x02\x02\u{c6}\u{c8}\x07\x03\x02\x02\u{c7}\u{c6}\x03\
	\x02\x02\x02\u{c8}\u{cb}\x03\x02\x02\x02\u{c9}\u{c7}\x03\x02\x02\x02\u{c9}\
	\u{ca}\x03\x02\x02\x02\u{ca}\x07\x03\x02\x02\x02\u{cb}\u{c9}\x03\x02\x02\
	\x02\u{cc}\u{cf}\x07\x49\x02\x02\u{cd}\u{ce}\x07\x74\x02\x02\u{ce}\u{d0}\
	\x07\x71\x02\x02\u{cf}\u{cd}\x03\x02\x02\x02\u{cf}\u{d0}\x03\x02\x02\x02\
	\u{d0}\u{d2}\x03\x02\x02\x02\u{d1}\u{cc}\x03\x02\x02\x02\u{d1}\u{d2}\x03\
	\x02\x02\x02\u{d2}\u{f1}\x03\x02\x02\x02\u{d3}\u{f2}\x05\x0a\x06\x02\u{d4}\
	\u{f2}\x05\x0c\x07\x02\u{d5}\u{f2}\x05\x0e\x08\x02\u{d6}\u{f2}\x05\x10\x09\
	\x02\u{d7}\u{f2}\x05\x12\x0a\x02\u{d8}\u{f2}\x05\x14\x0b\x02\u{d9}\u{f2}\
	\x05\x16\x0c\x02\u{da}\u{f2}\x05\x18\x0d\x02\u{db}\u{f2}\x05\x1a\x0e\x02\
	\u{dc}\u{f2}\x05\x1c\x0f\x02\u{dd}\u{f2}\x05\x1e\x10\x02\u{de}\u{f2}\x05\
	\x20\x11\x02\u{df}\u{f2}\x05\x22\x12\x02\u{e0}\u{f2}\x05\x24\x13\x02\u{e1}\
	\u{f2}\x05\x26\x14\x02\u{e2}\u{f2}\x05\x28\x15\x02\u{e3}\u{f2}\x05\x2a\x16\
	\x02\u{e4}\u{f2}\x05\x2c\x17\x02\u{e5}\u{f2}\x05\x2e\x18\x02\u{e6}\u{f2}\
	\x05\x30\x19\x02\u{e7}\u{f2}\x05\x32\x1a\x02\u{e8}\u{f2}\x05\x34\x1b\x02\
	\u{e9}\u{f2}\x05\x36\x1c\x02\u{ea}\u{f2}\x05\x38\x1d\x02\u{eb}\u{f2}\x05\
	\x3a\x1e\x02\u{ec}\u{f2}\x05\x3c\x1f\x02\u{ed}\u{f2}\x05\x3e\x20\x02\u{ee}\
	\u{f2}\x05\x42\x22\x02\u{ef}\u{f2}\x05\x44\x23\x02\u{f0}\u{f2}\x05\x46\x24\
	\x02\u{f1}\u{d3}\x03\x02\x02\x02\u{f1}\u{d4}\x03\x02\x02\x02\u{f1}\u{d5}\
	\x03\x02\x02\x02\u{f1}\u{d6}\x03\x02\x02\x02\u{f1}\u{d7}\x03\x02\x02\x02\
	\u{f1}\u{d8}\x03\x02\x02\x02\u{f1}\u{d9}\x03\x02\x02\x02\u{f1}\u{da}\x03\
	\x02\x02\x02\u{f1}\u{db}\x03\x02\x02\x02\u{f1}\u{dc}\x03\x02\x02\x02\u{f1}\
	\u{dd}\x03\x02\x02\x02\u{f1}\u{de}\x03\x02\x02\x02\u{f1}\u{df}\x03\x02\x02\
	\x02\u{f1}\u{e0}\x03\x02\x02\x02\u{f1}\u{e1}\x03\x02\x02\x02\u{f1}\u{e2}\
	\x03\x02\x02\x02\u{f1}\u{e3}\x03\x02\x02\x02\u{f1}\u{e4}\x03\x02\x02\x02\
	\u{f1}\u{e5}\x03\x02\x02\x02\u{f1}\u{e6}\x03\x02\x02\x02\u{f1}\u{e7}\x03\
	\x02\x02\x02\u{f1}\u{e8}\x03\x02\x02\x02\u{f1}\u{e9}\x03\x02\x02\x02\u{f1}\
	\u{ea}\x03\x02\x02\x02\u{f1}\u{eb}\x03\x02\x02\x02\u{f1}\u{ec}\x03\x02\x02\
	\x02\u{f1}\u{ed}\x03\x02\x02\x02\u{f1}\u{ee}\x03\x02\x02\x02\u{f1}\u{ef}\
	\x03\x02\x02\x02\u{f1}\u{f0}\x03\x02\x02\x02\u{f2}\x09\x03\x02\x02\x02\u{f3}\
	\u{f4}\x07\x20\x02\x02\u{f4}\u{f8}\x07\u{84}\x02\x02\u{f5}\u{f6}\x05\u{84}\
	\x43\x02\u{f6}\u{f7}\x07\x04\x02\x02\u{f7}\u{f9}\x03\x02\x02\x02\u{f8}\u{f5}\
	\x03\x02\x02\x02\u{f8}\u{f9}\x03\x02\x02\x02\u{f9}\u{fa}\x03\x02\x02\x02\
	\u{fa}\u{103}\x05\u{8a}\x46\x02\u{fb}\u{fc}\x07\x7b\x02\x02\u{fc}\u{fd}\
	\x07\u{88}\x02\x02\u{fd}\u{104}\x05\u{8e}\x48\x02\u{fe}\u{100}\x07\x1d\x02\
	\x02\u{ff}\u{101}\x07\x30\x02\x02\u{100}\u{ff}\x03\x02\x02\x02\u{100}\u{101}\
	\x03\x02\x02\x02\u{101}\u{102}\x03\x02\x02\x02\u{102}\u{104}\x05\x48\x25\
	\x02\u{103}\u{fb}\x03\x02\x02\x02\u{103}\u{fe}\x03\x02\x02\x02\u{104}\x0b\
	\x03\x02\x02\x02\u{105}\u{10c}\x07\x21\x02\x02\u{106}\u{10d}\x05\u{84}\x43\
	\x02\u{107}\u{10d}\x05\u{8c}\x47\x02\u{108}\u{109}\x05\u{84}\x43\x02\u{109}\
	\u{10a}\x07\x04\x02\x02\u{10a}\u{10b}\x05\u{8c}\x47\x02\u{10b}\u{10d}\x03\
	\x02\x02\x02\u{10c}\u{106}\x03\x02\x02\x02\u{10c}\u{107}\x03\x02\x02\x02\
	\u{10c}\u{108}\x03\x02\x02\x02\u{10c}\u{10d}\x03\x02\x02\x02\u{10d}\x0d\
	\x03\x02\x02\x02\u{10e}\u{110}\x07\x25\x02\x02\u{10f}\u{111}\x07\x39\x02\
	\x02\u{110}\u{10f}\x03\x02\x02\x02\u{110}\u{111}\x03\x02\x02\x02\u{111}\
	\u{112}\x03\x02\x02\x02\u{112}\u{113}\x05\x50\x29\x02\u{113}\u{114}\x07\
	\x23\x02\x02\u{114}\u{115}\x05\u{84}\x43\x02\u{115}\x0f\x03\x02\x02\x02\
	\u{116}\u{118}\x07\x28\x02\x02\u{117}\u{119}\x09\x02\x02\x02\u{118}\u{117}\
	\x03\x02\x02\x02\u{118}\u{119}\x03\x02\x02\x02\u{119}\u{11e}\x03\x02\x02\
	\x02\u{11a}\u{11c}\x07\u{89}\x02\x02\u{11b}\u{11d}\x05\u{a4}\x53\x02\u{11c}\
	\u{11b}\x03\x02\x02\x02\u{11c}\u{11d}\x03\x02\x02\x02\u{11d}\u{11f}\x03\
	\x02\x02\x02\u{11e}\u{11a}\x03\x02\x02\x02\u{11e}\u{11f}\x03\x02\x02\x02\
	\u{11f}\x11\x03\x02\x02\x02\u{120}\u{125}\x09\x03\x02\x02\u{121}\u{123}\
	\x07\u{89}\x02\x02\u{122}\u{124}\x05\u{a4}\x53\x02\u{123}\u{122}\x03\x02\
	\x02\x02\u{123}\u{124}\x03\x02\x02\x02\u{124}\u{126}\x03\x02\x02\x02\u{125}\
	\u{121}\x03\x02\x02\x02\u{125}\u{126}\x03\x02\x02\x02\u{126}\x13\x03\x02\
	\x02\x02\u{127}\u{129}\x05\x5a\x2e\x02\u{128}\u{127}\x03\x02\x02\x02\u{128}\
	\u{129}\x03\x02\x02\x02\u{129}\u{12a}\x03\x02\x02\x02\u{12a}\u{134}\x05\
	\x6e\x38\x02\u{12b}\u{12d}\x07\u{8b}\x02\x02\u{12c}\u{12e}\x07\x1f\x02\x02\
	\u{12d}\u{12c}\x03\x02\x02\x02\u{12d}\u{12e}\x03\x02\x02\x02\u{12e}\u{132}\
	\x03\x02\x02\x02\u{12f}\u{132}\x07\x5c\x02\x02\u{130}\u{132}\x07\x46\x02\
	\x02\u{131}\u{12b}\x03\x02\x02\x02\u{131}\u{12f}\x03\x02\x02\x02\u{131}\
	\u{130}\x03\x02\x02\x02\u{132}\u{133}\x03\x02\x02\x02\u{133}\u{135}\x05\
	\x6e\x38\x02\u{134}\u{131}\x03\x02\x02\x02\u{135}\u{136}\x03\x02\x02\x02\
	\u{136}\u{134}\x03\x02\x02\x02\u{136}\u{137}\x03\x02\x02\x02\u{137}\u{142}\
	\x03\x02\x02\x02\u{138}\u{139}\x07\x6f\x02\x02\u{139}\u{13a}\x07\x2a\x02\
	\x02\u{13a}\u{13f}\x05\x5e\x30\x02\u{13b}\u{13c}\x07\x07\x02\x02\u{13c}\
	\u{13e}\x05\x5e\x30\x02\u{13d}\u{13b}\x03\x02\x02\x02\u{13e}\u{141}\x03\
	\x02\x02\x02\u{13f}\u{13d}\x03\x02\x02\x02\u{13f}\u{140}\x03\x02\x02\x02\
	\u{140}\u{143}\x03\x02\x02\x02\u{141}\u{13f}\x03\x02\x02\x02\u{142}\u{138}\
	\x03\x02\x02\x02\u{142}\u{143}\x03\x02\x02\x02\u{143}\u{14a}\x03\x02\x02\
	\x02\u{144}\u{145}\x07\x64\x02\x02\u{145}\u{148}\x05\x50\x29\x02\u{146}\
	\u{147}\x09\x04\x02\x02\u{147}\u{149}\x05\x50\x29\x02\u{148}\u{146}\x03\
	\x02\x02\x02\u{148}\u{149}\x03\x02\x02\x02\u{149}\u{14b}\x03\x02\x02\x02\
	\u{14a}\u{144}\x03\x02\x02\x02\u{14a}\u{14b}\x03\x02\x02\x02\u{14b}\x15\
	\x03\x02\x02\x02\u{14c}\u{14e}\x07\x34\x02\x02\u{14d}\u{14f}\x07\u{8c}\x02\
	\x02\u{14e}\u{14d}\x03\x02\x02\x02\u{14e}\u{14f}\x03\x02\x02\x02\u{14f}\
	\u{150}\x03\x02\x02\x02\u{150}\u{154}\x07\x56\x02\x02\u{151}\u{152}\x07\
	\x52\x02\x02\u{152}\u{153}\x07\x68\x02\x02\u{153}\u{155}\x07\x48\x02\x02\
	\u{154}\u{151}\x03\x02\x02\x02\u{154}\u{155}\x03\x02\x02\x02\u{155}\u{159}\
	\x03\x02\x02\x02\u{156}\u{157}\x05\u{84}\x43\x02\u{157}\u{158}\x07\x04\x02\
	\x02\u{158}\u{15a}\x03\x02\x02\x02\u{159}\u{156}\x03\x02\x02\x02\u{159}\
	\u{15a}\x03\x02\x02\x02\u{15a}\u{15b}\x03\x02\x02\x02\u{15b}\u{15c}\x05\
	\u{96}\x4c\x02\u{15c}\u{15d}\x07\x6d\x02\x02\u{15d}\u{15e}\x05\u{8a}\x46\
	\x02\u{15e}\u{15f}\x07\x05\x02\x02\u{15f}\u{164}\x05\x56\x2c\x02\u{160}\
	\u{161}\x07\x07\x02\x02\u{161}\u{163}\x05\x56\x2c\x02\u{162}\u{160}\x03\
	\x02\x02\x02\u{163}\u{166}\x03\x02\x02\x02\u{164}\u{162}\x03\x02\x02\x02\
	\u{164}\u{165}\x03\x02\x02\x02\u{165}\u{167}\x03\x02\x02\x02\u{166}\u{164}\
	\x03\x02\x02\x02\u{167}\u{16a}\x07\x06\x02\x02\u{168}\u{169}\x07\u{94}\x02\
	\x02\u{169}\u{16b}\x05\x50\x29\x02\u{16a}\u{168}\x03\x02\x02\x02\u{16a}\
	\u{16b}\x03\x02\x02\x02\u{16b}\x17\x03\x02\x02\x02\u{16c}\u{16e}\x07\x34\
	\x02\x02\u{16d}\u{16f}\x09\x05\x02\x02\u{16e}\u{16d}\x03\x02\x02\x02\u{16e}\
	\u{16f}\x03\x02\x02\x02\u{16f}\u{170}\x03\x02\x02\x02\u{170}\u{174}\x07\
	\u{84}\x02\x02\u{171}\u{172}\x07\x52\x02\x02\u{172}\u{173}\x07\x68\x02\x02\
	\u{173}\u{175}\x07\x48\x02\x02\u{174}\u{171}\x03\x02\x02\x02\u{174}\u{175}\
	\x03\x02\x02\x02\u{175}\u{179}\x03\x02\x02\x02\u{176}\u{177}\x05\u{84}\x43\
	\x02\u{177}\u{178}\x07\x04\x02\x02\u{178}\u{17a}\x03\x02\x02\x02\u{179}\
	\u{176}\x03\x02\x02\x02\u{179}\u{17a}\x03\x02\x02\x02\u{17a}\u{17b}\x03\
	\x02\x02\x02\u{17b}\u{193}\x05\u{8a}\x46\x02\u{17c}\u{17d}\x07\x05\x02\x02\
	\u{17d}\u{182}\x05\x48\x25\x02\u{17e}\u{17f}\x07\x07\x02\x02\u{17f}\u{181}\
	\x05\x48\x25\x02\u{180}\u{17e}\x03\x02\x02\x02\u{181}\u{184}\x03\x02\x02\
	\x02\u{182}\u{183}\x03\x02\x02\x02\u{182}\u{180}\x03\x02\x02\x02\u{183}\
	\u{189}\x03\x02\x02\x02\u{184}\u{182}\x03\x02\x02\x02\u{185}\u{186}\x07\
	\x07\x02\x02\u{186}\u{188}\x05\x58\x2d\x02\u{187}\u{185}\x03\x02\x02\x02\
	\u{188}\u{18b}\x03\x02\x02\x02\u{189}\u{187}\x03\x02\x02\x02\u{189}\u{18a}\
	\x03\x02\x02\x02\u{18a}\u{18c}\x03\x02\x02\x02\u{18b}\u{189}\x03\x02\x02\
	\x02\u{18c}\u{18f}\x07\x06\x02\x02\u{18d}\u{18e}\x07\u{96}\x02\x02\u{18e}\
	\u{190}\x07\u{97}\x02\x02\u{18f}\u{18d}\x03\x02\x02\x02\u{18f}\u{190}\x03\
	\x02\x02\x02\u{190}\u{194}\x03\x02\x02\x02\u{191}\u{192}\x07\x23\x02\x02\
	\u{192}\u{194}\x05\x3e\x20\x02\u{193}\u{17c}\x03\x02\x02\x02\u{193}\u{191}\
	\x03\x02\x02\x02\u{194}\x19\x03\x02\x02\x02\u{195}\u{197}\x07\x34\x02\x02\
	\u{196}\u{198}\x09\x05\x02\x02\u{197}\u{196}\x03\x02\x02\x02\u{197}\u{198}\
	\x03\x02\x02\x02\u{198}\u{199}\x03\x02\x02\x02\u{199}\u{19d}\x07\u{8a}\x02\
	\x02\u{19a}\u{19b}\x07\x52\x02\x02\u{19b}\u{19c}\x07\x68\x02\x02\u{19c}\
	\u{19e}\x07\x48\x02\x02\u{19d}\u{19a}\x03\x02\x02\x02\u{19d}\u{19e}\x03\
	\x02\x02\x02\u{19e}\u{1a2}\x03\x02\x02\x02\u{19f}\u{1a0}\x05\u{84}\x43\x02\
	\u{1a0}\u{1a1}\x07\x04\x02\x02\u{1a1}\u{1a3}\x03\x02\x02\x02\u{1a2}\u{19f}\
	\x03\x02\x02\x02\u{1a2}\u{1a3}\x03\x02\x02\x02\u{1a3}\u{1a4}\x03\x02\x02\
	\x02\u{1a4}\u{1a9}\x05\u{98}\x4d\x02\u{1a5}\u{1aa}\x07\x27\x02\x02\u{1a6}\
	\u{1aa}\x07\x1e\x02\x02\u{1a7}\u{1a8}\x07\x5b\x02\x02\u{1a8}\u{1aa}\x07\
	\x6b\x02\x02\u{1a9}\u{1a5}\x03\x02\x02\x02\u{1a9}\u{1a6}\x03\x02\x02\x02\
	\u{1a9}\u{1a7}\x03\x02\x02\x02\u{1a9}\u{1aa}\x03\x02\x02\x02\u{1aa}\u{1b9}\
	\x03\x02\x02\x02\u{1ab}\u{1ba}\x07\x3d\x02\x02\u{1ac}\u{1ba}\x07\x5a\x02\
	\x02\u{1ad}\u{1b7}\x07\u{8d}\x02\x02\u{1ae}\u{1af}\x07\x6b\x02\x02\u{1af}\
	\u{1b4}\x05\u{90}\x49\x02\u{1b0}\u{1b1}\x07\x07\x02\x02\u{1b1}\u{1b3}\x05\
	\u{90}\x49\x02\u{1b2}\u{1b0}\x03\x02\x02\x02\u{1b3}\u{1b6}\x03\x02\x02\x02\
	\u{1b4}\u{1b2}\x03\x02\x02\x02\u{1b4}\u{1b5}\x03\x02\x02\x02\u{1b5}\u{1b8}\
	\x03\x02\x02\x02\u{1b6}\u{1b4}\x03\x02\x02\x02\u{1b7}\u{1ae}\x03\x02\x02\
	\x02\u{1b7}\u{1b8}\x03\x02\x02\x02\u{1b8}\u{1ba}\x03\x02\x02\x02\u{1b9}\
	\u{1ab}\x03\x02\x02\x02\u{1b9}\u{1ac}\x03\x02\x02\x02\u{1b9}\u{1ad}\x03\
	\x02\x02\x02\u{1ba}\u{1bb}\x03\x02\x02\x02\u{1bb}\u{1bf}\x07\x6d\x02\x02\
	\u{1bc}\u{1bd}\x05\u{84}\x43\x02\u{1bd}\u{1be}\x07\x04\x02\x02\u{1be}\u{1c0}\
	\x03\x02\x02\x02\u{1bf}\u{1bc}\x03\x02\x02\x02\u{1bf}\u{1c0}\x03\x02\x02\
	\x02\u{1c0}\u{1c1}\x03\x02\x02\x02\u{1c1}\u{1c5}\x05\u{8a}\x46\x02\u{1c2}\
	\u{1c3}\x07\x4b\x02\x02\u{1c3}\u{1c4}\x07\x42\x02\x02\u{1c4}\u{1c6}\x07\
	\u{80}\x02\x02\u{1c5}\u{1c2}\x03\x02\x02\x02\u{1c5}\u{1c6}\x03\x02\x02\x02\
	\u{1c6}\u{1c9}\x03\x02\x02\x02\u{1c7}\u{1c8}\x07\u{93}\x02\x02\u{1c8}\u{1ca}\
	\x05\x50\x29\x02\u{1c9}\u{1c7}\x03\x02\x02\x02\u{1c9}\u{1ca}\x03\x02\x02\
	\x02\u{1ca}\u{1cb}\x03\x02\x02\x02\u{1cb}\u{1d4}\x07\x28\x02\x02\u{1cc}\
	\u{1d1}\x05\x42\x22\x02\u{1cd}\u{1d1}\x05\x30\x19\x02\u{1ce}\u{1d1}\x05\
	\x20\x11\x02\u{1cf}\u{1d1}\x05\x3e\x20\x02\u{1d0}\u{1cc}\x03\x02\x02\x02\
	\u{1d0}\u{1cd}\x03\x02\x02\x02\u{1d0}\u{1ce}\x03\x02\x02\x02\u{1d0}\u{1cf}\
	\x03\x02\x02\x02\u{1d1}\u{1d2}\x03\x02\x02\x02\u{1d2}\u{1d3}\x07\x03\x02\
	\x02\u{1d3}\u{1d5}\x03\x02\x02\x02\u{1d4}\u{1d0}\x03\x02\x02\x02\u{1d5}\
	\u{1d6}\x03\x02\x02\x02\u{1d6}\u{1d4}\x03\x02\x02\x02\u{1d6}\u{1d7}\x03\
	\x02\x02\x02\u{1d7}\u{1d8}\x03\x02\x02\x02\u{1d8}\u{1d9}\x07\x44\x02\x02\
	\u{1d9}\x1b\x03\x02\x02\x02\u{1da}\u{1dc}\x07\x34\x02\x02\u{1db}\u{1dd}\
	\x09\x05\x02\x02\u{1dc}\u{1db}\x03\x02\x02\x02\u{1dc}\u{1dd}\x03\x02\x02\
	\x02\u{1dd}\u{1de}\x03\x02\x02\x02\u{1de}\u{1e2}\x07\u{91}\x02\x02\u{1df}\
	\u{1e0}\x07\x52\x02\x02\u{1e0}\u{1e1}\x07\x68\x02\x02\u{1e1}\u{1e3}\x07\
	\x48\x02\x02\u{1e2}\u{1df}\x03\x02\x02\x02\u{1e2}\u{1e3}\x03\x02\x02\x02\
	\u{1e3}\u{1e7}\x03\x02\x02\x02\u{1e4}\u{1e5}\x05\u{84}\x43\x02\u{1e5}\u{1e6}\
	\x07\x04\x02\x02\u{1e6}\u{1e8}\x03\x02\x02\x02\u{1e7}\u{1e4}\x03\x02\x02\
	\x02\u{1e7}\u{1e8}\x03\x02\x02\x02\u{1e8}\u{1e9}\x03\x02\x02\x02\u{1e9}\
	\u{1ea}\x05\u{9a}\x4e\x02\u{1ea}\u{1eb}\x07\x23\x02\x02\u{1eb}\u{1ec}\x05\
	\x3e\x20\x02\u{1ec}\x1d\x03\x02\x02\x02\u{1ed}\u{1ee}\x07\x34\x02\x02\u{1ee}\
	\u{1ef}\x07\u{92}\x02\x02\u{1ef}\u{1f3}\x07\u{84}\x02\x02\u{1f0}\u{1f1}\
	\x07\x52\x02\x02\u{1f1}\u{1f2}\x07\x68\x02\x02\u{1f2}\u{1f4}\x07\x48\x02\
	\x02\u{1f3}\u{1f0}\x03\x02\x02\x02\u{1f3}\u{1f4}\x03\x02\x02\x02\u{1f4}\
	\u{1f8}\x03\x02\x02\x02\u{1f5}\u{1f6}\x05\u{84}\x43\x02\u{1f6}\u{1f7}\x07\
	\x04\x02\x02\u{1f7}\u{1f9}\x03\x02\x02\x02\u{1f8}\u{1f5}\x03\x02\x02\x02\
	\u{1f8}\u{1f9}\x03\x02\x02\x02\u{1f9}\u{1fa}\x03\x02\x02\x02\u{1fa}\u{1fb}\
	\x05\u{8a}\x46\x02\u{1fb}\u{1fc}\x07\u{8e}\x02\x02\u{1fc}\u{208}\x05\u{9c}\
	\x4f\x02\u{1fd}\u{1fe}\x07\x05\x02\x02\u{1fe}\u{203}\x05\x7a\x3e\x02\u{1ff}\
	\u{200}\x07\x07\x02\x02\u{200}\u{202}\x05\x7a\x3e\x02\u{201}\u{1ff}\x03\
	\x02\x02\x02\u{202}\u{205}\x03\x02\x02\x02\u{203}\u{201}\x03\x02\x02\x02\
	\u{203}\u{204}\x03\x02\x02\x02\u{204}\u{206}\x03\x02\x02\x02\u{205}\u{203}\
	\x03\x02\x02\x02\u{206}\u{207}\x07\x06\x02\x02\u{207}\u{209}\x03\x02\x02\
	\x02\u{208}\u{1fd}\x03\x02\x02\x02\u{208}\u{209}\x03\x02\x02\x02\u{209}\
	\x1f\x03\x02\x02\x02\u{20a}\u{20c}\x05\x5a\x2e\x02\u{20b}\u{20a}\x03\x02\
	\x02\x02\u{20b}\u{20c}\x03\x02\x02\x02\u{20c}\u{20d}\x03\x02\x02\x02\u{20d}\
	\u{20e}\x07\x3d\x02\x02\u{20e}\u{20f}\x07\x4d\x02\x02\u{20f}\u{212}\x05\
	\x5c\x2f\x02\u{210}\u{211}\x07\u{94}\x02\x02\u{211}\u{213}\x05\x50\x29\x02\
	\u{212}\u{210}\x03\x02\x02\x02\u{212}\u{213}\x03\x02\x02\x02\u{213}\x21\
	\x03\x02\x02\x02\u{214}\u{216}\x05\x5a\x2e\x02\u{215}\u{214}\x03\x02\x02\
	\x02\u{215}\u{216}\x03\x02\x02\x02\u{216}\u{217}\x03\x02\x02\x02\u{217}\
	\u{218}\x07\x3d\x02\x02\u{218}\u{219}\x07\x4d\x02\x02\u{219}\u{21c}\x05\
	\x5c\x2f\x02\u{21a}\u{21b}\x07\u{94}\x02\x02\u{21b}\u{21d}\x05\x50\x29\x02\
	\u{21c}\u{21a}\x03\x02\x02\x02\u{21c}\u{21d}\x03\x02\x02\x02\u{21d}\u{230}\
	\x03\x02\x02\x02\u{21e}\u{21f}\x07\x6f\x02\x02\u{21f}\u{220}\x07\x2a\x02\
	\x02\u{220}\u{225}\x05\x5e\x30\x02\u{221}\u{222}\x07\x07\x02\x02\u{222}\
	\u{224}\x05\x5e\x30\x02\u{223}\u{221}\x03\x02\x02\x02\u{224}\u{227}\x03\
	\x02\x02\x02\u{225}\u{223}\x03\x02\x02\x02\u{225}\u{226}\x03\x02\x02\x02\
	\u{226}\u{229}\x03\x02\x02\x02\u{227}\u{225}\x03\x02\x02\x02\u{228}\u{21e}\
	\x03\x02\x02\x02\u{228}\u{229}\x03\x02\x02\x02\u{229}\u{22a}\x03\x02\x02\
	\x02\u{22a}\u{22b}\x07\x64\x02\x02\u{22b}\u{22e}\x05\x50\x29\x02\u{22c}\
	\u{22d}\x09\x04\x02\x02\u{22d}\u{22f}\x05\x50\x29\x02\u{22e}\u{22c}\x03\
	\x02\x02\x02\u{22e}\u{22f}\x03\x02\x02\x02\u{22f}\u{231}\x03\x02\x02\x02\
	\u{230}\u{228}\x03\x02\x02\x02\u{230}\u{231}\x03\x02\x02\x02\u{231}\x23\
	\x03\x02\x02\x02\u{232}\u{234}\x07\x3f\x02\x02\u{233}\u{235}\x07\x39\x02\
	\x02\u{234}\u{233}\x03\x02\x02\x02\u{234}\u{235}\x03\x02\x02\x02\u{235}\
	\u{236}\x03\x02\x02\x02\u{236}\u{237}\x05\u{84}\x43\x02\u{237}\x25\x03\x02\
	\x02\x02\u{238}\u{239}\x07\x41\x02\x02\u{239}\u{23c}\x07\x56\x02\x02\u{23a}\
	\u{23b}\x07\x52\x02\x02\u{23b}\u{23d}\x07\x48\x02\x02\u{23c}\u{23a}\x03\
	\x02\x02\x02\u{23c}\u{23d}\x03\x02\x02\x02\u{23d}\u{241}\x03\x02\x02\x02\
	\u{23e}\u{23f}\x05\u{84}\x43\x02\u{23f}\u{240}\x07\x04\x02\x02\u{240}\u{242}\
	\x03\x02\x02\x02\u{241}\u{23e}\x03\x02\x02\x02\u{241}\u{242}\x03\x02\x02\
	\x02\u{242}\u{243}\x03\x02\x02\x02\u{243}\u{244}\x05\u{96}\x4c\x02\u{244}\
	\x27\x03\x02\x02\x02\u{245}\u{246}\x07\x41\x02\x02\u{246}\u{249}\x07\u{84}\
	\x02\x02\u{247}\u{248}\x07\x52\x02\x02\u{248}\u{24a}\x07\x48\x02\x02\u{249}\
	\u{247}\x03\x02\x02\x02\u{249}\u{24a}\x03\x02\x02\x02\u{24a}\u{24e}\x03\
	\x02\x02\x02\u{24b}\u{24c}\x05\u{84}\x43\x02\u{24c}\u{24d}\x07\x04\x02\x02\
	\u{24d}\u{24f}\x03\x02\x02\x02\u{24e}\u{24b}\x03\x02\x02\x02\u{24e}\u{24f}\
	\x03\x02\x02\x02\u{24f}\u{250}\x03\x02\x02\x02\u{250}\u{251}\x05\u{8a}\x46\
	\x02\u{251}\x29\x03\x02\x02\x02\u{252}\u{253}\x07\x41\x02\x02\u{253}\u{256}\
	\x07\u{8a}\x02\x02\u{254}\u{255}\x07\x52\x02\x02\u{255}\u{257}\x07\x48\x02\
	\x02\u{256}\u{254}\x03\x02\x02\x02\u{256}\u{257}\x03\x02\x02\x02\u{257}\
	\u{25b}\x03\x02\x02\x02\u{258}\u{259}\x05\u{84}\x43\x02\u{259}\u{25a}\x07\
	\x04\x02\x02\u{25a}\u{25c}\x03\x02\x02\x02\u{25b}\u{258}\x03\x02\x02\x02\
	\u{25b}\u{25c}\x03\x02\x02\x02\u{25c}\u{25d}\x03\x02\x02\x02\u{25d}\u{25e}\
	\x05\u{98}\x4d\x02\u{25e}\x2b\x03\x02\x02\x02\u{25f}\u{260}\x07\x41\x02\
	\x02\u{260}\u{263}\x07\u{91}\x02\x02\u{261}\u{262}\x07\x52\x02\x02\u{262}\
	\u{264}\x07\x48\x02\x02\u{263}\u{261}\x03\x02\x02\x02\u{263}\u{264}\x03\
	\x02\x02\x02\u{264}\u{268}\x03\x02\x02\x02\u{265}\u{266}\x05\u{84}\x43\x02\
	\u{266}\u{267}\x07\x04\x02\x02\u{267}\u{269}\x03\x02\x02\x02\u{268}\u{265}\
	\x03\x02\x02\x02\u{268}\u{269}\x03\x02\x02\x02\u{269}\u{26a}\x03\x02\x02\
	\x02\u{26a}\u{26b}\x05\u{9a}\x4e\x02\u{26b}\x2d\x03\x02\x02\x02\u{26c}\u{26e}\
	\x05\x5a\x2e\x02\u{26d}\u{26c}\x03\x02\x02\x02\u{26d}\u{26e}\x03\x02\x02\
	\x02\u{26e}\u{26f}\x03\x02\x02\x02\u{26f}\u{275}\x05\x6e\x38\x02\u{270}\
	\u{271}\x05\x70\x39\x02\u{271}\u{272}\x05\x6e\x38\x02\u{272}\u{274}\x03\
	\x02\x02\x02\u{273}\u{270}\x03\x02\x02\x02\u{274}\u{277}\x03\x02\x02\x02\
	\u{275}\u{273}\x03\x02\x02\x02\u{275}\u{276}\x03\x02\x02\x02\u{276}\u{282}\
	\x03\x02\x02\x02\u{277}\u{275}\x03\x02\x02\x02\u{278}\u{279}\x07\x6f\x02\
	\x02\u{279}\u{27a}\x07\x2a\x02\x02\u{27a}\u{27f}\x05\x5e\x30\x02\u{27b}\
	\u{27c}\x07\x07\x02\x02\u{27c}\u{27e}\x05\x5e\x30\x02\u{27d}\u{27b}\x03\
	\x02\x02\x02\u{27e}\u{281}\x03\x02\x02\x02\u{27f}\u{27d}\x03\x02\x02\x02\
	\u{27f}\u{280}\x03\x02\x02\x02\u{280}\u{283}\x03\x02\x02\x02\u{281}\u{27f}\
	\x03\x02\x02\x02\u{282}\u{278}\x03\x02\x02\x02\u{282}\u{283}\x03\x02\x02\
	\x02\u{283}\u{28a}\x03\x02\x02\x02\u{284}\u{285}\x07\x64\x02\x02\u{285}\
	\u{288}\x05\x50\x29\x02\u{286}\u{287}\x09\x04\x02\x02\u{287}\u{289}\x05\
	\x50\x29\x02\u{288}\u{286}\x03\x02\x02\x02\u{288}\u{289}\x03\x02\x02\x02\
	\u{289}\u{28b}\x03\x02\x02\x02\u{28a}\u{284}\x03\x02\x02\x02\u{28a}\u{28b}\
	\x03\x02\x02\x02\u{28b}\x2f\x03\x02\x02\x02\u{28c}\u{28e}\x05\x5a\x2e\x02\
	\u{28d}\u{28c}\x03\x02\x02\x02\u{28d}\u{28e}\x03\x02\x02\x02\u{28e}\u{2a0}\
	\x03\x02\x02\x02\u{28f}\u{2a1}\x07\x5a\x02\x02\u{290}\u{2a1}\x07\x7c\x02\
	\x02\u{291}\u{292}\x07\x5a\x02\x02\u{292}\u{293}\x07\x6e\x02\x02\u{293}\
	\u{2a1}\x07\x7c\x02\x02\u{294}\u{295}\x07\x5a\x02\x02\u{295}\u{296}\x07\
	\x6e\x02\x02\u{296}\u{2a1}\x07\x7f\x02\x02\u{297}\u{298}\x07\x5a\x02\x02\
	\u{298}\u{299}\x07\x6e\x02\x02\u{299}\u{2a1}\x07\x1b\x02\x02\u{29a}\u{29b}\
	\x07\x5a\x02\x02\u{29b}\u{29c}\x07\x6e\x02\x02\u{29c}\u{2a1}\x07\x4a\x02\
	\x02\u{29d}\u{29e}\x07\x5a\x02\x02\u{29e}\u{29f}\x07\x6e\x02\x02\u{29f}\
	\u{2a1}\x07\x53\x02\x02\u{2a0}\u{28f}\x03\x02\x02\x02\u{2a0}\u{290}\x03\
	\x02\x02\x02\u{2a0}\u{291}\x03\x02\x02\x02\u{2a0}\u{294}\x03\x02\x02\x02\
	\u{2a0}\u{297}\x03\x02\x02\x02\u{2a0}\u{29a}\x03\x02\x02\x02\u{2a0}\u{29d}\
	\x03\x02\x02\x02\u{2a1}\u{2a2}\x03\x02\x02\x02\u{2a2}\u{2a6}\x07\x5d\x02\
	\x02\u{2a3}\u{2a4}\x05\u{84}\x43\x02\u{2a4}\u{2a5}\x07\x04\x02\x02\u{2a5}\
	\u{2a7}\x03\x02\x02\x02\u{2a6}\u{2a3}\x03\x02\x02\x02\u{2a6}\u{2a7}\x03\
	\x02\x02\x02\u{2a7}\u{2a8}\x03\x02\x02\x02\u{2a8}\u{2b4}\x05\u{8a}\x46\x02\
	\u{2a9}\u{2aa}\x07\x05\x02\x02\u{2aa}\u{2af}\x05\u{90}\x49\x02\u{2ab}\u{2ac}\
	\x07\x07\x02\x02\u{2ac}\u{2ae}\x05\u{90}\x49\x02\u{2ad}\u{2ab}\x03\x02\x02\
	\x02\u{2ae}\u{2b1}\x03\x02\x02\x02\u{2af}\u{2ad}\x03\x02\x02\x02\u{2af}\
	\u{2b0}\x03\x02\x02\x02\u{2b0}\u{2b2}\x03\x02\x02\x02\u{2b1}\u{2af}\x03\
	\x02\x02\x02\u{2b2}\u{2b3}\x07\x06\x02\x02\u{2b3}\u{2b5}\x03\x02\x02\x02\
	\u{2b4}\u{2a9}\x03\x02\x02\x02\u{2b4}\u{2b5}\x03\x02\x02\x02\u{2b5}\u{2d5}\
	\x03\x02\x02\x02\u{2b6}\u{2b7}\x07\u{90}\x02\x02\u{2b7}\u{2b8}\x07\x05\x02\
	\x02\u{2b8}\u{2bd}\x05\x50\x29\x02\u{2b9}\u{2ba}\x07\x07\x02\x02\u{2ba}\
	\u{2bc}\x05\x50\x29\x02\u{2bb}\u{2b9}\x03\x02\x02\x02\u{2bc}\u{2bf}\x03\
	\x02\x02\x02\u{2bd}\u{2bb}\x03\x02\x02\x02\u{2bd}\u{2be}\x03\x02\x02\x02\
	\u{2be}\u{2c0}\x03\x02\x02\x02\u{2bf}\u{2bd}\x03\x02\x02\x02\u{2c0}\u{2cf}\
	\x07\x06\x02\x02\u{2c1}\u{2c2}\x07\x07\x02\x02\u{2c2}\u{2c3}\x07\x05\x02\
	\x02\u{2c3}\u{2c8}\x05\x50\x29\x02\u{2c4}\u{2c5}\x07\x07\x02\x02\u{2c5}\
	\u{2c7}\x05\x50\x29\x02\u{2c6}\u{2c4}\x03\x02\x02\x02\u{2c7}\u{2ca}\x03\
	\x02\x02\x02\u{2c8}\u{2c6}\x03\x02\x02\x02\u{2c8}\u{2c9}\x03\x02\x02\x02\
	\u{2c9}\u{2cb}\x03\x02\x02\x02\u{2ca}\u{2c8}\x03\x02\x02\x02\u{2cb}\u{2cc}\
	\x07\x06\x02\x02\u{2cc}\u{2ce}\x03\x02\x02\x02\u{2cd}\u{2c1}\x03\x02\x02\
	\x02\u{2ce}\u{2d1}\x03\x02\x02\x02\u{2cf}\u{2cd}\x03\x02\x02\x02\u{2cf}\
	\u{2d0}\x03\x02\x02\x02\u{2d0}\u{2d6}\x03\x02\x02\x02\u{2d1}\u{2cf}\x03\
	\x02\x02\x02\u{2d2}\u{2d6}\x05\x3e\x20\x02\u{2d3}\u{2d4}\x07\x3a\x02\x02\
	\u{2d4}\u{2d6}\x07\u{90}\x02\x02\u{2d5}\u{2b6}\x03\x02\x02\x02\u{2d5}\u{2d2}\
	\x03\x02\x02\x02\u{2d5}\u{2d3}\x03\x02\x02\x02\u{2d6}\x31\x03\x02\x02\x02\
	\u{2d7}\u{2db}\x07\x72\x02\x02\u{2d8}\u{2d9}\x05\u{84}\x43\x02\u{2d9}\u{2da}\
	\x07\x04\x02\x02\u{2da}\u{2dc}\x03\x02\x02\x02\u{2db}\u{2d8}\x03\x02\x02\
	\x02\u{2db}\u{2dc}\x03\x02\x02\x02\u{2dc}\u{2dd}\x03\x02\x02\x02\u{2dd}\
	\u{2e4}\x05\u{9e}\x50\x02\u{2de}\u{2df}\x07\x08\x02\x02\u{2df}\u{2e5}\x05\
	\x60\x31\x02\u{2e0}\u{2e1}\x07\x05\x02\x02\u{2e1}\u{2e2}\x05\x60\x31\x02\
	\u{2e2}\u{2e3}\x07\x06\x02\x02\u{2e3}\u{2e5}\x03\x02\x02\x02\u{2e4}\u{2de}\
	\x03\x02\x02\x02\u{2e4}\u{2e0}\x03\x02\x02\x02\u{2e4}\u{2e5}\x03\x02\x02\
	\x02\u{2e5}\x33\x03\x02\x02\x02\u{2e6}\u{2f1}\x07\x79\x02\x02\u{2e7}\u{2f2}\
	\x05\u{92}\x4a\x02\u{2e8}\u{2e9}\x05\u{84}\x43\x02\u{2e9}\u{2ea}\x07\x04\
	\x02\x02\u{2ea}\u{2ec}\x03\x02\x02\x02\u{2eb}\u{2e8}\x03\x02\x02\x02\u{2eb}\
	\u{2ec}\x03\x02\x02\x02\u{2ec}\u{2ef}\x03\x02\x02\x02\u{2ed}\u{2f0}\x05\
	\u{8a}\x46\x02\u{2ee}\u{2f0}\x05\u{96}\x4c\x02\u{2ef}\u{2ed}\x03\x02\x02\
	\x02\u{2ef}\u{2ee}\x03\x02\x02\x02\u{2f0}\u{2f2}\x03\x02\x02\x02\u{2f1}\
	\u{2e7}\x03\x02\x02\x02\u{2f1}\u{2eb}\x03\x02\x02\x02\u{2f1}\u{2f2}\x03\
	\x02\x02\x02\u{2f2}\x35\x03\x02\x02\x02\u{2f3}\u{2f5}\x07\x7a\x02\x02\u{2f4}\
	\u{2f6}\x07\u{81}\x02\x02\u{2f5}\u{2f4}\x03\x02\x02\x02\u{2f5}\u{2f6}\x03\
	\x02\x02\x02\u{2f6}\u{2f7}\x03\x02\x02\x02\u{2f7}\u{2f8}\x05\u{a0}\x51\x02\
	\u{2f8}\x37\x03\x02\x02\x02\u{2f9}\u{2fe}\x07\x7f\x02\x02\u{2fa}\u{2fc}\
	\x07\u{89}\x02\x02\u{2fb}\u{2fd}\x05\u{a4}\x53\x02\u{2fc}\u{2fb}\x03\x02\
	\x02\x02\u{2fc}\u{2fd}\x03\x02\x02\x02\u{2fd}\u{2ff}\x03\x02\x02\x02\u{2fe}\
	\u{2fa}\x03\x02\x02\x02\u{2fe}\u{2ff}\x03\x02\x02\x02\u{2ff}\u{305}\x03\
	\x02\x02\x02\u{300}\u{302}\x07\u{88}\x02\x02\u{301}\u{303}\x07\u{81}\x02\
	\x02\u{302}\u{301}\x03\x02\x02\x02\u{302}\u{303}\x03\x02\x02\x02\u{303}\
	\u{304}\x03\x02\x02\x02\u{304}\u{306}\x05\u{a0}\x51\x02\u{305}\u{300}\x03\
	\x02\x02\x02\u{305}\u{306}\x03\x02\x02\x02\u{306}\x39\x03\x02\x02\x02\u{307}\
	\u{308}\x07\u{81}\x02\x02\u{308}\u{309}\x05\u{a0}\x51\x02\u{309}\x3b\x03\
	\x02\x02\x02\u{30a}\u{30c}\x05\x5a\x2e\x02\u{30b}\u{30a}\x03\x02\x02\x02\
	\u{30b}\u{30c}\x03\x02\x02\x02\u{30c}\u{30d}\x03\x02\x02\x02\u{30d}\u{318}\
	\x05\x6e\x38\x02\u{30e}\u{30f}\x07\x6f\x02\x02\u{30f}\u{310}\x07\x2a\x02\
	\x02\u{310}\u{315}\x05\x5e\x30\x02\u{311}\u{312}\x07\x07\x02\x02\u{312}\
	\u{314}\x05\x5e\x30\x02\u{313}\u{311}\x03\x02\x02\x02\u{314}\u{317}\x03\
	\x02\x02\x02\u{315}\u{313}\x03\x02\x02\x02\u{315}\u{316}\x03\x02\x02\x02\
	\u{316}\u{319}\x03\x02\x02\x02\u{317}\u{315}\x03\x02\x02\x02\u{318}\u{30e}\
	\x03\x02\x02\x02\u{318}\u{319}\x03\x02\x02\x02\u{319}\u{320}\x03\x02\x02\
	\x02\u{31a}\u{31b}\x07\x64\x02\x02\u{31b}\u{31e}\x05\x50\x29\x02\u{31c}\
	\u{31d}\x09\x04\x02\x02\u{31d}\u{31f}\x05\x50\x29\x02\u{31e}\u{31c}\x03\
	\x02\x02\x02\u{31e}\u{31f}\x03\x02\x02\x02\u{31f}\u{321}\x03\x02\x02\x02\
	\u{320}\u{31a}\x03\x02\x02\x02\u{320}\u{321}\x03\x02\x02\x02\u{321}\x3d\
	\x03\x02\x02\x02\u{322}\u{324}\x05\x5a\x2e\x02\u{323}\u{322}\x03\x02\x02\
	\x02\u{323}\u{324}\x03\x02\x02\x02\u{324}\u{325}\x03\x02\x02\x02\u{325}\
	\u{32b}\x05\x40\x21\x02\u{326}\u{327}\x05\x70\x39\x02\u{327}\u{328}\x05\
	\x40\x21\x02\u{328}\u{32a}\x03\x02\x02\x02\u{329}\u{326}\x03\x02\x02\x02\
	\u{32a}\u{32d}\x03\x02\x02\x02\u{32b}\u{329}\x03\x02\x02\x02\u{32b}\u{32c}\
	\x03\x02\x02\x02\u{32c}\u{338}\x03\x02\x02\x02\u{32d}\u{32b}\x03\x02\x02\
	\x02\u{32e}\u{32f}\x07\x6f\x02\x02\u{32f}\u{330}\x07\x2a\x02\x02\u{330}\
	\u{335}\x05\x5e\x30\x02\u{331}\u{332}\x07\x07\x02\x02\u{332}\u{334}\x05\
	\x5e\x30\x02\u{333}\u{331}\x03\x02\x02\x02\u{334}\u{337}\x03\x02\x02\x02\
	\u{335}\u{333}\x03\x02\x02\x02\u{335}\u{336}\x03\x02\x02\x02\u{336}\u{339}\
	\x03\x02\x02\x02\u{337}\u{335}\x03\x02\x02\x02\u{338}\u{32e}\x03\x02\x02\
	\x02\u{338}\u{339}\x03\x02\x02\x02\u{339}\u{340}\x03\x02\x02\x02\u{33a}\
	\u{33b}\x07\x64\x02\x02\u{33b}\u{33e}\x05\x50\x29\x02\u{33c}\u{33d}\x09\
	\x04\x02\x02\u{33d}\u{33f}\x05\x50\x29\x02\u{33e}\u{33c}\x03\x02\x02\x02\
	\u{33e}\u{33f}\x03\x02\x02\x02\u{33f}\u{341}\x03\x02\x02\x02\u{340}\u{33a}\
	\x03\x02\x02\x02\u{340}\u{341}\x03\x02\x02\x02\u{341}\x3f\x03\x02\x02\x02\
	\u{342}\u{344}\x07\u{82}\x02\x02\u{343}\u{345}\x09\x06\x02\x02\u{344}\u{343}\
	\x03\x02\x02\x02\u{344}\u{345}\x03\x02\x02\x02\u{345}\u{346}\x03\x02\x02\
	\x02\u{346}\u{34b}\x05\x64\x33\x02\u{347}\u{348}\x07\x07\x02\x02\u{348}\
	\u{34a}\x05\x64\x33\x02\u{349}\u{347}\x03\x02\x02\x02\u{34a}\u{34d}\x03\
	\x02\x02\x02\u{34b}\u{349}\x03\x02\x02\x02\u{34b}\u{34c}\x03\x02\x02\x02\
	\u{34c}\u{35a}\x03\x02\x02\x02\u{34d}\u{34b}\x03\x02\x02\x02\u{34e}\u{358}\
	\x07\x4d\x02\x02\u{34f}\u{354}\x05\x66\x34\x02\u{350}\u{351}\x07\x07\x02\
	\x02\u{351}\u{353}\x05\x66\x34\x02\u{352}\u{350}\x03\x02\x02\x02\u{353}\
	\u{356}\x03\x02\x02\x02\u{354}\u{352}\x03\x02\x02\x02\u{354}\u{355}\x03\
	\x02\x02\x02\u{355}\u{359}\x03\x02\x02\x02\u{356}\u{354}\x03\x02\x02\x02\
	\u{357}\u{359}\x05\x68\x35\x02\u{358}\u{34f}\x03\x02\x02\x02\u{358}\u{357}\
	\x03\x02\x02\x02\u{359}\u{35b}\x03\x02\x02\x02\u{35a}\u{34e}\x03\x02\x02\
	\x02\u{35a}\u{35b}\x03\x02\x02\x02\u{35b}\u{35e}\x03\x02\x02\x02\u{35c}\
	\u{35d}\x07\u{94}\x02\x02\u{35d}\u{35f}\x05\x50\x29\x02\u{35e}\u{35c}\x03\
	\x02\x02\x02\u{35e}\u{35f}\x03\x02\x02\x02\u{35f}\u{36e}\x03\x02\x02\x02\
	\u{360}\u{361}\x07\x50\x02\x02\u{361}\u{362}\x07\x2a\x02\x02\u{362}\u{367}\
	\x05\x50\x29\x02\u{363}\u{364}\x07\x07\x02\x02\u{364}\u{366}\x05\x50\x29\
	\x02\u{365}\u{363}\x03\x02\x02\x02\u{366}\u{369}\x03\x02\x02\x02\u{367}\
	\u{365}\x03\x02\x02\x02\u{367}\u{368}\x03\x02\x02\x02\u{368}\u{36c}\x03\
	\x02\x02\x02\u{369}\u{367}\x03\x02\x02\x02\u{36a}\u{36b}\x07\x51\x02\x02\
	\u{36b}\u{36d}\x05\x50\x29\x02\u{36c}\u{36a}\x03\x02\x02\x02\u{36c}\u{36d}\
	\x03\x02\x02\x02\u{36d}\u{36f}\x03\x02\x02\x02\u{36e}\u{360}\x03\x02\x02\
	\x02\u{36e}\u{36f}\x03\x02\x02\x02\u{36f}\u{38d}\x03\x02\x02\x02\u{370}\
	\u{371}\x07\u{90}\x02\x02\u{371}\u{372}\x07\x05\x02\x02\u{372}\u{377}\x05\
	\x50\x29\x02\u{373}\u{374}\x07\x07\x02\x02\u{374}\u{376}\x05\x50\x29\x02\
	\u{375}\u{373}\x03\x02\x02\x02\u{376}\u{379}\x03\x02\x02\x02\u{377}\u{375}\
	\x03\x02\x02\x02\u{377}\u{378}\x03\x02\x02\x02\u{378}\u{37a}\x03\x02\x02\
	\x02\u{379}\u{377}\x03\x02\x02\x02\u{37a}\u{389}\x07\x06\x02\x02\u{37b}\
	\u{37c}\x07\x07\x02\x02\u{37c}\u{37d}\x07\x05\x02\x02\u{37d}\u{382}\x05\
	\x50\x29\x02\u{37e}\u{37f}\x07\x07\x02\x02\u{37f}\u{381}\x05\x50\x29\x02\
	\u{380}\u{37e}\x03\x02\x02\x02\u{381}\u{384}\x03\x02\x02\x02\u{382}\u{380}\
	\x03\x02\x02\x02\u{382}\u{383}\x03\x02\x02\x02\u{383}\u{385}\x03\x02\x02\
	\x02\u{384}\u{382}\x03\x02\x02\x02\u{385}\u{386}\x07\x06\x02\x02\u{386}\
	\u{388}\x03\x02\x02\x02\u{387}\u{37b}\x03\x02\x02\x02\u{388}\u{38b}\x03\
	\x02\x02\x02\u{389}\u{387}\x03\x02\x02\x02\u{389}\u{38a}\x03\x02\x02\x02\
	\u{38a}\u{38d}\x03\x02\x02\x02\u{38b}\u{389}\x03\x02\x02\x02\u{38c}\u{342}\
	\x03\x02\x02\x02\u{38c}\u{370}\x03\x02\x02\x02\u{38d}\x41\x03\x02\x02\x02\
	\u{38e}\u{390}\x05\x5a\x2e\x02\u{38f}\u{38e}\x03\x02\x02\x02\u{38f}\u{390}\
	\x03\x02\x02\x02\u{390}\u{391}\x03\x02\x02\x02\u{391}\u{39c}\x07\u{8d}\x02\
	\x02\u{392}\u{393}\x07\x6e\x02\x02\u{393}\u{39d}\x07\x7f\x02\x02\u{394}\
	\u{395}\x07\x6e\x02\x02\u{395}\u{39d}\x07\x1b\x02\x02\u{396}\u{397}\x07\
	\x6e\x02\x02\u{397}\u{39d}\x07\x7c\x02\x02\u{398}\u{399}\x07\x6e\x02\x02\
	\u{399}\u{39d}\x07\x4a\x02\x02\u{39a}\u{39b}\x07\x6e\x02\x02\u{39b}\u{39d}\
	\x07\x53\x02\x02\u{39c}\u{392}\x03\x02\x02\x02\u{39c}\u{394}\x03\x02\x02\
	\x02\u{39c}\u{396}\x03\x02\x02\x02\u{39c}\u{398}\x03\x02\x02\x02\u{39c}\
	\u{39a}\x03\x02\x02\x02\u{39c}\u{39d}\x03\x02\x02\x02\u{39d}\u{39e}\x03\
	\x02\x02\x02\u{39e}\u{39f}\x05\x5c\x2f\x02\u{39f}\u{3a0}\x07\u{83}\x02\x02\
	\u{3a0}\u{3a1}\x05\u{90}\x49\x02\u{3a1}\u{3a2}\x07\x08\x02\x02\u{3a2}\u{3aa}\
	\x05\x50\x29\x02\u{3a3}\u{3a4}\x07\x07\x02\x02\u{3a4}\u{3a5}\x05\u{90}\x49\
	\x02\u{3a5}\u{3a6}\x07\x08\x02\x02\u{3a6}\u{3a7}\x05\x50\x29\x02\u{3a7}\
	\u{3a9}\x03\x02\x02\x02\u{3a8}\u{3a3}\x03\x02\x02\x02\u{3a9}\u{3ac}\x03\
	\x02\x02\x02\u{3aa}\u{3a8}\x03\x02\x02\x02\u{3aa}\u{3ab}\x03\x02\x02\x02\
	\u{3ab}\u{3af}\x03\x02\x02\x02\u{3ac}\u{3aa}\x03\x02\x02\x02\u{3ad}\u{3ae}\
	\x07\u{94}\x02\x02\u{3ae}\u{3b0}\x05\x50\x29\x02\u{3af}\u{3ad}\x03\x02\x02\
	\x02\u{3af}\u{3b0}\x03\x02\x02\x02\u{3b0}\x43\x03\x02\x02\x02\u{3b1}\u{3b3}\
	\x05\x5a\x2e\x02\u{3b2}\u{3b1}\x03\x02\x02\x02\u{3b2}\u{3b3}\x03\x02\x02\
	\x02\u{3b3}\u{3b4}\x03\x02\x02\x02\u{3b4}\u{3bf}\x07\u{8d}\x02\x02\u{3b5}\
	\u{3b6}\x07\x6e\x02\x02\u{3b6}\u{3c0}\x07\x7f\x02\x02\u{3b7}\u{3b8}\x07\
	\x6e\x02\x02\u{3b8}\u{3c0}\x07\x1b\x02\x02\u{3b9}\u{3ba}\x07\x6e\x02\x02\
	\u{3ba}\u{3c0}\x07\x7c\x02\x02\u{3bb}\u{3bc}\x07\x6e\x02\x02\u{3bc}\u{3c0}\
	\x07\x4a\x02\x02\u{3bd}\u{3be}\x07\x6e\x02\x02\u{3be}\u{3c0}\x07\x53\x02\
	\x02\u{3bf}\u{3b5}\x03\x02\x02\x02\u{3bf}\u{3b7}\x03\x02\x02\x02\u{3bf}\
	\u{3b9}\x03\x02\x02\x02\u{3bf}\u{3bb}\x03\x02\x02\x02\u{3bf}\u{3bd}\x03\
	\x02\x02\x02\u{3bf}\u{3c0}\x03\x02\x02\x02\u{3c0}\u{3c1}\x03\x02\x02\x02\
	\u{3c1}\u{3c2}\x05\x5c\x2f\x02\u{3c2}\u{3c3}\x07\u{83}\x02\x02\u{3c3}\u{3c4}\
	\x05\u{90}\x49\x02\u{3c4}\u{3c5}\x07\x08\x02\x02\u{3c5}\u{3cd}\x05\x50\x29\
	\x02\u{3c6}\u{3c7}\x07\x07\x02\x02\u{3c7}\u{3c8}\x05\u{90}\x49\x02\u{3c8}\
	\u{3c9}\x07\x08\x02\x02\u{3c9}\u{3ca}\x05\x50\x29\x02\u{3ca}\u{3cc}\x03\
	\x02\x02\x02\u{3cb}\u{3c6}\x03\x02\x02\x02\u{3cc}\u{3cf}\x03\x02\x02\x02\
	\u{3cd}\u{3cb}\x03\x02\x02\x02\u{3cd}\u{3ce}\x03\x02\x02\x02\u{3ce}\u{3d2}\
	\x03\x02\x02\x02\u{3cf}\u{3cd}\x03\x02\x02\x02\u{3d0}\u{3d1}\x07\u{94}\x02\
	\x02\u{3d1}\u{3d3}\x05\x50\x29\x02\u{3d2}\u{3d0}\x03\x02\x02\x02\u{3d2}\
	\u{3d3}\x03\x02\x02\x02\u{3d3}\u{3e6}\x03\x02\x02\x02\u{3d4}\u{3d5}\x07\
	\x6f\x02\x02\u{3d5}\u{3d6}\x07\x2a\x02\x02\u{3d6}\u{3db}\x05\x5e\x30\x02\
	\u{3d7}\u{3d8}\x07\x07\x02\x02\u{3d8}\u{3da}\x05\x5e\x30\x02\u{3d9}\u{3d7}\
	\x03\x02\x02\x02\u{3da}\u{3dd}\x03\x02\x02\x02\u{3db}\u{3d9}\x03\x02\x02\
	\x02\u{3db}\u{3dc}\x03\x02\x02\x02\u{3dc}\u{3df}\x03\x02\x02\x02\u{3dd}\
	\u{3db}\x03\x02\x02\x02\u{3de}\u{3d4}\x03\x02\x02\x02\u{3de}\u{3df}\x03\
	\x02\x02\x02\u{3df}\u{3e0}\x03\x02\x02\x02\u{3e0}\u{3e1}\x07\x64\x02\x02\
	\u{3e1}\u{3e4}\x05\x50\x29\x02\u{3e2}\u{3e3}\x09\x04\x02\x02\u{3e3}\u{3e5}\
	\x05\x50\x29\x02\u{3e4}\u{3e2}\x03\x02\x02\x02\u{3e4}\u{3e5}\x03\x02\x02\
	\x02\u{3e5}\u{3e7}\x03\x02\x02\x02\u{3e6}\u{3de}\x03\x02\x02\x02\u{3e6}\
	\u{3e7}\x03\x02\x02\x02\u{3e7}\x45\x03\x02\x02\x02\u{3e8}\u{3e9}\x07\u{8f}\
	\x02\x02\u{3e9}\x47\x03\x02\x02\x02\u{3ea}\u{3ec}\x05\u{90}\x49\x02\u{3eb}\
	\u{3ed}\x05\x4a\x26\x02\u{3ec}\u{3eb}\x03\x02\x02\x02\u{3ec}\u{3ed}\x03\
	\x02\x02\x02\u{3ed}\u{3f1}\x03\x02\x02\x02\u{3ee}\u{3f0}\x05\x4c\x27\x02\
	\u{3ef}\u{3ee}\x03\x02\x02\x02\u{3f0}\u{3f3}\x03\x02\x02\x02\u{3f1}\u{3ef}\
	\x03\x02\x02\x02\u{3f1}\u{3f2}\x03\x02\x02\x02\u{3f2}\x49\x03\x02\x02\x02\
	\u{3f3}\u{3f1}\x03\x02\x02\x02\u{3f4}\u{3f6}\x05\u{80}\x41\x02\u{3f5}\u{3f4}\
	\x03\x02\x02\x02\u{3f6}\u{3f7}\x03\x02\x02\x02\u{3f7}\u{3f8}\x03\x02\x02\
	\x02\u{3f7}\u{3f5}\x03\x02\x02\x02\u{3f8}\u{403}\x03\x02\x02\x02\u{3f9}\
	\u{3fa}\x07\x05\x02\x02\u{3fa}\u{3fb}\x05\x72\x3a\x02\u{3fb}\u{3fc}\x07\
	\x06\x02\x02\u{3fc}\u{404}\x03\x02\x02\x02\u{3fd}\u{3fe}\x07\x05\x02\x02\
	\u{3fe}\u{3ff}\x05\x72\x3a\x02\u{3ff}\u{400}\x07\x07\x02\x02\u{400}\u{401}\
	\x05\x72\x3a\x02\u{401}\u{402}\x07\x06\x02\x02\u{402}\u{404}\x03\x02\x02\
	\x02\u{403}\u{3f9}\x03\x02\x02\x02\u{403}\u{3fd}\x03\x02\x02\x02\u{403}\
	\u{404}\x03\x02\x02\x02\u{404}\x4b\x03\x02\x02\x02\u{405}\u{406}\x07\x33\
	\x02\x02\u{406}\u{408}\x05\u{80}\x41\x02\u{407}\u{405}\x03\x02\x02\x02\u{407}\
	\u{408}\x03\x02\x02\x02\u{408}\u{42a}\x03\x02\x02\x02\u{409}\u{40a}\x07\
	\x73\x02\x02\u{40a}\u{40c}\x07\x61\x02\x02\u{40b}\u{40d}\x09\x07\x02\x02\
	\u{40c}\u{40b}\x03\x02\x02\x02\u{40c}\u{40d}\x03\x02\x02\x02\u{40d}\u{40e}\
	\x03\x02\x02\x02\u{40e}\u{410}\x05\x4e\x28\x02\u{40f}\u{411}\x07\x26\x02\
	\x02\u{410}\u{40f}\x03\x02\x02\x02\u{410}\u{411}\x03\x02\x02\x02\u{411}\
	\u{42b}\x03\x02\x02\x02\u{412}\u{414}\x07\x68\x02\x02\u{413}\u{412}\x03\
	\x02\x02\x02\u{413}\u{414}\x03\x02\x02\x02\u{414}\u{415}\x03\x02\x02\x02\
	\u{415}\u{416}\x07\x6a\x02\x02\u{416}\u{42b}\x05\x4e\x28\x02\u{417}\u{418}\
	\x07\u{8c}\x02\x02\u{418}\u{42b}\x05\x4e\x28\x02\u{419}\u{41a}\x07\x2e\x02\
	\x02\u{41a}\u{41b}\x07\x05\x02\x02\u{41b}\u{41c}\x05\x50\x29\x02\u{41c}\
	\u{41d}\x07\x06\x02\x02\u{41d}\u{42b}\x03\x02\x02\x02\u{41e}\u{425}\x07\
	\x3a\x02\x02\u{41f}\u{426}\x05\x72\x3a\x02\u{420}\u{426}\x05\x74\x3b\x02\
	\u{421}\u{422}\x07\x05\x02\x02\u{422}\u{423}\x05\x50\x29\x02\u{423}\u{424}\
	\x07\x06\x02\x02\u{424}\u{426}\x03\x02\x02\x02\u{425}\u{41f}\x03\x02\x02\
	\x02\u{425}\u{420}\x03\x02\x02\x02\u{425}\u{421}\x03\x02\x02\x02\u{426}\
	\u{42b}\x03\x02\x02\x02\u{427}\u{428}\x07\x2f\x02\x02\u{428}\u{42b}\x05\
	\u{92}\x4a\x02\u{429}\u{42b}\x05\x52\x2a\x02\u{42a}\u{409}\x03\x02\x02\x02\
	\u{42a}\u{413}\x03\x02\x02\x02\u{42a}\u{417}\x03\x02\x02\x02\u{42a}\u{419}\
	\x03\x02\x02\x02\u{42a}\u{41e}\x03\x02\x02\x02\u{42a}\u{427}\x03\x02\x02\
	\x02\u{42a}\u{429}\x03\x02\x02\x02\u{42b}\x4d\x03\x02\x02\x02\u{42c}\u{42d}\
	\x07\x6d\x02\x02\u{42d}\u{42e}\x07\x32\x02\x02\u{42e}\u{430}\x09\x08\x02\
	\x02\u{42f}\u{42c}\x03\x02\x02\x02\u{42f}\u{430}\x03\x02\x02\x02\u{430}\
	\x4f\x03\x02\x02\x02\u{431}\u{432}\x08\x29\x01\x02\u{432}\u{47e}\x05\x74\
	\x3b\x02\u{433}\u{47e}\x07\u{99}\x02\x02\u{434}\u{435}\x05\u{84}\x43\x02\
	\u{435}\u{436}\x07\x04\x02\x02\u{436}\u{438}\x03\x02\x02\x02\u{437}\u{434}\
	\x03\x02\x02\x02\u{437}\u{438}\x03\x02\x02\x02\u{438}\u{439}\x03\x02\x02\
	\x02\u{439}\u{43a}\x05\u{8a}\x46\x02\u{43a}\u{43b}\x07\x04\x02\x02\u{43b}\
	\u{43d}\x03\x02\x02\x02\u{43c}\u{437}\x03\x02\x02\x02\u{43c}\u{43d}\x03\
	\x02\x02\x02\u{43d}\u{43e}\x03\x02\x02\x02\u{43e}\u{47e}\x05\u{90}\x49\x02\
	\u{43f}\u{440}\x05\x76\x3c\x02\u{440}\u{441}\x05\x50\x29\x17\u{441}\u{47e}\
	\x03\x02\x02\x02\u{442}\u{443}\x05\u{82}\x42\x02\u{443}\u{450}\x07\x05\x02\
	\x02\u{444}\u{446}\x07\x40\x02\x02\u{445}\u{444}\x03\x02\x02\x02\u{445}\
	\u{446}\x03\x02\x02\x02\u{446}\u{447}\x03\x02\x02\x02\u{447}\u{44c}\x05\
	\x50\x29\x02\u{448}\u{449}\x07\x07\x02\x02\u{449}\u{44b}\x05\x50\x29\x02\
	\u{44a}\u{448}\x03\x02\x02\x02\u{44b}\u{44e}\x03\x02\x02\x02\u{44c}\u{44a}\
	\x03\x02\x02\x02\u{44c}\u{44d}\x03\x02\x02\x02\u{44d}\u{451}\x03\x02\x02\
	\x02\u{44e}\u{44c}\x03\x02\x02\x02\u{44f}\u{451}\x07\x09\x02\x02\u{450}\
	\u{445}\x03\x02\x02\x02\u{450}\u{44f}\x03\x02\x02\x02\u{450}\u{451}\x03\
	\x02\x02\x02\u{451}\u{452}\x03\x02\x02\x02\u{452}\u{453}\x07\x06\x02\x02\
	\u{453}\u{47e}\x03\x02\x02\x02\u{454}\u{455}\x07\x05\x02\x02\u{455}\u{456}\
	\x05\x50\x29\x02\u{456}\u{457}\x07\x06\x02\x02\u{457}\u{47e}\x03\x02\x02\
	\x02\u{458}\u{459}\x07\x2d\x02\x02\u{459}\u{45a}\x07\x05\x02\x02\u{45a}\
	\u{45b}\x05\x50\x29\x02\u{45b}\u{45c}\x07\x23\x02\x02\u{45c}\u{45d}\x05\
	\x4a\x26\x02\u{45d}\u{45e}\x07\x06\x02\x02\u{45e}\u{47e}\x03\x02\x02\x02\
	\u{45f}\u{461}\x07\x68\x02\x02\u{460}\u{45f}\x03\x02\x02\x02\u{460}\u{461}\
	\x03\x02\x02\x02\u{461}\u{462}\x03\x02\x02\x02\u{462}\u{464}\x07\x48\x02\
	\x02\u{463}\u{460}\x03\x02\x02\x02\u{463}\u{464}\x03\x02\x02\x02\u{464}\
	\u{465}\x03\x02\x02\x02\u{465}\u{466}\x07\x05\x02\x02\u{466}\u{467}\x05\
	\x3e\x20\x02\u{467}\u{468}\x07\x06\x02\x02\u{468}\u{47e}\x03\x02\x02\x02\
	\u{469}\u{46b}\x07\x2c\x02\x02\u{46a}\u{46c}\x05\x50\x29\x02\u{46b}\u{46a}\
	\x03\x02\x02\x02\u{46b}\u{46c}\x03\x02\x02\x02\u{46c}\u{472}\x03\x02\x02\
	\x02\u{46d}\u{46e}\x07\u{93}\x02\x02\u{46e}\u{46f}\x05\x50\x29\x02\u{46f}\
	\u{470}\x07\u{87}\x02\x02\u{470}\u{471}\x05\x50\x29\x02\u{471}\u{473}\x03\
	\x02\x02\x02\u{472}\u{46d}\x03\x02\x02\x02\u{473}\u{474}\x03\x02\x02\x02\
	\u{474}\u{472}\x03\x02\x02\x02\u{474}\u{475}\x03\x02\x02\x02\u{475}\u{478}\
	\x03\x02\x02\x02\u{476}\u{477}\x07\x43\x02\x02\u{477}\u{479}\x05\x50\x29\
	\x02\u{478}\u{476}\x03\x02\x02\x02\u{478}\u{479}\x03\x02\x02\x02\u{479}\
	\u{47a}\x03\x02\x02\x02\u{47a}\u{47b}\x07\x44\x02\x02\u{47b}\u{47e}\x03\
	\x02\x02\x02\u{47c}\u{47e}\x05\x54\x2b\x02\u{47d}\u{431}\x03\x02\x02\x02\
	\u{47d}\u{433}\x03\x02\x02\x02\u{47d}\u{43c}\x03\x02\x02\x02\u{47d}\u{43f}\
	\x03\x02\x02\x02\u{47d}\u{442}\x03\x02\x02\x02\u{47d}\u{454}\x03\x02\x02\
	\x02\u{47d}\u{458}\x03\x02\x02\x02\u{47d}\u{463}\x03\x02\x02\x02\u{47d}\
	\u{469}\x03\x02\x02\x02\u{47d}\u{47c}\x03\x02\x02\x02\u{47e}\u{4d6}\x03\
	\x02\x02\x02\u{47f}\u{480}\x0c\x16\x02\x02\u{480}\u{481}\x07\x0d\x02\x02\
	\u{481}\u{4d5}\x05\x50\x29\x17\u{482}\u{483}\x0c\x15\x02\x02\u{483}\u{484}\
	\x09\x09\x02\x02\u{484}\u{4d5}\x05\x50\x29\x16\u{485}\u{486}\x0c\x14\x02\
	\x02\u{486}\u{487}\x09\x0a\x02\x02\u{487}\u{4d5}\x05\x50\x29\x15\u{488}\
	\u{489}\x0c\x13\x02\x02\u{489}\u{48a}\x09\x0b\x02\x02\u{48a}\u{4d5}\x05\
	\x50\x29\x14\u{48b}\u{48c}\x0c\x12\x02\x02\u{48c}\u{48d}\x09\x0c\x02\x02\
	\u{48d}\u{4d5}\x05\x50\x29\x13\u{48e}\u{48f}\x0c\x11\x02\x02\u{48f}\u{490}\
	\x09\x0d\x02\x02\u{490}\u{4d5}\x05\x50\x29\x12\u{491}\u{492}\x0c\x0f\x02\
	\x02\u{492}\u{493}\x07\x22\x02\x02\u{493}\u{4d5}\x05\x50\x29\x10\u{494}\
	\u{495}\x0c\x0e\x02\x02\u{495}\u{496}\x07\x6e\x02\x02\u{496}\u{4d5}\x05\
	\x50\x29\x0f\u{497}\u{498}\x0c\x07\x02\x02\u{498}\u{49a}\x07\x5e\x02\x02\
	\u{499}\u{49b}\x07\x68\x02\x02\u{49a}\u{499}\x03\x02\x02\x02\u{49a}\u{49b}\
	\x03\x02\x02\x02\u{49b}\u{49c}\x03\x02\x02\x02\u{49c}\u{4d5}\x05\x50\x29\
	\x08\u{49d}\u{49f}\x0c\x06\x02\x02\u{49e}\u{4a0}\x07\x68\x02\x02\u{49f}\
	\u{49e}\x03\x02\x02\x02\u{49f}\u{4a0}\x03\x02\x02\x02\u{4a0}\u{4a1}\x03\
	\x02\x02\x02\u{4a1}\u{4a2}\x07\x29\x02\x02\u{4a2}\u{4a3}\x05\x50\x29\x02\
	\u{4a3}\u{4a4}\x07\x22\x02\x02\u{4a4}\u{4a5}\x05\x50\x29\x07\u{4a5}\u{4d5}\
	\x03\x02\x02\x02\u{4a6}\u{4a8}\x0c\x10\x02\x02\u{4a7}\u{4a9}\x07\x68\x02\
	\x02\u{4a8}\u{4a7}\x03\x02\x02\x02\u{4a8}\u{4a9}\x03\x02\x02\x02\u{4a9}\
	\u{4aa}\x03\x02\x02\x02\u{4aa}\u{4be}\x07\x55\x02\x02\u{4ab}\u{4b5}\x07\
	\x05\x02\x02\u{4ac}\u{4b6}\x05\x3e\x20\x02\u{4ad}\u{4b2}\x05\x50\x29\x02\
	\u{4ae}\u{4af}\x07\x07\x02\x02\u{4af}\u{4b1}\x05\x50\x29\x02\u{4b0}\u{4ae}\
	\x03\x02\x02\x02\u{4b1}\u{4b4}\x03\x02\x02\x02\u{4b2}\u{4b0}\x03\x02\x02\
	\x02\u{4b2}\u{4b3}\x03\x02\x02\x02\u{4b3}\u{4b6}\x03\x02\x02\x02\u{4b4}\
	\u{4b2}\x03\x02\x02\x02\u{4b5}\u{4ac}\x03\x02\x02\x02\u{4b5}\u{4ad}\x03\
	\x02\x02\x02\u{4b5}\u{4b6}\x03\x02\x02\x02\u{4b6}\u{4b7}\x03\x02\x02\x02\
	\u{4b7}\u{4bf}\x07\x06\x02\x02\u{4b8}\u{4b9}\x05\u{84}\x43\x02\u{4b9}\u{4ba}\
	\x07\x04\x02\x02\u{4ba}\u{4bc}\x03\x02\x02\x02\u{4bb}\u{4b8}\x03\x02\x02\
	\x02\u{4bb}\u{4bc}\x03\x02\x02\x02\u{4bc}\u{4bd}\x03\x02\x02\x02\u{4bd}\
	\u{4bf}\x05\u{8a}\x46\x02\u{4be}\u{4ab}\x03\x02\x02\x02\u{4be}\u{4bb}\x03\
	\x02\x02\x02\u{4bf}\u{4d5}\x03\x02\x02\x02\u{4c0}\u{4c1}\x0c\x0a\x02\x02\
	\u{4c1}\u{4c2}\x07\x2f\x02\x02\u{4c2}\u{4d5}\x05\u{92}\x4a\x02\u{4c3}\u{4c5}\
	\x0c\x09\x02\x02\u{4c4}\u{4c6}\x07\x68\x02\x02\u{4c5}\u{4c4}\x03\x02\x02\
	\x02\u{4c5}\u{4c6}\x03\x02\x02\x02\u{4c6}\u{4c7}\x03\x02\x02\x02\u{4c7}\
	\u{4c8}\x09\x0e\x02\x02\u{4c8}\u{4cb}\x05\x50\x29\x02\u{4c9}\u{4ca}\x07\
	\x45\x02\x02\u{4ca}\u{4cc}\x05\x50\x29\x02\u{4cb}\u{4c9}\x03\x02\x02\x02\
	\u{4cb}\u{4cc}\x03\x02\x02\x02\u{4cc}\u{4d5}\x03\x02\x02\x02\u{4cd}\u{4d2}\
	\x0c\x08\x02\x02\u{4ce}\u{4d3}\x07\x5f\x02\x02\u{4cf}\u{4d3}\x07\x69\x02\
	\x02\u{4d0}\u{4d1}\x07\x68\x02\x02\u{4d1}\u{4d3}\x07\x6a\x02\x02\u{4d2}\
	\u{4ce}\x03\x02\x02\x02\u{4d2}\u{4cf}\x03\x02\x02\x02\u{4d2}\u{4d0}\x03\
	\x02\x02\x02\u{4d3}\u{4d5}\x03\x02\x02\x02\u{4d4}\u{47f}\x03\x02\x02\x02\
	\u{4d4}\u{482}\x03\x02\x02\x02\u{4d4}\u{485}\x03\x02\x02\x02\u{4d4}\u{488}\
	\x03\x02\x02\x02\u{4d4}\u{48b}\x03\x02\x02\x02\u{4d4}\u{48e}\x03\x02\x02\
	\x02\u{4d4}\u{491}\x03\x02\x02\x02\u{4d4}\u{494}\x03\x02\x02\x02\u{4d4}\
	\u{497}\x03\x02\x02\x02\u{4d4}\u{49d}\x03\x02\x02\x02\u{4d4}\u{4a6}\x03\
	\x02\x02\x02\u{4d4}\u{4c0}\x03\x02\x02\x02\u{4d4}\u{4c3}\x03\x02\x02\x02\
	\u{4d4}\u{4cd}\x03\x02\x02\x02\u{4d5}\u{4d8}\x03\x02\x02\x02\u{4d6}\u{4d4}\
	\x03\x02\x02\x02\u{4d6}\u{4d7}\x03\x02\x02\x02\u{4d7}\x51\x03\x02\x02\x02\
	\u{4d8}\u{4d6}\x03\x02\x02\x02\u{4d9}\u{4da}\x07\x77\x02\x02\u{4da}\u{4e6}\
	\x05\u{94}\x4b\x02\u{4db}\u{4dc}\x07\x05\x02\x02\u{4dc}\u{4e1}\x05\u{90}\
	\x49\x02\u{4dd}\u{4de}\x07\x07\x02\x02\u{4de}\u{4e0}\x05\u{90}\x49\x02\u{4df}\
	\u{4dd}\x03\x02\x02\x02\u{4e0}\u{4e3}\x03\x02\x02\x02\u{4e1}\u{4df}\x03\
	\x02\x02\x02\u{4e1}\u{4e2}\x03\x02\x02\x02\u{4e2}\u{4e4}\x03\x02\x02\x02\
	\u{4e3}\u{4e1}\x03\x02\x02\x02\u{4e4}\u{4e5}\x07\x06\x02\x02\u{4e5}\u{4e7}\
	\x03\x02\x02\x02\u{4e6}\u{4db}\x03\x02\x02\x02\u{4e6}\u{4e7}\x03\x02\x02\
	\x02\u{4e7}\u{4fa}\x03\x02\x02\x02\u{4e8}\u{4e9}\x07\x6d\x02\x02\u{4e9}\
	\u{4f2}\x09\x0f\x02\x02\u{4ea}\u{4eb}\x07\u{83}\x02\x02\u{4eb}\u{4f3}\x07\
	\x6a\x02\x02\u{4ec}\u{4ed}\x07\u{83}\x02\x02\u{4ed}\u{4f3}\x07\x3a\x02\x02\
	\u{4ee}\u{4f3}\x07\x2b\x02\x02\u{4ef}\u{4f3}\x07\x7d\x02\x02\u{4f0}\u{4f1}\
	\x07\x67\x02\x02\u{4f1}\u{4f3}\x07\x1c\x02\x02\u{4f2}\u{4ea}\x03\x02\x02\
	\x02\u{4f2}\u{4ec}\x03\x02\x02\x02\u{4f2}\u{4ee}\x03\x02\x02\x02\u{4f2}\
	\u{4ef}\x03\x02\x02\x02\u{4f2}\u{4f0}\x03\x02\x02\x02\u{4f3}\u{4f7}\x03\
	\x02\x02\x02\u{4f4}\u{4f5}\x07\x65\x02\x02\u{4f5}\u{4f7}\x05\u{80}\x41\x02\
	\u{4f6}\u{4e8}\x03\x02\x02\x02\u{4f6}\u{4f4}\x03\x02\x02\x02\u{4f7}\u{4f9}\
	\x03\x02\x02\x02\u{4f8}\u{4f6}\x03\x02\x02\x02\u{4f9}\u{4fc}\x03\x02\x02\
	\x02\u{4fa}\u{4f8}\x03\x02\x02\x02\u{4fa}\u{4fb}\x03\x02\x02\x02\u{4fb}\
	\u{507}\x03\x02\x02\x02\u{4fc}\u{4fa}\x03\x02\x02\x02\u{4fd}\u{4ff}\x07\
	\x68\x02\x02\u{4fe}\u{4fd}\x03\x02\x02\x02\u{4fe}\u{4ff}\x03\x02\x02\x02\
	\u{4ff}\u{500}\x03\x02\x02\x02\u{500}\u{505}\x07\x3b\x02\x02\u{501}\u{502}\
	\x07\x58\x02\x02\u{502}\u{506}\x07\x3c\x02\x02\u{503}\u{504}\x07\x58\x02\
	\x02\u{504}\u{506}\x07\x54\x02\x02\u{505}\u{501}\x03\x02\x02\x02\u{505}\
	\u{503}\x03\x02\x02\x02\u{505}\u{506}\x03\x02\x02\x02\u{506}\u{508}\x03\
	\x02\x02\x02\u{507}\u{4fe}\x03\x02\x02\x02\u{507}\u{508}\x03\x02\x02\x02\
	\u{508}\x53\x03\x02\x02\x02\u{509}\u{50a}\x07\x75\x02\x02\u{50a}\u{50f}\
	\x07\x05\x02\x02\u{50b}\u{510}\x07\x53\x02\x02\u{50c}\u{50d}\x09\x10\x02\
	\x02\u{50d}\u{50e}\x07\x07\x02\x02\u{50e}\u{510}\x05\x78\x3d\x02\u{50f}\
	\u{50b}\x03\x02\x02\x02\u{50f}\u{50c}\x03\x02\x02\x02\u{510}\u{511}\x03\
	\x02\x02\x02\u{511}\u{512}\x07\x06\x02\x02\u{512}\x55\x03\x02\x02\x02\u{513}\
	\u{516}\x05\u{90}\x49\x02\u{514}\u{515}\x07\x2f\x02\x02\u{515}\u{517}\x05\
	\u{92}\x4a\x02\u{516}\u{514}\x03\x02\x02\x02\u{516}\u{517}\x03\x02\x02\x02\
	\u{517}\u{519}\x03\x02\x02\x02\u{518}\u{51a}\x09\x07\x02\x02\u{519}\u{518}\
	\x03\x02\x02\x02\u{519}\u{51a}\x03\x02\x02\x02\u{51a}\x57\x03\x02\x02\x02\
	\u{51b}\u{51c}\x07\x33\x02\x02\u{51c}\u{51e}\x05\u{80}\x41\x02\u{51d}\u{51b}\
	\x03\x02\x02\x02\u{51d}\u{51e}\x03\x02\x02\x02\u{51e}\u{543}\x03\x02\x02\
	\x02\u{51f}\u{520}\x07\x73\x02\x02\u{520}\u{523}\x07\x61\x02\x02\u{521}\
	\u{523}\x07\u{8c}\x02\x02\u{522}\u{51f}\x03\x02\x02\x02\u{522}\u{521}\x03\
	\x02\x02\x02\u{523}\u{524}\x03\x02\x02\x02\u{524}\u{525}\x07\x05\x02\x02\
	\u{525}\u{52a}\x05\x56\x2c\x02\u{526}\u{527}\x07\x07\x02\x02\u{527}\u{529}\
	\x05\x56\x2c\x02\u{528}\u{526}\x03\x02\x02\x02\u{529}\u{52c}\x03\x02\x02\
	\x02\u{52a}\u{528}\x03\x02\x02\x02\u{52a}\u{52b}\x03\x02\x02\x02\u{52b}\
	\u{52d}\x03\x02\x02\x02\u{52c}\u{52a}\x03\x02\x02\x02\u{52d}\u{52e}\x07\
	\x06\x02\x02\u{52e}\u{52f}\x05\x4e\x28\x02\u{52f}\u{544}\x03\x02\x02\x02\
	\u{530}\u{531}\x07\x2e\x02\x02\u{531}\u{532}\x07\x05\x02\x02\u{532}\u{533}\
	\x05\x50\x29\x02\u{533}\u{534}\x07\x06\x02\x02\u{534}\u{544}\x03\x02\x02\
	\x02\u{535}\u{536}\x07\x4c\x02\x02\u{536}\u{537}\x07\x61\x02\x02\u{537}\
	\u{538}\x07\x05\x02\x02\u{538}\u{53d}\x05\u{90}\x49\x02\u{539}\u{53a}\x07\
	\x07\x02\x02\u{53a}\u{53c}\x05\u{90}\x49\x02\u{53b}\u{539}\x03\x02\x02\x02\
	\u{53c}\u{53f}\x03\x02\x02\x02\u{53d}\u{53b}\x03\x02\x02\x02\u{53d}\u{53e}\
	\x03\x02\x02\x02\u{53e}\u{540}\x03\x02\x02\x02\u{53f}\u{53d}\x03\x02\x02\
	\x02\u{540}\u{541}\x07\x06\x02\x02\u{541}\u{542}\x05\x52\x2a\x02\u{542}\
	\u{544}\x03\x02\x02\x02\u{543}\u{522}\x03\x02\x02\x02\u{543}\u{530}\x03\
	\x02\x02\x02\u{543}\u{535}\x03\x02\x02\x02\u{544}\x59\x03\x02\x02\x02\u{545}\
	\u{547}\x07\u{95}\x02\x02\u{546}\u{548}\x07\x76\x02\x02\u{547}\u{546}\x03\
	\x02\x02\x02\u{547}\u{548}\x03\x02\x02\x02\u{548}\u{549}\x03\x02\x02\x02\
	\u{549}\u{54e}\x05\x62\x32\x02\u{54a}\u{54b}\x07\x07\x02\x02\u{54b}\u{54d}\
	\x05\x62\x32\x02\u{54c}\u{54a}\x03\x02\x02\x02\u{54d}\u{550}\x03\x02\x02\
	\x02\u{54e}\u{54c}\x03\x02\x02\x02\u{54e}\u{54f}\x03\x02\x02\x02\u{54f}\
	\x5b\x03\x02\x02\x02\u{550}\u{54e}\x03\x02\x02\x02\u{551}\u{552}\x05\u{84}\
	\x43\x02\u{552}\u{553}\x07\x04\x02\x02\u{553}\u{555}\x03\x02\x02\x02\u{554}\
	\u{551}\x03\x02\x02\x02\u{554}\u{555}\x03\x02\x02\x02\u{555}\u{556}\x03\
	\x02\x02\x02\u{556}\u{55c}\x05\u{8a}\x46\x02\u{557}\u{558}\x07\x57\x02\x02\
	\u{558}\u{559}\x07\x2a\x02\x02\u{559}\u{55d}\x05\u{96}\x4c\x02\u{55a}\u{55b}\
	\x07\x68\x02\x02\u{55b}\u{55d}\x07\x57\x02\x02\u{55c}\u{557}\x03\x02\x02\
	\x02\u{55c}\u{55a}\x03\x02\x02\x02\u{55c}\u{55d}\x03\x02\x02\x02\u{55d}\
	\x5d\x03\x02\x02\x02\u{55e}\u{561}\x05\x50\x29\x02\u{55f}\u{560}\x07\x2f\
	\x02\x02\u{560}\u{562}\x05\u{92}\x4a\x02\u{561}\u{55f}\x03\x02\x02\x02\u{561}\
	\u{562}\x03\x02\x02\x02\u{562}\u{564}\x03\x02\x02\x02\u{563}\u{565}\x09\
	\x07\x02\x02\u{564}\u{563}\x03\x02\x02\x02\u{564}\u{565}\x03\x02\x02\x02\
	\u{565}\x5f\x03\x02\x02\x02\u{566}\u{56a}\x05\x72\x3a\x02\u{567}\u{56a}\
	\x05\u{80}\x41\x02\u{568}\u{56a}\x07\u{9a}\x02\x02\u{569}\u{566}\x03\x02\
	\x02\x02\u{569}\u{567}\x03\x02\x02\x02\u{569}\u{568}\x03\x02\x02\x02\u{56a}\
	\x61\x03\x02\x02\x02\u{56b}\u{577}\x05\u{8a}\x46\x02\u{56c}\u{56d}\x07\x05\
	\x02\x02\u{56d}\u{572}\x05\u{90}\x49\x02\u{56e}\u{56f}\x07\x07\x02\x02\u{56f}\
	\u{571}\x05\u{90}\x49\x02\u{570}\u{56e}\x03\x02\x02\x02\u{571}\u{574}\x03\
	\x02\x02\x02\u{572}\u{570}\x03\x02\x02\x02\u{572}\u{573}\x03\x02\x02\x02\
	\u{573}\u{575}\x03\x02\x02\x02\u{574}\u{572}\x03\x02\x02\x02\u{575}\u{576}\
	\x07\x06\x02\x02\u{576}\u{578}\x03\x02\x02\x02\u{577}\u{56c}\x03\x02\x02\
	\x02\u{577}\u{578}\x03\x02\x02\x02\u{578}\u{579}\x03\x02\x02\x02\u{579}\
	\u{57a}\x07\x23\x02\x02\u{57a}\u{57b}\x07\x05\x02\x02\u{57b}\u{57c}\x05\
	\x3e\x20\x02\u{57c}\u{57d}\x07\x06\x02\x02\u{57d}\x63\x03\x02\x02\x02\u{57e}\
	\u{58b}\x07\x09\x02\x02\u{57f}\u{580}\x05\u{8a}\x46\x02\u{580}\u{581}\x07\
	\x04\x02\x02\u{581}\u{582}\x07\x09\x02\x02\u{582}\u{58b}\x03\x02\x02\x02\
	\u{583}\u{588}\x05\x50\x29\x02\u{584}\u{586}\x07\x23\x02\x02\u{585}\u{584}\
	\x03\x02\x02\x02\u{585}\u{586}\x03\x02\x02\x02\u{586}\u{587}\x03\x02\x02\
	\x02\u{587}\u{589}\x05\x7c\x3f\x02\u{588}\u{585}\x03\x02\x02\x02\u{588}\
	\u{589}\x03\x02\x02\x02\u{589}\u{58b}\x03\x02\x02\x02\u{58a}\u{57e}\x03\
	\x02\x02\x02\u{58a}\u{57f}\x03\x02\x02\x02\u{58a}\u{583}\x03\x02\x02\x02\
	\u{58b}\x65\x03\x02\x02\x02\u{58c}\u{58d}\x05\u{86}\x44\x02\u{58d}\u{58e}\
	\x07\x04\x02\x02\u{58e}\u{590}\x03\x02\x02\x02\u{58f}\u{58c}\x03\x02\x02\
	\x02\u{58f}\u{590}\x03\x02\x02\x02\u{590}\u{591}\x03\x02\x02\x02\u{591}\
	\u{596}\x05\u{8a}\x46\x02\u{592}\u{594}\x07\x23\x02\x02\u{593}\u{592}\x03\
	\x02\x02\x02\u{593}\u{594}\x03\x02\x02\x02\u{594}\u{595}\x03\x02\x02\x02\
	\u{595}\u{597}\x05\u{a2}\x52\x02\u{596}\u{593}\x03\x02\x02\x02\u{596}\u{597}\
	\x03\x02\x02\x02\u{597}\u{59d}\x03\x02\x02\x02\u{598}\u{599}\x07\x57\x02\
	\x02\u{599}\u{59a}\x07\x2a\x02\x02\u{59a}\u{59e}\x05\u{96}\x4c\x02\u{59b}\
	\u{59c}\x07\x68\x02\x02\u{59c}\u{59e}\x07\x57\x02\x02\u{59d}\u{598}\x03\
	\x02\x02\x02\u{59d}\u{59b}\x03\x02\x02\x02\u{59d}\u{59e}\x03\x02\x02\x02\
	\u{59e}\u{5cf}\x03\x02\x02\x02\u{59f}\u{5a0}\x05\u{86}\x44\x02\u{5a0}\u{5a1}\
	\x07\x04\x02\x02\u{5a1}\u{5a3}\x03\x02\x02\x02\u{5a2}\u{59f}\x03\x02\x02\
	\x02\u{5a2}\u{5a3}\x03\x02\x02\x02\u{5a3}\u{5a4}\x03\x02\x02\x02\u{5a4}\
	\u{5a5}\x05\u{88}\x45\x02\u{5a5}\u{5ae}\x07\x05\x02\x02\u{5a6}\u{5ab}\x05\
	\x50\x29\x02\u{5a7}\u{5a8}\x07\x07\x02\x02\u{5a8}\u{5aa}\x05\x50\x29\x02\
	\u{5a9}\u{5a7}\x03\x02\x02\x02\u{5aa}\u{5ad}\x03\x02\x02\x02\u{5ab}\u{5a9}\
	\x03\x02\x02\x02\u{5ab}\u{5ac}\x03\x02\x02\x02\u{5ac}\u{5af}\x03\x02\x02\
	\x02\u{5ad}\u{5ab}\x03\x02\x02\x02\u{5ae}\u{5a6}\x03\x02\x02\x02\u{5ae}\
	\u{5af}\x03\x02\x02\x02\u{5af}\u{5b0}\x03\x02\x02\x02\u{5b0}\u{5b5}\x07\
	\x06\x02\x02\u{5b1}\u{5b3}\x07\x23\x02\x02\u{5b2}\u{5b1}\x03\x02\x02\x02\
	\u{5b2}\u{5b3}\x03\x02\x02\x02\u{5b3}\u{5b4}\x03\x02\x02\x02\u{5b4}\u{5b6}\
	\x05\u{a2}\x52\x02\u{5b5}\u{5b2}\x03\x02\x02\x02\u{5b5}\u{5b6}\x03\x02\x02\
	\x02\u{5b6}\u{5cf}\x03\x02\x02\x02\u{5b7}\u{5c1}\x07\x05\x02\x02\u{5b8}\
	\u{5bd}\x05\x66\x34\x02\u{5b9}\u{5ba}\x07\x07\x02\x02\u{5ba}\u{5bc}\x05\
	\x66\x34\x02\u{5bb}\u{5b9}\x03\x02\x02\x02\u{5bc}\u{5bf}\x03\x02\x02\x02\
	\u{5bd}\u{5bb}\x03\x02\x02\x02\u{5bd}\u{5be}\x03\x02\x02\x02\u{5be}\u{5c2}\
	\x03\x02\x02\x02\u{5bf}\u{5bd}\x03\x02\x02\x02\u{5c0}\u{5c2}\x05\x68\x35\
	\x02\u{5c1}\u{5b8}\x03\x02\x02\x02\u{5c1}\u{5c0}\x03\x02\x02\x02\u{5c2}\
	\u{5c3}\x03\x02\x02\x02\u{5c3}\u{5c4}\x07\x06\x02\x02\u{5c4}\u{5cf}\x03\
	\x02\x02\x02\u{5c5}\u{5c6}\x07\x05\x02\x02\u{5c6}\u{5c7}\x05\x3e\x20\x02\
	\u{5c7}\u{5cc}\x07\x06\x02\x02\u{5c8}\u{5ca}\x07\x23\x02\x02\u{5c9}\u{5c8}\
	\x03\x02\x02\x02\u{5c9}\u{5ca}\x03\x02\x02\x02\u{5ca}\u{5cb}\x03\x02\x02\
	\x02\u{5cb}\u{5cd}\x05\u{a2}\x52\x02\u{5cc}\u{5c9}\x03\x02\x02\x02\u{5cc}\
	\u{5cd}\x03\x02\x02\x02\u{5cd}\u{5cf}\x03\x02\x02\x02\u{5ce}\u{58f}\x03\
	\x02\x02\x02\u{5ce}\u{5a2}\x03\x02\x02\x02\u{5ce}\u{5b7}\x03\x02\x02\x02\
	\u{5ce}\u{5c5}\x03\x02\x02\x02\u{5cf}\x67\x03\x02\x02\x02\u{5d0}\u{5d7}\
	\x05\x66\x34\x02\u{5d1}\u{5d2}\x05\x6a\x36\x02\u{5d2}\u{5d3}\x05\x66\x34\
	\x02\u{5d3}\u{5d4}\x05\x6c\x37\x02\u{5d4}\u{5d6}\x03\x02\x02\x02\u{5d5}\
	\u{5d1}\x03\x02\x02\x02\u{5d6}\u{5d9}\x03\x02\x02\x02\u{5d7}\u{5d5}\x03\
	\x02\x02\x02\u{5d7}\u{5d8}\x03\x02\x02\x02\u{5d8}\x69\x03\x02\x02\x02\u{5d9}\
	\u{5d7}\x03\x02\x02\x02\u{5da}\u{5e8}\x07\x07\x02\x02\u{5db}\u{5dd}\x07\
	\x66\x02\x02\u{5dc}\u{5db}\x03\x02\x02\x02\u{5dc}\u{5dd}\x03\x02\x02\x02\
	\u{5dd}\u{5e4}\x03\x02\x02\x02\u{5de}\u{5e0}\x07\x62\x02\x02\u{5df}\u{5e1}\
	\x07\x70\x02\x02\u{5e0}\u{5df}\x03\x02\x02\x02\u{5e0}\u{5e1}\x03\x02\x02\
	\x02\u{5e1}\u{5e5}\x03\x02\x02\x02\u{5e2}\u{5e5}\x07\x59\x02\x02\u{5e3}\
	\u{5e5}\x07\x35\x02\x02\u{5e4}\u{5de}\x03\x02\x02\x02\u{5e4}\u{5e2}\x03\
	\x02\x02\x02\u{5e4}\u{5e3}\x03\x02\x02\x02\u{5e4}\u{5e5}\x03\x02\x02\x02\
	\u{5e5}\u{5e6}\x03\x02\x02\x02\u{5e6}\u{5e8}\x07\x60\x02\x02\u{5e7}\u{5da}\
	\x03\x02\x02\x02\u{5e7}\u{5dc}\x03\x02\x02\x02\u{5e8}\x6b\x03\x02\x02\x02\
	\u{5e9}\u{5ea}\x07\x6d\x02\x02\u{5ea}\u{5f8}\x05\x50\x29\x02\u{5eb}\u{5ec}\
	\x07\u{8e}\x02\x02\u{5ec}\u{5ed}\x07\x05\x02\x02\u{5ed}\u{5f2}\x05\u{90}\
	\x49\x02\u{5ee}\u{5ef}\x07\x07\x02\x02\u{5ef}\u{5f1}\x05\u{90}\x49\x02\u{5f0}\
	\u{5ee}\x03\x02\x02\x02\u{5f1}\u{5f4}\x03\x02\x02\x02\u{5f2}\u{5f0}\x03\
	\x02\x02\x02\u{5f2}\u{5f3}\x03\x02\x02\x02\u{5f3}\u{5f5}\x03\x02\x02\x02\
	\u{5f4}\u{5f2}\x03\x02\x02\x02\u{5f5}\u{5f6}\x07\x06\x02\x02\u{5f6}\u{5f8}\
	\x03\x02\x02\x02\u{5f7}\u{5e9}\x03\x02\x02\x02\u{5f7}\u{5eb}\x03\x02\x02\
	\x02\u{5f7}\u{5f8}\x03\x02\x02\x02\u{5f8}\x6d\x03\x02\x02\x02\u{5f9}\u{5fb}\
	\x07\u{82}\x02\x02\u{5fa}\u{5fc}\x09\x06\x02\x02\u{5fb}\u{5fa}\x03\x02\x02\
	\x02\u{5fb}\u{5fc}\x03\x02\x02\x02\u{5fc}\u{5fd}\x03\x02\x02\x02\u{5fd}\
	\u{602}\x05\x64\x33\x02\u{5fe}\u{5ff}\x07\x07\x02\x02\u{5ff}\u{601}\x05\
	\x64\x33\x02\u{600}\u{5fe}\x03\x02\x02\x02\u{601}\u{604}\x03\x02\x02\x02\
	\u{602}\u{600}\x03\x02\x02\x02\u{602}\u{603}\x03\x02\x02\x02\u{603}\u{611}\
	\x03\x02\x02\x02\u{604}\u{602}\x03\x02\x02\x02\u{605}\u{60f}\x07\x4d\x02\
	\x02\u{606}\u{60b}\x05\x66\x34\x02\u{607}\u{608}\x07\x07\x02\x02\u{608}\
	\u{60a}\x05\x66\x34\x02\u{609}\u{607}\x03\x02\x02\x02\u{60a}\u{60d}\x03\
	\x02\x02\x02\u{60b}\u{609}\x03\x02\x02\x02\u{60b}\u{60c}\x03\x02\x02\x02\
	\u{60c}\u{610}\x03\x02\x02\x02\u{60d}\u{60b}\x03\x02\x02\x02\u{60e}\u{610}\
	\x05\x68\x35\x02\u{60f}\u{606}\x03\x02\x02\x02\u{60f}\u{60e}\x03\x02\x02\
	\x02\u{610}\u{612}\x03\x02\x02\x02\u{611}\u{605}\x03\x02\x02\x02\u{611}\
	\u{612}\x03\x02\x02\x02\u{612}\u{615}\x03\x02\x02\x02\u{613}\u{614}\x07\
	\u{94}\x02\x02\u{614}\u{616}\x05\x50\x29\x02\u{615}\u{613}\x03\x02\x02\x02\
	\u{615}\u{616}\x03\x02\x02\x02\u{616}\u{625}\x03\x02\x02\x02\u{617}\u{618}\
	\x07\x50\x02\x02\u{618}\u{619}\x07\x2a\x02\x02\u{619}\u{61e}\x05\x50\x29\
	\x02\u{61a}\u{61b}\x07\x07\x02\x02\u{61b}\u{61d}\x05\x50\x29\x02\u{61c}\
	\u{61a}\x03\x02\x02\x02\u{61d}\u{620}\x03\x02\x02\x02\u{61e}\u{61c}\x03\
	\x02\x02\x02\u{61e}\u{61f}\x03\x02\x02\x02\u{61f}\u{623}\x03\x02\x02\x02\
	\u{620}\u{61e}\x03\x02\x02\x02\u{621}\u{622}\x07\x51\x02\x02\u{622}\u{624}\
	\x05\x50\x29\x02\u{623}\u{621}\x03\x02\x02\x02\u{623}\u{624}\x03\x02\x02\
	\x02\u{624}\u{626}\x03\x02\x02\x02\u{625}\u{617}\x03\x02\x02\x02\u{625}\
	\u{626}\x03\x02\x02\x02\u{626}\u{644}\x03\x02\x02\x02\u{627}\u{628}\x07\
	\u{90}\x02\x02\u{628}\u{629}\x07\x05\x02\x02\u{629}\u{62e}\x05\x50\x29\x02\
	\u{62a}\u{62b}\x07\x07\x02\x02\u{62b}\u{62d}\x05\x50\x29\x02\u{62c}\u{62a}\
	\x03\x02\x02\x02\u{62d}\u{630}\x03\x02\x02\x02\u{62e}\u{62c}\x03\x02\x02\
	\x02\u{62e}\u{62f}\x03\x02\x02\x02\u{62f}\u{631}\x03\x02\x02\x02\u{630}\
	\u{62e}\x03\x02\x02\x02\u{631}\u{640}\x07\x06\x02\x02\u{632}\u{633}\x07\
	\x07\x02\x02\u{633}\u{634}\x07\x05\x02\x02\u{634}\u{639}\x05\x50\x29\x02\
	\u{635}\u{636}\x07\x07\x02\x02\u{636}\u{638}\x05\x50\x29\x02\u{637}\u{635}\
	\x03\x02\x02\x02\u{638}\u{63b}\x03\x02\x02\x02\u{639}\u{637}\x03\x02\x02\
	\x02\u{639}\u{63a}\x03\x02\x02\x02\u{63a}\u{63c}\x03\x02\x02\x02\u{63b}\
	\u{639}\x03\x02\x02\x02\u{63c}\u{63d}\x07\x06\x02\x02\u{63d}\u{63f}\x03\
	\x02\x02\x02\u{63e}\u{632}\x03\x02\x02\x02\u{63f}\u{642}\x03\x02\x02\x02\
	\u{640}\u{63e}\x03\x02\x02\x02\u{640}\u{641}\x03\x02\x02\x02\u{641}\u{644}\
	\x03\x02\x02\x02\u{642}\u{640}\x03\x02\x02\x02\u{643}\u{5f9}\x03\x02\x02\
	\x02\u{643}\u{627}\x03\x02\x02\x02\u{644}\x6f\x03\x02\x02\x02\u{645}\u{64b}\
	\x07\u{8b}\x02\x02\u{646}\u{647}\x07\u{8b}\x02\x02\u{647}\u{64b}\x07\x1f\
	\x02\x02\u{648}\u{64b}\x07\x5c\x02\x02\u{649}\u{64b}\x07\x46\x02\x02\u{64a}\
	\u{645}\x03\x02\x02\x02\u{64a}\u{646}\x03\x02\x02\x02\u{64a}\u{648}\x03\
	\x02\x02\x02\u{64a}\u{649}\x03\x02\x02\x02\u{64b}\x71\x03\x02\x02\x02\u{64c}\
	\u{64e}\x09\x0a\x02\x02\u{64d}\u{64c}\x03\x02\x02\x02\u{64d}\u{64e}\x03\
	\x02\x02\x02\u{64e}\u{64f}\x03\x02\x02\x02\u{64f}\u{650}\x07\u{98}\x02\x02\
	\u{650}\x73\x03\x02\x02\x02\u{651}\u{652}\x09\x11\x02\x02\u{652}\x75\x03\
	\x02\x02\x02\u{653}\u{654}\x09\x12\x02\x02\u{654}\x77\x03\x02\x02\x02\u{655}\
	\u{656}\x07\u{9a}\x02\x02\u{656}\x79\x03\x02\x02\x02\u{657}\u{65a}\x05\x50\
	\x29\x02\u{658}\u{65a}\x05\x48\x25\x02\u{659}\u{657}\x03\x02\x02\x02\u{659}\
	\u{658}\x03\x02\x02\x02\u{65a}\x7b\x03\x02\x02\x02\u{65b}\u{65c}\x09\x13\
	\x02\x02\u{65c}\x7d\x03\x02\x02\x02\u{65d}\u{65e}\x09\x14\x02\x02\u{65e}\
	\x7f\x03\x02\x02\x02\u{65f}\u{660}\x05\u{a6}\x54\x02\u{660}\u{81}\x03\x02\
	\x02\x02\u{661}\u{662}\x05\u{a6}\x54\x02\u{662}\u{83}\x03\x02\x02\x02\u{663}\
	\u{664}\x05\u{a6}\x54\x02\u{664}\u{85}\x03\x02\x02\x02\u{665}\u{666}\x05\
	\u{a6}\x54\x02\u{666}\u{87}\x03\x02\x02\x02\u{667}\u{668}\x05\u{a6}\x54\
	\x02\u{668}\u{89}\x03\x02\x02\x02\u{669}\u{66a}\x05\u{a6}\x54\x02\u{66a}\
	\u{8b}\x03\x02\x02\x02\u{66b}\u{66c}\x05\u{a6}\x54\x02\u{66c}\u{8d}\x03\
	\x02\x02\x02\u{66d}\u{66e}\x05\u{a6}\x54\x02\u{66e}\u{8f}\x03\x02\x02\x02\
	\u{66f}\u{670}\x05\u{a6}\x54\x02\u{670}\u{91}\x03\x02\x02\x02\u{671}\u{672}\
	\x05\u{a6}\x54\x02\u{672}\u{93}\x03\x02\x02\x02\u{673}\u{674}\x05\u{a6}\
	\x54\x02\u{674}\u{95}\x03\x02\x02\x02\u{675}\u{676}\x05\u{a6}\x54\x02\u{676}\
	\u{97}\x03\x02\x02\x02\u{677}\u{678}\x05\u{a6}\x54\x02\u{678}\u{99}\x03\
	\x02\x02\x02\u{679}\u{67a}\x05\u{a6}\x54\x02\u{67a}\u{9b}\x03\x02\x02\x02\
	\u{67b}\u{67c}\x05\u{a6}\x54\x02\u{67c}\u{9d}\x03\x02\x02\x02\u{67d}\u{67e}\
	\x05\u{a6}\x54\x02\u{67e}\u{9f}\x03\x02\x02\x02\u{67f}\u{680}\x05\u{a6}\
	\x54\x02\u{680}\u{a1}\x03\x02\x02\x02\u{681}\u{688}\x07\u{97}\x02\x02\u{682}\
	\u{688}\x07\u{9a}\x02\x02\u{683}\u{684}\x07\x05\x02\x02\u{684}\u{685}\x05\
	\u{a2}\x52\x02\u{685}\u{686}\x07\x06\x02\x02\u{686}\u{688}\x03\x02\x02\x02\
	\u{687}\u{681}\x03\x02\x02\x02\u{687}\u{682}\x03\x02\x02\x02\u{687}\u{683}\
	\x03\x02\x02\x02\u{688}\u{a3}\x03\x02\x02\x02\u{689}\u{68a}\x05\u{a6}\x54\
	\x02\u{68a}\u{a5}\x03\x02\x02\x02\u{68b}\u{693}\x07\u{97}\x02\x02\u{68c}\
	\u{693}\x05\x7e\x40\x02\u{68d}\u{693}\x07\u{9a}\x02\x02\u{68e}\u{68f}\x07\
	\x05\x02\x02\u{68f}\u{690}\x05\u{a6}\x54\x02\u{690}\u{691}\x07\x06\x02\x02\
	\u{691}\u{693}\x03\x02\x02\x02\u{692}\u{68b}\x03\x02\x02\x02\u{692}\u{68c}\
	\x03\x02\x02\x02\u{692}\u{68d}\x03\x02\x02\x02\u{692}\u{68e}\x03\x02\x02\
	\x02\u{693}\u{a7}\x03\x02\x02\x02\u{ef}\u{aa}\u{ac}\u{b7}\u{be}\u{c3}\u{c9}\
	\u{cf}\u{d1}\u{f1}\u{f8}\u{100}\u{103}\u{10c}\u{110}\u{118}\u{11c}\u{11e}\
	\u{123}\u{125}\u{128}\u{12d}\u{131}\u{136}\u{13f}\u{142}\u{148}\u{14a}\u{14e}\
	\u{154}\u{159}\u{164}\u{16a}\u{16e}\u{174}\u{179}\u{182}\u{189}\u{18f}\u{193}\
	\u{197}\u{19d}\u{1a2}\u{1a9}\u{1b4}\u{1b7}\u{1b9}\u{1bf}\u{1c5}\u{1c9}\u{1d0}\
	\u{1d6}\u{1dc}\u{1e2}\u{1e7}\u{1f3}\u{1f8}\u{203}\u{208}\u{20b}\u{212}\u{215}\
	\u{21c}\u{225}\u{228}\u{22e}\u{230}\u{234}\u{23c}\u{241}\u{249}\u{24e}\u{256}\
	\u{25b}\u{263}\u{268}\u{26d}\u{275}\u{27f}\u{282}\u{288}\u{28a}\u{28d}\u{2a0}\
	\u{2a6}\u{2af}\u{2b4}\u{2bd}\u{2c8}\u{2cf}\u{2d5}\u{2db}\u{2e4}\u{2eb}\u{2ef}\
	\u{2f1}\u{2f5}\u{2fc}\u{2fe}\u{302}\u{305}\u{30b}\u{315}\u{318}\u{31e}\u{320}\
	\u{323}\u{32b}\u{335}\u{338}\u{33e}\u{340}\u{344}\u{34b}\u{354}\u{358}\u{35a}\
	\u{35e}\u{367}\u{36c}\u{36e}\u{377}\u{382}\u{389}\u{38c}\u{38f}\u{39c}\u{3aa}\
	\u{3af}\u{3b2}\u{3bf}\u{3cd}\u{3d2}\u{3db}\u{3de}\u{3e4}\u{3e6}\u{3ec}\u{3f1}\
	\u{3f7}\u{403}\u{407}\u{40c}\u{410}\u{413}\u{425}\u{42a}\u{42f}\u{437}\u{43c}\
	\u{445}\u{44c}\u{450}\u{460}\u{463}\u{46b}\u{474}\u{478}\u{47d}\u{49a}\u{49f}\
	\u{4a8}\u{4b2}\u{4b5}\u{4bb}\u{4be}\u{4c5}\u{4cb}\u{4d2}\u{4d4}\u{4d6}\u{4e1}\
	\u{4e6}\u{4f2}\u{4f6}\u{4fa}\u{4fe}\u{505}\u{507}\u{50f}\u{516}\u{519}\u{51d}\
	\u{522}\u{52a}\u{53d}\u{543}\u{547}\u{54e}\u{554}\u{55c}\u{561}\u{564}\u{569}\
	\u{572}\u{577}\u{585}\u{588}\u{58a}\u{58f}\u{593}\u{596}\u{59d}\u{5a2}\u{5ab}\
	\u{5ae}\u{5b2}\u{5b5}\u{5bd}\u{5c1}\u{5c9}\u{5cc}\u{5ce}\u{5d7}\u{5dc}\u{5e0}\
	\u{5e4}\u{5e7}\u{5f2}\u{5f7}\u{5fb}\u{602}\u{60b}\u{60f}\u{611}\u{615}\u{61e}\
	\u{623}\u{625}\u{62e}\u{639}\u{640}\u{643}\u{64a}\u{64d}\u{659}\u{687}\u{692}";
