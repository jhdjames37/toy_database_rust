#![allow(nonstandard_style)]
#![allow(clippy::all)]
// Generated from SQL.g4 by ANTLR 4.8
use super::sqlparser::*;
use antlr_rust::tree::ParseTreeListener;

pub trait SQLListener<'input>: ParseTreeListener<'input, SQLParserContextType> {
    /**
     * Enter a parse tree produced by {@link SQLParser#program}.
     * @param ctx the parse tree
     */
    fn enter_program(&mut self, _ctx: &ProgramContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLParser#program}.
     * @param ctx the parse tree
     */
    fn exit_program(&mut self, _ctx: &ProgramContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLParser#statement}.
     * @param ctx the parse tree
     */
    fn enter_statement(&mut self, _ctx: &StatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLParser#statement}.
     * @param ctx the parse tree
     */
    fn exit_statement(&mut self, _ctx: &StatementContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code create_db}
     * labeled alternative in {@link SQLParser#db_statement}.
     * @param ctx the parse tree
     */
    fn enter_create_db(&mut self, _ctx: &Create_dbContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code create_db}
     * labeled alternative in {@link SQLParser#db_statement}.
     * @param ctx the parse tree
     */
    fn exit_create_db(&mut self, _ctx: &Create_dbContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code drop_db}
     * labeled alternative in {@link SQLParser#db_statement}.
     * @param ctx the parse tree
     */
    fn enter_drop_db(&mut self, _ctx: &Drop_dbContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code drop_db}
     * labeled alternative in {@link SQLParser#db_statement}.
     * @param ctx the parse tree
     */
    fn exit_drop_db(&mut self, _ctx: &Drop_dbContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code show_dbs}
     * labeled alternative in {@link SQLParser#db_statement}.
     * @param ctx the parse tree
     */
    fn enter_show_dbs(&mut self, _ctx: &Show_dbsContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code show_dbs}
     * labeled alternative in {@link SQLParser#db_statement}.
     * @param ctx the parse tree
     */
    fn exit_show_dbs(&mut self, _ctx: &Show_dbsContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code use_db}
     * labeled alternative in {@link SQLParser#db_statement}.
     * @param ctx the parse tree
     */
    fn enter_use_db(&mut self, _ctx: &Use_dbContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code use_db}
     * labeled alternative in {@link SQLParser#db_statement}.
     * @param ctx the parse tree
     */
    fn exit_use_db(&mut self, _ctx: &Use_dbContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code show_tables}
     * labeled alternative in {@link SQLParser#db_statement}.
     * @param ctx the parse tree
     */
    fn enter_show_tables(&mut self, _ctx: &Show_tablesContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code show_tables}
     * labeled alternative in {@link SQLParser#db_statement}.
     * @param ctx the parse tree
     */
    fn exit_show_tables(&mut self, _ctx: &Show_tablesContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code show_indexes}
     * labeled alternative in {@link SQLParser#db_statement}.
     * @param ctx the parse tree
     */
    fn enter_show_indexes(&mut self, _ctx: &Show_indexesContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code show_indexes}
     * labeled alternative in {@link SQLParser#db_statement}.
     * @param ctx the parse tree
     */
    fn exit_show_indexes(&mut self, _ctx: &Show_indexesContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code load_data}
     * labeled alternative in {@link SQLParser#io_statement}.
     * @param ctx the parse tree
     */
    fn enter_load_data(&mut self, _ctx: &Load_dataContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code load_data}
     * labeled alternative in {@link SQLParser#io_statement}.
     * @param ctx the parse tree
     */
    fn exit_load_data(&mut self, _ctx: &Load_dataContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code dump_data}
     * labeled alternative in {@link SQLParser#io_statement}.
     * @param ctx the parse tree
     */
    fn enter_dump_data(&mut self, _ctx: &Dump_dataContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code dump_data}
     * labeled alternative in {@link SQLParser#io_statement}.
     * @param ctx the parse tree
     */
    fn exit_dump_data(&mut self, _ctx: &Dump_dataContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code create_table}
     * labeled alternative in {@link SQLParser#table_statement}.
     * @param ctx the parse tree
     */
    fn enter_create_table(&mut self, _ctx: &Create_tableContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code create_table}
     * labeled alternative in {@link SQLParser#table_statement}.
     * @param ctx the parse tree
     */
    fn exit_create_table(&mut self, _ctx: &Create_tableContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code drop_table}
     * labeled alternative in {@link SQLParser#table_statement}.
     * @param ctx the parse tree
     */
    fn enter_drop_table(&mut self, _ctx: &Drop_tableContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code drop_table}
     * labeled alternative in {@link SQLParser#table_statement}.
     * @param ctx the parse tree
     */
    fn exit_drop_table(&mut self, _ctx: &Drop_tableContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code describe_table}
     * labeled alternative in {@link SQLParser#table_statement}.
     * @param ctx the parse tree
     */
    fn enter_describe_table(&mut self, _ctx: &Describe_tableContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code describe_table}
     * labeled alternative in {@link SQLParser#table_statement}.
     * @param ctx the parse tree
     */
    fn exit_describe_table(&mut self, _ctx: &Describe_tableContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code insert_into_table}
     * labeled alternative in {@link SQLParser#table_statement}.
     * @param ctx the parse tree
     */
    fn enter_insert_into_table(&mut self, _ctx: &Insert_into_tableContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code insert_into_table}
     * labeled alternative in {@link SQLParser#table_statement}.
     * @param ctx the parse tree
     */
    fn exit_insert_into_table(&mut self, _ctx: &Insert_into_tableContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code delete_from_table}
     * labeled alternative in {@link SQLParser#table_statement}.
     * @param ctx the parse tree
     */
    fn enter_delete_from_table(&mut self, _ctx: &Delete_from_tableContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code delete_from_table}
     * labeled alternative in {@link SQLParser#table_statement}.
     * @param ctx the parse tree
     */
    fn exit_delete_from_table(&mut self, _ctx: &Delete_from_tableContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code update_table}
     * labeled alternative in {@link SQLParser#table_statement}.
     * @param ctx the parse tree
     */
    fn enter_update_table(&mut self, _ctx: &Update_tableContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code update_table}
     * labeled alternative in {@link SQLParser#table_statement}.
     * @param ctx the parse tree
     */
    fn exit_update_table(&mut self, _ctx: &Update_tableContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code select_table_}
     * labeled alternative in {@link SQLParser#table_statement}.
     * @param ctx the parse tree
     */
    fn enter_select_table_(&mut self, _ctx: &Select_table_Context<'input>) {}
    /**
     * Exit a parse tree produced by the {@code select_table_}
     * labeled alternative in {@link SQLParser#table_statement}.
     * @param ctx the parse tree
     */
    fn exit_select_table_(&mut self, _ctx: &Select_table_Context<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLParser#select_table}.
     * @param ctx the parse tree
     */
    fn enter_select_table(&mut self, _ctx: &Select_tableContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLParser#select_table}.
     * @param ctx the parse tree
     */
    fn exit_select_table(&mut self, _ctx: &Select_tableContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code alter_add_index}
     * labeled alternative in {@link SQLParser#alter_statement}.
     * @param ctx the parse tree
     */
    fn enter_alter_add_index(&mut self, _ctx: &Alter_add_indexContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code alter_add_index}
     * labeled alternative in {@link SQLParser#alter_statement}.
     * @param ctx the parse tree
     */
    fn exit_alter_add_index(&mut self, _ctx: &Alter_add_indexContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code alter_drop_index}
     * labeled alternative in {@link SQLParser#alter_statement}.
     * @param ctx the parse tree
     */
    fn enter_alter_drop_index(&mut self, _ctx: &Alter_drop_indexContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code alter_drop_index}
     * labeled alternative in {@link SQLParser#alter_statement}.
     * @param ctx the parse tree
     */
    fn exit_alter_drop_index(&mut self, _ctx: &Alter_drop_indexContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code alter_table_drop_pk}
     * labeled alternative in {@link SQLParser#alter_statement}.
     * @param ctx the parse tree
     */
    fn enter_alter_table_drop_pk(&mut self, _ctx: &Alter_table_drop_pkContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code alter_table_drop_pk}
     * labeled alternative in {@link SQLParser#alter_statement}.
     * @param ctx the parse tree
     */
    fn exit_alter_table_drop_pk(&mut self, _ctx: &Alter_table_drop_pkContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code alter_table_drop_foreign_key}
     * labeled alternative in {@link SQLParser#alter_statement}.
     * @param ctx the parse tree
     */
    fn enter_alter_table_drop_foreign_key(
        &mut self,
        _ctx: &Alter_table_drop_foreign_keyContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by the {@code alter_table_drop_foreign_key}
     * labeled alternative in {@link SQLParser#alter_statement}.
     * @param ctx the parse tree
     */
    fn exit_alter_table_drop_foreign_key(
        &mut self,
        _ctx: &Alter_table_drop_foreign_keyContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by the {@code alter_table_add_pk}
     * labeled alternative in {@link SQLParser#alter_statement}.
     * @param ctx the parse tree
     */
    fn enter_alter_table_add_pk(&mut self, _ctx: &Alter_table_add_pkContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code alter_table_add_pk}
     * labeled alternative in {@link SQLParser#alter_statement}.
     * @param ctx the parse tree
     */
    fn exit_alter_table_add_pk(&mut self, _ctx: &Alter_table_add_pkContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code alter_table_add_foreign_key}
     * labeled alternative in {@link SQLParser#alter_statement}.
     * @param ctx the parse tree
     */
    fn enter_alter_table_add_foreign_key(
        &mut self,
        _ctx: &Alter_table_add_foreign_keyContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by the {@code alter_table_add_foreign_key}
     * labeled alternative in {@link SQLParser#alter_statement}.
     * @param ctx the parse tree
     */
    fn exit_alter_table_add_foreign_key(
        &mut self,
        _ctx: &Alter_table_add_foreign_keyContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by the {@code alter_table_add_unique}
     * labeled alternative in {@link SQLParser#alter_statement}.
     * @param ctx the parse tree
     */
    fn enter_alter_table_add_unique(&mut self, _ctx: &Alter_table_add_uniqueContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code alter_table_add_unique}
     * labeled alternative in {@link SQLParser#alter_statement}.
     * @param ctx the parse tree
     */
    fn exit_alter_table_add_unique(&mut self, _ctx: &Alter_table_add_uniqueContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLParser#field_list}.
     * @param ctx the parse tree
     */
    fn enter_field_list(&mut self, _ctx: &Field_listContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLParser#field_list}.
     * @param ctx the parse tree
     */
    fn exit_field_list(&mut self, _ctx: &Field_listContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code normal_field}
     * labeled alternative in {@link SQLParser#field}.
     * @param ctx the parse tree
     */
    fn enter_normal_field(&mut self, _ctx: &Normal_fieldContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code normal_field}
     * labeled alternative in {@link SQLParser#field}.
     * @param ctx the parse tree
     */
    fn exit_normal_field(&mut self, _ctx: &Normal_fieldContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code primary_key_field}
     * labeled alternative in {@link SQLParser#field}.
     * @param ctx the parse tree
     */
    fn enter_primary_key_field(&mut self, _ctx: &Primary_key_fieldContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code primary_key_field}
     * labeled alternative in {@link SQLParser#field}.
     * @param ctx the parse tree
     */
    fn exit_primary_key_field(&mut self, _ctx: &Primary_key_fieldContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code foreign_key_field}
     * labeled alternative in {@link SQLParser#field}.
     * @param ctx the parse tree
     */
    fn enter_foreign_key_field(&mut self, _ctx: &Foreign_key_fieldContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code foreign_key_field}
     * labeled alternative in {@link SQLParser#field}.
     * @param ctx the parse tree
     */
    fn exit_foreign_key_field(&mut self, _ctx: &Foreign_key_fieldContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLParser#type_}.
     * @param ctx the parse tree
     */
    fn enter_type_(&mut self, _ctx: &Type_Context<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLParser#type_}.
     * @param ctx the parse tree
     */
    fn exit_type_(&mut self, _ctx: &Type_Context<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLParser#value_lists}.
     * @param ctx the parse tree
     */
    fn enter_value_lists(&mut self, _ctx: &Value_listsContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLParser#value_lists}.
     * @param ctx the parse tree
     */
    fn exit_value_lists(&mut self, _ctx: &Value_listsContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLParser#value_list}.
     * @param ctx the parse tree
     */
    fn enter_value_list(&mut self, _ctx: &Value_listContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLParser#value_list}.
     * @param ctx the parse tree
     */
    fn exit_value_list(&mut self, _ctx: &Value_listContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLParser#value}.
     * @param ctx the parse tree
     */
    fn enter_value(&mut self, _ctx: &ValueContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLParser#value}.
     * @param ctx the parse tree
     */
    fn exit_value(&mut self, _ctx: &ValueContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLParser#where_and_clause}.
     * @param ctx the parse tree
     */
    fn enter_where_and_clause(&mut self, _ctx: &Where_and_clauseContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLParser#where_and_clause}.
     * @param ctx the parse tree
     */
    fn exit_where_and_clause(&mut self, _ctx: &Where_and_clauseContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code where_operator_expression}
     * labeled alternative in {@link SQLParser#where_clause}.
     * @param ctx the parse tree
     */
    fn enter_where_operator_expression(&mut self, _ctx: &Where_operator_expressionContext<'input>) {
    }
    /**
     * Exit a parse tree produced by the {@code where_operator_expression}
     * labeled alternative in {@link SQLParser#where_clause}.
     * @param ctx the parse tree
     */
    fn exit_where_operator_expression(&mut self, _ctx: &Where_operator_expressionContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code where_operator_select}
     * labeled alternative in {@link SQLParser#where_clause}.
     * @param ctx the parse tree
     */
    fn enter_where_operator_select(&mut self, _ctx: &Where_operator_selectContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code where_operator_select}
     * labeled alternative in {@link SQLParser#where_clause}.
     * @param ctx the parse tree
     */
    fn exit_where_operator_select(&mut self, _ctx: &Where_operator_selectContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code where_null}
     * labeled alternative in {@link SQLParser#where_clause}.
     * @param ctx the parse tree
     */
    fn enter_where_null(&mut self, _ctx: &Where_nullContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code where_null}
     * labeled alternative in {@link SQLParser#where_clause}.
     * @param ctx the parse tree
     */
    fn exit_where_null(&mut self, _ctx: &Where_nullContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code where_in_list}
     * labeled alternative in {@link SQLParser#where_clause}.
     * @param ctx the parse tree
     */
    fn enter_where_in_list(&mut self, _ctx: &Where_in_listContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code where_in_list}
     * labeled alternative in {@link SQLParser#where_clause}.
     * @param ctx the parse tree
     */
    fn exit_where_in_list(&mut self, _ctx: &Where_in_listContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code where_in_select}
     * labeled alternative in {@link SQLParser#where_clause}.
     * @param ctx the parse tree
     */
    fn enter_where_in_select(&mut self, _ctx: &Where_in_selectContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code where_in_select}
     * labeled alternative in {@link SQLParser#where_clause}.
     * @param ctx the parse tree
     */
    fn exit_where_in_select(&mut self, _ctx: &Where_in_selectContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code where_like_string}
     * labeled alternative in {@link SQLParser#where_clause}.
     * @param ctx the parse tree
     */
    fn enter_where_like_string(&mut self, _ctx: &Where_like_stringContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code where_like_string}
     * labeled alternative in {@link SQLParser#where_clause}.
     * @param ctx the parse tree
     */
    fn exit_where_like_string(&mut self, _ctx: &Where_like_stringContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLParser#column}.
     * @param ctx the parse tree
     */
    fn enter_column(&mut self, _ctx: &ColumnContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLParser#column}.
     * @param ctx the parse tree
     */
    fn exit_column(&mut self, _ctx: &ColumnContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLParser#expression}.
     * @param ctx the parse tree
     */
    fn enter_expression(&mut self, _ctx: &ExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLParser#expression}.
     * @param ctx the parse tree
     */
    fn exit_expression(&mut self, _ctx: &ExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLParser#set_clause}.
     * @param ctx the parse tree
     */
    fn enter_set_clause(&mut self, _ctx: &Set_clauseContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLParser#set_clause}.
     * @param ctx the parse tree
     */
    fn exit_set_clause(&mut self, _ctx: &Set_clauseContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLParser#selectors}.
     * @param ctx the parse tree
     */
    fn enter_selectors(&mut self, _ctx: &SelectorsContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLParser#selectors}.
     * @param ctx the parse tree
     */
    fn exit_selectors(&mut self, _ctx: &SelectorsContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLParser#selector}.
     * @param ctx the parse tree
     */
    fn enter_selector(&mut self, _ctx: &SelectorContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLParser#selector}.
     * @param ctx the parse tree
     */
    fn exit_selector(&mut self, _ctx: &SelectorContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLParser#identifiers}.
     * @param ctx the parse tree
     */
    fn enter_identifiers(&mut self, _ctx: &IdentifiersContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLParser#identifiers}.
     * @param ctx the parse tree
     */
    fn exit_identifiers(&mut self, _ctx: &IdentifiersContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLParser#operator_}.
     * @param ctx the parse tree
     */
    fn enter_operator_(&mut self, _ctx: &Operator_Context<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLParser#operator_}.
     * @param ctx the parse tree
     */
    fn exit_operator_(&mut self, _ctx: &Operator_Context<'input>) {}
    /**
     * Enter a parse tree produced by {@link SQLParser#aggregator}.
     * @param ctx the parse tree
     */
    fn enter_aggregator(&mut self, _ctx: &AggregatorContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link SQLParser#aggregator}.
     * @param ctx the parse tree
     */
    fn exit_aggregator(&mut self, _ctx: &AggregatorContext<'input>) {}
}

antlr_rust::coerce_from! { 'input : SQLListener<'input> }
