#![allow(dead_code)]
// Design a data structure that keeps track of the values in it and answers some queries
// regarding their frequencies.

// Implement the FrequencyTracker class.

// FrequencyTracker(): Initializes the FrequencyTracker object with an empty array initially.
// void add(int number): Adds number to the data structure.
// void deleteOne(int number): Deletes one occurrence of number from the data structure. The
// data structure may not contain number, and in this case nothing is deleted. bool
// hasFrequency(int frequency): Returns true if there is a number in the data structure that
// occurs frequency number of times, otherwise, it returns false.

// Example 1:

// Input
// ["FrequencyTracker", "add", "add", "hasFrequency"]
// [[], [3], [3], [2]]
// Output
// [null, null, null, true]

// Explanation
// FrequencyTracker frequencyTracker = new FrequencyTracker();
// frequencyTracker.add(3); // The data structure now contains [3]
// frequencyTracker.add(3); // The data structure now contains [3, 3]
// frequencyTracker.hasFrequency(2); // Returns true, because 3 occurs twice

// Example 2:

// Input
// ["FrequencyTracker", "add", "deleteOne", "hasFrequency"]
// [[], [1], [1], [1]]
// Output
// [null, null, null, false]

// Explanation
// FrequencyTracker frequencyTracker = new FrequencyTracker();
// frequencyTracker.add(1); // The data structure now contains [1]
// frequencyTracker.deleteOne(1); // The data structure becomes empty []
// frequencyTracker.hasFrequency(1); // Returns false, because the data structure is empty

// Example 3:

// Input
// ["FrequencyTracker", "hasFrequency", "add", "hasFrequency"]
// [[], [2], [3], [1]]
// Output
// [null, false, null, true]

// Explanation
// FrequencyTracker frequencyTracker = new FrequencyTracker();
// frequencyTracker.hasFrequency(2); // Returns false, because the data structure is empty
// frequencyTracker.add(3); // The data structure now contains [3]
// frequencyTracker.hasFrequency(1); // Returns true, because 3 occurs once

// Constraints:

// 1 <= number <= 105
// 1 <= frequency <= 105
// At most, 2 * 105 calls will be made to add, deleteOne, and hasFrequency in total.
use std::collections::{HashMap, HashSet};

struct FrequencyTracker {
    /// frequency -> numbers
    fre: HashMap<i32, HashSet<i32>>,
    /// number -> frequency
    num: HashMap<i32, i32>,
}

/// `&self` means the method takes an immutable reference.
/// If you need a mutable reference, change it to `&mut self` instead.
impl FrequencyTracker {
    fn new() -> Self {
        Self {
            fre: HashMap::new(),
            num: HashMap::new(),
        }
    }

    fn add(&mut self, number: i32) {
        let frequency = self.num.get(&number).unwrap_or(&0) + 1;
        self.num.insert(number, frequency);
        self.fre.entry(frequency).or_default().insert(number);
        if frequency != 1 {
            self.fre.entry(frequency - 1).and_modify(|s| {
                s.remove(&number);
            });
        }
    }

    fn delete_one(&mut self, number: i32) {
        let frequency = self.num.get(&number).unwrap_or(&0) - 1;
        if frequency == -1 {
            return;
        }

        self.num.insert(number, frequency);
        self.fre.entry(frequency).or_default().insert(number);
        self.fre.entry(frequency + 1).and_modify(|s| {
            s.remove(&number);
        });
    }

    fn has_frequency(&self, frequency: i32) -> bool {
        if let Some(v) = self.fre.get(&frequency) {
            !v.is_empty()
        } else {
            false
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2671() {
        let mut ft = FrequencyTracker::new();
        ft.add(3);
        ft.add(3);
        assert!(ft.has_frequency(2));
        ft.delete_one(3);
        assert!(!ft.has_frequency(2));

        let mut ft = FrequencyTracker::new();
        ft.add(1);
        assert!(ft.has_frequency(1));
        ft.delete_one(1);
        assert!(!ft.has_frequency(1));

        let mut ft = FrequencyTracker::new();
        assert!(!ft.has_frequency(2));
        ft.add(3);
        assert!(ft.has_frequency(1));
        ft.delete_one(3);
        assert!(!ft.has_frequency(1));
    }
}
