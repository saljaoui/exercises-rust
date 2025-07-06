use std::collections::BTreeSet;

pub fn flatten_tree<T: ToOwned<Owned = T> + std::fmt::Debug + Clone>(tree: &BTreeSet<T>) -> Vec<T> {
    let mut res = vec![];
    for t in tree {
        res.push(t.clone());
    }
    res
}