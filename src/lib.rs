use std::collections::HashMap;
use std::hash::Hash;

pub struct BiMap<L, R> {
    left_to_right: HashMap<L, R>,
    right_to_left: HashMap<R, L>,
}

impl<L: Eq + Hash + Clone, R: Eq + Hash + Clone> BiMap<L, R> {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            left_to_right: HashMap::new(),
            right_to_left: HashMap::new(),
        }
    }

    #[inline(always)]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            left_to_right: HashMap::with_capacity(capacity),
            right_to_left: HashMap::with_capacity(capacity),
        }
    }

    #[inline(always)]
    pub fn insert(&mut self, left: L, right: R) -> Option<(L, R)> {
        if let Some(old_left) = self.right_to_left.get(&right) {
            if old_left != &left {
                let old_left = old_left.clone();
                let old_right = self.left_to_right.remove(&old_left).unwrap();
                self.right_to_left.remove(&right);
                self.left_to_right.insert(left.clone(), right.clone());
                self.right_to_left.insert(right, left.clone());
                return Some((old_left, old_right));
            }
        }

        if let Some(old_right) = self.left_to_right.get(&left) {
            if old_right == &right {
                return None;
            }
            let old_right = old_right.clone();
            self.right_to_left.remove(&old_right);
            self.left_to_right.insert(left.clone(), right.clone());
            self.right_to_left.insert(right, left.clone());
            return Some((left, old_right));
        }

        self.left_to_right.insert(left.clone(), right.clone());
        self.right_to_left.insert(right, left);
        None
    }

    #[inline(always)]
    pub fn get_left(&self, left: &L) -> Option<&R> {
        self.left_to_right.get(left)
    }

    #[inline(always)]
    pub fn get_right(&self, right: &R) -> Option<&L> {
        self.right_to_left.get(right)
    }

    #[inline(always)]
    pub fn remove_left(&mut self, left: &L) -> Option<(L, R)> {
        let right = self.left_to_right.remove(left)?;
        let left = self.right_to_left.remove(&right)?;
        Some((left, right))
    }

    #[inline(always)]
    pub fn remove_right(&mut self, right: &R) -> Option<(L, R)> {
        let left = self.right_to_left.remove(right)?;
        let right = self.left_to_right.remove(&left)?;
        Some((left, right))
    }

    #[inline(always)]
    pub fn contains_left(&self, left: &L) -> bool {
        self.left_to_right.contains_key(left)
    }

    #[inline(always)]
    pub fn contains_right(&self, right: &R) -> bool {
        self.right_to_left.contains_key(right)
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.left_to_right.len()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.left_to_right.is_empty()
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.left_to_right.clear();
        self.right_to_left.clear();
    }

    #[inline(always)]
    pub fn left_values(&self) -> impl Iterator<Item = &L> {
        self.left_to_right.keys()
    }

    #[inline(always)]
    pub fn right_values(&self) -> impl Iterator<Item = &R> {
        self.right_to_left.keys()
    }

    #[inline(always)]
    pub fn iter(&self) -> impl Iterator<Item = (&L, &R)> {
        self.left_to_right.iter()
    }
}

impl<L: Eq + Hash + Clone, R: Eq + Hash + Clone> Default for BiMap<L, R> {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl<L: Eq + Hash + Clone, R: Eq + Hash + Clone> Clone for BiMap<L, R> {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self {
            left_to_right: self.left_to_right.clone(),
            right_to_left: self.right_to_left.clone(),
        }
    }
}
