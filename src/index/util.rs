use crate::{
    filesystem::{PageBuffer, PAGE_SIZE},
    record::column::DataType,
};

pub const MAGIC: u32 = 0x74f3c125;
pub const OFFSET_MAGIC: u32 = 0;
pub const OFFSET_ISLEAF: u32 = 4;
pub const OFFSET_PREC: u32 = 8;
pub const OFFSET_SUCC: u32 = 12;
pub const OFFSET_ROOT: u32 = 16;
pub const OFFSET_SIZE: u32 = 20;
pub const OFFSET_TOT_PAGE: u32 = 24;
pub const NULLPTR: u32 = 0xffffffff;
pub const HEADER_SIZE: u32 = 32;

impl PageBuffer {
    pub fn check_magic(&self) -> bool {
        self.parse_u32(OFFSET_MAGIC) == Some(MAGIC)
    }
    pub fn write_magic(&mut self) {
        self.write_u32(OFFSET_MAGIC, MAGIC);
    }
    pub fn is_leaf(&self) -> bool {
        self.parse_u32(OFFSET_ISLEAF) != Some(0)
    }
    pub fn set_leaf(&mut self, x: bool) {
        self.write_u32(OFFSET_ISLEAF, if x { 1 } else { 0 });
    }
    pub fn get_prec(&self) -> Option<u32> {
        let res = self.parse_u32(OFFSET_PREC).unwrap();
        if res == NULLPTR {
            None
        } else {
            Some(res)
        }
    }
    pub fn get_succ(&self) -> Option<u32> {
        let res = self.parse_u32(OFFSET_SUCC).unwrap();
        if res == NULLPTR {
            None
        } else {
            Some(res)
        }
    }
    pub fn set_prec(&mut self, val: Option<u32>) {
        self.write_u32(OFFSET_PREC, val.unwrap_or(NULLPTR));
    }
    pub fn set_succ(&mut self, val: Option<u32>) {
        self.write_u32(OFFSET_SUCC, val.unwrap_or(NULLPTR));
    }
    pub fn get_root(&self) -> u32 {
        self.parse_u32(OFFSET_ROOT).unwrap()
    }
    pub fn set_root(&mut self, x: u32) {
        self.write_u32(OFFSET_ROOT, x);
    }
    pub fn get_size(&self) -> u32 {
        self.parse_u32(OFFSET_SIZE).unwrap()
    }
    pub fn set_size(&mut self, x: u32) {
        self.write_u32(OFFSET_SIZE, x);
    }
    pub fn get_tot_page(&self) -> u32 {
        self.parse_u32(OFFSET_TOT_PAGE).unwrap()
    }
    pub fn set_tot_page(&mut self, x: u32) {
        self.write_u32(OFFSET_TOT_PAGE, x);
    }
}

pub fn calc_entry_size(meta: &Vec<DataType>) -> u32 {
    return 8 + // Pos
        meta.iter().map(|x| match x {
            DataType::Float | DataType::Int => 4,
            DataType::VarStr(x) => *x,
        }).sum::<u32>();
}
pub fn calc_max_entry(size: u32) -> u32 {
    (PAGE_SIZE as u32 - HEADER_SIZE) / size
}
