use std::cmp::Ordering;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Ord for ListNode {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val)
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// helper function for test
pub fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut current = None;
    for &v in vec.iter().rev() {
        let mut node = ListNode::new(v);
        node.next = current;
        current = Some(Box::new(node));
    }
    current
}

#[macro_export]
macro_rules! linked {
    ($($e:expr),*) => {to_list(vec![$($e.to_owned()), *])};
    ($($e:expr,)*) => {to_list(vec![$($e.to_owned()), *])};
}

#[macro_export]
macro_rules! list {
    () => {
        None::<Box<ListNode>>
    };
    ($x:expr$(,)?)=> {
        Some(Box::new(ListNode::new($x)))
    };
    ($x:expr$(,$y:expr)+$(,)?) => {{
        let mut head = Some(Box::new(ListNode::new($x)));
        let mut next = &mut head;
        $(
            next = &mut next.as_mut().unwrap().next;
            *next = Some(Box::new(ListNode::new($y)));
        )*
        head
    }};
}
