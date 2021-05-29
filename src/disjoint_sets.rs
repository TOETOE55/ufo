use crate::traits::UnionFind;
use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;

type InvariantLifetime<'id> = PhantomData<*mut &'id ()>;

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
    _marker: InvariantLifetime<'id>,
}

impl<'id, T: Eq + Hash> UnionFind<T> for DisjointSets<'id, T> {
    type Rep = RootRep<'id>;
    fn find(&self, elem: &T) -> Option<RootRep<'id>> {
        if !self.map.contains_key(elem) {
            return None;
        }
        // TODO: 路径压缩
        let mut root = self.map[elem];
        while let Some(p) = self.alloc[root] {
            root = p;
        }
        Some(RootRep {
            addr: root,
            _marker: Default::default(),
        })
    }

    fn union(&mut self, rep1: RootRep<'id>, rep2: RootRep<'id>) -> RootRep<'id> {
        self.alloc[rep2.addr] = Some(rep1.addr);
        rep1
    }

    fn insert(&mut self, elem: T) -> RootRep<'id> {
        if let Some(rep) = self.find(&elem) {
            return rep;
        }

        let addr = *self.map.entry(elem).or_insert({
            self.alloc.push(None);
            self.alloc.len() - 1
        });

        RootRep {
            addr,
            _marker: Default::default(),
        }
    }
}

pub fn scoped<T, R>(scope: impl for<'a> FnOnce(DisjointSets<'a, T>) -> R) -> R {
    scope(DisjointSets {
        alloc: vec![],
        map: Default::default(),
        _marker: Default::default(),
    })
}

impl<'id, T> DisjointSets<'id, T> {
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
