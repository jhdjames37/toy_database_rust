use crate::record::{
    column::{DataType, TableMetaManager},
    data::DataValue,
};

use super::{
    types::{Aggregator, Cmp, FilterRowType, SelectTree, SelectTreeVisitor, Selector, Selectors},
    Header,
};

use std::cell::RefCell;
use std::rc::Rc;

use crate::util::error::Error as DBError;

pub struct SelectValidator {
    meta: Rc<RefCell<Option<TableMetaManager>>>,
}

impl SelectValidator {
    pub fn new(meta_temp: &Rc<RefCell<Option<TableMetaManager>>>) -> SelectValidator {
        SelectValidator {
            meta: Rc::clone(meta_temp),
        }
    }
    pub fn run(&mut self, tree: &SelectTree) -> Result<(), Box<dyn std::error::Error>> {
        self.visit(tree)?;
        Ok(())
    }
}

// helper function in visit select column
fn check_conflict_col(
    col: &String,
    child_data: &Vec<(String, String, bool)>,
) -> Result<(), Box<dyn std::error::Error>> {
    // check whether there is only a valid field
    // of this column
    let mut valid = 0;
    // for the sake of convience
    // use sequential search
    // O(nm)

    // it seems that we have no method to
    // distinguish col from the same table
    // so we don't check it
    for (_table, child_col, _) in child_data {
        if col == child_col {
            valid += 1
        }
    }
    if valid == 0 {
        return Err(Box::new(DBError::new(&format!("Column {} not found", col))));
    } else if valid >= 2 {
        return Err(Box::new(DBError::new(&format!(
            "Column {} duplicates",
            col
        ))));
    }
    Ok(())
}

fn check_conflict_table_col(
    tab: &String,
    col: &String,
    child_data: &Vec<(String, String, bool)>,
) -> Result<(), Box<dyn std::error::Error>> {
    // it is almost the same as above
    let mut valid = 0;
    for (child_table, child_col, _) in child_data {
        if col == child_col && tab == child_table {
            valid += 1
        }
    }

    if valid == 0 {
        return Err(Box::new(DBError::new(&format!(
            "Column {}.{} not found",
            tab, col
        ))));
    } else if valid >= 2 {
        return Err(Box::new(DBError::new(&format!(
            "Column {}.{} duplicates",
            tab, col
        ))));
    }
    Ok(())
}

fn check_conflict_selector(
    sel: &Selector,
    child_data: &Vec<(String, String, bool)>,
) -> Result<(), Box<dyn std::error::Error>> {
    match sel {
        Selector::Column(col) => check_conflict_col(col, child_data),
        Selector::TableColumn(tab, col) => check_conflict_table_col(tab, col, child_data),
    }
}
fn get_idx(sel: &Selector, dat: &[(String, String, bool)]) -> usize {
    match sel {
        Selector::Column(col) => {
            for (i, item) in dat.iter().enumerate() {
                if *col == item.1 {
                    return i;
                }
            }
        }
        Selector::TableColumn(tab, col) => {
            for (i, item) in dat.iter().enumerate() {
                if *tab == item.0 && *col == item.1 {
                    return i;
                }
            }
        }
    }
    unreachable!();
}

