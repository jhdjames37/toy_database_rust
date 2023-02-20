use crate::filesystem::LRUBufPageManager;
use crate::record::column::{DataType, TableMeta};
use crate::record::data::{DataEntry, DataValue};
use crate::record::table::{Pos, RecordManager};
use crate::util::error::Error as DBError;
use std::cell::RefCell;
use std::rc::Rc;

use super::node::BPTreeNode;
use super::tree::BPlusTree;

#[derive(PartialEq, Debug)]
pub enum DataValueBounded {
    Value(DataValue),
    NegativeInf,
    PositiveInf,
}

impl PartialOrd for DataValueBounded {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (DataValueBounded::NegativeInf, DataValueBounded::NegativeInf)
            | (DataValueBounded::PositiveInf, DataValueBounded::PositiveInf) => {
                Some(std::cmp::Ordering::Equal)
            }
            (DataValueBounded::NegativeInf, _) | (_, DataValueBounded::PositiveInf) => {
                Some(std::cmp::Ordering::Less)
            }
            (DataValueBounded::PositiveInf, _) | (_, DataValueBounded::NegativeInf) => {
                Some(std::cmp::Ordering::Greater)
            }
            (DataValueBounded::Value(x), DataValueBounded::Value(y)) => x.partial_cmp(y),
        }
    }
}
impl Into<DataValueBounded> for DataValue {
    fn into(self) -> DataValueBounded {
        DataValueBounded::Value(self)
    }
}

pub struct IndexManager {
    tree: BPlusTree,
    idx_sel: Vec<usize>,
}

pub enum IndexOptions {
    PrimaryKey,
    Index(u32),
}

impl IndexManager {
    pub fn new(
        path: &str,
        buf: &Rc<RefCell<LRUBufPageManager>>,
        meta: &TableMeta,
        idx: &IndexOptions,
    ) -> Result<IndexManager, Box<dyn std::error::Error>> {
        let (btree_meta, idx_sel): (Vec<DataType>, Vec<usize>) = match idx {
            IndexOptions::PrimaryKey => meta
                .field
                .iter()
                .enumerate()
                .filter(|(_, x)| x.is_primary)
                .map(|(y, x)| (x.data_type.clone(), y))
                .unzip(),
            IndexOptions::Index(idx) => {
                let f = || {
                    for item in &meta.index {
                        if item.id == *idx {
                            return Ok(item
                                .col
                                .iter()
                                .map(|x| {
                                    for (y, f_item) in meta.field.iter().enumerate() {
                                        if x == &f_item.name {
                                            return (f_item.data_type.clone(), y);
                                        }
                                    }
                                    unreachable!();
                                })
                                .unzip());
                        }
                    }
                    Err(DBError::new("index id not found!"))
                };
                f()?
            }
        };
        let tree = BPlusTree::new(path, buf, btree_meta)?;
        Ok(IndexManager { tree, idx_sel })
    }
    pub fn insert(
        &mut self,
        key: &Vec<DataValue>,
        value: &Pos,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let key: Vec<_> = self.idx_sel.iter().map(|x| key[*x].clone()).collect();
        // Since no NULL selection will be executed through index, we don't need to store NULL value
        if key.iter().any(|x| DataValue::Null == *x) {
            return Ok(());
        }
        self.tree.insert(&key, value)
    }
    pub fn remove(
        &mut self,
        key: &Vec<DataValue>,
        value: &Pos,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let key: Vec<_> = self.idx_sel.iter().map(|x| key[*x].clone()).collect();
        // Since no NULL selection will be executed through index, we don't need to store NULL value
        if key.iter().any(|x| DataValue::Null == *x) {
            return Ok(true);
        }
        self.tree.remove(&key, value)
    }

    pub fn find_key(
        &self,
        key: &Vec<DataValue>,
    ) -> Result<Option<Pos>, Box<dyn std::error::Error>> {
        // Since no NULL selection will be executed through index, we don't need to store NULL value
        if key.iter().any(|x| DataValue::Null == *x) {
            return Ok(None);
        }

        self.tree.find(&key)
    }

