use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use crate::index::manager::{IndexIterator, IndexOptions};
use crate::query::types::Selector;
use crate::record::column::{DataType, TableMetaManager};
use crate::record::data::DataValue;
use crate::util::error::Error as DBError;
use crate::util::iter::{CartesianEqIter, CartesianIter};
use crate::util::regex::Matcher;
use crate::{
    filesystem::LRUBufPageManager,
    record::{column::TableMeta, data::DataEntry, table::RecordManager},
};

use super::types::{Aggregator, Cmp, FilterRowType, Selectors};
use super::Header;
use super::{
    types::{SelectTree, SelectTreeVisitor},
    QueryResult,
};

pub struct RecordQuery {
    path: String,
    meta: Rc<RefCell<Option<TableMetaManager>>>,
    meta_pool: Vec<TableMeta>,
    buf: Rc<RefCell<LRUBufPageManager>>,
    filters: Vec<(bool, Selector, DataValue)>,
    filter_eq: Vec<(Selector, Selector)>,
}

impl RecordQuery {
    pub fn new(
        path: &str,
        buf: &Rc<RefCell<LRUBufPageManager>>,
        meta: &Rc<RefCell<Option<TableMetaManager>>>,
    ) -> Result<RecordQuery, Box<dyn std::error::Error>> {
        Ok(RecordQuery {
            path: path.to_string(),
            buf: Rc::clone(buf),
            meta: Rc::clone(meta),
            meta_pool: vec![],
            filters: vec![],
            filter_eq: vec![],
        })
    }
    pub fn run(&mut self, tree: &SelectTree) -> Result<QueryResult, Box<dyn std::error::Error>> {
        let res = self.visit(tree)?;
        Ok(QueryResult::Result(
            match res.0 {
                Header::Selector(r) => {
                    let col: Vec<_> = r.iter().map(|x| x.1.clone()).collect();
                    r.into_iter()
                        .map(|item| {
                            if item.0.is_empty()
                                || (item.2
                                    && col.iter().map(|x| (item.1 == *x) as i32).sum::<i32>() == 1)
                            {
                                item.1
                            } else {
                                format!("{}.{}", item.0, item.1)
                            }
                        })
                        .collect()
                }
                Header::Aggregator(r) => {
                    let col: Vec<_> = r.iter().map(|x| x.1.clone()).collect();
                    r.into_iter()
                        .map(|item| {
                            if item.0.is_empty()
                                || (item.3
                                    && col.iter().map(|x| (item.1 == *x) as i32).sum::<i32>() == 1)
                            {
                                format!("{}({})", item.2, item.1)
                            } else {
                                format!("{}({}.{})", item.2, item.0, item.1)
                            }
                        })
                        .collect()
                }
            },
            res.1.collect(),
        ))
    }
}

// The lifetime indicated at trait will limit the lifetime check
// in the trait bound.
impl<'a> SelectTreeVisitor<'a> for RecordQuery {
    type Return = Result<
        (
            // (Table, Column)
            Header,
            // A iterator of Vec<DataValue>, i.e. a row of data
            Box<dyn Iterator<Item = Vec<DataValue>> + 'a>,
        ),
        Box<dyn std::error::Error>,
    >;

