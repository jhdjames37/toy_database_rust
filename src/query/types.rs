use crate::record::data::{DataEntry, DataValue};

#[derive(Clone)]
pub enum SelectTree {
    SelectTable(SelectTable),
    SelectColumn(SelectColumn),
    FilterRow(FilterRow),
    JoinTable(JoinTable),
    OffsetLimit(OffsetLimit),
}

pub trait SelectTreeVisitor<'a> {
    type Return;
    fn visit_select_table(&'a mut self, node: &SelectTable) -> Self::Return;

    fn visit_select_column(&'a mut self, node: &SelectColumn) -> Self::Return;

    fn visit_filter_row(&'a mut self, node: &FilterRow) -> Self::Return;

    fn visit_join_table(&'a mut self, node: &JoinTable) -> Self::Return;

    fn visit_offset_limit(&'a mut self, node: &OffsetLimit) -> Self::Return;

    fn visit(&'a mut self, node: &SelectTree) -> Self::Return {
        match node {
            SelectTree::SelectTable(x) => self.visit_select_table(x),
            SelectTree::SelectColumn(x) => self.visit_select_column(x),
            SelectTree::FilterRow(x) => self.visit_filter_row(x),
            SelectTree::JoinTable(x) => self.visit_join_table(x),
            SelectTree::OffsetLimit(x) => self.visit_offset_limit(x),
        }
    }
}

#[derive(Clone)]
pub struct SelectTable {
    pub name: String,
    pub lower_bound: Vec<Option<DataValue>>,
    pub upper_bound: Vec<Option<DataValue>>,
}

#[derive(Clone)]
pub enum Selectors {
    All,
    Select(Vec<Selector>),
    Aggregate(Vec<Aggregator>),
}

#[derive(Clone)]
pub struct SelectColumn {
    pub selector: Selectors,
    pub data: Option<Box<SelectTree>>,
}

#[derive(Clone)]
pub enum Selector {
    Column(String),
    TableColumn(String, String),
}

#[derive(Clone)]
pub enum Aggregator {
    CountAll,
    Count(Selector),
    Min(Selector),
    Max(Selector),
    Sum(Selector),
    Average(Selector),
}

#[derive(Clone)]
pub struct FilterRow {
    pub kind: FilterRowType,
    pub data: Option<Box<SelectTree>>,
}

#[derive(Clone)]
pub enum FilterRowType {
    CmpValue(Selector, Cmp, DataValue),
    CmpColumn(Selector, Cmp, Selector),
    Like(Selector, String),
    In(Selector, Vec<DataValue>),
    CmpSelect(Selector, Cmp, Box<SelectTree>),
    InSelect(Selector, Box<SelectTree>),
}

#[derive(Clone)]
pub enum Cmp {
    Eq,
    Lt,
    Le,
    Gt,
    Ge,
    Neq,
}

#[derive(Clone)]
pub struct JoinTable {
    pub data1: Option<Box<SelectTree>>,
    pub data2: Option<Box<SelectTree>>,
}

#[derive(Clone)]
pub struct OffsetLimit {
    pub offset: Option<i32>,
    pub limit: i32,
    pub data: Option<Box<SelectTree>>,
}
