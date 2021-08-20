
struct Node{
    value: i32,
    next:Option<Box<Node>>,
}
fn main(){
    let mut first = Box::new(Node{value:1,next:None});
    let mut second = Box::new(Node{value:2,next:None});
    let mut third = Box::new(Node{value:3,next:None});
    second.next = Some(third);
    first.next = Some(second);

    let mut curr:Option<&Box<Node>> = Some(&first);
    while curr.is_some(){
        println!("{}",curr.unwrap().value);
        let next:&Option<Box<Node>> = &curr.unwrap().next;
        curr = next.as_ref();
    }

}