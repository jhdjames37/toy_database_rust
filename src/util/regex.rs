use std::collections::{hash_set, HashSet};

/// A simple matcher only capable for % and _
pub struct Matcher {
    pattern: Vec<WildcardType>,
}

#[derive(PartialEq)]
enum WildcardType {
    Char(char),
    Any,
    AnyNumber,
}

impl Matcher {
    pub fn new(pattern: String) -> Matcher {
        let mut last_any_number = false;
        let mut res = vec![];
        for c in pattern.chars() {
            match c {
                '%' => {
                    if !last_any_number {
                        res.push(WildcardType::AnyNumber)
                    }
                }
                '_' => res.push(WildcardType::Any),
                _ => res.push(WildcardType::Char(c)),
            }
            last_any_number = c == '%';
        }
        Matcher { pattern: res }
    }

    pub fn check(&self, test_str: &str) -> bool {
        let mut reachable_node: HashSet<usize> = HashSet::new();

        reachable_node.insert(0);
        if 0 < self.pattern.len() && self.pattern[0] == WildcardType::AnyNumber {
            reachable_node.insert(1);
        }

        for c in test_str.chars() {
            let mut node_next: HashSet<usize> = HashSet::new();
            let mut add_node = |x: usize| {
                node_next.insert(x);
                if x < self.pattern.len() && self.pattern[x] == WildcardType::AnyNumber {
                    node_next.insert(x + 1);
                }
            };

            for node in reachable_node {
                if node == self.pattern.len() {
                    continue;
                }
                match &self.pattern[node] {
                    WildcardType::Char(w) => {
                        if *w == c {
                            add_node(node + 1);
                        }
                    }
                    WildcardType::Any => {
                        add_node(node + 1);
                    }
                    WildcardType::AnyNumber => {
                        add_node(node);
                    }
                }
            }
            reachable_node = node_next;
        }
        return reachable_node.contains(&self.pattern.len());
    }
}

#[cfg(test)]
mod test {
    use super::Matcher;

    #[test]
    fn check_simple_wildcard2() {
        let matcher = Matcher::new("%G".to_string());
        assert_eq!(matcher.check("GG"), true);
        assert_eq!(matcher.check("G"), true);
        assert_eq!(matcher.check("gG"), true);
        assert_eq!(matcher.check("Gg"), false);
        assert_eq!(matcher.check("gg"), false);

        assert_eq!(matcher.check("abcdefG"), true);
        assert_eq!(matcher.check("wwwwwwG"), true);
        assert_eq!(matcher.check("GGGGGGG"), true);

        let matcher = Matcher::new("G%".to_string());
        assert_eq!(matcher.check("GG"), true);
        assert_eq!(matcher.check("gG"), false);
        assert_eq!(matcher.check("Gg"), true);
        assert_eq!(matcher.check("G"), true);
        assert_eq!(matcher.check("gg"), false);
        assert_eq!(matcher.check("Gefefefefe"), true);
        assert_eq!(matcher.check("gegegegege"), false);
        assert_eq!(matcher.check("GGGGGGGGGGG"), true);

        let matcher = Matcher::new("%%%%%%%%G%%%%%%%%%%".to_string());
        assert_eq!(matcher.check("GG"), true);
        assert_eq!(matcher.check("gG"), true);
        assert_eq!(matcher.check("Gg"), true);
        assert_eq!(matcher.check("G"), true);
        assert_eq!(matcher.check("gg"), false);
        assert_eq!(matcher.check("Gefefefefe"), true);
        assert_eq!(matcher.check("gegegGgege"), true);
        assert_eq!(matcher.check("GGGGGGGGGGG"), true);

        let matcher = Matcher::new("G_".to_string());
        assert_eq!(matcher.check("GG"), true);
        assert_eq!(matcher.check("gG"), false);
        assert_eq!(matcher.check("Gg"), true);
        assert_eq!(matcher.check("G"), false);
        assert_eq!(matcher.check("gg"), false);
        assert_eq!(matcher.check("Gefefefefe"), false);
        assert_eq!(matcher.check("gegegGgege"), false);
        assert_eq!(matcher.check("GGGGGGGGGGG"), false);

        let matcher = Matcher::new("_G_G_G_G_".to_string());
        assert_eq!(matcher.check("xGxGxGxGx"), true);
        assert_eq!(matcher.check("xGxGxgxGx"), false);

        let matcher = Matcher::new("_G_G%G_G_".to_string());
        assert_eq!(matcher.check("xGxGxGxGx"), true);
        assert_eq!(matcher.check("xGxGxgxGxGx"), true);
        assert_eq!(matcher.check("xGxGxgxGx"), false);

        let matcher = Matcher::new("_G_%G_%G_".to_string());
        assert_eq!(matcher.check("xGxxxGxxxGxGx"), true);
        assert_eq!(matcher.check("xGGxxxGx"), false);
        assert_eq!(matcher.check("GGGGGGG"), true);
    }
}