    fn visit_select_column(&'a mut self, node: &super::types::SelectColumn) -> Self::Return {
        let res = if let (Header::Selector(x), y) = self.visit(node.data.as_ref().unwrap())? {
            (x, y)
        } else {
            unreachable!();
        };
        match &node.selector {
            // select * ...
            Selectors::All => Ok((Header::Selector(res.0), res.1)),
            Selectors::Select(list) => {
                // select new table header and
                // new table content (index)
                let (new_meta, idx): (Vec<_>, Vec<_>) = list
                    .iter()
                    .map(|x| {
                        match &x {
                            Selector::Column(col) => {
                                // `col' is in the selector
                                // `item' is from the subtree
                                for (i, item) in res.0.iter().enumerate() {
                                    if *col == item.1 {
                                        return ((String::from(""), item.1.clone(), item.2), i);
                                    }
                                }
                            }
                            Selector::TableColumn(tab, col) => {
                                for (i, item) in res.0.iter().enumerate() {
                                    if *tab == item.0 && *col == item.1 {
                                        return ((item.0.clone(), item.1.clone(), false), i);
                                    }
                                }
                            }
                        }
                        unreachable!();
                    })
                    .unzip();
                Ok((
                    Header::Selector(new_meta),
                    Box::new(
                        res.1
                            .map(move |entry| idx.iter().map(|idx| entry[*idx].clone()).collect()),
                    ),
                ))
            }
            Selectors::Aggregate(agg_list) => {
                // Reduce Data type
                enum Data {
                    Count(i32),
                    Average(Option<DataValue>, i32),
                    Other(Option<DataValue>),
                }
                fn data_add(x: &Option<DataValue>, y: &DataValue) -> Result<DataValue, DBError> {
                    if let Some(x) = x {
                        match (x, y) {
                            (DataValue::Int(x), DataValue::Int(y)) => Ok(DataValue::Int(x + y)),
                            (DataValue::Float(x), DataValue::Float(y)) => {
                                Ok(DataValue::Float(x + y))
                            }
                            (DataValue::VarStr(_), DataValue::VarStr(_)) => {
                                Err(DBError::new("cannot add string"))
                            }
                            (_, DataValue::Null) | (DataValue::Null, _) => Ok(DataValue::Null),
                            _ => Err(DBError::new("[internal] inconsistenct type")),
                        }
                    } else {
                        Ok(y.clone())
                    }
                }
                fn data_max(x: &Option<DataValue>, y: &DataValue) -> Result<DataValue, DBError> {
                    if let Some(x) = x {
                        match (x, y) {
                            (DataValue::Int(x), DataValue::Int(y)) => {
                                Ok(DataValue::Int(if x > y { *x } else { *y }))
                            }
                            (DataValue::Float(x), DataValue::Float(y)) => {
                                Ok(DataValue::Float(if x > y { *x } else { *y }))
                            }
                            (DataValue::VarStr(x), DataValue::VarStr(y)) => {
                                Ok(DataValue::VarStr(if x > y { x.clone() } else { y.clone() }))
                            }
                            (_, DataValue::Null) | (DataValue::Null, _) => Ok(DataValue::Null),
                            _ => Err(DBError::new("[internal] inconsistenct type")),
                        }
                    } else {
                        Ok(y.clone())
                    }
                }
                fn data_min(x: &Option<DataValue>, y: &DataValue) -> Result<DataValue, DBError> {
                    if let Some(x) = x {
                        match (x, y) {
                            (DataValue::Int(x), DataValue::Int(y)) => {
                                Ok(DataValue::Int(if x < y { *x } else { *y }))
                            }
                            (DataValue::Float(x), DataValue::Float(y)) => {
                                Ok(DataValue::Float(if x < y { *x } else { *y }))
                            }
                            (DataValue::VarStr(x), DataValue::VarStr(y)) => {
                                Ok(DataValue::VarStr(if x < y { x.clone() } else { y.clone() }))
                            }
                            (_, DataValue::Null) | (DataValue::Null, _) => Ok(DataValue::Null),
                            _ => Err(DBError::new("[internal] inconsistenct type")),
                        }
                    } else {
                        Ok(y.clone())
                    }
                }

                // Helpers
                let get_idx = |sel: &Selector| -> usize {
                    match sel {
                        Selector::Column(col) => {
                            for (i, item) in res.0.iter().enumerate() {
                                if *col == item.1 {
                                    return i;
                                }
                            }
                        }
                        Selector::TableColumn(tab, col) => {
                            for (i, item) in res.0.iter().enumerate() {
                                if *tab == item.0 && *col == item.1 {
                                    return i;
                                }
                            }
                        }
                    }
                    unreachable!();
                };
                let get_name = |sel: &Selector, name: &str| -> (String, String, String, bool) {
                    match sel {
                        Selector::Column(col) => {
                            ("".to_string(), col.clone(), name.to_string(), true)
                        }
                        Selector::TableColumn(tab, col) => {
                            (tab.clone(), col.clone(), name.to_string(), false)
                        }
                    }
                };
                // The idea of this function
                // is to reduce all columns all together.
                // in order to do this, we have to caculate not only
                // header and idx, but also all reduce function (and inital value)
                type ReduceFn = dyn Fn(Result<Data, DBError>, &DataValue) -> Result<Data, DBError>;

                let (new_meta, (idx, (inital_data, reduce))): (
                    Vec<_>,
                    (Vec<Option<usize>>, (Vec<Result<Data, DBError>>,
                                          Vec<Box<ReduceFn>>)),
                ) = agg_list
                    .iter()
                    .map(|x| match x {
                        Aggregator::CountAll => (
                            ("".to_string(), "*".to_string(), "Count".to_string(), true), // meta
                            (
                                None, // idx: No Column is need for Count(*)
                                (
                                    Ok(Data::Count(0)), // inital
                                    // reduce fn
                                    Box::new(|x: Result<Data, DBError>,
                                             _: &DataValue|
                                     -> Result<Data, DBError> {
                                        match x? {
                                            Data::Count(x) => Ok(Data::Count(x + 1)),
                                            _ => Err(DBError::new(
                                                "Unexpected Err(mismatch) in COUNT(*)",
                                            )),
                                        }
                                    }) as Box<ReduceFn>,
                                ),
                            ),
                        ),
                        Aggregator::Count(sel) => (
                            get_name(sel, "Count"), // meta
                            (
                                Some(get_idx(sel)), // idx: 
                                (
                                    Ok(Data::Count(0)), // inital
                                    // reduce fn
                                    Box::new(|x: Result<Data, DBError>,
                                             _: &DataValue|
                                     -> Result<Data, DBError> {
                                        match x? {
                                            Data::Count(x) => Ok(Data::Count(x + 1)),
                                            _ => Err(DBError::new(
                                                "Unexpected Err(mismatch) in COUNT",
                                            )),
                                        }
                                    }) as Box<ReduceFn>,
                                ),
                            ),
                        ),
                        Aggregator::Average(sel) => (
                            get_name(sel, "Average"), // meta
                            (
                                Some(get_idx(sel)), // idx: 
                                (
                                    Ok(Data::Average(None, 0)), // inital
                                    // reduce fn
                                    Box::new(|x: Result<Data, DBError>,
                                             y: &DataValue|
                                     -> Result<Data, DBError> {
                                        match x? {
                                            Data::Average(data, cnt) => Ok(Data::Average(Some(data_add(&data, y)?), cnt + 1)),
                                            _ => Err(DBError::new(
                                                "Unexpected Err(mismatch) in AVERAGE",
                                            )),
                                        }
                                    }) as Box<ReduceFn>,
                                ),
                            ),
                        ),
                        Aggregator::Sum(sel) => (
                            get_name(sel, "Sum"), // meta
                            (
                                Some(get_idx(sel)), // idx: 
                                (
                                    Ok(Data::Other(None)), // inital
                                    // reduce fn
                                    Box::new(|x: Result<Data, DBError>,
                                             y: &DataValue|
                                     -> Result<Data, DBError> {
                                        match x? {
                                            Data::Other(data) => Ok(Data::Other(Some(data_add(&data, y)?))),
                                            _ => Err(DBError::new(
                                                "Unexpected Err(mismatch) in SUM",
                                            )),
                                        }
                                    }) as Box<ReduceFn>,
                                ),
                            ),
                        ),
                        Aggregator::Min(sel) => (
                            get_name(sel, "Min"), // meta
                            (
                                Some(get_idx(sel)), // idx: 
                                (
                                    Ok(Data::Other(None)), // inital
                                    // reduce fn
                                    Box::new(|x: Result<Data, DBError>,
                                             y: &DataValue|
                                     -> Result<Data, DBError> {
                                        match x? {
                                            Data::Other(data) => Ok(Data::Other(Some(data_min(&data, y)?))),
                                            _ => Err(DBError::new(
                                                "Unexpected Err(mismatch) in MIN",
                                            )),
                                        }
                                    }) as Box<ReduceFn>,
                                ),
                            ),
                        ),
                        Aggregator::Max(sel) => (
                            get_name(sel, "Max"), // meta
                            (
                                Some(get_idx(sel)), // idx: 
                                (
                                    Ok(Data::Other(None)), // inital
                                    // reduce fn
                                    Box::new(|x: Result<Data, DBError>,
                                             y: &DataValue|
                                     -> Result<Data, DBError> {
                                        match x? {
                                            Data::Other(data) => Ok(Data::Other(Some(data_max(&data, y)?))),
                                            _ => Err(DBError::new(
                                                "Unexpected Err(mismatch) in MAX",
                                            )),
                                        }
                                    }) as Box<ReduceFn>,
                                ),
                            ),
                        )

                    })
                    .unzip();

                // here, we reduce all results
                let result = res.1.fold(inital_data, |acc, dat| {
                    acc.into_iter()
                        .zip(idx.iter())
                        .zip(reduce.iter())
                        .map(|((acc_ele, idx_ele), reduce_func)| {
                            // reduce one column
                            reduce_func(
                                acc_ele,
                                if let Some(i) = idx_ele {
                                    &dat[*i]
                                } else {
                                    &DataValue::Null
                                },
                            )
                        })
                        .collect()
                });

                let mut res = vec![];
                for item in result {
                    res.push(match item? {
                        Data::Count(x) => DataValue::Int(x),
                        Data::Average(x, y) => {
                            if y == 0 {
                                DataValue::Null
                            } else {
                                match x {
                                    Some(DataValue::Int(t)) => {
                                        DataValue::Float((t as f32) / (y as f32))
                                    }
                                    Some(DataValue::Float(t)) => DataValue::Float(t / (y as f32)),
                                    _ => DataValue::Null,
                                }
                            }
                        }
                        Data::Other(Some(x)) => x,
                        Data::Other(None) => DataValue::Null,
                    });
                }

                Ok((
                    Header::Aggregator(new_meta),
                    Box::new(vec![res].into_iter()),
                ))
            }
        }
    }
    fn visit_select_table(&'a mut self, node: &super::types::SelectTable) -> Self::Return {
        if let Some(ctx) = &*self.meta.try_borrow()? {
            let mut path = PathBuf::from(&self.path);
            path.push(ctx.get_path(&node.name));
            let path = match path.to_str() {
                Some(x) => x.to_string(),
                None => {
                    return Err(Box::new(DBError::new("path error")));
                }
            };
            self.meta_pool.push(ctx.read_table(&node.name)?);
            let meta = self.meta_pool.last().unwrap();

            let mut lower_bound: Vec<Option<DataValue>> = vec![None; meta.field.len()];
            let mut upper_bound: Vec<Option<DataValue>> = vec![None; meta.field.len()];
            for (is_lower, column, value) in &self.filters {
                match column {
                    Selector::Column(col) => {
                        for (i, item) in meta.field.iter().enumerate() {
                            if *col == item.name {
                                if *is_lower {
                                    lower_bound[i] = Some(value.clone())
                                } else {
                                    upper_bound[i] = Some(value.clone())
                                }
                                break;
                            }
                        }
                    }
                    Selector::TableColumn(tab, col) => {
                        if *tab == node.name {
                            for (i, item) in meta.field.iter().enumerate() {
                                if *col == item.name {
                                    if *is_lower {
                                        lower_bound[i] = Some(value.clone())
                                    } else {
                                        upper_bound[i] = Some(value.clone())
                                    }
                                    break;
                                }
                            }
                        }
                    }
                }
            }

            Ok((
                Header::Selector(
                    // create header according to table meta
                    meta.field
                        .iter()
                        .map(|item| (node.name.clone(), item.name.clone(), true))
                        .collect(),
                ),
                // get data
                {
                    // check use primary key
                    let mut options: Vec<_> = meta
                        .index
                        .iter()
                        .map(|x| IndexOptions::Index(x.id))
                        .collect();

                    options.push(IndexOptions::PrimaryKey);
                    match options
                        .iter()
                        .map(|x| (x, check_use_index(&meta, &x, &lower_bound, &upper_bound)))
                        .max_by_key(|x| x.1)
                    {
                        Some((op, val)) if val > 0 => {
                            let mut idx_path = PathBuf::from(&self.path);
                            idx_path.push(ctx.get_index_path(&node.name, &op));
                            let idx_path = match idx_path.to_str() {
                                Some(x) => x.to_string(),
                                None => {
                                    return Err(Box::new(DBError::new("path error")));
                                }
                            };
                            println!(
                                "{}",
                                if let IndexOptions::PrimaryKey = op {
                                    "Use Primary Key Index"
                                } else {
                                    "Use Index"
                                }
                            );

                            Box::new(
                                IndexIterator::new(
                                    &idx_path,
                                    &self.buf,
                                    &meta,
                                    &op,
                                    &lower_bound,
                                    &upper_bound,
                                    RecordManager::new(&path, &self.buf, meta)?,
                                )?
                                .map(|x| x.data),
                            )
                        }
                        _ => Box::new(
                            RecordManager::new(&path, &self.buf, meta)?
                                .into_iter()
                                .map(|x| x.data),
                        ),
                    }
                },
            ))
        } else {
            Err(Box::new(DBError::new("database not found")))
        }
    }