impl SelectTreeVisitor<'_> for SelectValidator {
    type Return = Result<(Header, Vec<DataType>), Box<dyn std::error::Error>>;

    fn visit_select_column(&'_ mut self, node: &super::types::SelectColumn) -> Self::Return {
        if let Some(child) = node.data.as_ref() {
            let (child_data, data_type) = if let (Header::Selector(x), y) = self.visit(child)? {
                (x, y)
            } else {
                return Err(Box::new(DBError::new("Cannot select on aggregated data!")));
            };

            match &node.selector {
                Selectors::All => Ok((Header::Selector(child_data), data_type)),
                Selectors::Select(select_list) => {
                    let mut res = vec![];
                    let mut tp_res = vec![];
                    for select_item in select_list {
                        match select_item {
                            Selector::Column(col) => {
                                check_conflict_col(col, &child_data)?;
                                res.push(("".to_string(), col.clone(), true));
                                tp_res.push(data_type[get_idx(select_item, &child_data)].clone());
                            }
                            Selector::TableColumn(tab, col) => {
                                check_conflict_table_col(tab, col, &child_data)?;
                                res.push((tab.clone(), col.clone(), true));
                                tp_res.push(data_type[get_idx(select_item, &child_data)].clone());
                            }
                        }
                    }
                    Ok((Header::Selector(res), tp_res))
                }
                Selectors::Aggregate(aggregator_list) => {
                    let mut res = vec![];
                    for agg in aggregator_list {
                        match agg {
                            Aggregator::CountAll => {
                                res.push((
                                    "".to_string(),
                                    "*".to_string(),
                                    "count".to_string(),
                                    true,
                                ));
                                continue;
                            }
                            Aggregator::Max(x)
                            | Aggregator::Average(x)
                            | Aggregator::Min(x)
                            | Aggregator::Sum(x)
                            | Aggregator::Count(x) => {
                                // we cannot check its type here,
                                // as `string` cannot do Sum and Avg.
                                // so just leave it unchecked here
                                match x {
                                    Selector::Column(col) => {
                                        check_conflict_col(col, &child_data)?;
                                        res.push((
                                            "".to_string(),
                                            col.clone(),
                                            "some_op".to_string(),
                                            true,
                                        ));
                                    }
                                    Selector::TableColumn(tab, col) => {
                                        check_conflict_table_col(tab, col, &child_data)?;
                                        res.push((
                                            tab.clone(),
                                            col.clone(),
                                            "some_op".to_string(),
                                            true,
                                        ));
                                    }
                                }
                            }
                        }
                    }
                    Ok((Header::Aggregator(res), vec![]))
                }
            }
        } else {
            Err(Box::new(DBError::new("[Internal] Imcomplete Query Tree!")))
        }
    }

    fn visit_select_table(&'_ mut self, node: &super::types::SelectTable) -> Self::Return {
        if let Some(ctx) = &*self.meta.try_borrow()? {
            let meta = ctx.read_table(&node.name)?;
            Ok((
                Header::Selector(
                    meta.field
                        .iter()
                        .map(|x| (node.name.clone(), x.name.clone(), true))
                        .collect(),
                ),
                meta.field.iter().map(|x| x.data_type.clone()).collect(),
            ))
        } else {
            Err(Box::new(DBError::new("database not found")))
        }
    }

    fn visit_filter_row(&'_ mut self, node: &super::types::FilterRow) -> Self::Return {
        if let Some(child) = node.data.as_ref() {
            let (child_data, data_type) = if let (Header::Selector(x), y) = self.visit(child)? {
                (x, y)
            } else {
                return Err(Box::new(DBError::new("Cannot select on aggregated data!")));
            };

            match &node.kind {
                FilterRowType::CmpValue(col, _, value) => {
                    check_conflict_selector(col, &child_data)?;
                    match (&data_type[get_idx(col, &child_data)], value) {
                        (DataType::Int, DataValue::Int(_))
                        | (DataType::Float, DataValue::Float(_))
                        | (DataType::VarStr(_), DataValue::VarStr(_)) => {}
                        _ => return Err(Box::new(DBError::new("Inconsistent filter type!"))),
                    }
                }
                FilterRowType::CmpColumn(col1, _, col2) => {
                    check_conflict_selector(col1, &child_data)?;
                    check_conflict_selector(col2, &child_data)?;
                    match (
                        &data_type[get_idx(col1, &child_data)],
                        &data_type[get_idx(col2, &child_data)],
                    ) {
                        (DataType::Int, DataType::Int)
                        | (DataType::Float, DataType::Float)
                        | (DataType::VarStr(_), DataType::VarStr(_)) => {}
                        _ => return Err(Box::new(DBError::new("Inconsistent filter type!"))),
                    }
                }
                FilterRowType::Like(col, _) => {
                    check_conflict_selector(col, &child_data)?;
                    match &data_type[get_idx(col, &child_data)] {
                        DataType::VarStr(_) => {}
                        _ => return Err(Box::new(DBError::new("Inconsistent filter type!"))),
                    }
                }
                FilterRowType::In(col, value) => {
                    check_conflict_selector(col, &child_data)?;
                    for item in value {
                        match (&data_type[get_idx(col, &child_data)], item) {
                            (DataType::Int, DataValue::Int(_))
                            | (DataType::Float, DataValue::Float(_))
                            | (DataType::VarStr(_), DataValue::VarStr(_)) => {}
                            _ => return Err(Box::new(DBError::new("Inconsistent filter type!"))),
                        }
                    }
                }
                FilterRowType::CmpSelect(col, _, sel) => {
                    check_conflict_selector(col, &child_data)?;
                    let mut sub_validator = SelectValidator::new(&self.meta);
                    let (_, sub_type) = sub_validator.visit(sel)?;
                    if sub_type.len() > 1 || sub_type.is_empty() {
                        return Err(Box::new(DBError::new("Sub SELECT number not match!")));
                    }
                    match (&data_type[get_idx(col, &child_data)], &sub_type[0]) {
                        (DataType::Int, DataType::Int)
                        | (DataType::Float, DataType::Float)
                        | (DataType::VarStr(_), DataType::VarStr(_)) => {}
                        _ => return Err(Box::new(DBError::new("Inconsistent filter type!"))),
                    }
                }
                FilterRowType::InSelect(col, sel) => {
                    check_conflict_selector(col, &child_data)?;
                    let mut sub_validator = SelectValidator::new(&self.meta);
                    let (_, sub_type) = sub_validator.visit(sel)?;
                    if sub_type.len() > 1 || sub_type.is_empty() {
                        return Err(Box::new(DBError::new("Sub SELECT number not match!")));
                    }
                    match (&data_type[get_idx(col, &child_data)], &sub_type[0]) {
                        (DataType::Int, DataType::Int)
                        | (DataType::Float, DataType::Float)
                        | (DataType::VarStr(_), DataType::VarStr(_)) => {}
                        _ => return Err(Box::new(DBError::new("Inconsistent filter type!"))),
                    }
                }
            }

            Ok((Header::Selector(child_data), data_type))
        } else {
            Err(Box::new(DBError::new("[Internal] Imcomplete Query Tree!")))
        }
    }
    fn visit_join_table(&'_ mut self, node: &super::types::JoinTable) -> Self::Return {
        if let (Some(child1), Some(child2)) = (node.data1.as_ref(), node.data2.as_ref()) {
            let (mut child1_data, mut data1_type) =
                if let (Header::Selector(x), y) = self.visit(child1)? {
                    (x, y)
                } else {
                    return Err(Box::new(DBError::new("Cannot select on aggregated data!")));
                };
            let (mut child2_data, mut data2_type) =
                if let (Header::Selector(x), y) = self.visit(child2)? {
                    (x, y)
                } else {
                    return Err(Box::new(DBError::new("Cannot select on aggregated data!")));
                };
            child1_data.append(&mut child2_data);
            data1_type.append(&mut data2_type);
            Ok((Header::Selector(child1_data), data1_type))
        } else {
            Err(Box::new(DBError::new("[Internal] Imcomplete Query Tree!")))
        }
    }

    fn visit_offset_limit(&'_ mut self, node: &super::types::OffsetLimit) -> Self::Return {
        if let Some(child) = node.data.as_ref() {
            self.visit(child)
        } else {
            Err(Box::new(DBError::new("[Internal] Imcomplete Query Tree!")))
        }
    }
}
