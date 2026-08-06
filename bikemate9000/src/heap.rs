use crate::heart_rate::{RawHeartRateReading};

#[derive(Debug)]
pub struct MinHeap {
    k: usize,
    nodes: Vec<RawHeartRateReading>,
}

impl MinHeap {
    pub fn new(k: usize) -> Self {
        assert!(k > 0, "k must be greater than 0");
        MinHeap{ k: k, nodes: Vec::new() }
    }

    fn get_parent_idx(&self, i: usize) -> usize {
        (i - 1) / 2
    }

    fn get_left_idx(&self, i: usize) -> usize {
        (i * 2) + 1
    }

    fn get_right_idx(&self, i: usize) -> usize {
        (i * 2) + 2
    }

    fn get_left_child(&self, i: usize) -> Option<&RawHeartRateReading> {
        let l = self.get_left_idx(i);
        self.nodes.get(l)
    }

    fn get_right_child(&self, i: usize) -> Option<&RawHeartRateReading> {
        let r = self.get_right_idx(i);
        self.nodes.get(r)
    }

    fn get_parent(&self, i: usize) -> RawHeartRateReading {
        let p = self.get_parent_idx(i);
        self.nodes[p]
    }

    fn bubble_up(&mut self) {
        let mut v = self.nodes.len() - 1;
        let val = self.nodes[v];
        loop {
            if v < 1 {
                break
            }
            let p = self.get_parent_idx(v);
            let parent = self.get_parent(v);
            if parent > val {
                self.swap(v, p);
                v = p;
            } else {
                break
            }
        }
    }

    // This should only be called after swapping a new node with the root node.
    // Following the swap, the new value needs to be swapped with the lesser-valued
    // of its children until it is smaller than both children.
    fn sift(&mut self) {
        let mut v = 0;
        let val = self.nodes[v];
        loop {
            // The tree is always complete so no left node implies no right node.
            if let Some(left) = self.get_left_child(v) {
                if let Some(right) = self.get_right_child(v) {
                    if left < right && *left < val {
                        let l = self.get_left_idx(v);
                        self.swap(v, l);
                        v = l;
                    } else if *right < val {
                        let r = self.get_right_idx(v);
                        self.swap(v, r);
                        v = r;
                    // The node is smaller than both it's children.
                    } else {
                        break
                    }
                } else if *left < val {
                    let l = self.get_left_idx(v);
                    self.swap(v, l);
                    v = l;
                // No right child and left is larger, node is correctly placed.
                } else {
                    break
                }
            // No left child implies this node is in the correct place
            } else {
                break
            }
        }
    }

    // This means the heap always has the top-k values for some stream, and we can insert in O(log n)
    pub fn insert(&mut self, val: RawHeartRateReading) {
        let v = self.nodes.len();

        // If the heap has occupancy, insert the new reading at the end.
        if v < self.k {
            self.nodes.push(val);
            self.bubble_up();
        // If the heap is full, check if the new value is larger than the root and if so, swap them.
        } else if val > self.nodes[0] {
            self.nodes[0] = val;
            self.sift()
        }
    }

    fn swap(&mut self, a: usize, b: usize) {
        let temp = self.nodes[a];
        self.nodes[a] = self.nodes[b];
        self.nodes[b] = temp;
    }

    pub fn top(&self) -> Vec<RawHeartRateReading> {
        let mut nodes = self.nodes.clone();
        nodes.sort();
        nodes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_heap_returns_empty_top() {
        let heap = MinHeap::new(3);
        assert_eq!(heap.top(), Vec::<RawHeartRateReading>::new());
    }

    #[test]
    fn single_insert_below_capacity() {
        let mut heap = MinHeap::new(3);
        heap.insert(5);
        assert_eq!(heap.top(), vec![5]);
    }

    #[test]
    fn inserts_below_capacity_keeps_all_values() {
        let mut heap = MinHeap::new(3);
        heap.insert(3);
        heap.insert(1);
        assert_eq!(heap.top(), vec![1, 3]);
    }

    #[test]
    fn fills_to_capacity_and_sorts_on_top() {
        let mut heap = MinHeap::new(3);
        heap.insert(3);
        heap.insert(1);
        heap.insert(2);
        assert_eq!(heap.top(), vec![1, 2, 3]);
    }

    #[test]
    fn insert_beyond_capacity_replaces_min_if_larger() {
        let mut heap = MinHeap::new(3);
        heap.insert(3);
        heap.insert(1);
        heap.insert(2);
        // 1 is the current min; 4 > 1 so it should replace it.
        heap.insert(4);
        assert_eq!(heap.top(), vec![2, 3, 4]);
    }

    #[test]
    fn insert_beyond_capacity_ignores_value_smaller_than_min() {
        let mut heap = MinHeap::new(3);
        heap.insert(3);
        heap.insert(1);
        heap.insert(2);
        // 0 is smaller than the current min (1), so it should be dropped.
        heap.insert(0);
        assert_eq!(heap.top(), vec![1, 2, 3]);
    }

    #[test]
    fn insert_beyond_capacity_ignores_value_equal_to_min() {
        let mut heap = MinHeap::new(3);
        heap.insert(3);
        heap.insert(1);
        heap.insert(2);
        heap.insert(1);
        assert_eq!(heap.top(), vec![1, 2, 3]);
    }

    #[test]
    fn tracks_top_k_of_larger_stream() {
        let mut heap = MinHeap::new(3);
        for v in [5, 1, 9, 3, 7, 2, 8, 4, 6] {
            heap.insert(v);
        }
        // Top 3 of 1..=9 should be 7, 8, 9.
        assert_eq!(heap.top(), vec![7, 8, 9]);
    }

    #[test]
    fn handles_duplicate_values() {
        let mut heap = MinHeap::new(3);
        heap.insert(4);
        heap.insert(4);
        heap.insert(4);
        heap.insert(5);
        assert_eq!(heap.top(), vec![4, 4, 5]);
    }

    #[test]
    #[should_panic(expected = "k must be greater than 0")]
    fn zero_capacity_heap_panics_on_construction() {
        MinHeap::new(0);
    }

    #[test]
    fn capacity_one_keeps_only_max_seen() {
        let mut heap = MinHeap::new(1);
        heap.insert(3);
        heap.insert(1);
        heap.insert(9);
        heap.insert(4);
        assert_eq!(heap.top(), vec![9]);
    }
}
