use super::page_buffer::*;
use std::io::{self, Read, Write};
use std::{fs::File, io::Seek, io::SeekFrom};

pub type Page = u64;

// File Descriptor
struct FileDescriptor {
    fd: File,
}

impl FileDescriptor {
    /* Open or create file
     * As a constructor of the function
     *
     * Automatic closes it when destroying this object
     */
    fn open(path: &str) -> Result<Self, io::Error> {
        let fd = File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;

        Ok(FileDescriptor { fd })
    }
    fn read_page(&mut self, offset: u64, page: &mut PageBuffer) -> Result<(), io::Error> {
        self.fd.seek(SeekFrom::Start(offset * (PAGE_SIZE as u64)))?;

        // initalize data with zero
        // so that unread part will surely be zero
        page.data.fill(0);
        self.fd.read(&mut page.data)?;

        Ok(())
    }
    fn write_page(&mut self, offset: u64, page: &PageBuffer) -> Result<(), io::Error> {
        self.fd.seek(SeekFrom::Start(offset * (PAGE_SIZE as u64)))?;
        self.fd.write(&page.data)?;
        Ok(())
    }
}

pub type Fd = usize;
// Real file manager
pub struct FileManager {
    file_pool: Vec<Option<FileDescriptor>>,
    free_pool: Vec<Fd>,
}

impl FileManager {
    pub fn create() -> Self {
        FileManager {
            file_pool: Vec::new(),
            free_pool: Vec::new(),
        }
    }
    pub fn open(&mut self, path: &str) -> Result<Fd, io::Error> {
        if let Some(fd) = self.free_pool.pop() {
            *self.get_file_option(fd)? = Some(FileDescriptor::open(path)?);
            Ok(fd)
        } else {
            self.file_pool.push(Some(FileDescriptor::open(path)?));
            Ok(self.file_pool.len() - 1)
        }
    }

    fn get_file_option(&mut self, fd: Fd) -> Result<&mut Option<FileDescriptor>, io::Error> {
        if let Some(f) = self.file_pool.get_mut(fd) {
            Ok(f)
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "File not exist"))
        }
    }

    fn get_file(&mut self, fd: Fd) -> Result<&mut FileDescriptor, io::Error> {
        if let Some(f) = self.get_file_option(fd)? {
            Ok(f)
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "File not exist or already closed",
            ))
        }
    }

    pub fn close(&mut self, fd: Fd) -> Result<(), io::Error> {
        *self.get_file_option(fd)? = None;
        self.free_pool.push(fd);
        Ok(())
    }

    pub fn read_page(
        &mut self,
        fd: Fd,
        offset: Page,
        page: &mut PageBuffer,
    ) -> Result<(), io::Error> {
        self.get_file(fd)?.read_page(offset, page)
    }

    pub fn write_page(&mut self, fd: Fd, offset: Page, page: &PageBuffer) -> Result<(), io::Error> {
        self.get_file(fd)?.write_page(offset, page)
    }
}
