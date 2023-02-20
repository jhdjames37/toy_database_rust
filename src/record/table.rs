use crate::filesystem::{Fd, LRUBufPageManager, PAGE_SIZE};
use crate::util::bitmap::Bitmap;
use crate::util::error::Error as DBError;
use std::cell::RefCell;
use std::mem;
use std::path::PathBuf;
use std::rc::Rc;

use super::column::TableMeta;
use super::data::DataEntry;

pub struct RecordManager<'a> {
    fd: Fd,
    buf: Rc<RefCell<LRUBufPageManager>>,
    metadata: &'a TableMeta,
    total_page: u32,
    max_entries: u32,
    entry_size: u32,
}

#[derive(PartialEq, Eq, Clone, Default)]
pub struct Pos(pub u32, pub u32);
/*
 * Layout of the Page
 * Header (32 Bytes)
 * + Magic number (4 Bytes)
 * + Total page (4 Bytes)
 * + Maximum entries (4 Bytes)
 * + Table Meta id (4 Bytes)
 * + Reserved (16 Bytes)
 * Bitmap of the usage
 * + ceil(total_page / 8) bytes
 * Contents
 */
const OFFSET_MAGIC: u32 = 0;
const OFFSET_TOT_PAGE: u32 = 4;
const OFFSET_MAX_ENTRIES: u32 = 8;
const OFFSET_META_ID: u32 = 12;
const HEADER_SIZE: u32 = 32;
const MAGIC: u32 = 0x358f1b9b;

type TError = Box<dyn std::error::Error>;

fn calc_max_entry(size: u32) -> u32 {
    // each entry needs (size + 1/8) bytes.
    // solving equation:
    //     (size+1/8)*entry = tot_size
    // yields:
    //     entry = PAGE_SIZE*8/(size*8+1)
    let tot_size = (PAGE_SIZE as u32 - HEADER_SIZE) as u32;
    let num = tot_size * 8 / (size * 8 + 1);

    // however, as aligning exists, we may need to adjust
    // the value by -1

    if (num + 7) / 8 + num * size > tot_size {
        num - 1
    } else {
        num
    }
}

impl<'a> RecordManager<'a> {
    /**
     * Create a new record manager
     * @param path: open file path
     */
    pub fn new(
        path: &str,
        buf: &Rc<RefCell<LRUBufPageManager>>,
        meta: &'a TableMeta,
    ) -> Result<RecordManager<'a>, TError> {
        let buf = Rc::clone(buf);

        let new_file = !PathBuf::from(path).exists();
        let fd = buf.try_borrow_mut()?.open(path)?;
        let max_entries;
        let total_page;

        if new_file {
            let mut buf = buf.try_borrow_mut()?;
            let page = buf.fetch_page_for_write(fd, 0)?;

            total_page = 1;
            max_entries = calc_max_entry(meta.entry_size_fixed());

            page.write_u32(OFFSET_MAGIC, MAGIC);
            page.write_u32(OFFSET_TOT_PAGE, 1);
            page.write_u32(OFFSET_MAX_ENTRIES, max_entries);
            page.write_u32(OFFSET_META_ID, meta.id);

            let bitmap = Bitmap::new(max_entries);
            page.write_bitmap(HEADER_SIZE, &bitmap);
        } else {
            let mut buf = buf.try_borrow_mut()?;
            let page = buf.fetch_page_for_read(fd, 0)?;

            if MAGIC != page.parse_u32(OFFSET_MAGIC).unwrap()
                || meta.id != page.parse_u32(OFFSET_META_ID).unwrap()
            {
                return Err(Box::new(DBError::new("Corrupted file!")));
            }
            max_entries = page.parse_u32(OFFSET_MAX_ENTRIES).unwrap();
            total_page = page.parse_u32(OFFSET_TOT_PAGE).unwrap();
        }
        debug_assert!(
            (max_entries + 7) / 8 + max_entries * meta.entry_size_fixed() + HEADER_SIZE
                <= PAGE_SIZE as u32
        );

