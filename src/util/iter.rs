use crate::record::data::DataValue;

pub struct CartesianIter<I1, I2> {
    iter1_item: Vec<I1>,
    iter2_item: Vec<I2>,
    pos1: usize,
    pos2: usize,
}

impl<I1, I2> CartesianIter<I1, I2> {
    pub fn new(iter1: Vec<I1>, iter2: Vec<I2>) -> CartesianIter<I1, I2> {
        CartesianIter {
            iter1_item: iter1,
            iter2_item: iter2,
            pos1: 0,
            pos2: 0,
        }
    }
}

impl<I1, I2> Iterator for CartesianIter<I1, I2>
where
    I1: Clone,
    I2: Clone,
{
    type Item = (I1, I2);
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos1 == self.iter1_item.len() || self.iter2_item.is_empty() {
            None
        } else {
            let res1 = self.iter1_item[self.pos1].clone();
            let res2 = self.iter2_item[self.pos2].clone();
            self.pos2 += 1;
            if self.pos2 == self.iter2_item.len() {
                self.pos1 += 1;
                self.pos2 = 0;
            }
            Some((res1, res2))
        }
    }
}

type EqDataType = Vec<DataValue>;
pub struct CartesianEqIter {
    iter1_item: Vec<EqDataType>,
    iter2_item: Vec<EqDataType>,
    idx1: usize,
    idx2: usize,
    pos1_head: usize,
    pos1_tail: usize,
    pos1_cur: usize,
    pos2: usize,
}

impl CartesianEqIter {
    fn compare(&self, x: &EqDataType, y: &EqDataType) -> std::cmp::Ordering {
        match (&x[self.idx1], &y[self.idx2]) {
            (DataValue::Null, DataValue::Null) => std::cmp::Ordering::Equal,
            (DataValue::Null, _) => std::cmp::Ordering::Less,
            (_, DataValue::Null) => std::cmp::Ordering::Greater,
            (DataValue::Int(x), DataValue::Int(y)) => x.cmp(&y),
            (DataValue::Float(x), DataValue::Float(y)) => x.total_cmp(&y),
            (DataValue::VarStr(x), DataValue::VarStr(y)) => x.cmp(&y),
            _ => unreachable!(),
        }
    }
    pub fn new(
        mut a: Vec<EqDataType>,
        mut b: Vec<EqDataType>,
        idx1: usize,
        idx2: usize,
    ) -> CartesianEqIter {
        let cmp = |x: &DataValue, y: &DataValue| -> std::cmp::Ordering {
            match (x, y) {
                (DataValue::Null, DataValue::Null) => std::cmp::Ordering::Equal,
                (DataValue::Null, _) => std::cmp::Ordering::Less,
                (_, DataValue::Null) => std::cmp::Ordering::Greater,
                (DataValue::Int(x), DataValue::Int(y)) => x.cmp(&y),
                (DataValue::Float(x), DataValue::Float(y)) => x.total_cmp(&y),
                (DataValue::VarStr(x), DataValue::VarStr(y)) => x.cmp(&y),
                _ => unreachable!(),
            }
        };
        a.sort_by(|x, y| cmp(&x[idx1], &y[idx1]));
        b.sort_by(|x, y| cmp(&x[idx2], &y[idx2]));
        let pos1_head = 0;
        let mut pos1_tail = 0;
        while pos1_tail < a.len() && a[pos1_head][idx1] == a[pos1_tail][idx1] {
            pos1_tail += 1;
        }

        CartesianEqIter {
            iter1_item: a,
            iter2_item: b,
            idx1,
            idx2,
            pos1_head,
            pos1_tail,
            pos1_cur: 0,
            pos2: 0,
        }
    }
}

