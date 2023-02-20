use crate::filesystem::{Fd, LRUBufPageManager, PageBuffer, PAGE_SIZE};
use crate::record::column::DataType;
use crate::record::data::DataValue;
use crate::record::table::Pos;
use crate::util::error::Error as DBError;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use super::manager::DataValueBounded;
use super::node::BPTreeNode;
use super::util::*;

pub struct BPlusTree {
    fd: Fd,
    buf: Rc<RefCell<LRUBufPageManager>>,
    meta: Vec<DataType>,
    root: u32,
    entry_size: u32,
    tot_entry: u32,
    tot_page: u32,
}

impl BPlusTree {
    pub fn new(
        path: &str,
        buf: &Rc<RefCell<LRUBufPageManager>>,
        meta: Vec<DataType>,
    ) -> Result<BPlusTree, Box<dyn std::error::Error>> {
        let buf = Rc::clone(buf);
        let new_file = !PathBuf::from(path).exists();
        let fd = buf.try_borrow_mut()?.open(path)?;
        let root: u32;
        let tot_page: u32;
        if new_file {
            let mut buf = buf.try_borrow_mut()?;
            let page = buf.fetch_page_for_write(fd, 0)?;
            root = 0;
            tot_page = 1;

            page.write_magic();
            page.set_leaf(true);
            page.set_root(root);
            page.set_prec(None);
            page.set_succ(None);
            page.set_size(0);
            page.set_tot_page(1);
        } else {
            let mut buf = buf.try_borrow_mut()?;
            let page = buf.fetch_page_for_read(fd, 0)?;
            if !page.check_magic() {
                return Err(Box::new(DBError::new("index file corrputed!")));
            }
            root = page.get_root();
            tot_page = page.get_tot_page();
        }
        let entry_size = calc_entry_size(&meta);
        let tot_entry = calc_max_entry(entry_size);

        debug_assert!(tot_entry >= 2);

        Ok(BPlusTree {
            fd,
            buf,
            meta,
            root,
            entry_size,
            tot_entry,
            tot_page,
        })
    }

    fn locate_leaf(
        &self,
        data: &Vec<DataValue>,
        cur: &mut u32,
        node: &mut BPTreeNode,
        visit_stack: &mut Vec<(u32, u32)>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        while !node.is_leaf {
            let mut nxt: Option<u32> = None;
            // seq search, use bsearch for better(?) permance
            for (i, (item_i, item_pos)) in node.data.iter().enumerate() {
                // the last item has the range +inf
                if data <= item_i || i == node.data.len() - 1 {
                    nxt = Some(item_pos.0);
                    visit_stack.push((*cur, i as u32));
                    break;
                }
            }
            *cur = nxt.unwrap();
            *node = self.get_page_node(*cur)?;
        }
        Ok(())
    }

    pub fn insert(
        &mut self,
        data: &Vec<DataValue>,
        pos: &Pos,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut cur = self.root;
        let mut node = self.get_page_node(cur)?;
        let mut visit_stack: Vec<(u32, u32)> = vec![];

        // find the leaf node needed
        self.locate_leaf(data, &mut cur, &mut node, &mut visit_stack)?;

        let mut insert_pos = {
            let size = node.data.len();
            let mut pos: Option<u32> = None;
            for (i, (item_i, _)) in node.data.iter().enumerate() {
                if data <= item_i {
                    pos = Some(i as u32);
                    break;
                }
            }
            pos.unwrap_or(size as u32)
        };
        let mut insert_data = (data.clone(), pos.clone());

        loop {
            node.data
                .insert(insert_pos as usize, std::mem::take(&mut insert_data));
            // overflow
            if node.data.len() > self.tot_entry as usize {
                let left_size = node.data.len() / 2;
                let (new_idx, mut new_node) = self.new_node()?;
                new_node.is_leaf = node.is_leaf;
                if let Some(nxt) = node.succ {
                    let mut buf = self.buf.try_borrow_mut()?;
                    let nxt_page = buf.fetch_page_for_write(self.fd, nxt.into())?;
                    nxt_page.set_prec(Some(new_idx));
                }
                new_node.succ = node.succ;
                new_node.prec = Some(cur);
                node.succ = Some(new_idx);

                new_node.data = node.data.split_off(left_size);
                self.store_page_node(cur, &node)?;
                self.store_page_node(new_idx, &new_node)?;
                if let Some((pos, idx)) = visit_stack.pop() {
                    insert_pos = idx + 1;
                    insert_data = (new_node.data.last().unwrap().0.clone(), Pos(new_idx, 0));
                    cur = pos;
                    let change_data = node.data.last().unwrap().0.clone();
                    node = self.get_page_node(cur)?;
                    node.data[idx as usize].0 = change_data;
                } else {
                    // modify root
                    let (root_idx, mut root_node) = self.new_node()?;
                    root_node.is_leaf = false;
                    root_node.data = vec![
                        (node.data.last().unwrap().0.clone(), Pos(cur, 0)),
                        (new_node.data.last().unwrap().0.clone(), Pos(new_idx, 0)),
                    ];
                    //println!("!!!");
                    self.store_page_node(root_idx, &root_node)?;
                    self.set_root(root_idx)?;
                    break;
                }
            } else {
                self.store_page_node(cur, &node)?;
                self.update(&node, &mut visit_stack)?;
                break;
            }
        }

        Ok(())
    }
    fn update(
        &mut self,
        node: &BPTreeNode,
        visit_stack: &mut Vec<(u32, u32)>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut max_data = node.data.last().unwrap().0.clone();
        // update max
        while let Some((pos, idx)) = visit_stack.pop() {
            let cur = pos;
            let mut node = self.get_page_node(cur)?;
            node.data[idx as usize].0 = std::mem::take(&mut max_data);
            max_data = node.data.last().unwrap().0.clone();
            self.store_page_node(cur, &node)?;
        }
        Ok(())
    }

