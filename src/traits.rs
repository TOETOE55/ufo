pub trait UnionFind<T> {
    type Rep: PartialEq;
    fn find(&self, elem: &T) -> Option<Self::Rep>;
    fn union(&mut self, rep1: Self::Rep, rep2: Self::Rep) -> Self::Rep;
    fn insert(&mut self, elem: T) -> Self::Rep;

    fn insert_union(&mut self, rep: Self::Rep, elem: T) -> Self::Rep {
        let rep2 = self.insert(elem);
        self.union(rep, rep2)
    }

    fn insert2_union(&mut self, elem1: T, elem2: T) -> Self::Rep {
        let rep1 = self.insert(elem1);
        let rep2 = self.insert(elem2);
        self.union(rep1, rep2)
    }
}
