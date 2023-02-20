use super::buf_page_manager::{CacheIndex, FindReplace};

#[derive(Debug)]
struct ListNode {
    prev: usize,
    next: usize,
}

#[derive(Debug)]
pub struct LRUReplacer {
    /* to quick get address
     * use array simulated linked-list
     * length is cap+1
     * list[cap] is the sentinel node
     */
    list: Vec<ListNode>,
    cap: CacheIndex,
}

impl LRUReplacer {
    pub fn create(c: CacheIndex) -> Self {
        let mut l = Vec::new();

        for i in 0..c + 1 {
            l.push(ListNode {
                prev: if i == 0 { c } else { i - 1 },
                next: if i == c { 0 } else { i + 1 },
            });
        }
        LRUReplacer { list: l, cap: c }
    }
    fn del_node(&mut self, i: usize) {
        let p = self.list[i].prev;
        let n = self.list[i].next;
        self.link(p, n);
    }
    fn link(&mut self, p: usize, n: usize) {
        self.list[p].next = n;
        self.list[n].prev = p;
    }
    fn check_correctness(&self) {
        let mut cur = self.cap;
        let mut tot = 0;
        cur = self.list[cur].next;
        while cur != self.cap {
            tot += 1;
            let nxt = self.list[cur].next;
            assert!(self.list[nxt].prev == cur);
            cur = nxt;
        }
        assert_eq!(tot, self.cap);
    }
}

impl FindReplace for LRUReplacer {
    fn access(&mut self, idx: CacheIndex) {
        // Move accessed node to the front
        self.del_node(idx);
        let x = self.cap;
        let y = self.list[x].next;

        // x -> idx -> y
        // insert
        self.link(x, idx);
        self.link(idx, y);
    }

    fn find(&mut self) -> CacheIndex {
        self.list[self.cap].prev
    }

    fn free(&mut self, idx: CacheIndex) {
        // Move freed noe to the rear

        self.del_node(idx);
        let x = self.cap;
        let y = self.list[x].prev;

        // y -> idx -> x
        self.link(y, idx);
        self.link(idx, x);
    }
}

#[cfg(test)]
mod tests {

    use crate::filesystem::buf_page_manager::FindReplace;

    use super::LRUReplacer;

    #[test]
    fn test_replacer_correctness() {
        let mut replacer = LRUReplacer::create(10);
        let seq: Vec<usize> = vec![1, 4, 2, 8, 5, 7, 3, 6, 9, 0];
        for i in seq {
            replacer.access(i);
            //println!("{:?}", replacer);
            replacer.check_correctness();
        }
        //println!("{:?}", replacer);
        assert_eq!(replacer.find(), 1);
        for (i, j) in std::iter::zip(vec![1_usize, 7, 4, 8, 2], vec![4_usize, 4, 2, 2, 5]) {
            replacer.access(i);
            assert_eq!(replacer.find(), j);
            replacer.check_correctness();
        }
        replacer.free(9);
        assert_eq!(replacer.find(), 9);
        replacer.check_correctness();
    }

    fn test_replace_single(cap: usize, time: i32, sd: u32) {
        let mut replacer = LRUReplacer::create(cap);
        let mut seq: Vec<usize> = (0usize..cap).collect();
        for i in &seq {
            replacer.access(*i);
        }

        assert_eq!(replacer.find(), 0);

        let mut seed: u32 = sd;
        let mut rng = move || {
            seed = seed ^ (seed << 13);
            seed = seed ^ (seed >> 17);
            seed = seed ^ (seed << 5);
            seed
        };

        for i in 0..time {
            let val: usize = (rng() as usize) % cap;
            let mut res: Vec<usize> = seq.clone().into_iter().filter(|x| *x != val).collect();
            res.push(val);
            replacer.access(val);
            assert_eq!(replacer.find(), res[0], "{}", i);
            assert_eq!(res.len(), cap);
            seq.clone_from_slice(&res[..]);
            if cap < 100 || i % 100 == 0 {
                replacer.check_correctness();
            }
        }
        for i in 0..time {
            let val: usize = (rng() as usize) % cap;
            replacer.free(val);
            assert_eq!(replacer.find(), val);
            if cap < 100 || i % 100 == 0 {
                replacer.check_correctness();
            }
        }
    }

    #[test]
    fn test_replace_all() {
        test_replace_single(3, 1000, 38324);
        test_replace_single(10, 1000, 142857);
        test_replace_single(1000, 100000, 998244353);
    }
}
