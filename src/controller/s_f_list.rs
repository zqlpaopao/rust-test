#![allow(unused)]

pub fn test_list() {
    //203 移除链表元素
    let l = ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 3, next: None })),
        })),
    };
    let res = remove_elements(Some(Box::new(l)), 2);
    println!("203 删除单链表元素结果{:?}", res);
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

//************************* 203 移除链表元素
pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    // let mut dummy_head = Box::new(ListNode{ val, next: None });
    //   dummy_head.next = head;
    //   let mut cur = dummy_head.as_mut();
    //   while let  Some (nxt) = cur.next.take() {
    //       if nxt.val == val{
    //           cur.next = nxt.next;
    //       }else{
    //           cur.next = Some(nxt);
    //           cur = cur.next.as_mut().unwrap();
    //       }
    //   }
    //
    //   dummy_head.next

    let mut dummy_head = Box::new(ListNode { val, next: None });
    dummy_head.next = head;
    let mut cur = dummy_head.as_mut();
    while let Some(nex) = cur.next.take() {
        if nex.val.eq(&val) {
            cur.next = nex.next;
        } else {
            cur.next = Some(nex);
            cur = cur.next.as_mut().unwrap();
        }
    }

    dummy_head.next
}

//************************* 206 反转链表
pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut cur = head;
    let mut pre = None;
    while let Some(mut node) = cur.take() {
        cur = node.next;
        node.next = pre;
        pre = Some(node);
    }
    pre
}
