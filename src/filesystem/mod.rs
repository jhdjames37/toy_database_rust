// submodule imports
mod file_manager;
pub use file_manager::*;

mod buf_page_manager;
pub use buf_page_manager::LRUBufPageManager;

mod page_buffer;
pub use page_buffer::*;

mod replacer;

#[cfg(test)]
mod tests {

    use super::*;
    use buf_page_manager::BufPageManager;

    #[test]
    fn test_file_manager() {
        // relative to root
        let mut fm = FileManager::create();
        let fd = fm.open("../test/0.dat").unwrap();
        let mut page = PageBuffer::new(0xab);

        fm.write_page(fd, 1, &page).unwrap();

        fm.read_page(fd, 1, &mut page).unwrap();

        assert_eq!(page.data[1024], 0xab);
    }

    #[test]
    fn test_buf_page_manager() {
        let mut buf = BufPageManager::create_lru(10);

        let fd1 = buf.open("../test/1.dat").unwrap();
        let fd2 = buf.open("../test/2.dat").unwrap();

        // basic tests
        for i in 0..10 {
            let page = buf.fetch_page_for_write(fd1, i).unwrap();
            page.data = [i as u8; PAGE_SIZE];
            let page = buf.fetch_page_for_write(fd2, i).unwrap();
            page.data = [i as u8 | 0xa0; PAGE_SIZE];

            let page = buf.fetch_page_for_read(fd1, i).unwrap();
            assert_eq!(page.data[1024], i as u8);
            let page = buf.fetch_page_for_read(fd2, i).unwrap();
            assert_eq!(page.data[1024], i as u8 | 0xa0);
        }

        // random io tests
        let mut res1: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut res2: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
            .iter()
            .map(|x| x | 0xa0)
            .collect();

        let mut seed: u32 = 123456;

        let mut rng = move || {
            seed = seed ^ (seed << 13);
            seed = seed ^ (seed >> 17);
            seed = seed ^ (seed << 5);
            seed
        };

        let mut cnt_read = 0;
        for _tests in 0..1000 {
            let val = rng();
            let fd = if ((val >> 5) & 1) == 1 { fd1 } else { fd2 };
            let page = (val >> 9) % 10;
            let res = if fd == fd1 { &mut res1 } else { &mut res2 };
            if ((val >> 7) & 1) == 1 {
                // read
                let content = buf.fetch_page_for_read(fd, page.into()).unwrap().data;
                assert_eq!(content[1024], res[page as usize]);
                cnt_read += 1;
            } else {
                let g: u8 = ((val >> 19) & 0xff) as u8;
                res[page as usize] = g;
                let content = buf.fetch_page_for_write(fd, page.into()).unwrap();
                content.data = [g; PAGE_SIZE];
            }
        }

        println!(
            "#read: {}\nfile1: {:x?}\nfile2: {:x?}",
            cnt_read, res1, res2
        );
    }

    #[test]
    fn fail_cases() {
        let mut buf = BufPageManager::create_lru(10);
        assert!(buf.fetch_page_for_read(1, 0).is_err());
        let fd = buf.open("../test/4.dat").unwrap();
        if fd == 1000 {
            assert!(buf.fetch_page_for_read(1001, 0).is_err());
        } else {
            assert!(buf.fetch_page_for_read(1000, 0).is_err());
        }
    }

    #[test]
    fn close_case() {
        let mut buf = BufPageManager::create_lru(10);
        let fd = buf.open("../test/3.dat").unwrap();
        let page0 = buf.fetch_page_for_write(fd, 0).unwrap();
        page0.data[384] = 0x78;

        buf.close(fd).unwrap();
        let fd = buf.open("../test/3.dat").unwrap();

        let page1 = buf.fetch_page_for_read(fd, 0).unwrap();
        let x = page1.data[384];
        assert_eq!(x, 0x78);

        println!("{}", x);
        let mut page2 = buf.fetch_page_for_write(fd, 1).unwrap();
        page2.data[384] = x;
        page2.data[385] = 0x85;

        buf.close(fd).unwrap();
        assert!(buf.fetch_page_for_read(fd, 0).is_err());
        assert!(buf.fetch_page_for_write(fd, 1).is_err());

        let fd = buf.open("../test/3.dat").unwrap();
        let page2 = buf.fetch_page_for_read(fd, 1).unwrap();
        assert_eq!(x, page2.data[384]);
        assert_eq!(0x85, page2.data[385]);
    }
}
