#![allow(nonstandard_style)]
#![allow(clippy::all)]
// Generated from SQL.g4 by ANTLR 4.8
use super::sqlparser::*;
use antlr_rust::tree::{ParseTreeVisitor, ParseTreeVisitorCompat};

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link SQLParser}.
 */
pub trait SQLVisitor<'input>: ParseTreeVisitor<'input, SQLParserContextType> {
    /**
     * Visit a parse tree produced by {@link SQLParser#program}.
     * @param ctx the parse tree
     */
    fn visit_program(&mut self, ctx: &ProgramContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SQLParser#statement}.
     * @param ctx the parse tree
     */
    fn visit_statement(&mut self, ctx: &StatementContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code create_db}
     * labeled alternative in {@link SQLParser#db_statement}.
     * @param ctx the parse tree
     */
    fn visit_create_db(&mut self, ctx: &Create_dbContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code drop_db}
     * labeled alternative in {@link SQLParser#db_statement}.
     * @param ctx the parse tree
     */
    fn visit_drop_db(&mut self, ctx: &Drop_dbContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code show_dbs}
     * labeled alternative in {@link SQLParser#db_statement}.
     * @param ctx the parse tree
     */
    fn visit_show_dbs(&mut self, ctx: &Show_dbsContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code use_db}
     * labeled alternative in {@link SQLParser#db_statement}.
     * @param ctx the parse tree
     */
    fn visit_use_db(&mut self, ctx: &Use_dbContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code show_tables}
     * labeled alternative in {@link SQLParser#db_statement}.
     * @param ctx the parse tree
     */
    fn visit_show_tables(&mut self, ctx: &Show_tablesContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code show_indexes}
     * labeled alternative in {@link SQLParser#db_statement}.
     * @param ctx the parse tree
     */
    fn visit_show_indexes(&mut self, ctx: &Show_indexesContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code load_data}
     * labeled alternative in {@link SQLParser#io_statement}.
     * @param ctx the parse tree
     */
    fn visit_load_data(&mut self, ctx: &Load_dataContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code dump_data}
     * labeled alternative in {@link SQLParser#io_statement}.
     * @param ctx the parse tree
     */
    fn visit_dump_data(&mut self, ctx: &Dump_dataContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code create_table}
     * labeled alternative in {@link SQLParser#table_statement}.
     * @param ctx the parse tree
     */
    fn visit_create_table(&mut self, ctx: &Create_tableContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code drop_table}
     * labeled alternative in {@link SQLParser#table_statement}.
     * @param ctx the parse tree
     */
    fn visit_drop_table(&mut self, ctx: &Drop_tableContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code describe_table}
     * labeled alternative in {@link SQLParser#table_statement}.
     * @param ctx the parse tree
     */
    fn visit_describe_table(&mut self, ctx: &Describe_tableContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code insert_into_table}
     * labeled alternative in {@link SQLParser#table_statement}.
     * @param ctx the parse tree
     */
    fn visit_insert_into_table(&mut self, ctx: &Insert_into_tableContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code delete_from_table}
     * labeled alternative in {@link SQLParser#table_statement}.
     * @param ctx the parse tree
     */
    fn visit_delete_from_table(&mut self, ctx: &Delete_from_tableContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code update_table}
     * labeled alternative in {@link SQLParser#table_statement}.
     * @param ctx the parse tree
     */
    fn visit_update_table(&mut self, ctx: &Update_tableContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code select_table_}
     * labeled alternative in {@link SQLParser#table_statement}.
     * @param ctx the parse tree
     */
    fn visit_select_table_(&mut self, ctx: &Select_table_Context<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SQLParser#select_table}.
     * @param ctx the parse tree
     */
    fn visit_select_table(&mut self, ctx: &Select_tableContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code alter_add_index}
     * labeled alternative in {@link SQLParser#alter_statement}.
     * @param ctx the parse tree
     */
    fn visit_alter_add_index(&mut self, ctx: &Alter_add_indexContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code alter_drop_index}
     * labeled alternative in {@link SQLParser#alter_statement}.
     * @param ctx the parse tree
     */
    fn visit_alter_drop_index(&mut self, ctx: &Alter_drop_indexContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code alter_table_drop_pk}
     * labeled alternative in {@link SQLParser#alter_statement}.
     * @param ctx the parse tree
     */
    fn visit_alter_table_drop_pk(&mut self, ctx: &Alter_table_drop_pkContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code alter_table_drop_foreign_key}
     * labeled alternative in {@link SQLParser#alter_statement}.
     * @param ctx the parse tree
     */
    fn visit_alter_table_drop_foreign_key(
        &mut self,
        ctx: &Alter_table_drop_foreign_keyContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code alter_table_add_pk}
     * labeled alternative in {@link SQLParser#alter_statement}.
     * @param ctx the parse tree
     */
    fn visit_alter_table_add_pk(&mut self, ctx: &Alter_table_add_pkContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code alter_table_add_foreign_key}
     * labeled alternative in {@link SQLParser#alter_statement}.
     * @param ctx the parse tree
     */
    fn visit_alter_table_add_foreign_key(
        &mut self,
        ctx: &Alter_table_add_foreign_keyContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code alter_table_add_unique}
     * labeled alternative in {@link SQLParser#alter_statement}.
     * @param ctx the parse tree
     */
    fn visit_alter_table_add_unique(&mut self, ctx: &Alter_table_add_uniqueContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SQLParser#field_list}.
     * @param ctx the parse tree
     */
    fn visit_field_list(&mut self, ctx: &Field_listContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code normal_field}
     * labeled alternative in {@link SQLParser#field}.
     * @param ctx the parse tree
     */
    fn visit_normal_field(&mut self, ctx: &Normal_fieldContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code primary_key_field}
     * labeled alternative in {@link SQLParser#field}.
     * @param ctx the parse tree
     */
    fn visit_primary_key_field(&mut self, ctx: &Primary_key_fieldContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code foreign_key_field}
     * labeled alternative in {@link SQLParser#field}.
     * @param ctx the parse tree
     */
    fn visit_foreign_key_field(&mut self, ctx: &Foreign_key_fieldContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SQLParser#type_}.
     * @param ctx the parse tree
     */
    fn visit_type_(&mut self, ctx: &Type_Context<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SQLParser#value_lists}.
     * @param ctx the parse tree
     */
    fn visit_value_lists(&mut self, ctx: &Value_listsContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SQLParser#value_list}.
     * @param ctx the parse tree
     */
    fn visit_value_list(&mut self, ctx: &Value_listContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SQLParser#value}.
     * @param ctx the parse tree
     */
    fn visit_value(&mut self, ctx: &ValueContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SQLParser#where_and_clause}.
     * @param ctx the parse tree
     */
    fn visit_where_and_clause(&mut self, ctx: &Where_and_clauseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code where_operator_expression}
     * labeled alternative in {@link SQLParser#where_clause}.
     * @param ctx the parse tree
     */
    fn visit_where_operator_expression(&mut self, ctx: &Where_operator_expressionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code where_operator_select}
     * labeled alternative in {@link SQLParser#where_clause}.
     * @param ctx the parse tree
     */
    fn visit_where_operator_select(&mut self, ctx: &Where_operator_selectContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code where_null}
     * labeled alternative in {@link SQLParser#where_clause}.
     * @param ctx the parse tree
     */
    fn visit_where_null(&mut self, ctx: &Where_nullContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code where_in_list}
     * labeled alternative in {@link SQLParser#where_clause}.
     * @param ctx the parse tree
     */
    fn visit_where_in_list(&mut self, ctx: &Where_in_listContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code where_in_select}
     * labeled alternative in {@link SQLParser#where_clause}.
     * @param ctx the parse tree
     */
    fn visit_where_in_select(&mut self, ctx: &Where_in_selectContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code where_like_string}
     * labeled alternative in {@link SQLParser#where_clause}.
     * @param ctx the parse tree
     */
    fn visit_where_like_string(&mut self, ctx: &Where_like_stringContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SQLParser#column}.
     * @param ctx the parse tree
     */
    fn visit_column(&mut self, ctx: &ColumnContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SQLParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SQLParser#set_clause}.
     * @param ctx the parse tree
     */
    fn visit_set_clause(&mut self, ctx: &Set_clauseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SQLParser#selectors}.
     * @param ctx the parse tree
     */
    fn visit_selectors(&mut self, ctx: &SelectorsContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SQLParser#selector}.
     * @param ctx the parse tree
     */
    fn visit_selector(&mut self, ctx: &SelectorContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SQLParser#identifiers}.
     * @param ctx the parse tree
     */
    fn visit_identifiers(&mut self, ctx: &IdentifiersContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SQLParser#operator_}.
     * @param ctx the parse tree
     */
    fn visit_operator_(&mut self, ctx: &Operator_Context<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SQLParser#aggregator}.
     * @param ctx the parse tree
     */
    fn visit_aggregator(&mut self, ctx: &AggregatorContext<'input>) {
        self.visit_children(ctx)
    }
}

pub trait SQLVisitorCompat<'input>:
    ParseTreeVisitorCompat<'input, Node = SQLParserContextType>
{
    /**
     * Visit a parse tree produced by {@link SQLParser#program}.
     * @param ctx the parse tree
     */
    fn visit_program(&mut self, ctx: &ProgramContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SQLParser#statement}.
     * @param ctx the parse tree
     */
    fn visit_statement(&mut self, ctx: &StatementContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code create_db}
     * labeled alternative in {@link SQLParser#db_statement}.
     * @param ctx the parse tree
     */
    fn visit_create_db(&mut self, ctx: &Create_dbContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code drop_db}
     * labeled alternative in {@link SQLParser#db_statement}.
     * @param ctx the parse tree
     */
    fn visit_drop_db(&mut self, ctx: &Drop_dbContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code show_dbs}
     * labeled alternative in {@link SQLParser#db_statement}.
     * @param ctx the parse tree
     */
    fn visit_show_dbs(&mut self, ctx: &Show_dbsContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code use_db}
     * labeled alternative in {@link SQLParser#db_statement}.
     * @param ctx the parse tree
     */
    fn visit_use_db(&mut self, ctx: &Use_dbContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code show_tables}
     * labeled alternative in {@link SQLParser#db_statement}.
     * @param ctx the parse tree
     */
    fn visit_show_tables(&mut self, ctx: &Show_tablesContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code show_indexes}
     * labeled alternative in {@link SQLParser#db_statement}.
     * @param ctx the parse tree
     */
    fn visit_show_indexes(&mut self, ctx: &Show_indexesContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code load_data}
     * labeled alternative in {@link SQLParser#io_statement}.
     * @param ctx the parse tree
     */
    fn visit_load_data(&mut self, ctx: &Load_dataContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code dump_data}
     * labeled alternative in {@link SQLParser#io_statement}.
     * @param ctx the parse tree
     */
    fn visit_dump_data(&mut self, ctx: &Dump_dataContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code create_table}
     * labeled alternative in {@link SQLParser#table_statement}.
     * @param ctx the parse tree
     */
    fn visit_create_table(&mut self, ctx: &Create_tableContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code drop_table}
     * labeled alternative in {@link SQLParser#table_statement}.
     * @param ctx the parse tree
     */
    fn visit_drop_table(&mut self, ctx: &Drop_tableContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code describe_table}
     * labeled alternative in {@link SQLParser#table_statement}.
     * @param ctx the parse tree
     */
    fn visit_describe_table(&mut self, ctx: &Describe_tableContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code insert_into_table}
     * labeled alternative in {@link SQLParser#table_statement}.
     * @param ctx the parse tree
     */
    fn visit_insert_into_table(&mut self, ctx: &Insert_into_tableContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code delete_from_table}
     * labeled alternative in {@link SQLParser#table_statement}.
     * @param ctx the parse tree
     */
    fn visit_delete_from_table(&mut self, ctx: &Delete_from_tableContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code update_table}
     * labeled alternative in {@link SQLParser#table_statement}.
     * @param ctx the parse tree
     */
    fn visit_update_table(&mut self, ctx: &Update_tableContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code select_table_}
     * labeled alternative in {@link SQLParser#table_statement}.
     * @param ctx the parse tree
     */
    fn visit_select_table_(&mut self, ctx: &Select_table_Context<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SQLParser#select_table}.
     * @param ctx the parse tree
     */
    fn visit_select_table(&mut self, ctx: &Select_tableContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code alter_add_index}
     * labeled alternative in {@link SQLParser#alter_statement}.
     * @param ctx the parse tree
     */
    fn visit_alter_add_index(&mut self, ctx: &Alter_add_indexContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code alter_drop_index}
     * labeled alternative in {@link SQLParser#alter_statement}.
     * @param ctx the parse tree
     */
    fn visit_alter_drop_index(&mut self, ctx: &Alter_drop_indexContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code alter_table_drop_pk}
     * labeled alternative in {@link SQLParser#alter_statement}.
     * @param ctx the parse tree
     */
    fn visit_alter_table_drop_pk(
        &mut self,
        ctx: &Alter_table_drop_pkContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code alter_table_drop_foreign_key}
     * labeled alternative in {@link SQLParser#alter_statement}.
     * @param ctx the parse tree
     */
    fn visit_alter_table_drop_foreign_key(
        &mut self,
        ctx: &Alter_table_drop_foreign_keyContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code alter_table_add_pk}
     * labeled alternative in {@link SQLParser#alter_statement}.
     * @param ctx the parse tree
     */
    fn visit_alter_table_add_pk(
        &mut self,
        ctx: &Alter_table_add_pkContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code alter_table_add_foreign_key}
     * labeled alternative in {@link SQLParser#alter_statement}.
     * @param ctx the parse tree
     */
    fn visit_alter_table_add_foreign_key(
        &mut self,
        ctx: &Alter_table_add_foreign_keyContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code alter_table_add_unique}
     * labeled alternative in {@link SQLParser#alter_statement}.
     * @param ctx the parse tree
     */
    fn visit_alter_table_add_unique(
        &mut self,
        ctx: &Alter_table_add_uniqueContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SQLParser#field_list}.
     * @param ctx the parse tree
     */
    fn visit_field_list(&mut self, ctx: &Field_listContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code normal_field}
     * labeled alternative in {@link SQLParser#field}.
     * @param ctx the parse tree
     */
    fn visit_normal_field(&mut self, ctx: &Normal_fieldContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code primary_key_field}
     * labeled alternative in {@link SQLParser#field}.
     * @param ctx the parse tree
     */
    fn visit_primary_key_field(&mut self, ctx: &Primary_key_fieldContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code foreign_key_field}
     * labeled alternative in {@link SQLParser#field}.
     * @param ctx the parse tree
     */
    fn visit_foreign_key_field(&mut self, ctx: &Foreign_key_fieldContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SQLParser#type_}.
     * @param ctx the parse tree
     */
    fn visit_type_(&mut self, ctx: &Type_Context<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SQLParser#value_lists}.
     * @param ctx the parse tree
     */
    fn visit_value_lists(&mut self, ctx: &Value_listsContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SQLParser#value_list}.
     * @param ctx the parse tree
     */
    fn visit_value_list(&mut self, ctx: &Value_listContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SQLParser#value}.
     * @param ctx the parse tree
     */
    fn visit_value(&mut self, ctx: &ValueContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SQLParser#where_and_clause}.
     * @param ctx the parse tree
     */
    fn visit_where_and_clause(&mut self, ctx: &Where_and_clauseContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code where_operator_expression}
     * labeled alternative in {@link SQLParser#where_clause}.
     * @param ctx the parse tree
     */
    fn visit_where_operator_expression(
        &mut self,
        ctx: &Where_operator_expressionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code where_operator_select}
     * labeled alternative in {@link SQLParser#where_clause}.
     * @param ctx the parse tree
     */
    fn visit_where_operator_select(
        &mut self,
        ctx: &Where_operator_selectContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code where_null}
     * labeled alternative in {@link SQLParser#where_clause}.
     * @param ctx the parse tree
     */
    fn visit_where_null(&mut self, ctx: &Where_nullContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code where_in_list}
     * labeled alternative in {@link SQLParser#where_clause}.
     * @param ctx the parse tree
     */
    fn visit_where_in_list(&mut self, ctx: &Where_in_listContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code where_in_select}
     * labeled alternative in {@link SQLParser#where_clause}.
     * @param ctx the parse tree
     */
    fn visit_where_in_select(&mut self, ctx: &Where_in_selectContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code where_like_string}
     * labeled alternative in {@link SQLParser#where_clause}.
     * @param ctx the parse tree
     */
    fn visit_where_like_string(&mut self, ctx: &Where_like_stringContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SQLParser#column}.
     * @param ctx the parse tree
     */
    fn visit_column(&mut self, ctx: &ColumnContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SQLParser#expression}.
     * @param ctx the parse tree
     */
    fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SQLParser#set_clause}.
     * @param ctx the parse tree
     */
    fn visit_set_clause(&mut self, ctx: &Set_clauseContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SQLParser#selectors}.
     * @param ctx the parse tree
     */
    fn visit_selectors(&mut self, ctx: &SelectorsContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SQLParser#selector}.
     * @param ctx the parse tree
     */
    fn visit_selector(&mut self, ctx: &SelectorContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SQLParser#identifiers}.
     * @param ctx the parse tree
     */
    fn visit_identifiers(&mut self, ctx: &IdentifiersContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SQLParser#operator_}.
     * @param ctx the parse tree
     */
    fn visit_operator_(&mut self, ctx: &Operator_Context<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link SQLParser#aggregator}.
     * @param ctx the parse tree
     */
    fn visit_aggregator(&mut self, ctx: &AggregatorContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }
}

impl<'input, T> SQLVisitor<'input> for T
where
    T: SQLVisitorCompat<'input>,
{
    fn visit_program(&mut self, ctx: &ProgramContext<'input>) {
        let result = <Self as SQLVisitorCompat>::visit_program(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_statement(&mut self, ctx: &StatementContext<'input>) {
        let result = <Self as SQLVisitorCompat>::visit_statement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_create_db(&mut self, ctx: &Create_dbContext<'input>) {
        let result = <Self as SQLVisitorCompat>::visit_create_db(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_drop_db(&mut self, ctx: &Drop_dbContext<'input>) {
        let result = <Self as SQLVisitorCompat>::visit_drop_db(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_show_dbs(&mut self, ctx: &Show_dbsContext<'input>) {
        let result = <Self as SQLVisitorCompat>::visit_show_dbs(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_use_db(&mut self, ctx: &Use_dbContext<'input>) {
        let result = <Self as SQLVisitorCompat>::visit_use_db(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_show_tables(&mut self, ctx: &Show_tablesContext<'input>) {
        let result = <Self as SQLVisitorCompat>::visit_show_tables(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_show_indexes(&mut self, ctx: &Show_indexesContext<'input>) {
        let result = <Self as SQLVisitorCompat>::visit_show_indexes(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_load_data(&mut self, ctx: &Load_dataContext<'input>) {
        let result = <Self as SQLVisitorCompat>::visit_load_data(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_dump_data(&mut self, ctx: &Dump_dataContext<'input>) {
        let result = <Self as SQLVisitorCompat>::visit_dump_data(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_create_table(&mut self, ctx: &Create_tableContext<'input>) {
        let result = <Self as SQLVisitorCompat>::visit_create_table(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_drop_table(&mut self, ctx: &Drop_tableContext<'input>) {
        let result = <Self as SQLVisitorCompat>::visit_drop_table(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_describe_table(&mut self, ctx: &Describe_tableContext<'input>) {
        let result = <Self as SQLVisitorCompat>::visit_describe_table(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_insert_into_table(&mut self, ctx: &Insert_into_tableContext<'input>) {
        let result = <Self as SQLVisitorCompat>::visit_insert_into_table(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_delete_from_table(&mut self, ctx: &Delete_from_tableContext<'input>) {
        let result = <Self as SQLVisitorCompat>::visit_delete_from_table(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_update_table(&mut self, ctx: &Update_tableContext<'input>) {
        let result = <Self as SQLVisitorCompat>::visit_update_table(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_select_table_(&mut self, ctx: &Select_table_Context<'input>) {
        let result = <Self as SQLVisitorCompat>::visit_select_table_(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_select_table(&mut self, ctx: &Select_tableContext<'input>) {
        let result = <Self as SQLVisitorCompat>::visit_select_table(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_alter_add_index(&mut self, ctx: &Alter_add_indexContext<'input>) {
        let result = <Self as SQLVisitorCompat>::visit_alter_add_index(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_alter_drop_index(&mut self, ctx: &Alter_drop_indexContext<'input>) {
        let result = <Self as SQLVisitorCompat>::visit_alter_drop_index(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_alter_table_drop_pk(&mut self, ctx: &Alter_table_drop_pkContext<'input>) {
        let result = <Self as SQLVisitorCompat>::visit_alter_table_drop_pk(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_alter_table_drop_foreign_key(
        &mut self,
        ctx: &Alter_table_drop_foreign_keyContext<'input>,
    ) {
        let result = <Self as SQLVisitorCompat>::visit_alter_table_drop_foreign_key(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_alter_table_add_pk(&mut self, ctx: &Alter_table_add_pkContext<'input>) {
        let result = <Self as SQLVisitorCompat>::visit_alter_table_add_pk(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_alter_table_add_foreign_key(
        &mut self,
        ctx: &Alter_table_add_foreign_keyContext<'input>,
    ) {
        let result = <Self as SQLVisitorCompat>::visit_alter_table_add_foreign_key(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_alter_table_add_unique(&mut self, ctx: &Alter_table_add_uniqueContext<'input>) {
        let result = <Self as SQLVisitorCompat>::visit_alter_table_add_unique(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_field_list(&mut self, ctx: &Field_listContext<'input>) {
        let result = <Self as SQLVisitorCompat>::visit_field_list(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_normal_field(&mut self, ctx: &Normal_fieldContext<'input>) {
        let result = <Self as SQLVisitorCompat>::visit_normal_field(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_primary_key_field(&mut self, ctx: &Primary_key_fieldContext<'input>) {
        let result = <Self as SQLVisitorCompat>::visit_primary_key_field(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_foreign_key_field(&mut self, ctx: &Foreign_key_fieldContext<'input>) {
        let result = <Self as SQLVisitorCompat>::visit_foreign_key_field(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_type_(&mut self, ctx: &Type_Context<'input>) {
        let result = <Self as SQLVisitorCompat>::visit_type_(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_value_lists(&mut self, ctx: &Value_listsContext<'input>) {
        let result = <Self as SQLVisitorCompat>::visit_value_lists(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_value_list(&mut self, ctx: &Value_listContext<'input>) {
        let result = <Self as SQLVisitorCompat>::visit_value_list(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_value(&mut self, ctx: &ValueContext<'input>) {
        let result = <Self as SQLVisitorCompat>::visit_value(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_where_and_clause(&mut self, ctx: &Where_and_clauseContext<'input>) {
        let result = <Self as SQLVisitorCompat>::visit_where_and_clause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_where_operator_expression(&mut self, ctx: &Where_operator_expressionContext<'input>) {
        let result = <Self as SQLVisitorCompat>::visit_where_operator_expression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_where_operator_select(&mut self, ctx: &Where_operator_selectContext<'input>) {
        let result = <Self as SQLVisitorCompat>::visit_where_operator_select(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_where_null(&mut self, ctx: &Where_nullContext<'input>) {
        let result = <Self as SQLVisitorCompat>::visit_where_null(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_where_in_list(&mut self, ctx: &Where_in_listContext<'input>) {
        let result = <Self as SQLVisitorCompat>::visit_where_in_list(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_where_in_select(&mut self, ctx: &Where_in_selectContext<'input>) {
        let result = <Self as SQLVisitorCompat>::visit_where_in_select(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_where_like_string(&mut self, ctx: &Where_like_stringContext<'input>) {
        let result = <Self as SQLVisitorCompat>::visit_where_like_string(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_column(&mut self, ctx: &ColumnContext<'input>) {
        let result = <Self as SQLVisitorCompat>::visit_column(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) {
        let result = <Self as SQLVisitorCompat>::visit_expression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_set_clause(&mut self, ctx: &Set_clauseContext<'input>) {
        let result = <Self as SQLVisitorCompat>::visit_set_clause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_selectors(&mut self, ctx: &SelectorsContext<'input>) {
        let result = <Self as SQLVisitorCompat>::visit_selectors(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_selector(&mut self, ctx: &SelectorContext<'input>) {
        let result = <Self as SQLVisitorCompat>::visit_selector(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_identifiers(&mut self, ctx: &IdentifiersContext<'input>) {
        let result = <Self as SQLVisitorCompat>::visit_identifiers(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_operator_(&mut self, ctx: &Operator_Context<'input>) {
        let result = <Self as SQLVisitorCompat>::visit_operator_(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_aggregator(&mut self, ctx: &AggregatorContext<'input>) {
        let result = <Self as SQLVisitorCompat>::visit_aggregator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }
}
