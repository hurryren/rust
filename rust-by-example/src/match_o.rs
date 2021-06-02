#![allow(dead_code)]

use std::option;

pub fn main_match() {
    // f_match();
    // match_tuple();
    // match_enum();
    // f_reference();
    // match_struct();
    // match_guard();
    // f_if_net();
    while_let();
}

fn f_match() {
    let number = 5;

    println!("Tell me about {}", number);
    match number {
        1 => println!("one"),
        2 | 3 | 5 | 7 | 11 => println!("prime"),
        13..=19 => println!(" A teen"),
        _ => println!("ain't special"),
    }

    let boolean = false;
    let binary = match boolean {
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);
}

// 元组可以在 match 中结构
fn match_tuple() {
    let pair = (12, 1);

    println!("Tell me about {:?}", pair);

    match pair {
        (0, y) => println!("First is 0 and y is {:?}", y),
        (x, 0) => println!("x is {:?} and y is 0", x),
        _ => println!("It doesn't matter what they are"),
    }
}

fn match_enum() {
    enum Color {
        Red,
        Blue,
        Green,

        RGB(u32, u32, u32),
        HSV(u32, u32, u32),
        HSL(u32, u32, u32),
        CMY(u32, u32, u32),
        CMYK(u32, u32, u32, u32),
    }

    // let color = Color::RGB(122, 17, 40);
    let color = Color::Red;

    println!("what color is it?");
    match color {
        Color::Red => println!("The color is red"),
        Color::Blue => println!("The color is Blue"),
        Color::Green => println!("The color is Green"),
        Color::RGB(r, g, b) => println!("red: {}, green: {}, blue: {}. ", r, g, b),
        Color::HSV(h, s, v) => println!("Hue: {}, saturation: {}, value: {}. ", h, s, v),
        Color::HSL(h, s, l) => println!("Hue: {}, saturation: {}, lightness: {}. ", h, s, l),
        Color::CMY(c, m, y) => println!("Cyan: {}, mangenta: {}, yellow: {}. ", c, m, y),
        Color::CMYK(c, m, y,k) => println!("Cyan: {}, mangenta: {}, yellow: {}, key (black) {}. ", c, m, y,k),
    }
}

fn f_reference(){
    // 获得一个 i32 类型的引用， & 表示取引用
    let reference = &4;

    match reference{
        /*
        如果用 &val 取匹配reference 相当于 &i32---&val
        如果去掉匹配的 &, i32应该被赋值gei val
        因此可以用 val 表示被 reference 应用的值 4
        */
        &val => println!("Got a value via dereferenceing: {:?}",val),
    }

    // 如果不想用 &，需要在匹配前解引用
    match *reference {
        val => println!("Got a value via dereferenceing: {:?}",val),
    }

    /*
    如果 reference 不是一个 & 类型。
    可以使用 ref, ref 更改了赋值行为，从而可以对具体值创建引用
    */
    let _not_a_reference = 3;
    let ref is_a_reference = 3;

    // 对于非引用变量，通过 ref 和 ref mut 可以取得其引用
    let value = 5;
    let mut mut_value  = 6;

    match value{
        ref r => println!("Got a reference to a value {:?}",*r),
    }

    // 类似的使用 ref mut
    match mut_value{
        ref mut m => {
            // 已经获得了 mut_value 的引用，先要解引用，才能改变它的值
            *m += 10;
            println!("we added 10, ‘mut_value : {:?}",m);
        }
    }

}

fn match_struct(){
    struct Foo{x:(u32,u32),y:u32}

    let foo = Foo{x:(1,2),y:3};
    let Foo {x:(a,b),y} = foo;

    let Foo{y:i,x:j}=foo;
    println!("i = {:?}, j = {:?}",i,j);

    let Foo{y,..} = foo;
    println!("y = {}",y);

}

fn match_guard(){
    let pair = (3,3);

    println!("Tell me about {:?}", pair);
    match pair{
        (x,y) if x == y => println!("These are twins"),
        (x,y) if x % 2 == 1  => println!("the first one is odd"),
        _ => println!("no correlation..."),

    }
}

fn f_if_net(){
    let number = Some(7);
    let letter:Option<i32> = None;
    let emoticon:Option<i32> = None;

    if let Some(i) = number{
        println!("matched {:?}!",i);
    };

    if let Some(i) = letter{
        println!("matched {:?}!",i);
    }else{
        println!("Didn't match a number. Let's go with a letter!");
    };

    let i_like_letters = false;

    if let Some(i) = emoticon{
        println!("Matched {:?}",i);
    }else if i_like_letters{
        println!("Didn't match a number. Let's go with a letter");
    }else{
        println!("I don't like letters. let's go with an emotion:");
    };

}


fn while_let(){
    let mut optional = Some(0);

    while let Some(i) = optional {
        if i > 9{
            println!("greater than 9, quit");
            optional = None;
        }else {
            println!("i is {:?}. try again",i);
            optional = Some(i+1);
        }
    }
}