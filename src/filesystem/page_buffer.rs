pub const PAGE_SIZE: usize = 8192; // bytes

// Buffer
pub type Byte = u8;
pub struct PageBuffer {
    pub data: [Byte; PAGE_SIZE],
}

impl PageBuffer {
    pub fn new(init: Byte) -> Self {
        PageBuffer {
            data: [init; PAGE_SIZE],
        }
    }

    /**
     * A template function for all fix-length type convertions
     * @param idx: offset of the buffer
     * @param trans: translate function from an array of u8 to target types
     */
    fn parse_fixed<'a, T, U>(&'a self, idx: u32, trans: fn(U) -> T) -> Option<T>
    where
        U: std::convert::TryFrom<&'a [u8]>,
    {
        let idx = idx as usize;
        let size = std::mem::size_of::<T>();
        if idx >= PAGE_SIZE || idx + size > PAGE_SIZE {
            None
        } else {
            let p = &self.data[idx..idx + size];
            if let Ok(res) = p.try_into() {
                Some(trans(res))
            } else {
                // This branch should not be reached
                // but for convenientness, we return None
                None
            }
        }
    }

    /**
     * We use little endian order when restoring data
     */
    pub fn parse_u8(&self, idx: u32) -> Option<u8> {
        let idx = idx as usize;
        if idx >= PAGE_SIZE {
            None
        } else {
            Some(self.data[idx as usize])
        }
    }
    pub fn parse_i32(&self, idx: u32) -> Option<i32> {
        self.parse_fixed(idx, i32::from_le_bytes)
    }

    pub fn parse_f32(&self, idx: u32) -> Option<f32> {
        self.parse_fixed(idx, f32::from_le_bytes)
    }

    pub fn parse_u32(&self, idx: u32) -> Option<u32> {
        self.parse_fixed(idx, u32::from_le_bytes)
    }

    pub fn parse_u64(&self, idx: u32) -> Option<u64> {
        self.parse_fixed(idx, u64::from_le_bytes)
    }
    pub fn parse_str_fixed(&self, idx: u32, length: u32) -> Option<String> {
        let idx = idx as usize;
        let length = length as usize;
        if idx >= PAGE_SIZE || idx + length > PAGE_SIZE {
            None
        } else {
            let mut data = Vec::from(&self.data[idx..idx + length]);

            // trim zeros at the end
            while !data.is_empty() && data[data.len() - 1] == 0 {
                data.pop();
            }
            // We do expect valid utf8 string
            // however, I assume the testcases are all ASCII strings
            if let Ok(res) = String::from_utf8(data) {
                Some(res)
            } else {
                None
            }
        }
    }

    /**
     * It's undefined when the remaining space is not enough
     * for writing.
     * The implementation itself truncanates the data at the end
     * of the page
     */
    fn write(&mut self, idx: u32, data: &[u8]) {
        let idx = idx as usize;
        std::iter::zip(&mut self.data[idx..], data).for_each(|(x, y)| {
            *x = *y;
        });
    }
    pub fn write_u8(&mut self, idx: u32, val: u8) {
        let idx = idx as usize;
        if idx >= PAGE_SIZE {
            return;
        }
        self.data[idx] = val;
    }
    pub fn write_i32(&mut self, idx: u32, val: i32) {
        self.write(idx, &val.to_le_bytes());
    }
    pub fn write_u32(&mut self, idx: u32, val: u32) {
        self.write(idx, &val.to_le_bytes());
    }
    pub fn write_u64(&mut self, idx: u32, val: u64) {
        self.write(idx, &val.to_le_bytes());
    }
    pub fn write_f32(&mut self, idx: u32, val: f32) {
        self.write(idx, &val.to_le_bytes());
    }
    pub fn write_str_fixed(&mut self, idx: u32, val: &String, len: u32) {
        // first
        let val = String::from(val);
        let mut data = val.into_bytes();
        data.resize(len as usize, 0);
        self.write(idx, &data);
    }
}

#[cfg(test)]
mod tests {
    use super::PageBuffer;
    #[test]
    fn check_parse_fixed() {
        let mut page = PageBuffer::new(0);
        page.data[0] = 0x12;
        page.data[1] = 0x34;
        page.data[2] = 0x56;
        page.data[3] = 0x78;
        page.data[4] = 0x90;
        page.data[5] = 0xab;
        page.data[6] = 0xcd;
        page.data[7] = 0xef;
        page.data[8] = 0x00;
        page.data[9] = 0x00;
        page.data[10] = 0x48;
        page.data[11] = 0x41;
        assert_eq!(page.parse_i32(0).unwrap(), 0x78563412);
        assert_eq!(page.parse_u32(4).unwrap(), 0xefcdab90);
        assert_eq!(page.parse_u64(0).unwrap(), 0xefcdab9078563412);
        // example in the doc
        assert_eq!(page.parse_f32(8).unwrap(), 12.5);
    }

    #[test]
    fn check_write() {
        let mut page = PageBuffer::new(0);
        page.write(0, &[0x01, 0x23, 0x45, 0x67, 0x89]);
        assert_eq!(page.data[0], 0x01);
        assert_eq!(page.data[1], 0x23);
        assert_eq!(page.data[2], 0x45);
        assert_eq!(page.data[3], 0x67);
        assert_eq!(page.data[4], 0x89);

        page.write_u32(10, 0x12345678);
        assert_eq!(page.data[10], 0x78);
        assert_eq!(page.data[11], 0x56);
        assert_eq!(page.data[12], 0x34);
        assert_eq!(page.data[13], 0x12);
    }

    #[test]
    fn test_string_fixed() {
        let mut page = PageBuffer::new(0);
        let test_str = "abcdefgQWERTY!@#$%^";
        page.write_str_fixed(0, &String::from(test_str), 256);
        assert_eq!(
            page.parse_str_fixed(0, 256).unwrap(),
            String::from(test_str)
        );
        let test_str = "";
        page.write_str_fixed(1024, &String::from(test_str), 10);
        assert_eq!(
            page.parse_str_fixed(1024, 10).unwrap(),
            String::from(test_str)
        );
    }

    #[test]
    fn test_consistency() {
        let mut page = PageBuffer::new(0);
        let val: i32 = 998244353;
        page.write_i32(7, val);
        assert_eq!(page.parse_i32(7).unwrap(), val);

        let val: u64 = 0x7834ac90ef5612bd;
        page.write_u64(130, val);
        assert_eq!(page.parse_u64(130).unwrap(), val);

        let val: f32 = 3.1415926;
        page.write_f32(1024, val);
        assert_eq!(page.parse_f32(1024).unwrap(), val);
    }
}