    fn merge_two_node(&self, left: &mut BPTreeNode, right: &mut BPTreeNode) -> bool {
        left.data.append(&mut right.data);
        if left.data.len() <= self.tot_entry as usize {
            // merge two node to the left
            true
        } else {
            // shift some node
            let size = left.data.len();
            right.data = left.data.split_off(size / 2);
            false
        }
    }
    fn finger_increase(
        &self,
        visit_stack: &mut Vec<(u32, u32)>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if visit_stack.is_empty() {
            return Ok(());
        }
        loop {
            let (pos, idx) = visit_stack.pop().unwrap();
            let size = {
                let mut buf = self.buf.try_borrow_mut()?;
                let page = buf.fetch_page_for_read(self.fd, pos.into())?;
                page.get_size()
            };
            if idx + 1 < size {
                visit_stack.push((pos, idx + 1));
                let mut node = self.get_page_node(pos)?;
                let mut cur = node.data[idx as usize + 1].1 .0;
                node = self.get_page_node(cur)?;
                while !node.is_leaf {
                    visit_stack.push((cur, 0));
                    cur = node.data[0].1 .0;
                    node = self.get_page_node(cur)?;
                }
                break;
            }
        }
        Ok(())
    }
    pub fn remove(
        &mut self,
        data: &Vec<DataValue>,
        pos: &Pos,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut cur = self.root;
        let mut node = self.get_page_node(cur)?;
        let mut visit_stack: Vec<(u32, u32)> = vec![];

        self.locate_leaf(data, &mut cur, &mut node, &mut visit_stack)?;

        // find exact node
        let delete_pos = 'outer: loop {
            for (i, (item_i, item_pos)) in node.data.iter().enumerate() {
                if data == item_i && pos == item_pos {
                    break 'outer Some(i as u32);
                }
                if item_i > data {
                    break 'outer None;
                }
            }
            if let Some(nxt) = node.succ {
                cur = nxt;
                node = self.get_page_node(cur)?;
                self.finger_increase(&mut visit_stack)?;
            } else {
                break 'outer None;
            }
        };