    fn visit_filter_row(&'a mut self, node: &super::types::FilterRow) -> Self::Return {
        let _insert_num = {
            match &node.kind {
                FilterRowType::CmpValue(col, op, value) => match op {
                    Cmp::Eq => {
                        self.filters.push((false, col.clone(), value.clone()));
                        self.filters.push((true, col.clone(), value.clone()));
                        2
                    }
                    Cmp::Lt | Cmp::Le => {
                        self.filters.push((false, col.clone(), value.clone()));
                        1
                    }
                    Cmp::Ge | Cmp::Gt => {
                        self.filters.push((true, col.clone(), value.clone()));
                        1
                    }
                    _ => 0,
                },
                _ => 0,
            }
        };
        {
            match &node.kind {
                FilterRowType::CmpColumn(col1, Cmp::Eq, col2) => {
                    self.filter_eq.push((col1.clone(), col2.clone()))
                }
                _ => {}
            }
        }
        let tmp_path = self.path.clone();
        let tmp_buf = self.buf.clone();
        let tmp_meta = self.meta.clone();

        let res = if let (Header::Selector(x), y) = self.visit(node.data.as_ref().unwrap())? {
            (x, y)
        } else {
            unreachable!();
        };

        let cmp = |x1: &DataValue, op: &Cmp, x2: &DataValue| {
            // TODO: NULL cmp
            match op {
                Cmp::Eq => x1 == x2,
                Cmp::Neq => x1 != x2,
                Cmp::Ge => x1 >= x2,
                Cmp::Gt => x1 > x2,
                Cmp::Le => x1 <= x2,
                Cmp::Lt => x1 < x2,
            }
        };

        let filt_fn: Box<dyn Fn(&Vec<DataValue>) -> bool> = match &node.kind {
            FilterRowType::CmpValue(col, op, value) => {
                let idx = get_idx(col, &res.0);
                let n_op = op.clone();
                let n_value = value.clone();
                Box::new(move |row: &Vec<DataValue>| cmp(&row[idx], &n_op, &n_value))
            }
            FilterRowType::CmpColumn(col1, op, col2) => {
                let idx1 = get_idx(col1, &res.0);
                let idx2 = get_idx(col2, &res.0);
                let n_op = op.clone();
                Box::new(move |row: &Vec<DataValue>| cmp(&row[idx1], &n_op, &row[idx2]))
            }
            FilterRowType::Like(col, val) => {
                let idx = get_idx(col, &res.0);
                let matcher = Matcher::new(val.to_string());
                Box::new(move |row: &Vec<DataValue>| match &row[idx] {
                    DataValue::Null => false,
                    DataValue::VarStr(x) => matcher.check(x),
                    _ => unreachable!(),
                })
            }
            FilterRowType::In(col, val) => {
                let idx = get_idx(col, &res.0);
                let val = val.clone();
                Box::new(move |row: &Vec<DataValue>| val.contains(&row[idx]))
            }
            FilterRowType::CmpSelect(col, op, sel) => {
                let idx = get_idx(col, &res.0);
                let n_op = op.clone();
                // use tmp_* to avoid but immut and mut borrow
                let mut sub_sel = RecordQuery::new(&tmp_path, &tmp_buf, &tmp_meta)?;
                let (_, mut data) = sub_sel.visit(&sel)?;
                if let Some(value) = data.next() {
                    if data.next().is_some() {
                        return Err(Box::new(DBError::new("Multiple result in sub SELECT")));
                    } else {
                        Box::new(move |row: &Vec<DataValue>| cmp(&row[idx], &n_op, &value[0]))
                    }
                } else {
                    return Err(Box::new(DBError::new("No result in sub SELECT")));
                }
            }
            FilterRowType::InSelect(col, sel) => {
                let idx = get_idx(col, &res.0);
                // use tmp_* to avoid but immut and mut borrow
                let mut sub_sel = RecordQuery::new(&tmp_path, &tmp_buf, &tmp_meta)?;
                let (_, data) = sub_sel.visit(&sel)?;
                let value: Vec<_> = data.map(|mut x| x.pop().unwrap()).collect();
                Box::new(move |row: &Vec<DataValue>| value.contains(&row[idx]))
            }
        };

        Ok((Header::Selector(res.0), Box::new(res.1.filter(filt_fn))))
    }

