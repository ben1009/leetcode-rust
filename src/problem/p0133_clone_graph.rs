/**
[133] Clone Graph

Given a reference of a node in a connected undirected graph.

Return a deep copy (clone) of the graph.

Each node in the graph contains a value (int) and a list (List[Node]) of its neighbors.

class Node {
    public int val;
    public List<Node> neighbors;
}

Test case format:

For simplicity, each node's value is the same as the node's index (1-indexed). For example, the first node with val == 1, the second node with val == 2, and so on. The graph is represented in the test case using an adjacency list.

An adjacency list is a collection of unordered lists used to represent a finite graph. Each list describes the set of neighbors of a node in the graph.

The given node will always be the first node with val = 1. You must return the copy of the given node as a reference to the cloned graph.

Example 1:

Input: adjList = [[2,4],[1,3],[2,4],[1,3]]
Output: [[2,4],[1,3],[2,4],[1,3]]
Explanation: There are 4 nodes in the graph.
1st node (val = 1)'s neighbors are 2nd node (val = 2) and 4th node (val = 4).
2nd node (val = 2)'s neighbors are 1st node (val = 1) and 3rd node (val = 3).
3rd node (val = 3)'s neighbors are 2nd node (val = 2) and 4th node (val = 4).
4th node (val = 4)'s neighbors are 1st node (val = 1) and 3rd node (val = 3).
Example 2:

Input: adjList = [[]]
Output: [[]]
Explanation: Note that the input contains one empty list. The graph consists of only one node with val = 1 and it does not have any neighbors.
Example 3:

Input: adjList = []
Output: []
Explanation: This an empty graph, it does not have any nodes.

Constraints:

The number of nodes in the graph is in the range [0, 100].
1 <= Node.val <= 100
Node.val is unique for each node.
There are no repeated edges and no self-loops in the graph.
The Graph is connected and all nodes can be visited starting from the given node.
*/
pub struct Solution {}

use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct GraphNode {
    val: i32,
    neighbors: Vec<Rc<RefCell<GraphNode>>>,
}

impl GraphNode {
    pub fn new(val: i32) -> Self {
        Self {
            val,
            neighbors: vec![],
        }
    }
}

impl Solution {
    pub fn clone_graph(node: Option<Rc<RefCell<GraphNode>>>) -> Option<Rc<RefCell<GraphNode>>> {
        node.as_ref()?;

        let mut dic: HashMap<i32, Rc<RefCell<GraphNode>>> = HashMap::new();
        Self::bfs(node.unwrap(), &mut dic);

        Some(dic.get(&1).unwrap().clone())
    }

    fn bfs(node: Rc<RefCell<GraphNode>>, dic: &mut HashMap<i32, Rc<RefCell<GraphNode>>>) {
        let mut stack = vec![];
        let k = node.as_ref().borrow().val;
        let t = Rc::new(RefCell::new(GraphNode::new(k)));
        dic.insert(k, t);
        stack.push(node);

        while !stack.is_empty() {
            let node = stack.pop().unwrap();
            let k = node.as_ref().borrow().val;
            for n in node.as_ref().borrow().neighbors.iter() {
                let k1 = n.borrow().val;
                let t1 = dic.get(&k1);
                let mut tt = Rc::new(RefCell::new(GraphNode::new(0)));
                match t1 {
                    Some(t1) => {
                        tt = t1.clone();
                    }
                    None => {
                        tt = Rc::new(RefCell::new(GraphNode::new(k1)));
                        dic.insert(k1, tt.clone());
                        stack.push(n.clone());
                    }
                }
                dic.get(&k)
                    .unwrap()
                    .as_ref()
                    .borrow_mut()
                    .neighbors
                    .push(tt);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_133() {
        assert_eq!(
            Solution::clone_graph(Some(Rc::new(RefCell::new(GraphNode::new(1))))),
            Some(Rc::new(RefCell::new(GraphNode::new(1))))
        );

        let n1 = Some(Rc::new(RefCell::new(GraphNode::new(1))));
        let n2 = Some(Rc::new(RefCell::new(GraphNode::new(2))));
        n1.as_ref()
            .unwrap()
            .borrow_mut()
            .neighbors
            .push(n2.as_ref().unwrap().clone());
        n2.as_ref()
            .unwrap()
            .borrow_mut()
            .neighbors
            .push(Some(Rc::new(RefCell::new(GraphNode::new(3)))).unwrap());

        // TODO: should checked by is_same_graph instead of eq
        assert_eq!(
            Solution::clone_graph(n1.clone()),
            n1,
            "clone_graph test fail"
        );
        assert_ne!(
            Solution::clone_graph(n1.clone()).unwrap().as_ptr(),
            n1.clone().unwrap().as_ptr(),
            "clone_graph test fail"
        );
        assert_eq!(
            n1.clone().unwrap().as_ptr(),
            n1.unwrap().as_ptr(),
            "clone_graph test fail"
        );
    }
}
