pub trait TreeNode<T: PartialOrd> {
    fn children(&self) -> &Vec<Box<Self>>;

    fn dfsearch(&self, checker: fn(&Self) -> bool) -> Option<&Self> {
        if checker(&self) {
            return Some(self);
        }
        let children = self.children();
        for child in children {
            match child.dfsearch(checker) {
                None => continue,
                Some(r) => return Some(r),
            }
        }
        return None;
    }

    fn dfsearch_heuristic(
        &self,
        checker: fn(&Self) -> bool,
        heuristic: fn(&Self) -> T,
    ) -> Option<&Self> {
        if checker(&self) {
            return Some(self);
        }
        let children = self.children();
        let mut sorted = children.iter().collect::<Vec<_>>();
        sorted.sort_by(|a, b| heuristic(a).partial_cmp(&heuristic(b)).expect("Comparision failed :("));
        for child in sorted {
            match child.dfsearch_heuristic(checker, heuristic) {
                None => continue,
                Some(r) => return Some(r),
            }
        }
        return None;
    }

    fn bfsearch(&self, f: fn(&Self) -> Option<T>) -> &Self {
        todo!()
    }
}

