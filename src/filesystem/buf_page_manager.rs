use std::collections::{HashMap, HashSet};

use super::{
    file_manager::{Fd, FileManager},
    replacer::LRUReplacer,
    Page, PageBuffer,
};
use std::io;

// Trait for find and replacer algorithm
pub trait FindReplace {
    // Mark a access
    fn access(&mut self, idx: CacheIndex);
    // Find a free cache page
    fn find(&mut self) -> CacheIndex;
    // Free a cache page
    fn free(&mut self, idx: CacheIndex);
}

pub type CacheIndex = usize;

// cache data

struct CacheData {
    data: PageBuffer,
    dirty: bool,
    place: Option<(Fd, Page)>,
}

impl CacheData {
    pub fn new() -> Self {
        Self {
            data: PageBuffer::new(0),
            dirty: false,
            place: None,
        }
    }
}

pub struct BufPageManager<T: FindReplace> {
    file_manager: FileManager,
    hash: HashMap<(Fd, u64), CacheIndex>,
    replace: T,
    cache: Vec<CacheData>,
    history: HashMap<Fd, HashSet<u64>>,
}

impl<T: FindReplace> BufPageManager<T> {
    pub fn create(c: usize, r: T) -> Self {
        let mut cache = Vec::new();
        for _i in 0..c {
            cache.push(CacheData::new());
        }
        Self {
            file_manager: FileManager::create(),
            hash: HashMap::new(),
            replace: r,
            cache,
            history: HashMap::new(),
        }
    }
    pub fn open(&mut self, path: &str) -> Result<Fd, io::Error> {
        self.file_manager.open(path)
    }
    pub fn close(&mut self, fd: Fd) -> Result<(), io::Error> {
        // clean up page
        // use history item, to preserve some kind of speed
        if let Some(s) = self.history.get(&fd) {
            for page in s {
                let page = *page;
                // if hasn't been cleared
                if let Some(idx) = self.hash.get(&(fd, page)) {
                    // write back
                    if self.cache[*idx].dirty {
                        self.file_manager
                            .write_page(fd, page, &self.cache[*idx].data)?;
                        self.cache[*idx].dirty = false;
                    }

                    // Clean up cache entry
                    self.cache[*idx].place = None;
                    self.replace.free(*idx);
                }

                // Remove hash entry
                self.hash.remove(&(fd, page));
            }
        }
        // clean up history
        self.history.remove(&fd);
        // close file
        self.file_manager.close(fd)
    }
    fn fetch_page(&mut self, fd: Fd, page: Page) -> Result<CacheIndex, io::Error> {
        let idx = self.replace.find();

        // Not a empty page
        if let Some((old_fd, old_pg)) = self.cache[idx].place {
            // Dirty page
            if self.cache[idx].dirty {
                self.file_manager
                    .write_page(old_fd, old_pg, &self.cache[idx].data)?;
                self.cache[idx].dirty = false;
            }

            // Remove hash entry
            self.hash.remove(&(old_fd, old_pg));
        }

        // insert new entry
        self.file_manager
            .read_page(fd, page, &mut self.cache[idx].data)?;
        self.hash.insert((fd, page), idx);
        self.cache[idx].place = Some((fd, page));

        Ok(idx)
    }

    fn append_history(&mut self, fd: Fd, page: Page) {
        // append into history, only close uses this
        if let Some(s) = self.history.get_mut(&fd) {
            s.insert(page);
        } else {
            let mut s = HashSet::new();
            s.insert(page);
            self.history.insert(fd, s);
        }
    }

    pub fn fetch_page_for_read(&mut self, fd: Fd, page: Page) -> Result<&PageBuffer, io::Error> {
        if let Some(idx) = self.hash.get(&(fd, page)) {
            self.replace.access(*idx);
            Ok(&self.cache[*idx].data)
        } else {
            let idx = self.fetch_page(fd, page)?;

            self.append_history(fd, page);
            self.replace.access(idx);
            Ok(&self.cache[idx].data)
        }
    }
    pub fn fetch_page_for_write(
        &mut self,
        fd: Fd,
        page: Page,
    ) -> Result<&mut PageBuffer, io::Error> {
        let idx;
        if let Some(i) = self.hash.get(&(fd, page)) {
            idx = *i;
        } else {
            idx = self.fetch_page(fd, page)?;

            self.append_history(fd, page);
        }

        self.replace.access(idx);
        self.cache[idx].dirty = true;
        Ok(&mut self.cache[idx].data)
    }

    pub fn all_write_back(&mut self) {
        let mut success = true;
        for ele in &self.cache {
            if let Some((fd, pg)) = ele.place {
                if ele.dirty && self.file_manager.write_page(fd, pg, &ele.data).is_err() {
                    success = false;
                }
            }
        }
        if !success {
            eprintln!("Error occured when writing back elements!");
        }
    }
}

// Write back when exiting
impl<T: FindReplace> Drop for BufPageManager<T> {
    fn drop(&mut self) {
        self.all_write_back();
    }
}

impl BufPageManager<LRUReplacer> {
    pub fn create_lru(c: usize) -> Self {
        BufPageManager::create(c, LRUReplacer::create(c))
    }
}

pub type LRUBufPageManager = BufPageManager<LRUReplacer>;
