#![allow(dead_code)]

use std::rc::Weak;
pub fn main_data_type() {
    // structure1();
    // enumeration1();
    // test_list();
    const_value1();
}

fn structure1() {
    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    // 单元结构体
    struct Nil;

    // 元组结构体
    struct Pair(i32, f32);

    struct Point {
        x: f32,
        y: f32,
    }

    struct Retangle {
        p1: Point,
        p2: Point,
    }

    let name = "orange";
    let age = 27;
    let orange = Person { name, age };

    println!("{:?}", orange);

    let point: Point = Point { x: 0.3, y: 0.4 };

    println!("point coordinates: ({}, {})", point.x, point.y);

    // 使用结构体更新语法创建新的 point, 这样可以用到之前的字段
    let new_point = Point { x: 0.1, ..point };
    println!("second point : ({}, {})", new_point.x, new_point.y);

    // 使用结构体体来解构 point
    let Point { x: my_x, y: my_y } = point;

    let _rectangel = Retangle {
        // 结构体的实例化也是一个表达式
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };

    let _nil = Nil;

    let pair = Pair(1, 0.1);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}

fn enumeration1() {
    enum WebEvent {
        PageLoad,
        PageUnload,
        KeyPress(char),
        Paste(String),
        Click { x: i64, y: i64 },
    }

    // 此函数将一个 WebEvent enum 作为参数，无返回值
    fn inspect(event: WebEvent) {
        match event {
            WebEvent::PageLoad => println!("page loaded"),
            WebEvent::PageUnload => println!("page unloaded"),
            WebEvent::KeyPress(c) => println!("pressed '{}.", c),
            WebEvent::Paste(s) => println!("pasted \" {} \".", s),
            WebEvent::Click { x, y } => {
                println!("clicked at x = {}, y = {}.", x, y);
            }
        }
    }

    let pressed = WebEvent::KeyPress('x');
    // to_owned() 从一个字符串切片中创建一个具有所有权的 String
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}

fn test_list() {
    use List::*;

    enum List {
        // cons: 元组结构体， 包含一个元素和一个指向下一个节点的指针
        Cons(u32, Box<List>),
        // Nil: 末节点，表明链表结束
        Nil,
    }

    impl List {
        fn new() -> List {
            // Nil 为 list 类型， Nil的完整名称是 List::Nil
            Nil
        }

        // 处理一个list, 在其头部插入新元素，并发挥该 list
        fn prepend(self,elem:u32) -> List{
            Cons(elem,Box::new(self))
        }

        // 返回list的长度
        fn len(&self) ->u32{
            // 必须对 self 进行匹配（match)，
            // self 为 &List 类型， *self 为 List 类型
            // 匹配一个具体的 T 类型要好过匹配引用 &T
            match *self {
                Cons(_,ref tail) => 1 + tail.len(),
                // 递归的基准情形（base case): 一个长度为 0 的空列表
                Nil => 0
            }
        }

        // 返回列表的字符串表示（该字符串存储在堆中）
        fn stringify(&self) -> String{
            match *self{
                Cons(head, ref tail) =>{
                    format!("{}, {}",head,tail.stringify())
                },
                Nil => {
                    format!("Nil")
                },
            }
        }
    }

    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // 显示链表最后的状态
    println!("linked list has length: {}",list.len());
    println!("{}",list.stringify());
}

// 全局变量在所有其他作用域之外声明
static LANGUAGE: &'static str = "Rust";
const THRESHOLD: i32 = 10;
fn const_value1(){
    fn is_big(n : i32) -> bool{
        n > THRESHOLD
    }

    let n = 16;
    println!("This is {}", LANGUAGE);
    println!("This THRESHOLD is {}", THRESHOLD);
    println!("{} is {}",n, if is_big(n) {"big"}else{"small"});

}