        Ok(RecordManager {
            fd,
            buf,
            metadata: meta,
            max_entries,
            total_page,
            entry_size: meta.entry_size_fixed(),
        })
    }
    fn bitmap_size(&self) -> u32 {
        (self.max_entries + 7) / 8
    }

    /// create a new page, returns the **page index** (instead of **page num**)
    fn create_new_page(&mut self) -> Result<u32, TError> {
        self.total_page += 1;
        let mut buf = self.buf.try_borrow_mut()?;
        let page = buf.fetch_page_for_write(self.fd, (self.total_page - 1).into())?;

        page.write_u32(OFFSET_MAGIC, MAGIC);
        let bitmap = Bitmap::new(self.max_entries);
        page.write_bitmap(HEADER_SIZE, &bitmap);

        // update total page
        let page_0 = buf.fetch_page_for_write(self.fd, 0)?;
        page_0.write_u32(OFFSET_TOT_PAGE, self.total_page);

        Ok(self.total_page - 1)
    }

    fn clean_up(&mut self) -> Result<(), TError> {
        self.buf.try_borrow_mut()?.close(self.fd)?;
        Ok(())
    }

    /// check whether there's a free slot
    /// if not return None
    fn check_free_slot(&self) -> Result<Option<Pos>, TError> {
        for i in 0..self.total_page {
            let mut buf = self.buf.try_borrow_mut()?;
            let page = buf.fetch_page_for_read(self.fd, i as u64)?;
            let bitmap = page.parse_bitmap(HEADER_SIZE, self.max_entries).unwrap();
            for j in 0..self.max_entries {
                // to get avoid setting (0, 0)
                if (i, j) != (0, 0) && !bitmap.get(j).unwrap() {
                    return Ok(Some(Pos(i, j)));
                }
            }
        }

        Ok(None)
    }

    /// The default value is located at (0, 0)
    pub fn set_default_value(&mut self, data: &DataEntry) -> Result<(), TError> {
        let mut buf = self.buf.try_borrow_mut()?;
        let page = buf.fetch_page_for_write(self.fd, 0)?;
        let mut bitmap = page.parse_bitmap(HEADER_SIZE, self.max_entries).unwrap();
        let pos = HEADER_SIZE + self.bitmap_size();
        bitmap.set(0, true);
        page.write_bitmap(HEADER_SIZE, &bitmap);
        page.write_data_entry(pos + 0, self.metadata, data)?;
        Ok(())
    }

    /// insert into table
    pub fn insert(&mut self, data: &DataEntry) -> Result<Pos, TError> {
        // TODO: check record consistency
        let mut slot = self.check_free_slot()?;

        if slot.is_none() {
            slot = Some(Pos(self.create_new_page()?, 0));
        }

        // write entry
        let slot = slot.unwrap();
        {
            let mut buf = self.buf.try_borrow_mut()?;
            let page = buf.fetch_page_for_write(self.fd, slot.0.into())?;
            let mut bitmap = page.parse_bitmap(HEADER_SIZE, self.max_entries).unwrap();
            let pos = HEADER_SIZE + self.bitmap_size();

            bitmap.set(slot.1, true);
            page.write_bitmap(HEADER_SIZE, &bitmap);
            page.write_data_entry(pos + slot.1 * self.entry_size, self.metadata, data)?;
        }
        Ok(slot)
    }

    pub fn find(&self, pos: &Pos) -> Result<Option<DataEntry>, TError> {
        let mut buf = self.buf.try_borrow_mut()?;
        let page = buf.fetch_page_for_read(self.fd, pos.0.into())?;
        let bitmap = page.parse_bitmap(HEADER_SIZE, self.max_entries).unwrap();
        let start = HEADER_SIZE + self.bitmap_size();
        if bitmap.get(pos.1).unwrap() {
            Ok(page.parse_data_entry(start + pos.1 * self.entry_size, self.metadata, &pos))
        } else {
            Ok(None)
        }
    }

    pub fn update(&self, pos: &Pos, data: &DataEntry) -> Result<(), TError> {
        let mut buf = self.buf.try_borrow_mut()?;
        let page = buf.fetch_page_for_write(self.fd, pos.0.into())?;
        let bitmap = page.parse_bitmap(HEADER_SIZE, self.max_entries).unwrap();
        let start = HEADER_SIZE + self.bitmap_size();

        if bitmap.get(pos.1).unwrap() {
            page.write_data_entry(start + pos.1 * self.entry_size, self.metadata, data)?;
            Ok(())
        } else {
            Err(Box::new(DBError::new("No such entry!")))
        }
    }

    pub fn delete(&self, pos: &Pos) -> Result<(), TError> {
        let mut buf = self.buf.try_borrow_mut()?;
        let page = buf.fetch_page_for_write(self.fd, pos.0.into())?;
        let mut bitmap = page.parse_bitmap(HEADER_SIZE, self.max_entries).unwrap();

        if bitmap.get(pos.1).unwrap() {
            bitmap.set(pos.1, false);
            page.write_bitmap(HEADER_SIZE, &bitmap);
            Ok(())
        } else {
            Err(Box::new(DBError::new("No such entry!")))
        }
    }

    pub fn delete_where<T>(&self, f: T) -> Result<i32, TError>
    where
        T: Fn(DataEntry) -> bool,
    {
        let mut it = self.iter();
        let mut sum = 0;
        while let Some(x) = it.next() {
            it.is_error()?;
            if f(x) {
                self.delete(&it.pos)?;
                sum += 1;
            }
        }
        it.is_error()?;
        Ok(sum)
    }

    pub fn iter(&self) -> RecordIterator {
        RecordIterator {
            pos: Pos(0, 0),
            manager: self,
            error: None,
            //first: true,
        }
    }
}

