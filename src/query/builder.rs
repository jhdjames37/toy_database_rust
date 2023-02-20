use antlr_rust::tree::{ParseTree, ParseTreeVisitorCompat};

use crate::parser::sqlparser::{
    AggregatorContextAttrs, Alter_add_indexContextAttrs, Alter_drop_indexContextAttrs,
    Alter_table_add_foreign_keyContextAttrs, Alter_table_add_pkContextAttrs,
    Alter_table_drop_foreign_keyContextAttrs, Alter_table_drop_pkContextAttrs, ColumnContextAttrs,
    Create_dbContext, Create_dbContextAttrs, Create_tableContextAttrs,
    Delete_from_tableContextAttrs, Describe_tableContextAttrs, Drop_dbContextAttrs,
    Drop_tableContextAttrs, ExpressionContextAttrs, Foreign_key_fieldContextAttrs,
    IdentifiersContextAll, IdentifiersContextAttrs, Insert_into_tableContextAttrs,
    Load_dataContextAttrs, Normal_fieldContextAttrs, Operator_ContextAll, Operator_ContextAttrs,
    Primary_key_fieldContextAttrs, Select_tableContextAttrs, SelectorContextAttrs,
    SelectorsContextAttrs, Set_clauseContextAttrs, Type_ContextAttrs, Update_tableContextAttrs,
    Use_dbContextAttrs, ValueContextAttrs, Where_in_listContextAttrs, Where_in_selectContextAttrs,
    Where_like_stringContextAttrs, Where_operator_expressionContextAttrs,
    Where_operator_selectContextAttrs,
};
use crate::parser::{SQLParserContextType, SQLVisitorCompat};
use crate::record::column::DataType;
use crate::record::data::DataValue;

use super::types::{
    Aggregator, Cmp, FilterRow, JoinTable, OffsetLimit, SelectColumn, SelectTable, SelectTree,
    Selector, Selectors,
};

#[derive(Clone)]
pub enum Query {
    CreateDb(String),
    ListDb(),
    DropDb(String),
    UseDb(String),

    LoadData(String, String),

    // table, field
    CreateTable((String, Vec<Field>)),
    CreateTableField(Field),
    DescTable(String),
    ListTable(),
    DropTable(String),

    // ALTER
    CreateIndex(String, Vec<String>),
    DropIndex(String, Vec<String>),
    CreatePrimaryKey(String, Vec<String>),
    DropPrimaryKey(String),
    CreateForeignKey(String, String, Vec<String>, String, Vec<String>),
    DropForeignKey(String, String),

    // table, value
    InsertTable(String, Vec<Vec<DataValue>>),
    SelectTable(SelectTree),
    DeleteTable(String, Vec<FilterRow>),
    UpdateTable(String, Vec<FilterRow>, Vec<(String, DataValue)>),

    ValueList(Vec<DataValue>),
    Value(DataValue),
    Filter(FilterRow),
    Identifier(String),

    Err(String),
}

#[derive(Clone)]
pub enum Field {
    // name, type,
    Column(String, DataType, bool, DataValue),
    PrimaryKey(Vec<String>),
    ForeignKey(String, Vec<String>, String, Vec<String>),
}

impl Default for Query {
    fn default() -> Self {
        Query::Err("".to_string())
    }
}

pub struct QueryBuilder {
    s: Vec<Query>,
}

impl QueryBuilder {
    pub fn new() -> QueryBuilder {
        QueryBuilder { s: vec![] }
    }
}

impl ParseTreeVisitorCompat<'_> for QueryBuilder {
    type Node = SQLParserContextType;
    type Return = Vec<Query>;

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self.s
    }

    fn visit_terminal(
        &mut self,
        _node: &antlr_rust::tree::TerminalNode<'_, Self::Node>,
    ) -> Self::Return {
        vec![]
    }

    fn aggregate_results(&self, aggregate: Self::Return, next: Self::Return) -> Self::Return {
        [aggregate, next].concat()
    }
}