impl Iterator for CartesianEqIter {
    type Item = (EqDataType, EqDataType);
    fn next(&mut self) -> Option<Self::Item> {
        while self.pos1_head < self.iter1_item.len() && self.pos2 < self.iter2_item.len() {
            match self.compare(&self.iter1_item[self.pos1_cur], &self.iter2_item[self.pos2]) {
                std::cmp::Ordering::Less => {
                    // <
                    self.pos1_head = self.pos1_tail;
                    self.pos1_cur = self.pos1_tail;
                    while self.pos1_tail < self.iter1_item.len()
                        && self.iter1_item[self.pos1_head][self.idx1]
                            == self.iter1_item[self.pos1_tail][self.idx1]
                    {
                        self.pos1_tail += 1;
                    }
                }
                std::cmp::Ordering::Greater => {
                    // >
                    self.pos2 += 1
                }
                std::cmp::Ordering::Equal => {
                    let result1 = self.iter1_item[self.pos1_cur].clone();
                    let result2 = self.iter2_item[self.pos2].clone();
                    self.pos1_cur += 1;
                    if self.pos1_cur == self.pos1_tail {
                        self.pos2 += 1;
                        self.pos1_cur = self.pos1_head;
                    }
                    return Some((result1, result2));
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use crate::record::data::DataValue;

    use super::{CartesianEqIter, CartesianIter};

    #[test]
    fn check_normal() {
        let x = vec![1, 2, 3];
        let y = vec![4, 5, 6];

        let mut z = CartesianIter::new(x, y);
        assert_eq!(z.next(), Some((1, 4)));
        assert_eq!(z.next(), Some((1, 5)));
        assert_eq!(z.next(), Some((1, 6)));
        assert_eq!(z.next(), Some((2, 4)));
        assert_eq!(z.next(), Some((2, 5)));
        assert_eq!(z.next(), Some((2, 6)));
        assert_eq!(z.next(), Some((3, 4)));
        assert_eq!(z.next(), Some((3, 5)));
        assert_eq!(z.next(), Some((3, 6)));
        assert_eq!(z.next(), None);
        assert_eq!(z.next(), None);
    }

    #[test]
    fn check_empty() {
        let x: Vec<i32> = vec![];
        let y = vec![1, 2, 3];
        let mut z = CartesianIter::new(x.clone(), y.clone());
        assert_eq!(z.next(), None);
        let mut z = CartesianIter::new(y, x.clone());
        assert_eq!(z.next(), None);
        let mut z = CartesianIter::new(x.clone(), x);
        assert_eq!(z.next(), None);
    }

    #[test]
    fn check_eq_normal() {
        let x = vec![
            4, 2, 5, 3, 1, 5, 1, 3, 1, 4, 2, 1, 1, 4, 1, 3, 2, 4, 1, 3, 2,
        ];
        let y = vec![3, 4, 2, 5, 4, 2, 6, 5, 3, 6, 6, 3, 5, 1, 4, 5, 2];
        //let x = vec![1,2,2,1];
        //let y = vec![1,2,2,3];
        let a: Vec<_> = (0..x.len()).collect();
        let b: Vec<_> = (0..y.len()).map(|x| x * 2).collect();

        let x_arr: Vec<_> = x
            .iter()
            .zip(&a)
            .map(|(x, y)| vec![DataValue::Int(*y as i32), DataValue::Int(*x)])
            .collect();
        let y_arr: Vec<_> = y
            .iter()
            .zip(&b)
            .map(|(x, y)| vec![DataValue::Int(*x), DataValue::Int(*y as i32)])
            .collect();

        let iter = CartesianEqIter::new(x_arr, y_arr, 1, 0);

        let mut result_expected = vec![];
        for i in x.iter().zip(&a) {
            for j in y.iter().zip(&b) {
                if *i.0 as i32 == *j.0 {
                    result_expected.push((*i.1 as i32, *i.0, *j.0, *j.1 as i32));
                }
            }
        }
        result_expected.sort();
        let mut result_actual: Vec<_> = iter
            .map(|(x, y)| match (&x[0], &x[1], &y[0], &y[1]) {
                (DataValue::Int(a), DataValue::Int(b), DataValue::Int(c), DataValue::Int(d)) => {
                    (*a, *b, *c, *d)
                }
                _ => unreachable!(),
            })
            .collect();
        result_actual.sort();
        println!("{result_expected:?}\n{result_actual:?}");
        assert!(result_expected == result_actual);
    }

    #[test]
    fn check_eq_string() {
        let x = vec![
            "abc", "def", "xyz", "a", "b", "c", "c", "b", "a", "e", "f", "g",
        ];
        let y = vec!["a", "c", "xyz", "b", "c", "a"];
        let a: Vec<_> = (0..x.len()).collect();
        let b: Vec<_> = (0..y.len()).map(|x| x * 2).collect();

        let x_arr: Vec<_> = x
            .iter()
            .zip(&a)
            .map(|(x, y)| vec![DataValue::Int(*y as i32), DataValue::VarStr(x.to_string())])
            .collect();
        let y_arr: Vec<_> = y
            .iter()
            .zip(&b)
            .map(|(x, y)| vec![DataValue::VarStr(x.to_string()), DataValue::Int(*y as i32)])
            .collect();

        let iter = CartesianEqIter::new(x_arr, y_arr, 1, 0);

        let mut result_expected = vec![];
        for i in x.iter().zip(&a) {
            for j in y.iter().zip(&b) {
                if *i.0 == *j.0 {
                    result_expected.push((
                        *i.1 as i32,
                        i.0.to_string(),
                        j.0.to_string(),
                        *j.1 as i32,
                    ));
                }
            }
        }
        result_expected.sort();
        let mut result_actual: Vec<_> = iter
            .map(|(x, y)| match (&x[0], &x[1], &y[0], &y[1]) {
                (
                    DataValue::Int(a),
                    DataValue::VarStr(b),
                    DataValue::VarStr(c),
                    DataValue::Int(d),
                ) => (*a, b.clone(), c.clone(), *d),
                _ => unreachable!(),
            })
            .collect();
        result_actual.sort();
        println!("{result_expected:?}\n{result_actual:?}");
        assert!(result_expected == result_actual);
    }

    #[test]
    fn check_eq_float() {
        let x = vec![
            4.0, 2.0, 5.0, 3., 1., 5., 1., 3., 1., 4., 2., 1., 1., 4., 1., 3., 2., 4., 1., 3., 2.,
        ];
        let y = vec![
            3., 4., 2., 5., 4., 2., 6., 5., 3., 6., 6., 3., 5., 1., 4., 5., 2.,
        ];
        let a: Vec<_> = (0..x.len()).collect();
        let b: Vec<_> = (0..y.len()).map(|x| x * 2).collect();

        let x_arr: Vec<_> = x
            .iter()
            .zip(&a)
            .map(|(x, y)| vec![DataValue::Int(*y as i32), DataValue::Float(*x)])
            .collect();
        let y_arr: Vec<_> = y
            .iter()
            .zip(&b)
            .map(|(x, y)| vec![DataValue::Float(*x), DataValue::Int(*y as i32)])
            .collect();

        let iter = CartesianEqIter::new(x_arr, y_arr, 1, 0);

        let mut result_expected = vec![];
        for i in x.iter().zip(&a) {
            for j in y.iter().zip(&b) {
                if *i.0 == *j.0 {
                    result_expected.push((*i.1 as i32, *i.0, *j.0, *j.1 as i32));
                }
            }
        }
        result_expected.sort_by(|x, y| x.partial_cmp(&y).unwrap());
        let mut result_actual: Vec<_> = iter
            .map(|(x, y)| match (&x[0], &x[1], &y[0], &y[1]) {
                (
                    DataValue::Int(a),
                    DataValue::Float(b),
                    DataValue::Float(c),
                    DataValue::Int(d),
                ) => (*a, *b, *c, *d),
                _ => unreachable!(),
            })
            .collect();
        result_actual.sort_by(|x, y| x.partial_cmp(&y).unwrap());
        println!("{result_expected:?}\n{result_actual:?}");
        assert!(result_expected == result_actual);
    }

    #[test]
    fn check_eq_empty() {
        let x = vec![
            4, 2, 5, 3, 1, 5, 1, 3, 1, 4, 2, 1, 1, 4, 1, 3, 2, 4, 1, 3, 2,
        ];
        let y: Vec<i32> = vec![];
        //let x = vec![1,2,2,1];
        //let y = vec![1,2,2,3];
        let a: Vec<_> = (0..x.len()).collect();
        let b: Vec<_> = (0..y.len()).map(|x| x * 2).collect();

        let x_arr: Vec<_> = x
            .iter()
            .zip(&a)
            .map(|(x, y)| vec![DataValue::Int(*y as i32), DataValue::Int(*x)])
            .collect();
        let y_arr: Vec<_> = y
            .iter()
            .zip(&b)
            .map(|(x, y)| vec![DataValue::Int(*x), DataValue::Int(*y as i32)])
            .collect();

        let mut iter = CartesianEqIter::new(x_arr.clone(), y_arr.clone(), 1, 0);
        assert_eq!(iter.next(), None);
        let mut iter = CartesianEqIter::new(y_arr, x_arr, 1, 0);
        assert_eq!(iter.next(), None);
    }
}
