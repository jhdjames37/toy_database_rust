pub struct Bitmap {
    data: Vec<u64>,
    len: u32,
}

impl Bitmap {
    pub fn new(len: u32) -> Bitmap {
        let actual_len = (len + 63) / 64; // ceil
        Bitmap {
            data: vec![0; actual_len as usize],
            len,
        }
    }

    pub fn get(&self, idx: u32) -> Option<bool> {
        if idx >= self.len {
            None
        } else {
            let idx = idx as usize;
            Some(((self.data[idx / 64] >> (idx % 64)) & 1) == 1)
        }
    }
    pub fn set(&mut self, idx: u32, val: bool) -> Option<()> {
        if idx >= self.len {
            None
        } else {
            let idx = idx as usize;
            let p = idx % 64;
            self.data[idx / 64] &= !(1u64 << p);
            if val {
                self.data[idx / 64] |= 1u64 << p;
            }
            Some(())
        }
    }
}

impl crate::filesystem::PageBuffer {
    pub fn parse_bitmap(&self, pos: u32, len: u32) -> Option<Bitmap> {
        if len == 0 {
            // We do not read/write zero-length bitmap
            return Some(Bitmap::new(0));
        }

        let mut res = Bitmap::new(len);
        // 4 bytes aligned
        for i in 0..(res.data.len() - 1) {
            // Offset: 64 bits = 8 bytes
            res.data[i as usize] = self.parse_u64(pos + i as u32 * 8)?;
        }

        // 1 byte aligned
        // remaining length
        let rem_len = (len + 7) / 8 - (res.data.len() - 1) as u32 * 8;
        debug_assert!(rem_len <= 8);
        let pos = pos + (res.data.len() as u32 - 1) * 8;
        let mut num: u64 = 0;

        // It's just little endian
        for i in 0..rem_len {
            num |= (self.parse_u8(pos + i)? as u64) << (i * 8);
        }
        let tar_pos = res.data.len() - 1;
        res.data[tar_pos] = num;
        Some(res)
    }

    pub fn write_bitmap(&mut self, pos: u32, bitmap: &Bitmap) {
        if bitmap.len == 0 {
            // no need to write
            return;
        }
        for i in 0..(bitmap.data.len() - 1) {
            self.write_u64(pos + i as u32 * 8, bitmap.data[i]);
        }

        let rem_len = (bitmap.len + 7) / 8 - (bitmap.data.len() - 1) as u32 * 8;
        debug_assert!(rem_len <= 8);
        let pos = pos + (bitmap.data.len() as u32 - 1) * 8;
        let num = bitmap.data[bitmap.data.len() - 1];
        for i in 0..rem_len {
            self.write_u8(pos + i, ((num >> (8 * i)) & 0xff) as u8)
        }
    }
}

#[cfg(test)]
mod test {
    use super::Bitmap;

    fn test_bitmap_one(n: u32, seed: u32, time: i32) {
        let mut seed = seed;

        let mut rng = move || {
            seed = seed ^ (seed << 13);
            seed = seed ^ (seed >> 17);
            seed = seed ^ (seed << 5);
            seed
        };

        let mut bitmap = Bitmap::new(n);
        let mut result = vec![0; n as usize];
        for _i in 0..time {
            let pos = rng() % n;
            let val = ((rng() >> 19) & 1) == 1;
            assert_eq!(bitmap.get(pos).unwrap() as i32, result[pos as usize]);
            result[pos as usize] = val as i32;
            bitmap.set(pos, val);
        }
    }

    #[test]
    fn test_all() {
        test_bitmap_one(10, 0x342546, 10000);
        test_bitmap_one(300, 0x845234, 1000000);
    }

    #[test]
    fn test_io_all() {
        test_io_one(15, 0x34342323);
        test_io_one(63, 0x67674545);
        test_io_one(500, 0x44223355);
        test_io_one(256 + 15, 0x55776644);
        for i in 0..65 {
            test_io_one(512 + i, 0x35236432 + i as u32);
        }
    }

    fn test_io_one(n: usize, seed: u32) {
        let mut seed = seed;

        let mut rng = move || {
            seed = seed ^ (seed << 13);
            seed = seed ^ (seed >> 17);
            seed = seed ^ (seed << 5);
            seed
        };

        let mut result = vec![0u32; n];
        let mut bitmap = Bitmap::new(n as u32);
        let mut page = crate::filesystem::PageBuffer::new(0);

        for i in 0..n {
            let val = (rng() >> 10) & 1;
            bitmap.set(i as u32, val == 1);
            result[i] = val;
        }
        page.write_bitmap(100, &bitmap);
        // modify areas around to make sure
        // the data isn't writing to wrong place
        let m = (n + 7) / 8;
        for i in 100 + m..100 + 64 + m {
            page.data[i] ^= 0xff;
        }
        for i in 0..100 {
            page.data[i] ^= 0xff;
        }

        let bitmap = page.parse_bitmap(100, n as u32).unwrap();
        for i in 0..n {
            assert_eq!(
                bitmap.get(i as u32).unwrap() as u32,
                result[i],
                "n = {}, seed = {}",
                n,
                seed
            );
        }
    }
}
