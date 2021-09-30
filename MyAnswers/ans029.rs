use proconio::{input, fastout};
use crate::lazy_segment::LazySegTree;

mod lazy_segment {
    pub struct LazySegTree<S, T, F, G, H> {
        depth: usize,
        offset: usize,
        data: Vec<S>,
        lazy: Vec<T>,
        identity_data: S,
        identity_lazy: T,
        operation: F,
        mapping: G,
        composition: H,
    }

    fn bit_length(x: usize) -> usize {
        let mut y = x;
        let mut result = 0;
        while y > 0 {
            result += 1;
            y >>= 1;
        }
        result
    }

    impl<S, T, F, G, H> LazySegTree<S, T, F, G, H>
        where
            S: Copy + std::fmt::Debug,
            T: Copy + std::fmt::Debug,
            F: Fn(S, S) -> S,
            G: Fn(S, T) -> S,
            H: Fn(T, T) -> T,
    {
        pub fn new(
            n: usize,
            identity_data: S,
            identity_lazy: T,
            operation: F,
            mapping: G,
            composition: H,
        ) -> Self {
            let depth = bit_length(n);
            let offset = 1 << depth;
            Self {
                depth,
                offset,
                data: vec![identity_data; offset << 1],
                lazy: vec![identity_lazy; offset],
                identity_data,
                identity_lazy,
                operation,
                mapping,
                composition,
            }
        }

        fn push(&mut self, i: usize) {
            if i < self.offset {
                let lch = i << 1;
                let rch = lch + 1;
                self.data[lch] = (self.mapping)(self.data[lch], self.lazy[i]);
                self.data[rch] = (self.mapping)(self.data[rch], self.lazy[i]);
                if lch < self.offset {
                    self.lazy[lch] = (self.composition)(self.lazy[lch], self.lazy[i]);
                    self.lazy[rch] = (self.composition)(self.lazy[rch], self.lazy[i]);
                }
                self.lazy[i] = self.identity_lazy;
            }
        }

        fn update(&mut self, i: usize) {
            self.data[i] = (self.operation)(self.data[i << 1], self.data[(i << 1) + 1]);
        }

        fn all_apply(&mut self, i: usize, d: T) {
            self.data[i] = (self.mapping)(self.data[i], d);
            if i < self.offset { self.lazy[i] = (self.composition)(self.lazy[i], d); }
        }

        fn propagate(&mut self, l: usize, r: usize) {
            for i in (1..=self.depth).rev() {
                if ((l >> i) << i) != l { self.push(l >> i); }
                if ((r >> i) << i) != r { self.push((r - 1) >> i); }
            }
        }

        pub fn range_update(&mut self, mut l: usize, mut r: usize, d: T) {
            l += self.offset;
            r += self.offset;

            self.propagate(l, r);

            {
                let l2 = l;
                let r2 = r;
                while l < r {
                    if (l & 1) == 1 {
                        self.all_apply(l, d);
                        l += 1;
                    }
                    if (r & 1) == 1 {
                        r -= 1;
                        self.all_apply(r, d);
                    }
                    l >>= 1;
                    r >>= 1;
                }
                l = l2;
                r = r2;
            }

            for i in 1..=self.depth {
                if ((l >> i) << i) != l { self.update(l >> i); }
                if ((r >> i) << i) != r { self.update((r - 1) >> i); }
            }
        }

        pub fn range_query(&mut self, mut l: usize, mut r: usize) -> S {
            l += self.offset;
            r += self.offset;

            self.propagate(l, r);

            let mut sml = self.identity_data;
            let mut smr = self.identity_data;
            while l < r {
                if (l & 1) == 1 {
                    sml = (self.operation)(sml, self.data[l]);
                    l += 1;
                }
                if (r & 1) == 1 {
                    r -= 1;
                    smr = (self.operation)(smr, self.data[r]);
                }
                l >>= 1;
                r >>= 1;
            }

            (self.operation)(sml, smr)
        }

        #[allow(dead_code)]
        pub fn debug_print(&self) {
            let mut i = 1;
            while i <= self.offset {
                let row = self.data[i..2 * i].iter().map(|x| format!("{:?}", x)).collect::<String>();
                println!("{}", row);
                i <<= 1;
            }
            i = 1;
            while i < self.offset {
                let row = self.lazy[i..2 * i].iter().map(|x| format!("{:?}", x)).collect::<String>();
                println!("{}", row);
                i <<= 1;
            }
        }
    }
}

#[fastout]
fn main() {
    input! {
        w: usize,
        n: usize,
        lrs: [(usize, usize); n],
    }

    let mut sgt = LazySegTree::new(
        w + 5, 0, 0,
        std::cmp::max, std::cmp::max, std::cmp::max);

    for (l, r) in lrs {
        let ans = sgt.range_query(l, r + 1) + 1;
        println!("{}", ans);
        sgt.range_update(l, r + 1, ans);
    }
}
