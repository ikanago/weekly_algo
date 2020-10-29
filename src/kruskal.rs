use std::mem;

#[derive(Clone, Debug)]
pub struct UnionFind {
    root: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            root: (0..size).collect(),
            size: vec![0; size],
        }
    }

    // 指定した要素の根を返す(経路圧縮もする)．
    pub fn root(&mut self, x: usize) -> usize {
        if self.root[x] == x {
            x
        } else {
            let parent = self.root[x];
            let root = self.root(parent);
            self.root[x] = root;
            root
        }
    }

    // 二つの要素が同じ木にあるかを返す．
    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    // 二つの要素が含まれる木を併合する．
    pub fn unite(&mut self, x: usize, y: usize) -> bool {
        let mut x_root = self.root(x);
        let mut y_root = self.root(y);
        if x_root == y_root {
            return false;
        }
        if self.size(x_root) < self.size(y_root) {
            mem::swap(&mut x_root, &mut y_root);
        }
        self.root[y_root] = x_root;
        self.size[x_root] += self.size[y_root];
        true
    }

    // 指定した要素が属する木の大きさを返す．
    pub fn size(&self, x: usize) -> usize {
        self.size[x]
    }
}

#[derive(Clone, Debug, Ord, Eq, PartialOrd, PartialEq)]
pub struct Edge {
    from: usize,
    to: usize,
    cost: usize,
}

impl Edge {
    pub fn new(from: usize, to: usize, cost: usize) -> Self {
        Self { from, to, cost }
    }
}

pub fn kruskal(edges: Vec<Edge>, num_vertex: usize) -> Vec<Edge> {
    let mut edges = edges;
    edges.sort_by(|e1, e2| e1.cost.cmp(&e2.cost));
    let mut spanning = Vec::new();
    let mut uf = UnionFind::new(num_vertex);
    for e in edges {
        if !uf.same(e.from, e.to) {
            uf.unite(e.from, e.to);
            spanning.push(e);
        }
    }
    spanning
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimum_spanning_tree() {
        let edges = vec![
            Edge::new(0, 1, 2),
            Edge::new(0, 2, 8),
            Edge::new(0, 3, 4),
            Edge::new(0, 4, 6),
            Edge::new(1, 2, 7),
            Edge::new(1, 3, 3),
            Edge::new(1, 4, 6),
            Edge::new(2, 3, 9),
            Edge::new(2, 4, 8),
            Edge::new(3, 4, 5),
        ];
        assert_eq!(
            vec![
                Edge::new(0, 1, 2),
                Edge::new(1, 3, 3),
                Edge::new(3, 4, 5),
                Edge::new(1, 2, 7),
            ],
            kruskal(edges, 5)
        );
    }
}