    pub fn find(&self, key: &Vec<DataValue>) -> Result<Option<Pos>, Box<dyn std::error::Error>> {
        let key: Vec<_> = self.idx_sel.iter().map(|x| key[*x].clone()).collect();
        self.find_key(&key)
    }
}

pub struct IndexIterator<'a> {
    manager: IndexManager,
    record: RecordManager<'a>,
    lower_bound: Vec<DataValueBounded>,
    upper_bound: Vec<DataValueBounded>,
    cur: Option<Pos>,
    first: bool,
    node_cache: Option<(u32, BPTreeNode)>,
    pub error: Option<Box<dyn std::error::Error>>,
}
impl<'a> IndexIterator<'a> {
    pub fn new(
        path: &str,
        buf: &Rc<RefCell<LRUBufPageManager>>,
        meta: &TableMeta,
        idx: &IndexOptions,
        lower_bound: &Vec<Option<DataValue>>,
        upper_bound: &Vec<Option<DataValue>>,
        record: RecordManager<'a>,
    ) -> Result<IndexIterator<'a>, Box<dyn std::error::Error>> {
        let manager = IndexManager::new(path, buf, meta, idx)?;
        let (lower_bound, upper_bound) = manager
            .idx_sel
            .iter()
            .map(|x| {
                (
                    match &lower_bound[*x] {
                        Some(x) => DataValueBounded::Value(x.clone()),
                        None => DataValueBounded::NegativeInf,
                    },
                    match &upper_bound[*x] {
                        Some(x) => DataValueBounded::Value(x.clone()),
                        None => DataValueBounded::PositiveInf,
                    },
                )
            })
            .unzip();
        Ok(IndexIterator {
            manager,
            lower_bound,
            upper_bound,
            cur: None,
            first: true,
            error: None,
            node_cache: None,
            record,
        })
    }
    fn get_item(&self, pos: &Pos) -> Result<Option<DataEntry>, Box<dyn std::error::Error>> {
        self.record.find(pos)
    }

    fn fetch_node(&mut self, idx: u32) -> Result<(), Box<dyn std::error::Error>> {
        if let Some((x, _)) = self.node_cache {
            if x == idx {
                return Ok(());
            }
        }
        self.node_cache = Some((idx, self.manager.tree.get_page_node(idx)?));
        Ok(())
    }
    fn advance(&mut self) -> Result<Option<(Vec<DataValue>, Pos)>, Box<dyn std::error::Error>> {
        let mut page = if let Some(Pos(x, y)) = self.cur {
            (x, y)
        } else {
            unreachable!();
        };
        self.fetch_node(page.0)?;
        page.1 += 1;
        self.cur = Some(Pos(page.0, page.1));
        let node_cache = if let Some(x) = &self.node_cache {
            x
        } else {
            unreachable!();
        };
        if page.1 as usize >= node_cache.1.data.len() {
            if let Some(x) = node_cache.1.succ {
                self.cur = Some(Pos(x, 0));
                self.fetch_node(x)?;
                let node_cache = if let Some(x) = &self.node_cache {
                    x
                } else {
                    unreachable!();
                };
                Ok(Some(node_cache.1.data[0].clone()))
            } else {
                self.cur = None;
                Ok(None)
            }
        } else {
            Ok(Some(node_cache.1.data[page.1 as usize].clone()))
        }
    }

    fn next_wrap(&mut self) -> Result<Option<DataEntry>, Box<dyn std::error::Error>> {
        Ok(if self.first {
            self.first = false;
            if let Some((x, y)) = self.manager.tree.lower_bound(&self.lower_bound)? {
                self.cur = Some(x);
                self.get_item(&y)?
            } else {
                self.cur = None;
                None
            }
        } else {
            if self.cur.is_some() {
                if let Some((t, x)) = self.advance()? {
                    if t.into_iter()
                        .map(|x| x.into())
                        .collect::<Vec<DataValueBounded>>()
                        > self.upper_bound
                    {
                        None
                    } else {
                        self.get_item(&x)?
                    }
                } else {
                    None
                }
            } else {
                None
            }
        })
    }
}
impl Iterator for IndexIterator<'_> {
    type Item = DataEntry;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_wrap() {
            Ok(x) => x,
            Err(e) => {
                self.error = Some(e);
                None
            }
        }
    }
}