impl SQLVisitorCompat<'_> for QueryBuilder {
    fn visit_create_db(&mut self, ctx: &Create_dbContext<'_>) -> Self::Return {
        vec![Query::CreateDb(ctx.Identifier().unwrap().get_text())]
    }
    fn visit_show_dbs(
        &mut self,
        _ctx: &crate::parser::sqlparser::Show_dbsContext<'_>,
    ) -> Self::Return {
        vec![Query::ListDb()]
    }
    fn visit_drop_db(
        &mut self,
        ctx: &crate::parser::sqlparser::Drop_dbContext<'_>,
    ) -> Self::Return {
        vec![Query::DropDb(ctx.Identifier().unwrap().get_text())]
    }
    fn visit_use_db(&mut self, ctx: &crate::parser::sqlparser::Use_dbContext<'_>) -> Self::Return {
        vec![Query::UseDb(ctx.Identifier().unwrap().get_text())]
    }

    fn visit_load_data(
        &mut self,
        ctx: &crate::parser::sqlparser::Load_dataContext<'_>,
    ) -> Self::Return {
        vec![Query::LoadData(
            ctx.Identifier().unwrap().get_text(),
            ctx.String().unwrap().get_text(),
        )]
    }

    // create table
    // 'CREATE' 'TABLE' Identifier '(' field_list ')'
    //field
    //: Identifier type_ ('NOT' Null)? ('DEFAULT' value)?                            //                   # normal_field
    //| 'PRIMARY' 'KEY' (Identifier)? '(' identifiers ')'                            //                   # primary_key_field
    //| 'FOREIGN' 'KEY' (Identifier)? '(' identifiers ')' 'REFERENCES' Identifier '(' identifiers ')'   # foreign_key_field
    //;
    fn visit_create_table(
        &mut self,
        ctx: &crate::parser::sqlparser::Create_tableContext<'_>,
    ) -> Self::Return {
        vec![Query::CreateTable((
            ctx.Identifier().unwrap().get_text(),
            self.visit(&*ctx.field_list().unwrap())
                .into_iter()
                .map(|t| match t {
                    Query::CreateTableField(x) => x,
                    _ => unreachable!(),
                })
                .collect(),
        ))]
    }
    fn visit_normal_field(
        &mut self,
        ctx: &crate::parser::sqlparser::Normal_fieldContext<'_>,
    ) -> Self::Return {
        vec![Query::CreateTableField(Field::Column(
            // name
            ctx.Identifier().unwrap().get_text(),
            // type
            if ctx.type_().unwrap().get_text() == "INT" {
                DataType::Int
            } else if ctx.type_().unwrap().get_text() == "FLOAT" {
                DataType::Float
            } else {
                DataType::VarStr(
                    ctx.type_()
                        .unwrap()
                        .Integer()
                        .unwrap()
                        .get_text()
                        .parse()
                        .unwrap_or_default(),
                )
            },
            // not null
            ctx.Null().is_some(),
            // default
            if let Some(value) = ctx.value() {
                let mut res = self.visit(&*value);
                match &mut res[0] {
                    Query::Value(x) => std::mem::take(x),
                    _ => unreachable!(),
                }
            } else {
                DataValue::Null
            }, //DataValue::Null
        ))]
    }

    fn visit_primary_key_field(
        &mut self,
        ctx: &crate::parser::sqlparser::Primary_key_fieldContext<'_>,
    ) -> Self::Return {
        vec![Query::CreateTableField(Field::PrimaryKey(
            ctx.identifiers()
                .unwrap()
                .Identifier_all()
                .into_iter()
                .map(|x| x.get_text())
                .filter(|x| *x != ",".to_string()) // Probably some bug (or feature)
                .collect(),
        ))]
    }
    fn visit_foreign_key_field(
        &mut self,
        ctx: &crate::parser::sqlparser::Foreign_key_fieldContext<'_>,
    ) -> Self::Return {
        let (name, refer) = if ctx.Identifier(1).is_some() {
            (
                ctx.Identifier(0).unwrap().get_text(),
                ctx.Identifier(1).unwrap().get_text(),
            )
        } else {
            ("".to_string(), ctx.Identifier(0).unwrap().get_text())
        };

        vec![Query::CreateTableField(Field::ForeignKey(
            name,
            ctx.identifiers(0)
                .unwrap()
                .Identifier_all()
                .into_iter()
                .map(|x| x.get_text())
                .filter(|x| *x != ",".to_string()) // Probably some bug (or feature)
                .collect(),
            refer,
            ctx.identifiers(1)
                .unwrap()
                .Identifier_all()
                .into_iter()
                .map(|x| x.get_text())
                .filter(|x| *x != ",".to_string()) // Probably some bug (or feature)
                .collect(),
        ))]
    }

    // ALTER table
    fn visit_alter_add_index(
        &mut self,
        ctx: &crate::parser::sqlparser::Alter_add_indexContext<'_>,
    ) -> Self::Return {
        vec![Query::CreateIndex(
            ctx.Identifier().unwrap().get_text(),
            get_identifiers(&ctx.identifiers().unwrap()),
        )]
    }
    fn visit_alter_drop_index(
        &mut self,
        ctx: &crate::parser::sqlparser::Alter_drop_indexContext<'_>,
    ) -> Self::Return {
        vec![Query::DropIndex(
            ctx.Identifier().unwrap().get_text(),
            get_identifiers(&ctx.identifiers().unwrap()),
        )]
    }
    fn visit_alter_table_add_pk(
        &mut self,
        ctx: &crate::parser::sqlparser::Alter_table_add_pkContext<'_>,
    ) -> Self::Return {
        vec![Query::CreatePrimaryKey(
            ctx.Identifier(0).unwrap().get_text(),
            get_identifiers(&ctx.identifiers().unwrap()),
        )]
    }
    fn visit_alter_table_drop_pk(
        &mut self,
        ctx: &crate::parser::sqlparser::Alter_table_drop_pkContext<'_>,
    ) -> Self::Return {
        vec![Query::DropPrimaryKey(ctx.Identifier(0).unwrap().get_text())]
    }
    fn visit_alter_table_add_foreign_key(
        &mut self,
        ctx: &crate::parser::sqlparser::Alter_table_add_foreign_keyContext<'_>,
    ) -> Self::Return {
        let (from, name, to) = {
            if ctx.Identifier(2).is_some() {
                (
                    ctx.Identifier(0).unwrap().get_text(),
                    ctx.Identifier(1).unwrap().get_text(),
                    ctx.Identifier(2).unwrap().get_text(),
                )
            } else {
                (
                    ctx.Identifier(0).unwrap().get_text(),
                    "".to_string(),
                    ctx.Identifier(1).unwrap().get_text(),
                )
            }
        };
        vec![Query::CreateForeignKey(
            name,
            from,
            get_identifiers(&ctx.identifiers(0).unwrap()),
            to,
            get_identifiers(&ctx.identifiers(1).unwrap()),
        )]
    }
    fn visit_alter_table_drop_foreign_key(
        &mut self,
        ctx: &crate::parser::sqlparser::Alter_table_drop_foreign_keyContext<'_>,
    ) -> Self::Return {
        vec![Query::DropForeignKey(
            ctx.Identifier(0).unwrap().get_text(),
            ctx.Identifier(1).unwrap().get_text(),
        )]
    }

    fn visit_describe_table(
        &mut self,
        ctx: &crate::parser::sqlparser::Describe_tableContext<'_>,
    ) -> Self::Return {
        vec![Query::DescTable(ctx.Identifier().unwrap().get_text())]
    }

    fn visit_show_tables(
        &mut self,
        ctx: &crate::parser::sqlparser::Show_tablesContext<'_>,
    ) -> Self::Return {
        vec![Query::ListTable()]
    }
    fn visit_drop_table(
        &mut self,
        ctx: &crate::parser::sqlparser::Drop_tableContext<'_>,
    ) -> Self::Return {
        vec![Query::DropTable(ctx.Identifier().unwrap().get_text())]
    }

    // insert table
    // 'INSERT' 'INTO' Identifier 'VALUES' value_lists
    fn visit_insert_into_table(
        &mut self,
        ctx: &crate::parser::sqlparser::Insert_into_tableContext<'_>,
    ) -> Self::Return {
        let t_name = ctx.Identifier().unwrap().get_text();
        let t_value = self.visit(&*ctx.value_lists().unwrap());

        // in this way, we can collect value_lists' result automatically
        vec![Query::InsertTable(
            t_name,
            t_value
                .into_iter()
                .map(|x| {
                    if let Query::ValueList(p) = x {
                        p
                    } else {
                        panic!("Internal Type mismatch! Expected ValueList!");
                    }
                })
                .collect(),
        )]
    }

    // delete from table
    // 'DELETE' 'FROM' Identifier 'WHERE' where_and_clause
    fn visit_delete_from_table(
        &mut self,
        ctx: &crate::parser::sqlparser::Delete_from_tableContext<'_>,
    ) -> Self::Return {
        let name = ctx.Identifier().unwrap().get_text();
        let filter: Vec<_> = self
            .visit(&*ctx.where_and_clause().unwrap())
            .into_iter()
            .map(|x| {
                if let Query::Filter(x) = x {
                    x
                } else {
                    unreachable!();
                }
            })
            .collect();
        vec![Query::DeleteTable(name, filter)]
    }

    // update table
    // 'UPDATE' Identifier 'SET' set_clause 'WHERE' where_and_clause
    // Identifier EqualOrAssign value (',' Identifier EqualOrAssign value)*
    fn visit_update_table(
        &mut self,
        ctx: &crate::parser::sqlparser::Update_tableContext<'_>,
    ) -> Self::Return {
        let name = ctx.Identifier().unwrap().get_text();
        let filter: Vec<_> = self
            .visit(&*ctx.where_and_clause().unwrap())
            .into_iter()
            .map(|x| {
                if let Query::Filter(x) = x {
                    x
                } else {
                    unreachable!();
                }
            })
            .collect();
        let mut idx = 0;
        let mut set: Vec<(String, DataValue)> = vec![];

        // since Identifier_all has bug (verified in the code),
        // use loop instead
        while ctx.set_clause().unwrap().Identifier(idx).is_some() {
            let name = ctx
                .set_clause()
                .unwrap()
                .Identifier(idx)
                .unwrap()
                .get_text();
            let mut value = self.visit(&*ctx.set_clause().unwrap().value(idx).unwrap());
            let value = if let Query::Value(x) = value.pop().unwrap() {
                x
            } else {
                unreachable!();
            };
            set.push((name, value));
            idx += 1;
        }
        vec![Query::UpdateTable(name, filter, set)]
    }

    // value_list (',' value_list)*
    fn visit_value_list(
        &mut self,
        ctx: &crate::parser::sqlparser::Value_listContext<'_>,
    ) -> Self::Return {
        vec![Query::ValueList(
            self.visit_children(ctx)
                .into_iter()
                .map(|x| {
                    if let Query::Value(p) = x {
                        p
                    } else {
                        panic!("Internal type mismatch! Expected Value!");
                    }
                })
                .collect(),
        )]
    }
    fn visit_value(&mut self, ctx: &crate::parser::sqlparser::ValueContext<'_>) -> Self::Return {
        if let Some(x) = ctx.Integer() {
            vec![Query::Value(DataValue::Int(x.get_text().parse().unwrap()))]
        } else if let Some(x) = ctx.Float() {
            vec![Query::Value(DataValue::Float(
                x.get_text().parse().unwrap(),
            ))]
        } else if let Some(x) = ctx.String() {
            vec![Query::Value(DataValue::VarStr(
                x.get_text()
                    .strip_prefix('\'')
                    .unwrap()
                    .strip_suffix('\'')
                    .unwrap()
                    .to_string(),
            ))]
        } else {
            // NULL
            vec![Query::Value(DataValue::Null)]
        }
    }

    // selection
    // 'SELECT' selectors 'FROM' identifiers ('WHERE' where_and_clause)? ('GROUP' 'BY' column)? ('LIMIT' Integer ('OFFSET' Integer)?)?
    fn visit_select_table(
        &mut self,
        ctx: &crate::parser::sqlparser::Select_tableContext<'_>,
    ) -> Self::Return {
        /*
        identifiers
        : Identifier (',' Identifier)*
         */
        let identifiers: Vec<String> = ctx
            .identifiers()
            .unwrap()
            .Identifier_all()
            .into_iter()
            .map(|x| x.get_text())
            .filter(|x| *x != ",".to_string()) // Probably some bug (or feature)
            .collect();

        // parse selectors
        let selectors = ctx.selectors().unwrap().selector_all();
        let mut sel;
        if selectors.is_empty() {
            sel = SelectColumn {
                selector: Selectors::All,
                data: None,
            }
        } else {
            // For the sake of convience,
            // we validate combination of col and aggregator error here

            /*
            selector
            : column
            | aggregator '(' column ')'
            | Count '(' '*' ')'
             */
            let select_num: usize = selectors
                .iter()
                .map(|selector| {
                    if selector.aggregator().is_some() {
                        0
                    } else if selector.Count().is_some() {
                        0
                    } else {
                        1
                    }
                })
                .sum();

            let selector_ret = if select_num == 0 {
                // aggregation
                Selectors::Aggregate(
                    selectors
                        .into_iter()
                        .map(|x| {
                            if x.Count().is_some() {
                                // Count(*)
                                return Aggregator::CountAll;
                            } else if let Some(agg) = x.aggregator() {
                                // get columns
                                let col;
                                if let Some(cols) = x.column() {
                                    let id = cols.Identifier_all();
                                    col = get_col(&id);
                                } else {
                                    unreachable!();
                                };

                                if agg.Count().is_some() {
                                    return Aggregator::Count(col);
                                } else if agg.Average().is_some() {
                                    return Aggregator::Average(col);
                                } else if agg.Sum().is_some() {
                                    return Aggregator::Sum(col);
                                } else if agg.Max().is_some() {
                                    return Aggregator::Max(col);
                                } else if agg.Min().is_some() {
                                    return Aggregator::Min(col);
                                }
                            }
                            unreachable!();
                        })
                        .collect(),
                )
            } else if select_num == selectors.len() {
                // selection
                Selectors::Select(
                    selectors
                        .into_iter()
                        .map(|x| {
                            if let Some(col) = x.column() {
                                let id = col.Identifier_all();
                                return get_col(&id);
                            }
                            unreachable!();
                        })
                        .collect(),
                )
            } else {
                return vec![Query::Err(
                    "Aggregation and selection appear at the same time!".to_string(),
                )];
            };
            sel = SelectColumn {
                selector: selector_ret,
                data: None,
            }
        }

        let filters = if ctx.where_and_clause().is_some() {
            self.visit(&*ctx.where_and_clause().unwrap())
        } else {
            vec![]
        };

        // chain up all params
        let mut iter_id = identifiers.iter();
        let mut table = Box::new(SelectTree::SelectTable(SelectTable {
            name: iter_id.next().unwrap().clone(),
            lower_bound: vec![],
            upper_bound: vec![],
        }));

        for n in iter_id {
            table = Box::new(SelectTree::JoinTable(JoinTable {
                data1: Some(table),
                data2: Some(Box::new(SelectTree::SelectTable(SelectTable {
                    name: n.clone(),
                    lower_bound: vec![],
                    upper_bound: vec![],
                }))),
            }));
        }
        let mut chain = table;
        for filt in filters {
            if let Query::Filter(mut x) = filt {
                x.data = Some(chain);
                chain = Box::new(SelectTree::FilterRow(x));
            } else {
                unreachable!();
            }
        }
        sel.data = Some(chain);

        let sel = SelectTree::SelectColumn(sel);
        // limit and offset
        if let Some(limit) = ctx.Integer(0) {
            vec![Query::SelectTable(SelectTree::OffsetLimit(
                if let Some(offset) = ctx.Integer(1) {
                    OffsetLimit {
                        limit: limit.get_text().parse().unwrap(),
                        offset: Some(offset.get_text().parse().unwrap()),
                        data: Some(Box::new(sel)),
                    }
                } else {
                    OffsetLimit {
                        limit: limit.get_text().parse().unwrap(),
                        offset: None,
                        data: Some(Box::new(sel)),
                    }
                },
            ))]
        } else {
            vec![Query::SelectTable(sel)]
        }
    }

    //: column operator_ expression            # where_operator_expression
    fn visit_where_operator_expression(
        &mut self,
        ctx: &crate::parser::sqlparser::Where_operator_expressionContext<'_>,
    ) -> Self::Return {
        let col = ctx.column().unwrap().Identifier_all();
        let op = ctx.operator_().unwrap();
        let data = ctx.expression().unwrap();
        if data.value().is_some() {
            let mut data = self.visit(&*data.value().unwrap());
            if let Query::Value(x) = std::mem::take(&mut data[0]) {
                vec![Query::Filter(FilterRow {
                    kind: super::types::FilterRowType::CmpValue(get_col(&col), get_op(&op), x),
                    data: None,
                })]
            } else {
                unreachable!();
            }
        } else {
            let col2 = data.column().unwrap().Identifier_all();
            vec![Query::Filter(FilterRow {
                kind: super::types::FilterRowType::CmpColumn(
                    get_col(&col),
                    get_op(&op),
                    get_col(&col2),
                ),
                data: None,
            })]
        }
    }

    fn visit_where_like_string(
        &mut self,
        ctx: &crate::parser::sqlparser::Where_like_stringContext<'_>,
    ) -> Self::Return {
        let col = ctx.column().unwrap().Identifier_all();
        vec![Query::Filter(FilterRow {
            kind: super::types::FilterRowType::Like(
                get_col(&col),
                ctx.String()
                    .unwrap()
                    .get_text()
                    .strip_prefix('\'')
                    .unwrap()
                    .strip_suffix('\'')
                    .unwrap()
                    .to_string(),
            ),
            data: None,
        })]
    }

    fn visit_where_in_list(
        &mut self,
        ctx: &crate::parser::sqlparser::Where_in_listContext<'_>,
    ) -> Self::Return {
        let col = ctx.column().unwrap().Identifier_all();
        let values = match self.visit(&*ctx.value_list().unwrap()).pop().unwrap() {
            Query::ValueList(list) => list,
            _ => unreachable!(),
        };

        vec![Query::Filter(FilterRow {
            kind: super::types::FilterRowType::In(get_col(&col), values),
            data: None,
        })]
    }

    fn visit_where_operator_select(
        &mut self,
        ctx: &crate::parser::sqlparser::Where_operator_selectContext<'_>,
    ) -> Self::Return {
        let col = ctx.column().unwrap().Identifier_all();
        let op = ctx.operator_().unwrap();
        let sub_tree = match self.visit(&*ctx.select_table().unwrap()).pop().unwrap() {
            Query::SelectTable(b) => b,
            _ => unreachable!(),
        };
        vec![Query::Filter(FilterRow {
            kind: super::types::FilterRowType::CmpSelect(
                get_col(&col),
                get_op(&op),
                Box::new(sub_tree),
            ),
            data: None,
        })]
    }

    fn visit_where_in_select(
        &mut self,
        ctx: &crate::parser::sqlparser::Where_in_selectContext<'_>,
    ) -> Self::Return {
        let col = ctx.column().unwrap().Identifier_all();
        let sub_tree = match self.visit(&*ctx.select_table().unwrap()).pop().unwrap() {
            Query::SelectTable(b) => b,
            _ => unreachable!(),
        };
        vec![Query::Filter(FilterRow {
            kind: super::types::FilterRowType::InSelect(get_col(&col), Box::new(sub_tree)),
            data: None,
        })]
    }
}

// helper function
fn get_col(
    col: &Vec<std::rc::Rc<antlr_rust::tree::TerminalNode<SQLParserContextType>>>,
) -> Selector {
    let name: Vec<String> = col
        .iter()
        .map(|x| x.get_text())
        .filter(|x| *x != ".".to_string())
        .collect();
    if name.len() == 2 {
        Selector::TableColumn(name[0].clone(), name[1].clone())
    } else {
        Selector::Column(name[0].clone())
    }
}
fn get_op(op: &std::rc::Rc<Operator_ContextAll>) -> Cmp {
    if op.EqualOrAssign().is_some() {
        Cmp::Eq
    } else if op.Greater().is_some() {
        Cmp::Gt
    } else if op.GreaterEqual().is_some() {
        Cmp::Ge
    } else if op.Less().is_some() {
        Cmp::Lt
    } else if op.LessEqual().is_some() {
        Cmp::Le
    } else {
        Cmp::Neq
    }
}

fn get_identifiers(id: &std::rc::Rc<IdentifiersContextAll>) -> Vec<String> {
    id.Identifier_all()
        .into_iter()
        .map(|x| x.get_text())
        .filter(|x| *x != ",".to_string()) // Probably some bug (or feature)
        .collect()
}