    fn visit_join_table(&'a mut self, node: &super::types::JoinTable) -> Self::Return {
        //todo!();

        let (a1, v1) =
            if let (Header::Selector(x1), y1) = self.visit(node.data1.as_ref().unwrap())? {
                (x1, y1.collect())
            } else {
                unreachable!();
            };

        let (a2, v2) =
            if let (Header::Selector(x2), y2) = self.visit(node.data2.as_ref().unwrap())? {
                (x2, y2.collect())
            } else {
                unreachable!();
            };
        // in order to use return to break the for loop
        let res_iter = (|| -> Box<dyn Iterator<Item = (Vec<DataValue>, Vec<DataValue>)>> {
            // check whether we can use A.x=B.y
            for (col1, col2) in &self.filter_eq {
                let check = |x: &Selector, tab: &String, col: &String| -> bool {
                    match &x {
                        Selector::Column(c) => col == c,
                        Selector::TableColumn(t, c) => t == tab && c == col,
                    }
                };
                let res11 = a1
                    .iter()
                    .enumerate()
                    .find(|(_, (tab, col, _))| check(&col1, tab, col));
                let res12 = a2
                    .iter()
                    .enumerate()
                    .find(|(_, (tab, col, _))| check(&col2, tab, col));
                let res21 = a2
                    .iter()
                    .enumerate()
                    .find(|(_, (tab, col, _))| check(&col1, tab, col));
                let res22 = a1
                    .iter()
                    .enumerate()
                    .find(|(_, (tab, col, _))| check(&col2, tab, col));
                if let (Some((i, _)), Some((j, _))) = (res11, res12) {
                    println!("Use A.x=B.y");
                    return Box::new(CartesianEqIter::new(v1, v2, i, j));
                } else if let (Some((i, _)), Some((j, _))) = (res21, res22) {
                    println!("Use B.y=A.x");
                    return Box::new(CartesianEqIter::new(v1, v2, j, i));
                }
            }
            Box::new(CartesianIter::new(v1, v2))
        })();
        Ok((
            Header::Selector([a1, a2].concat()),
            Box::new(res_iter.map(|(x, y)| [x, y].concat())),
        ))
    }

