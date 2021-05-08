#![allow(dead_code)]

pub fn main_grammer() {
    // f_loop();
    // f_loop1();
    // f_loop_back();
    // f_for()
    f_for_iter();
}

fn f_loop() {
    let mut count = 0u32;

    println!("let's count untiil infinity");

    loop {
        count += 1;
        if count == 3 {
            println!("three");

            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("Ok, that's enough");
            break;
        }
    }
}

fn f_loop1() {
    'outer: loop {
        println!("entered the outer loop");

        'inner: loop {
            println!("entered the inner loop");

            // break; // 终断内部循环

            break 'outer;
        }
        println!("This point will never be reached");
    }

    println!("exited the outer loop");
}

fn f_loop_back() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}

/*
a .. b 左闭右开
a ..=b 闭区间

*/
fn f_for(){
    for n in 1 ..101{
        if n % 15 == 0{
            println!("fizzbuzz");
        }else if n % 3 == 0{
            println!("fizz");
        }else if n % 5 == 0 {
            println!("buzz");
        }else{
            println!("{}",n);
        }
    }
}

/*
    for in 结构能以几种方式与 Iterator 互动，如果没有特别指定，
    for 循环会对给出的集合应用 into_iter 函数，
    还有 iter 和 iter_mut 函数

    - iter
        每次迭代中借用集合中的一个元素。这样集合本身不会被改变，循环之后仍可以使用
    - into_iter
        所有权被转移，之后无法在使用
    - iter_mut
        可变借用，允许集合被修改
*/

fn f_for_iter(){
    let names = vec!["xiao", "ming","hong"];

    for name in names.iter(){
        match name{
            &"xiao" => println!("there is a chinese among us!"),
            _ => println!("hello {}",name),
        }
    }
}