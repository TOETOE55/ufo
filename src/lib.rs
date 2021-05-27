use std::marker::PhantomData;
use std::collections::HashMap;
use std::hash::Hash;
use std::borrow::Borrow;

#[cfg(test)]
mod tests;

#[derive(Default, Copy, Clone, Debug, Eq, PartialEq)]
struct InvariantLifetime<'id>(PhantomData<*mut &'id ()>);

#[derive(Debug)]
pub struct DisjointSets<'id, T> {
    alloc: Vec<Option<usize>>,
    map: HashMap<T, usize>,
    _marker: InvariantLifetime<'id>,
}


/// 等价类代表元
/// 'id限制了RootRep只能在对应的并查集中使用
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RootRep<'id> {
    addr: usize,
    _marker: InvariantLifetime<'id>
}

impl<'id, T: Eq + Hash> DisjointSets<'id, T> {
    /// 限制了并查集只能在一个scope内使用
    pub fn new<R>(f: impl for<'a> FnOnce(DisjointSets<'a, T>) -> R) -> R {
        f(Self {
            alloc: Default::default(),
            map: Default::default(),
            _marker: Default::default()
        })
    }

    pub fn find<Q: ?Sized>(&self, elem: &Q) -> Option<RootRep<'id>>
        where
            T: Borrow<Q>,
            Q: Eq + Hash,
    {
        if !self.map.contains_key(elem) {
            return None;
        }
        // TODO: 路径压缩
        let mut root = self.map[elem];
        while let Some(p) = self.alloc[root] {
            root = p;
        }
        Some(RootRep { addr: root, _marker: Default::default() })
    }

    pub fn union(&mut self, rep1: RootRep<'id>, rep2: RootRep<'id>) -> RootRep<'id> {
        self.alloc[rep2.addr] = Some(rep1.addr);
        rep1
    }

    pub fn insert_union(&mut self, rep: RootRep<'id>, elem: T) -> RootRep<'id>

    {
        let rep2 = self.insert(elem);
        self.alloc[rep2.addr] = Some(rep.addr);
        rep
    }

    pub fn insert2_union(&mut self, elem1: T, elem2: T) -> RootRep<'id> {
        let rep1 = self.insert(elem1);
        let rep2 = self.insert(elem2);
        self.alloc[rep2.addr] = Some(rep1.addr);
        rep1
    }

    pub fn insert(&mut self, elem: T) -> RootRep<'id> {
        if let Some(rep) = self.find(&elem) {
            return rep;
        }

        let addr = *self.map.entry(elem).or_insert({
            self.alloc.push(None);
            self.alloc.len() - 1
        });

        RootRep {
            addr,
            _marker: Default::default()
        }
    }

    pub fn groups(self) -> impl Iterator<Item = Vec<T>> {
        let mut m = HashMap::new();

        for (elem, rep) in self.map {
            let mut root = rep;
            while let Some(p) = self.alloc[root] {
                root = p;
            }
            m.entry(root).or_insert(vec![]).push(elem);
        }

        m.into_iter().map(|s| s.1)
    }
}