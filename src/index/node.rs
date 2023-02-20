use super::util::*;
use crate::{
    filesystem::PageBuffer,
    record::{
        column::{DataColumn, DataType},
        data::DataValue,
        table::Pos,
    },
};

pub struct BPTreeNode {
    pub is_leaf: bool,
    pub prec: Option<u32>,
    pub succ: Option<u32>,
    pub data: Vec<(Vec<DataValue>, Pos)>,
}

impl BPTreeNode {
    pub fn new() -> BPTreeNode {
        BPTreeNode {
            is_leaf: false,
            prec: None,
            succ: None,
            data: vec![],
        }
    }
    pub fn read(page: &PageBuffer, meta: &Vec<DataType>) -> BPTreeNode {
        let is_leaf = page.is_leaf();
        let prec = page.get_prec();
        let succ = page.get_succ();
        let size = page.get_size();
        //println!("!!{size}");
        let mut data = vec![];
        for i in 0..size {
            data.push(page.get_element(i, meta).unwrap());
        }
        BPTreeNode {
            is_leaf,
            prec,
            succ,
            data,
        }
    }
    pub fn write(&self, page: &mut PageBuffer, meta: &Vec<DataType>) {
        page.set_leaf(self.is_leaf);
        page.set_prec(self.prec);
        page.set_succ(self.succ);
        page.set_size(self.data.len() as u32);
        for (i, item) in self.data.iter().enumerate() {
            page.write_element(i as u32, item, meta);
        }
    }
}
impl PageBuffer {
    fn get_element(&self, offset: u32, meta: &Vec<DataType>) -> Option<(Vec<DataValue>, Pos)> {
        let entry_size = calc_entry_size(meta);
        let start = HEADER_SIZE + offset * entry_size;
        let mut cur = start;
        let mut res: Vec<DataValue> = vec![];
        //println!("{cur}");
        for item in meta {
            match item {
                DataType::Int => {
                    res.push(DataValue::Int(self.parse_i32(cur)?));
                    cur += 4;
                }
                DataType::Float => {
                    res.push(DataValue::Float(self.parse_f32(cur)?));
                    cur += 4;
                }
                DataType::VarStr(x) => {
                    res.push(DataValue::VarStr(self.parse_str_fixed(cur, *x)?));
                    cur += x;
                }
            }
        }
        //println!("{cur}?");
        let a = self.parse_u32(cur)?;
        //println!("{cur}r?");
        let b = self.parse_u32(cur + 4)?;
        //println!("{cur}g?");
        Some((res, Pos(a, b)))
    }
    fn write_element(&mut self, offset: u32, data: &(Vec<DataValue>, Pos), meta: &Vec<DataType>) {
        let entry_size = calc_entry_size(meta);
        let start = HEADER_SIZE + offset * entry_size;
        let mut cur = start;
        for (item, meta_item) in data.0.iter().zip(meta) {
            match (item, meta_item) {
                (DataValue::Int(x), DataType::Int) => {
                    self.write_i32(cur, *x);
                    cur += 4;
                }
                (DataValue::Float(x), DataType::Int) => {
                    self.write_f32(cur, *x);
                    cur += 4;
                }
                (DataValue::VarStr(x), DataType::VarStr(y)) => {
                    self.write_str_fixed(cur, x, *y);
                    cur += y;
                }
                _ => unreachable!(),
            }
        }
        self.write_u32(cur, data.1 .0);
        self.write_u32(cur + 4, data.1 .1);
    }
}