    fn visit_offset_limit(&'a mut self, node: &super::types::OffsetLimit) -> Self::Return {
        let (header, data) = self.visit(node.data.as_ref().unwrap())?;

        if let Some(offset) = node.offset {
            Ok((
                header,
                Box::new(data.skip(offset as usize).take(node.limit as usize)),
            ))
        } else {
            Ok((header, Box::new(data.take(node.limit as usize))))
        }
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

pub fn check_use_index(
    meta: &TableMeta,
    idx: &IndexOptions,
    lower_bound: &Vec<Option<DataValue>>,
    upper_bound: &Vec<Option<DataValue>>,
) -> i32 {
    let check_idx: Vec<usize> = match idx {
        IndexOptions::PrimaryKey => meta
            .field
            .iter()
            .enumerate()
            .filter(|(_, x)| x.is_primary)
            .map(|(x, _)| x)
            .collect(),
        IndexOptions::Index(id) => {
            let mut res: Option<Vec<usize>> = None;
            for i_item in &meta.index {
                if i_item.id == *id {
                    res = Some(
                        i_item
                            .col
                            .iter()
                            .map(|x| {
                                for (i, item) in meta.field.iter().enumerate() {
                                    if *x == item.name {
                                        return i;
                                    }
                                }
                                unreachable!();
                            })
                            .collect(),
                    );
                    break;
                }
            }
            if let Some(x) = res {
                x
            } else {
                return 0;
            }
        }
    };

    if check_idx.is_empty() {
        0
    } else {
        check_idx
            .iter()
            .map(|x| lower_bound[*x].is_some() as i32 + upper_bound[*x].is_some() as i32)
            .sum()
    }
}
