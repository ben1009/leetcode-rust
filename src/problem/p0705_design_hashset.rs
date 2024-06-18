// Design a HashSet without using any built-in hash table libraries.

// Implement MyHashSet class:

// void add(key) Inserts the value key into the HashSet.
// bool contains(key) Returns whether the value key exists in the HashSet or not.
// void remove(key) Removes the value key in the HashSet. If key does not exist in the HashSet, do
// nothing.

// Example 1:

// Input
// ["MyHashSet", "add", "add", "contains", "contains", "add", "contains", "remove", "contains"]
// [[], [1], [2], [1], [3], [2], [2], [2], [2]]
// Output
// [null, null, null, true, false, null, true, null, false]

// Explanation
// MyHashSet myHashSet = new MyHashSet();
// myHashSet.add(1);      // set = [1]
// myHashSet.add(2);      // set = [1, 2]
// myHashSet.contains(1); // return True
// myHashSet.contains(3); // return False, (not found)
// myHashSet.add(2);      // set = [1, 2]
// myHashSet.contains(2); // return True
// myHashSet.remove(2);   // set = [1]
// myHashSet.contains(2); // return False, (already removed)

// Constraints:

// 0 <= key <= 10^6
// At most 10^4 calls will be made to add, remove, and contains.

#[allow(dead_code)]
struct MyHashSet {
    arr: Vec<i32>,
    size: i32,
}

/// `&self` means the method takes an immutable reference.
/// If you need a mutable reference, change it to `&mut self` instead.
#[allow(dead_code)]
impl MyHashSet {
    fn new() -> Self {
        let size = 100000;

        Self {
            arr: vec![-1; size],
            size: size as i32,
        }
    }

    fn add(&mut self, key: i32) {
        let mut idx = (key % self.size) as usize;
        while self.arr[idx] != -1 {
            if self.arr[idx] == key {
                return;
            }
            idx += 1;
        }
        self.arr[idx] = key;
    }

    fn remove(&mut self, key: i32) {
        let mut idx = (key % self.size) as usize;
        while self.arr[idx] != key {
            if self.arr[idx] != -1 {
                return;
            }
            idx += 1;
            if idx == self.size as usize {
                return;
            }
        }
        self.arr[idx] = -1;
    }

    fn contains(&self, key: i32) -> bool {
        let mut idx = (key % self.size) as usize;
        while self.arr[idx] != key {
            if self.arr[idx] == -1 {
                return false;
            }

            idx += 1;
            if idx == self.size as usize {
                return false;
            }
        }

        true
    }
}
// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_705() {
        let mut m = MyHashSet::new();
        m.add(1);
        m.add(2);
        assert!(m.contains(1));
        assert!(!m.contains(3));
        m.add(2);
        assert!(m.contains(2));
        m.remove(2);
        assert!(!m.contains(2));
    }
}
