use std::cmp;

/// [155] Min Stack
///
/// Design a stack that supports push, pop, top, and retrieving the minimum element in constant
/// time. Implement the MinStack class:
///
///     MinStack() initializes the stack object.
///     void push(int val) pushes the element val onto the stack.
///     void pop() removes the element on the top of the stack.
///     int top() gets the top element of the stack.
///     int getMin() retrieves the minimum element in the stack.
///
/// You must implement a solution with O(1) time complexity for each function.
///  
/// <strong class="example">Example 1:
///
/// Input
/// ["MinStack","push","push","push","getMin","pop","top","getMin"]
/// [[],[-2],[0],[-3],[],[],[],[]]
/// Output
/// [null,null,null,null,-3,null,0,-2]
/// Explanation
/// MinStack minStack = new MinStack();
/// minStack.push(-2);
/// minStack.push(0);
/// minStack.push(-3);
/// minStack.getMin(); // return -3
/// minStack.pop();
/// minStack.top();    // return 0
/// minStack.getMin(); // return -2
///
///  
/// Constraints:
///
///     -2^31 <= val <= 2^31 - 1
///     Methods pop, top and getMin operations will always be called on non-empty stacks.
///     At most 3 * 10^4 calls will be made to push, pop, top, and getMin.
pub struct Solution {}

// problem: https://leetcode.com/problems/min-stack/
// discuss: https://leetcode.com/problems/min-stack/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(dead_code)]
struct MinStack {
    vec: Vec<(i32, i32)>,
}

#[allow(dead_code)]
impl MinStack {
    fn new() -> Self {
        Self { vec: vec![] }
    }

    fn push(&mut self, val: i32) {
        let mut min = val;
        if let Some(v) = self.vec.last() {
            min = cmp::min(v.1, min);
        }

        self.vec.push((val, min));
    }

    fn pop(&mut self) {
        self.vec.pop();
    }

    fn top(&self) -> i32 {
        self.vec.last().unwrap().0
    }

    fn get_min(&self) -> i32 {
        self.vec.last().unwrap().1
    }
}

// /// Your MinStack object will be instantiated and called as such:
// /// let obj = MinStack::new();
// /// obj.push(val);
// /// obj.pop();
// /// let ret_3: i32 = obj.top();
// /// let ret_4: i32 = obj.get_min();

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_155() {
        let mut min_stack = MinStack::new();
        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);
        assert_eq!(min_stack.get_min(), -3);
        min_stack.pop();
        assert_eq!(min_stack.top(), 0);
        assert_eq!(min_stack.get_min(), -2);
        min_stack.pop();
        assert_eq!(min_stack.top(), -2);
        assert_eq!(min_stack.get_min(), -2);
    }
}