        //println!("{visit_stack:?}");
        if let Some(mut delete_pos) = delete_pos {
            loop {
                //println!("{cur}, {delete_pos}");
                node.data.remove(delete_pos as usize);

                // underflow
                if self.root != cur && node.data.len() < ((self.tot_entry + 1) / 2) as usize {
                    let fa = visit_stack.last().unwrap();
                    let fa_size = {
                        let mut buf = self.buf.try_borrow_mut()?;
                        let page = buf.fetch_page_for_read(self.fd, fa.0.into())?;
                        page.get_size()
                    };
                    if fa.1 + 1 < fa_size {
                        // has succ
                        let succ = node.succ.unwrap();
                        let mut succ_node = self.get_page_node(succ)?;

                        if self.merge_two_node(&mut node, &mut succ_node) {
                            //println!("A1");

                            if let Some(nxt) = succ_node.succ {
                                let mut buf = self.buf.try_borrow_mut()?;
                                let nxt_page = buf.fetch_page_for_write(self.fd, nxt.into())?;
                                nxt_page.set_prec(Some(cur));
                                node.succ = Some(nxt);
                            } else {
                                node.succ = None;
                            }
                            self.store_page_node(cur, &node)?;
                            self.delete_node(succ);

                            let fa = visit_stack.pop().unwrap();
                            delete_pos = fa.1 + 1;
                            cur = fa.0;
                            let last_data = node.data.last().unwrap().0.clone();
                            node = self.get_page_node(cur)?;
                            node.data[fa.1 as usize].0 = last_data;
                        } else {
                            //println!("A2");

                            self.store_page_node(cur, &node)?;
                            self.store_page_node(succ, &succ_node)?;

                            let fa = visit_stack.pop().unwrap();
                            let mut fa_node = self.get_page_node(fa.0)?;
                            fa_node.data[fa.1 as usize].0 = node.data.last().unwrap().0.clone();
                            fa_node.data[fa.1 as usize + 1].0 =
                                succ_node.data.last().unwrap().0.clone();
                            self.store_page_node(fa.0, &fa_node)?;
                            self.update(&fa_node, &mut visit_stack)?;
                            break;
                        }
                    } else if fa.1 != 0 {
                        let prev = node.prec.unwrap();
                        let mut prev_node = self.get_page_node(prev)?;
                        if self.merge_two_node(&mut prev_node, &mut node) {
                            //println!("B1");
                            if let Some(nxt) = node.succ {
                                let mut buf = self.buf.try_borrow_mut()?;
                                let nxt_page = buf.fetch_page_for_write(self.fd, nxt.into())?;
                                nxt_page.set_prec(Some(prev));
                                prev_node.succ = Some(nxt);
                            } else {
                                prev_node.succ = None;
                            }

                            self.store_page_node(prev, &prev_node)?;
                            self.delete_node(cur);

                            let fa = visit_stack.pop().unwrap();
                            delete_pos = fa.1;
                            cur = fa.0;
                            node = self.get_page_node(cur)?;
                            node.data[fa.1 as usize - 1].0 =
                                prev_node.data.last().unwrap().0.clone();
                        } else {
                            //println!("B2");
                            self.store_page_node(cur, &node)?;
                            self.store_page_node(prev, &prev_node)?;

                            let fa = visit_stack.pop().unwrap();
                            let mut fa_node = self.get_page_node(fa.0)?;
                            fa_node.data[fa.1 as usize - 1].0 =
                                prev_node.data.last().unwrap().0.clone();
                            fa_node.data[fa.1 as usize].0 = node.data.last().unwrap().0.clone();
                            self.store_page_node(fa.0, &fa_node)?;
                            self.update(&fa_node, &mut visit_stack)?;
                            break;
                        }
                    } else {
                        let fa = visit_stack.pop().unwrap();
                        if visit_stack.last().is_some() {
                            // only m = 2 order B+-tree will have this case
                            debug_assert!(self.tot_entry == 2);
                            // and also means that cur node is empty
                            debug_assert!(node.data.is_empty());
                            // delete this directly
                            delete_pos = fa.1;
                            self.delete_node(cur);
                            cur = fa.0;
                            node = self.get_page_node(cur)?;
                        } else {
                            // root case
                            //println!("C");
                            self.store_page_node(cur, &node)?;
                            self.set_root(cur)?;
                            self.delete_node(fa.0);
                            break;
                        }
                    }
                } else {
                    // normal case
                    self.store_page_node(cur, &node)?;
                    if node.data.is_empty() {
                        debug_assert!(self.root == cur);
                    } else {
                        self.update(&node, &mut visit_stack)?;
                    }
                    break;
                }
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }

    // return: (Pos, Value)
    pub fn lower_bound(
        &self,
        data: &Vec<DataValueBounded>,
    ) -> Result<Option<(Pos, Pos)>, Box<dyn std::error::Error>> {
        let mut cur = self.root;
        let mut node = self.get_page_node(cur)?;

        while !node.is_leaf {
            let mut nxt: Option<u32> = None;
            // seq search, use bsearch for better(?) permance
            for (i, (item_i, item_pos)) in node.data.iter().enumerate() {
                // the last item has the range +inf
                if data
                    <= &item_i
                        .clone()
                        .into_iter()
                        .map(|x| x.into())
                        .collect::<Vec<DataValueBounded>>()
                    || i == node.data.len() - 1
                {
                    nxt = Some(item_pos.0);
                    break;
                }
            }
            cur = nxt.unwrap();
            node = self.get_page_node(cur)?;
        }

        for (i, (item_data, item_pos)) in node.data.iter().enumerate() {
            if &item_data
                .clone()
                .into_iter()
                .map(|x| x.into())
                .collect::<Vec<DataValueBounded>>()
                >= data
            {
                return Ok(Some((Pos(cur, i as u32), item_pos.clone())));
            }
        }
        Ok(None)
    }

    pub fn find(&self, data: &Vec<DataValue>) -> Result<Option<Pos>, Box<dyn std::error::Error>> {
        let mut cur = self.root;
        let mut node = self.get_page_node(cur)?;
        let mut visit_stack: Vec<(u32, u32)> = vec![];

        self.locate_leaf(data, &mut cur, &mut node, &mut visit_stack)?;
        for (item_i, i) in node.data.iter() {
            if item_i == data {
                return Ok(Some(i.clone()));
            }
        }
        Ok(None)
    }

    fn delete_node(&mut self, _root: u32) {
        // TODO: reallocation
    }

    fn set_root(&mut self, root: u32) -> Result<(), Box<dyn std::error::Error>> {
        let mut buf = self.buf.try_borrow_mut()?;
        let page = buf.fetch_page_for_write(self.fd, 0)?;
        page.set_root(root);
        self.root = root;
        Ok(())
    }
    pub fn get_page_node(&self, idx: u32) -> Result<BPTreeNode, Box<dyn std::error::Error>> {
        let mut buf = self.buf.try_borrow_mut()?;
        let page = buf.fetch_page_for_read(self.fd, idx.into())?;
        Ok(BPTreeNode::read(page, &self.meta))
    }
    fn store_page_node(
        &self,
        idx: u32,
        node: &BPTreeNode,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut buf = self.buf.try_borrow_mut()?;
        let mut page = buf.fetch_page_for_write(self.fd, idx.into())?;
        node.write(&mut page, &self.meta);
        Ok(())
    }
    fn new_node(&mut self) -> Result<(u32, BPTreeNode), Box<dyn std::error::Error>> {
        self.tot_page += 1;
        let mut buf = self.buf.try_borrow_mut()?;
        let page = buf.fetch_page_for_write(self.fd, 0)?;
        page.set_tot_page(self.tot_page);
        let page = buf.fetch_page_for_write(self.fd, self.tot_page as u64 - 1)?;
        page.write_magic();
        Ok((self.tot_page - 1, BPTreeNode::new()))
    }
    fn clean_up(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.buf.try_borrow_mut()?.close(self.fd)?;
        Ok(())
    }
    fn get_all(&self) -> Vec<(Vec<DataValue>, Pos)> {
        let mut cur = self.root;
        let mut node = self.get_page_node(cur).unwrap();
        while !node.is_leaf {
            cur = node.data[0].1 .0;
            node = self.get_page_node(cur).unwrap();
        }
        let mut res = vec![];
        loop {
            res.append(&mut node.data.clone());
            if let Some(nxt) = node.succ {
                cur = nxt;
                node = self.get_page_node(cur).unwrap();
            } else {
                break;
            }
        }
        res
    }
    fn print(&self) {
        println!("Root = {}", self.root);
        for i in 0..self.tot_page {
            let node = self.get_page_node(i).unwrap();
            println!(
                "Page {}, is_leaf = {}, prec = {:?}, succ = {:?}",
                i, node.is_leaf, node.prec, node.succ
            );
            if !node.is_leaf {
                for item in node.data {
                    println!("\t {:?}: {}, {}", item.0, item.1 .0, item.1 .1);
                }
            } else if !node.data.is_empty() {
                println!(
                    "{:?} - {:?} ({})",
                    node.data[0].0,
                    node.data.last().unwrap().0,
                    node.data.len()
                );
            }
        }
    }
    fn check(&self) {
        let mut buf = self.buf.try_borrow_mut().unwrap();
        let page = buf.fetch_page_for_write(self.fd, 0).unwrap();
        assert!(self.root == page.get_root());
        assert!(self.tot_page == page.get_tot_page());
        let mut cur = self.root;
        std::mem::drop(buf);

        let mut node = self.get_page_node(cur).unwrap();
        assert!(node.prec.is_none());
        assert!(node.succ.is_none());
        let mut son = node.data.clone();

        while !node.is_leaf {
            let mut x = node.data[0].1 .0;
            let mut s_node = self.get_page_node(x).unwrap();
            let mut son_iter = son.iter();
            let mut data = vec![];
            loop {
                let y = son_iter.next().unwrap();
                assert!(y.0 == s_node.data.last().unwrap().0, "{}", x);
                assert!(y.1 .0 == x);
                data.append(&mut s_node.data.clone());
                if let Some(nxt) = s_node.succ {
                    let n_node = self.get_page_node(nxt).unwrap();
                    assert!(n_node.prec == Some(x));
                    x = nxt;
                    s_node = n_node;
                } else {
                    assert!(son_iter.next() == None);
                    break;
                }
            }
            cur = node.data[0].1 .0;
            node = self.get_page_node(cur).unwrap();
            son = data;
        }
        for (x, y) in son.iter().zip(son.iter().next()) {
            assert!(x.0 <= y.0);
        }
    }
}

impl Drop for BPlusTree {
    fn drop(&mut self) {
        if self.clean_up().is_err() {
            eprintln!("Error when cleaning up B+ Tree!");
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::BTreeSet;

    use crate::{
        filesystem::LRUBufPageManager,
        record::{column::DataType, data::DataValue, table::Pos},
    };

    use super::BPlusTree;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn next(mut seed: u32) -> u32 {
        seed ^= seed << 13;
        seed ^= seed >> 17;
        seed ^= seed << 5;
        seed
    }

    fn check_correctness(tree_test: &BPlusTree, tree_ref: &BTreeSet<(u32, u32)>) {
        let res1 = tree_test.get_all();
        //tree_test.print();
        tree_test.check();
        assert!(res1.len() == tree_ref.len());
        assert!(tree_ref.iter().zip(&res1).all(|(x, y)| {
            y.0.len() == 1 && DataValue::Int(x.0 as i32) == y.0[0] && y.1 .0 == y.1 .1
        }));
        let mut res1: Vec<_> = res1
            .into_iter()
            .map(|x| {
                (
                    match x.0[0] {
                        DataValue::Int(x) => x as u32,
                        _ => unreachable!(),
                    },
                    x.1 .0,
                )
            })
            .collect();
        res1.sort();
        assert!(tree_ref.iter().zip(&res1).all(|(x, y)| { x == y }));
    }

    fn test_insert(n: i32, mut seed: u32, range: u32) {
        let mut tree_ref: BTreeSet<(u32, u32)> = BTreeSet::new();
        let buf = Rc::new(RefCell::new(LRUBufPageManager::create_lru(100000)));
        let mut tree_test =
            BPlusTree::new("../test/btree_test.dat", &buf, vec![DataType::Int]).unwrap();

        let mut gen = move || {
            seed = next(seed);
            seed
        };
        for i in 0..n {
            if i % 1000 == 0 {
                println!("{i}");
                //check_correctness(&tree_test, &tree_ref);
            }
            let mut key = gen() % range;
            let mut value = gen() % range;
            while !tree_ref.insert((key, value)) {
                key = gen() % range;
                value = gen() % range;
            }
            tree_test
                .insert(&vec![DataValue::Int(key as i32)], &Pos(value, value))
                .unwrap();
        }

        check_correctness(&tree_test, &tree_ref);
        std::mem::drop(tree_test);
        let mut tree_test =
            BPlusTree::new("../test/btree_test.dat", &buf, vec![DataType::Int]).unwrap();
        check_correctness(&tree_test, &tree_ref);

        let mut del_order: Vec<_> = tree_ref.iter().cloned().collect();
        for i in 1..del_order.len() {
            let p = gen() as usize % (i + 1);
            let t = del_order[i];
            del_order[i] = del_order[p];
            del_order[p] = t;
        }
        println!("Deleting");
        //tree_test.print();
        for (i, item) in del_order.iter().enumerate() {
            //println!("{i}, {item:?}");
            if i % 1000 == 0 {
                println!("{i}");
                check_correctness(&tree_test, &tree_ref);
            }
            tree_ref.remove(item);
            assert!(tree_test
                .remove(&vec![DataValue::Int(item.0 as i32)], &Pos(item.1, item.1))
                .unwrap());
            //tree_test.print();
            //if i == 158 { tree_test.print(); }
            //tree_test.check();
        }
    }

    #[test]
    #[ignore = "long tests"]
    fn test() {
        test_insert(1000000, 0x12345678, 100000000);
        test_insert(1000000, 0x12345678, 10000);
        //test_insert(1000, 0x23523423);
    }
}
