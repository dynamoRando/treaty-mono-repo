#![allow(nonstandard_style)]
// Generated from SQLite.g4 by ANTLR 4.8
use super::sqliteparser::*;
use antlr_rust::tree::ParseTreeListener;

pub trait SQLiteListener<'input>: ParseTreeListener<'input, SQLiteParserContextType> {
    /**
     * Enter a parse tree produced by {@link SQLiteParser#parse}.
     * @param ctx the parse tree
     */
    fn enter_parse(&mut self, _ctx: &ParseContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#parse}.
     * @param ctx the parse tree
     */
    fn exit_parse(&mut self, _ctx: &ParseContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#error}.
     * @param ctx the parse tree
     */
    fn enter_error(&mut self, _ctx: &ErrorContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#error}.
     * @param ctx the parse tree
     */
    fn exit_error(&mut self, _ctx: &ErrorContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#sql_stmt_list}.
     * @param ctx the parse tree
     */
    fn enter_sql_stmt_list(&mut self, _ctx: &Sql_stmt_listContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#sql_stmt_list}.
     * @param ctx the parse tree
     */
    fn exit_sql_stmt_list(&mut self, _ctx: &Sql_stmt_listContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#sql_stmt}.
     * @param ctx the parse tree
     */
    fn enter_sql_stmt(&mut self, _ctx: &Sql_stmtContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#sql_stmt}.
     * @param ctx the parse tree
     */
    fn exit_sql_stmt(&mut self, _ctx: &Sql_stmtContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#alter_table_stmt}.
     * @param ctx the parse tree
     */
    fn enter_alter_table_stmt(&mut self, _ctx: &Alter_table_stmtContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#alter_table_stmt}.
     * @param ctx the parse tree
     */
    fn exit_alter_table_stmt(&mut self, _ctx: &Alter_table_stmtContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#analyze_stmt}.
     * @param ctx the parse tree
     */
    fn enter_analyze_stmt(&mut self, _ctx: &Analyze_stmtContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#analyze_stmt}.
     * @param ctx the parse tree
     */
    fn exit_analyze_stmt(&mut self, _ctx: &Analyze_stmtContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#attach_stmt}.
     * @param ctx the parse tree
     */
    fn enter_attach_stmt(&mut self, _ctx: &Attach_stmtContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#attach_stmt}.
     * @param ctx the parse tree
     */
    fn exit_attach_stmt(&mut self, _ctx: &Attach_stmtContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#begin_stmt}.
     * @param ctx the parse tree
     */
    fn enter_begin_stmt(&mut self, _ctx: &Begin_stmtContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#begin_stmt}.
     * @param ctx the parse tree
     */
    fn exit_begin_stmt(&mut self, _ctx: &Begin_stmtContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#commit_stmt}.
     * @param ctx the parse tree
     */
    fn enter_commit_stmt(&mut self, _ctx: &Commit_stmtContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#commit_stmt}.
     * @param ctx the parse tree
     */
    fn exit_commit_stmt(&mut self, _ctx: &Commit_stmtContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#compound_select_stmt}.
     * @param ctx the parse tree
     */
    fn enter_compound_select_stmt(&mut self, _ctx: &Compound_select_stmtContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#compound_select_stmt}.
     * @param ctx the parse tree
     */
    fn exit_compound_select_stmt(&mut self, _ctx: &Compound_select_stmtContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#create_index_stmt}.
     * @param ctx the parse tree
     */
    fn enter_create_index_stmt(&mut self, _ctx: &Create_index_stmtContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#create_index_stmt}.
     * @param ctx the parse tree
     */
    fn exit_create_index_stmt(&mut self, _ctx: &Create_index_stmtContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#create_table_stmt}.
     * @param ctx the parse tree
     */
    fn enter_create_table_stmt(&mut self, _ctx: &Create_table_stmtContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#create_table_stmt}.
     * @param ctx the parse tree
     */
    fn exit_create_table_stmt(&mut self, _ctx: &Create_table_stmtContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#create_trigger_stmt}.
     * @param ctx the parse tree
     */
    fn enter_create_trigger_stmt(&mut self, _ctx: &Create_trigger_stmtContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#create_trigger_stmt}.
     * @param ctx the parse tree
     */
    fn exit_create_trigger_stmt(&mut self, _ctx: &Create_trigger_stmtContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#create_view_stmt}.
     * @param ctx the parse tree
     */
    fn enter_create_view_stmt(&mut self, _ctx: &Create_view_stmtContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#create_view_stmt}.
     * @param ctx the parse tree
     */
    fn exit_create_view_stmt(&mut self, _ctx: &Create_view_stmtContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#create_virtual_table_stmt}.
     * @param ctx the parse tree
     */
    fn enter_create_virtual_table_stmt(&mut self, _ctx: &Create_virtual_table_stmtContext<'input>) {
    }
    /**
     * Exit a parse tree produced by {@link SQLiteParser#create_virtual_table_stmt}.
     * @param ctx the parse tree
     */
    fn exit_create_virtual_table_stmt(&mut self, _ctx: &Create_virtual_table_stmtContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#delete_stmt}.
     * @param ctx the parse tree
     */
    fn enter_delete_stmt(&mut self, _ctx: &Delete_stmtContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#delete_stmt}.
     * @param ctx the parse tree
     */
    fn exit_delete_stmt(&mut self, _ctx: &Delete_stmtContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#delete_stmt_limited}.
     * @param ctx the parse tree
     */
    fn enter_delete_stmt_limited(&mut self, _ctx: &Delete_stmt_limitedContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#delete_stmt_limited}.
     * @param ctx the parse tree
     */
    fn exit_delete_stmt_limited(&mut self, _ctx: &Delete_stmt_limitedContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#detach_stmt}.
     * @param ctx the parse tree
     */
    fn enter_detach_stmt(&mut self, _ctx: &Detach_stmtContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#detach_stmt}.
     * @param ctx the parse tree
     */
    fn exit_detach_stmt(&mut self, _ctx: &Detach_stmtContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#drop_index_stmt}.
     * @param ctx the parse tree
     */
    fn enter_drop_index_stmt(&mut self, _ctx: &Drop_index_stmtContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#drop_index_stmt}.
     * @param ctx the parse tree
     */
    fn exit_drop_index_stmt(&mut self, _ctx: &Drop_index_stmtContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#drop_table_stmt}.
     * @param ctx the parse tree
     */
    fn enter_drop_table_stmt(&mut self, _ctx: &Drop_table_stmtContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#drop_table_stmt}.
     * @param ctx the parse tree
     */
    fn exit_drop_table_stmt(&mut self, _ctx: &Drop_table_stmtContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#drop_trigger_stmt}.
     * @param ctx the parse tree
     */
    fn enter_drop_trigger_stmt(&mut self, _ctx: &Drop_trigger_stmtContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#drop_trigger_stmt}.
     * @param ctx the parse tree
     */
    fn exit_drop_trigger_stmt(&mut self, _ctx: &Drop_trigger_stmtContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#drop_view_stmt}.
     * @param ctx the parse tree
     */
    fn enter_drop_view_stmt(&mut self, _ctx: &Drop_view_stmtContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#drop_view_stmt}.
     * @param ctx the parse tree
     */
    fn exit_drop_view_stmt(&mut self, _ctx: &Drop_view_stmtContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#factored_select_stmt}.
     * @param ctx the parse tree
     */
    fn enter_factored_select_stmt(&mut self, _ctx: &Factored_select_stmtContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#factored_select_stmt}.
     * @param ctx the parse tree
     */
    fn exit_factored_select_stmt(&mut self, _ctx: &Factored_select_stmtContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#insert_stmt}.
     * @param ctx the parse tree
     */
    fn enter_insert_stmt(&mut self, _ctx: &Insert_stmtContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#insert_stmt}.
     * @param ctx the parse tree
     */
    fn exit_insert_stmt(&mut self, _ctx: &Insert_stmtContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#pragma_stmt}.
     * @param ctx the parse tree
     */
    fn enter_pragma_stmt(&mut self, _ctx: &Pragma_stmtContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#pragma_stmt}.
     * @param ctx the parse tree
     */
    fn exit_pragma_stmt(&mut self, _ctx: &Pragma_stmtContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#reindex_stmt}.
     * @param ctx the parse tree
     */
    fn enter_reindex_stmt(&mut self, _ctx: &Reindex_stmtContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#reindex_stmt}.
     * @param ctx the parse tree
     */
    fn exit_reindex_stmt(&mut self, _ctx: &Reindex_stmtContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#release_stmt}.
     * @param ctx the parse tree
     */
    fn enter_release_stmt(&mut self, _ctx: &Release_stmtContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#release_stmt}.
     * @param ctx the parse tree
     */
    fn exit_release_stmt(&mut self, _ctx: &Release_stmtContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#rollback_stmt}.
     * @param ctx the parse tree
     */
    fn enter_rollback_stmt(&mut self, _ctx: &Rollback_stmtContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#rollback_stmt}.
     * @param ctx the parse tree
     */
    fn exit_rollback_stmt(&mut self, _ctx: &Rollback_stmtContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#savepoint_stmt}.
     * @param ctx the parse tree
     */
    fn enter_savepoint_stmt(&mut self, _ctx: &Savepoint_stmtContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#savepoint_stmt}.
     * @param ctx the parse tree
     */
    fn exit_savepoint_stmt(&mut self, _ctx: &Savepoint_stmtContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#simple_select_stmt}.
     * @param ctx the parse tree
     */
    fn enter_simple_select_stmt(&mut self, _ctx: &Simple_select_stmtContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#simple_select_stmt}.
     * @param ctx the parse tree
     */
    fn exit_simple_select_stmt(&mut self, _ctx: &Simple_select_stmtContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#select_stmt}.
     * @param ctx the parse tree
     */
    fn enter_select_stmt(&mut self, _ctx: &Select_stmtContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#select_stmt}.
     * @param ctx the parse tree
     */
    fn exit_select_stmt(&mut self, _ctx: &Select_stmtContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#select_or_values}.
     * @param ctx the parse tree
     */
    fn enter_select_or_values(&mut self, _ctx: &Select_or_valuesContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#select_or_values}.
     * @param ctx the parse tree
     */
    fn exit_select_or_values(&mut self, _ctx: &Select_or_valuesContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#update_stmt}.
     * @param ctx the parse tree
     */
    fn enter_update_stmt(&mut self, _ctx: &Update_stmtContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#update_stmt}.
     * @param ctx the parse tree
     */
    fn exit_update_stmt(&mut self, _ctx: &Update_stmtContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#update_stmt_limited}.
     * @param ctx the parse tree
     */
    fn enter_update_stmt_limited(&mut self, _ctx: &Update_stmt_limitedContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#update_stmt_limited}.
     * @param ctx the parse tree
     */
    fn exit_update_stmt_limited(&mut self, _ctx: &Update_stmt_limitedContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#vacuum_stmt}.
     * @param ctx the parse tree
     */
    fn enter_vacuum_stmt(&mut self, _ctx: &Vacuum_stmtContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#vacuum_stmt}.
     * @param ctx the parse tree
     */
    fn exit_vacuum_stmt(&mut self, _ctx: &Vacuum_stmtContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#column_def}.
     * @param ctx the parse tree
     */
    fn enter_column_def(&mut self, _ctx: &Column_defContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#column_def}.
     * @param ctx the parse tree
     */
    fn exit_column_def(&mut self, _ctx: &Column_defContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#type_name}.
     * @param ctx the parse tree
     */
    fn enter_type_name(&mut self, _ctx: &Type_nameContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#type_name}.
     * @param ctx the parse tree
     */
    fn exit_type_name(&mut self, _ctx: &Type_nameContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#column_constraint}.
     * @param ctx the parse tree
     */
    fn enter_column_constraint(&mut self, _ctx: &Column_constraintContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#column_constraint}.
     * @param ctx the parse tree
     */
    fn exit_column_constraint(&mut self, _ctx: &Column_constraintContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#conflict_clause}.
     * @param ctx the parse tree
     */
    fn enter_conflict_clause(&mut self, _ctx: &Conflict_clauseContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#conflict_clause}.
     * @param ctx the parse tree
     */
    fn exit_conflict_clause(&mut self, _ctx: &Conflict_clauseContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#expr}.
     * @param ctx the parse tree
     */
    fn enter_expr(&mut self, _ctx: &ExprContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#expr}.
     * @param ctx the parse tree
     */
    fn exit_expr(&mut self, _ctx: &ExprContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#foreign_key_clause}.
     * @param ctx the parse tree
     */
    fn enter_foreign_key_clause(&mut self, _ctx: &Foreign_key_clauseContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#foreign_key_clause}.
     * @param ctx the parse tree
     */
    fn exit_foreign_key_clause(&mut self, _ctx: &Foreign_key_clauseContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#raise_function}.
     * @param ctx the parse tree
     */
    fn enter_raise_function(&mut self, _ctx: &Raise_functionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#raise_function}.
     * @param ctx the parse tree
     */
    fn exit_raise_function(&mut self, _ctx: &Raise_functionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#indexed_column}.
     * @param ctx the parse tree
     */
    fn enter_indexed_column(&mut self, _ctx: &Indexed_columnContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#indexed_column}.
     * @param ctx the parse tree
     */
    fn exit_indexed_column(&mut self, _ctx: &Indexed_columnContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#table_constraint}.
     * @param ctx the parse tree
     */
    fn enter_table_constraint(&mut self, _ctx: &Table_constraintContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#table_constraint}.
     * @param ctx the parse tree
     */
    fn exit_table_constraint(&mut self, _ctx: &Table_constraintContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#with_clause}.
     * @param ctx the parse tree
     */
    fn enter_with_clause(&mut self, _ctx: &With_clauseContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#with_clause}.
     * @param ctx the parse tree
     */
    fn exit_with_clause(&mut self, _ctx: &With_clauseContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#qualified_table_name}.
     * @param ctx the parse tree
     */
    fn enter_qualified_table_name(&mut self, _ctx: &Qualified_table_nameContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#qualified_table_name}.
     * @param ctx the parse tree
     */
    fn exit_qualified_table_name(&mut self, _ctx: &Qualified_table_nameContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#ordering_term}.
     * @param ctx the parse tree
     */
    fn enter_ordering_term(&mut self, _ctx: &Ordering_termContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#ordering_term}.
     * @param ctx the parse tree
     */
    fn exit_ordering_term(&mut self, _ctx: &Ordering_termContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#pragma_value}.
     * @param ctx the parse tree
     */
    fn enter_pragma_value(&mut self, _ctx: &Pragma_valueContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#pragma_value}.
     * @param ctx the parse tree
     */
    fn exit_pragma_value(&mut self, _ctx: &Pragma_valueContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#common_table_expression}.
     * @param ctx the parse tree
     */
    fn enter_common_table_expression(&mut self, _ctx: &Common_table_expressionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#common_table_expression}.
     * @param ctx the parse tree
     */
    fn exit_common_table_expression(&mut self, _ctx: &Common_table_expressionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#result_column}.
     * @param ctx the parse tree
     */
    fn enter_result_column(&mut self, _ctx: &Result_columnContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#result_column}.
     * @param ctx the parse tree
     */
    fn exit_result_column(&mut self, _ctx: &Result_columnContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#table_or_subquery}.
     * @param ctx the parse tree
     */
    fn enter_table_or_subquery(&mut self, _ctx: &Table_or_subqueryContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#table_or_subquery}.
     * @param ctx the parse tree
     */
    fn exit_table_or_subquery(&mut self, _ctx: &Table_or_subqueryContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#join_clause}.
     * @param ctx the parse tree
     */
    fn enter_join_clause(&mut self, _ctx: &Join_clauseContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#join_clause}.
     * @param ctx the parse tree
     */
    fn exit_join_clause(&mut self, _ctx: &Join_clauseContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#join_operator}.
     * @param ctx the parse tree
     */
    fn enter_join_operator(&mut self, _ctx: &Join_operatorContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#join_operator}.
     * @param ctx the parse tree
     */
    fn exit_join_operator(&mut self, _ctx: &Join_operatorContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#join_constraint}.
     * @param ctx the parse tree
     */
    fn enter_join_constraint(&mut self, _ctx: &Join_constraintContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#join_constraint}.
     * @param ctx the parse tree
     */
    fn exit_join_constraint(&mut self, _ctx: &Join_constraintContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#select_core}.
     * @param ctx the parse tree
     */
    fn enter_select_core(&mut self, _ctx: &Select_coreContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#select_core}.
     * @param ctx the parse tree
     */
    fn exit_select_core(&mut self, _ctx: &Select_coreContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#compound_operator}.
     * @param ctx the parse tree
     */
    fn enter_compound_operator(&mut self, _ctx: &Compound_operatorContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#compound_operator}.
     * @param ctx the parse tree
     */
    fn exit_compound_operator(&mut self, _ctx: &Compound_operatorContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#signed_number}.
     * @param ctx the parse tree
     */
    fn enter_signed_number(&mut self, _ctx: &Signed_numberContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#signed_number}.
     * @param ctx the parse tree
     */
    fn exit_signed_number(&mut self, _ctx: &Signed_numberContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#literal_value}.
     * @param ctx the parse tree
     */
    fn enter_literal_value(&mut self, _ctx: &Literal_valueContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#literal_value}.
     * @param ctx the parse tree
     */
    fn exit_literal_value(&mut self, _ctx: &Literal_valueContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#unary_operator}.
     * @param ctx the parse tree
     */
    fn enter_unary_operator(&mut self, _ctx: &Unary_operatorContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#unary_operator}.
     * @param ctx the parse tree
     */
    fn exit_unary_operator(&mut self, _ctx: &Unary_operatorContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#error_message}.
     * @param ctx the parse tree
     */
    fn enter_error_message(&mut self, _ctx: &Error_messageContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#error_message}.
     * @param ctx the parse tree
     */
    fn exit_error_message(&mut self, _ctx: &Error_messageContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#module_argument}.
     * @param ctx the parse tree
     */
    fn enter_module_argument(&mut self, _ctx: &Module_argumentContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#module_argument}.
     * @param ctx the parse tree
     */
    fn exit_module_argument(&mut self, _ctx: &Module_argumentContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#column_alias}.
     * @param ctx the parse tree
     */
    fn enter_column_alias(&mut self, _ctx: &Column_aliasContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#column_alias}.
     * @param ctx the parse tree
     */
    fn exit_column_alias(&mut self, _ctx: &Column_aliasContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#keyword}.
     * @param ctx the parse tree
     */
    fn enter_keyword(&mut self, _ctx: &KeywordContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#keyword}.
     * @param ctx the parse tree
     */
    fn exit_keyword(&mut self, _ctx: &KeywordContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#name}.
     * @param ctx the parse tree
     */
    fn enter_name(&mut self, _ctx: &NameContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#name}.
     * @param ctx the parse tree
     */
    fn exit_name(&mut self, _ctx: &NameContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#function_name}.
     * @param ctx the parse tree
     */
    fn enter_function_name(&mut self, _ctx: &Function_nameContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#function_name}.
     * @param ctx the parse tree
     */
    fn exit_function_name(&mut self, _ctx: &Function_nameContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#database_name}.
     * @param ctx the parse tree
     */
    fn enter_database_name(&mut self, _ctx: &Database_nameContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#database_name}.
     * @param ctx the parse tree
     */
    fn exit_database_name(&mut self, _ctx: &Database_nameContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#schema_name}.
     * @param ctx the parse tree
     */
    fn enter_schema_name(&mut self, _ctx: &Schema_nameContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#schema_name}.
     * @param ctx the parse tree
     */
    fn exit_schema_name(&mut self, _ctx: &Schema_nameContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#table_function_name}.
     * @param ctx the parse tree
     */
    fn enter_table_function_name(&mut self, _ctx: &Table_function_nameContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#table_function_name}.
     * @param ctx the parse tree
     */
    fn exit_table_function_name(&mut self, _ctx: &Table_function_nameContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#table_name}.
     * @param ctx the parse tree
     */
    fn enter_table_name(&mut self, _ctx: &Table_nameContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#table_name}.
     * @param ctx the parse tree
     */
    fn exit_table_name(&mut self, _ctx: &Table_nameContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#table_or_index_name}.
     * @param ctx the parse tree
     */
    fn enter_table_or_index_name(&mut self, _ctx: &Table_or_index_nameContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#table_or_index_name}.
     * @param ctx the parse tree
     */
    fn exit_table_or_index_name(&mut self, _ctx: &Table_or_index_nameContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#new_table_name}.
     * @param ctx the parse tree
     */
    fn enter_new_table_name(&mut self, _ctx: &New_table_nameContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#new_table_name}.
     * @param ctx the parse tree
     */
    fn exit_new_table_name(&mut self, _ctx: &New_table_nameContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#column_name}.
     * @param ctx the parse tree
     */
    fn enter_column_name(&mut self, _ctx: &Column_nameContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#column_name}.
     * @param ctx the parse tree
     */
    fn exit_column_name(&mut self, _ctx: &Column_nameContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#collation_name}.
     * @param ctx the parse tree
     */
    fn enter_collation_name(&mut self, _ctx: &Collation_nameContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#collation_name}.
     * @param ctx the parse tree
     */
    fn exit_collation_name(&mut self, _ctx: &Collation_nameContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#foreign_table}.
     * @param ctx the parse tree
     */
    fn enter_foreign_table(&mut self, _ctx: &Foreign_tableContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#foreign_table}.
     * @param ctx the parse tree
     */
    fn exit_foreign_table(&mut self, _ctx: &Foreign_tableContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#index_name}.
     * @param ctx the parse tree
     */
    fn enter_index_name(&mut self, _ctx: &Index_nameContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#index_name}.
     * @param ctx the parse tree
     */
    fn exit_index_name(&mut self, _ctx: &Index_nameContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#trigger_name}.
     * @param ctx the parse tree
     */
    fn enter_trigger_name(&mut self, _ctx: &Trigger_nameContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#trigger_name}.
     * @param ctx the parse tree
     */
    fn exit_trigger_name(&mut self, _ctx: &Trigger_nameContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#view_name}.
     * @param ctx the parse tree
     */
    fn enter_view_name(&mut self, _ctx: &View_nameContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#view_name}.
     * @param ctx the parse tree
     */
    fn exit_view_name(&mut self, _ctx: &View_nameContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#module_name}.
     * @param ctx the parse tree
     */
    fn enter_module_name(&mut self, _ctx: &Module_nameContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#module_name}.
     * @param ctx the parse tree
     */
    fn exit_module_name(&mut self, _ctx: &Module_nameContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#pragma_name}.
     * @param ctx the parse tree
     */
    fn enter_pragma_name(&mut self, _ctx: &Pragma_nameContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#pragma_name}.
     * @param ctx the parse tree
     */
    fn exit_pragma_name(&mut self, _ctx: &Pragma_nameContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#savepoint_name}.
     * @param ctx the parse tree
     */
    fn enter_savepoint_name(&mut self, _ctx: &Savepoint_nameContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#savepoint_name}.
     * @param ctx the parse tree
     */
    fn exit_savepoint_name(&mut self, _ctx: &Savepoint_nameContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#table_alias}.
     * @param ctx the parse tree
     */
    fn enter_table_alias(&mut self, _ctx: &Table_aliasContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#table_alias}.
     * @param ctx the parse tree
     */
    fn exit_table_alias(&mut self, _ctx: &Table_aliasContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#transaction_name}.
     * @param ctx the parse tree
     */
    fn enter_transaction_name(&mut self, _ctx: &Transaction_nameContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#transaction_name}.
     * @param ctx the parse tree
     */
    fn exit_transaction_name(&mut self, _ctx: &Transaction_nameContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLiteParser#any_name}.
     * @param ctx the parse tree
     */
    fn enter_any_name(&mut self, _ctx: &Any_nameContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLiteParser#any_name}.
     * @param ctx the parse tree
     */
    fn exit_any_name(&mut self, _ctx: &Any_nameContext<'input>) {}
}

antlr_rust::coerce_from! { 'input : SQLiteListener<'input> }
