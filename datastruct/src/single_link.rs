use std::fmt::{Debug, Display, Error};
use std::option::Option::Some;

pub fn description(){
    println!("this is single link")
}

// 链表节点数据结构
#[derive(Debug, Clone)]
struct link_node<T> {
    data: T,
    next: Option<Box<link_node<T>>>,
}

// 链表
#[derive(Debug)]
struct link_list<T>{
    head: Option<Box<link_node<T>>>,
}
// 链表节点
impl<T> link_node<T> {
    // 新建节点
    fn new(data: T)->Self{
        Self{data:data, next: None}
    }
}

// 链表及操作
impl<T> link_list<T> {
    // 新建一个链表
    fn new()->Self{
        link_list{
            head: None
        }
    }
    // 从头部插入
    fn head_insert(&mut self, data: T) {
        let mut insert = Box::new(link_node::new(data));
        match self.head {
            None => {
                self.head = Some(insert)
            }
            Some(_) => {
                insert.next = self.head.take();
                self.head = Some(insert);
            }
        }
    }
    // 从尾部插入
    fn rear_insert(&mut self, data: T){
        let insert = Box::new(link_node::new(data));
        match self.head {
            None => {
                self.head = Some(insert);
            }
            Some(_) => {
                let mut last = &mut self.head;
                while !last.as_mut().unwrap().next.is_none() {
                    last = &mut last.as_mut().unwrap().next;
                }
                last.as_mut().unwrap().next = Some(insert);
            }
        }
    }

    /**
    从指定位置插入 ， 使用Box类型无法实现从指定位置插入
    实现方法:
    1. 根据pos找到要插入位置的前驱节点previous_node
    2. 设新节点为insert则:
        insert.next = previous_node.next
        previous_node.next = insert
    */
    fn position_insert(&mut self, data: T, pos: i32) ->Result<i32, String> {
        // 1.构造新节点
        let mut insert = Box::new(link_node::new(data));
        if self.head.is_none() && pos == 0 {
            self.head = Some(insert);
            return Ok(pos);
        }
        // 2.寻找前驱节点
        let mut previous_node = &mut self.head;
        for i in 0..pos{
            if previous_node.is_none() {
                return Err("position not exists".to_string());
            }
            previous_node = &mut previous_node.as_mut().unwrap().next;
        }
        // 3. 将新节点插入到链表中间
        // 这里编译不通过 实现两端对接新节点insert， 涉及到前后两部分数据所有权问题，不好搞
        //insert.next = previous_node.clone().unwrap().next;
        //previous_node.unwrap().next = Some(insert);
        //

        // previous_node.as_mut().unwrap().next = Some(insert);


        // let mut a = previous_node.take().unwrap();
        // insert.next = a.next;
        // a.next = Some(insert);

        //insert.next = previous_node.unwrap().next;
        //previous_node.unwrap().next = Some(insert);

        return Ok(pos);
    }
}

#[test]
fn demo_head_insert(){
    let mut list = link_list::new();
    list.head_insert(1);
    list.head_insert(2);
    list.head_insert(3);
    list.head_insert(4);
    list.head_insert(5);
    let mut next = &list.head;
    println!("head_insert print link list data: ");
    while let Some(node) = next{
        print!("{}->", node.data);
        next = &node.next
    }
   println!("\n{:?}", list)
}

#[test]
fn demo_rear_insert(){
    let mut list = link_list::new();
    list.rear_insert(10);
    list.rear_insert(11);
    list.rear_insert(12);
    list.rear_insert(13);
    let mut next = &list.head;
    println!("rear_insert print link list data: ");
    while let Some(node) = next{
        print!("{}->", node.data);
        next = &node.next
    }
    println!("\n{:?}", list)
}

// 测试从指定位置插入新节点 todo
#[test]
fn demo_position_insert() {
    let mut list = link_list::new();
    list.rear_insert(20);
    list.rear_insert(21);
    list.rear_insert(22);
    list.rear_insert(23);

    // 在第二个位置插入100
    list.position_insert(100, 2);
    list.position_insert(101, 4);

    let mut next = &list.head;
    println!("position_insert print link list data: ");
    while let Some(node) = next{
        print!("{}->", node.data);
        next = &node.next
    }
    println!("\n{:?}", list)
}