#[derive(Clone)]
pub struct RecordIntoIterator<'a> {
    pos: Pos,
    manager: Rc<RefCell<RecordManager<'a>>>,
}

impl<'a> IntoIterator for RecordManager<'a> {
    type Item = DataEntry;
    type IntoIter = RecordIntoIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        RecordIntoIterator {
            pos: Pos(0, 0),
            manager: Rc::new(RefCell::new(self)),
        }
    }
}
impl RecordIntoIterator<'_> {
    fn advance(&mut self) {
        self.pos.1 += 1;
        if self.pos.1 == self.manager.borrow().max_entries {
            self.pos.1 = 0;
            self.pos.0 += 1;
        }
    }
}

impl Iterator for RecordIntoIterator<'_> {
    type Item = DataEntry;
    fn next(&mut self) -> Option<Self::Item> {
        // Except the first call, `pos' will be at
        // the last valid entry.
        //if self.first {
        //    self.first = false;
        //} else {
        // now that Pos(0, 0) is always used (default value), so that
        // we do not have to check if it is the first entry
        self.advance();
        //}

        loop {
            if self.pos.0 >= self.manager.borrow().total_page {
                return None;
            }

            if let Some(x) = self.manager.borrow_mut().find(&self.pos).unwrap_or(None) {
                return Some(x);
            }
            self.advance();
        }
    }
}

pub struct RecordIterator<'a> {
    pos: Pos,
    manager: &'a RecordManager<'a>,
    error: Option<TError>,
    //first: bool,
}

impl<'a> RecordIterator<'a> {
    fn advance(&mut self) {
        self.pos.1 += 1;
        if self.pos.1 == self.manager.max_entries {
            self.pos.1 = 0;
            self.pos.0 += 1;
        }
    }
    pub fn is_error(&mut self) -> Result<(), TError> {
        if let Some(x) = mem::take(&mut self.error) {
            Err(x)
        } else {
            Ok(())
        }
    }
}

impl<'a> Iterator for RecordIterator<'a> {
    type Item = DataEntry;
    fn next(&mut self) -> Option<Self::Item> {
        // Except the first call, `pos' will be at
        // the last valid entry.
        //if self.first {
        //    self.first = false;
        //} else {
        self.advance();
        //}

        loop {
            if self.pos.0 >= self.manager.total_page {
                return None;
            }

            if let Some(x) = self.manager.find(&self.pos).unwrap_or_else(|e| {
                self.error = Some(e);
                None
            }) {
                return Some(x);
            }
            self.advance();
        }
    }
}

impl Drop for RecordManager<'_> {
    fn drop(&mut self) {
        if self.clean_up().is_err() {
            println!("Error when cleaning up RecordManager!");
        }
    }
}
