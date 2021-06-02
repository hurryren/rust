#![allow(dead_code)]

use std::borrow::Borrow;

pub fn main_function(){
    // hello_func();
    method_func();
}

fn hello_func(){
    fizzbuzz_to(100);

    fn fizzbuzz_to(n:u32){
        for n in 1..=n{
            fizzbuzz(n);
        }
    }

    fn fizzbuzz(n:u32) -> () {
        if is_divisible_by(n,15){
            println!("fizzbuzz");
        }else if is_divisible_by(n,3){
            println!("fizz");
        }else if is_divisible_by(n,5){
            println!("buzz");
        }else{
            println!("{}",n);
        }
    }

    fn is_divisible_by(lhs:u32,rhs:u32) -> bool{
        if rhs == 0{
            return false
        }

        lhs % rhs == 0
    }


}

fn method_func(){
    struct Point{
        x:f64,
        y:f64,
    }

    impl Point {
        fn origin() -> Point{
            Point{x:0.0,y:0.0}
        }

        fn new(x:f64,y:f64) -> Point{
            Point{x:x,y:y}
        }
    }

    struct Retangle{
        p1:Point,
        p2:Point,
    }

    impl Retangle{
        fn area(&self) -> f64{
            let Point{x:x1,y:y1} = self.p1;
            let Point{x:x2,y:y2} = self.p2;
            ((x1-x2) * (y1-y2)).abs()
        }

        fn perimeter(&self) -> f64{
            let Point{x:x1,y:y1} = self.p1;
            let Point{x:x2,y:y2} = self.p2;
            2.0 * ((x1-x2).abs() * (y1-y2).abs())
        }

        fn translate(&mut self, x:f64,y:f64){
            self.p1.x += x;
            self.p2.x += x;

            self.p1.y += y;
            self.p2.y += y;
        }
    }

    // pair 拥有两个堆中的整形数据
    struct Pair(Box<i32>, Box<i32>);

    impl Pair{
        fn destroy(self){
            let Pair(first,second ) = self;
            println!("Destroying Pair ({}, {})",first,second);

        }
    }

    // 静态方法使用双冒号调用
    // 示例方法通过点运算符调用
    let rectangle = Retangle{
        p1:Point::origin(),
        p2:Point::new(3.0,4.0),
    };
    println!("Rectangle perimeter: {}",rectangle.perimeter());
    println!("Rectangle area: {}",rectangle.area());

    let mut square = Retangle{
        p1: Point::origin(),
        p2: Point::new(1.0,1.0),
    };

    // rectangle.translate(1.0,0.0);

    square.translate(1.0,1.0);

    let pair = Pair(Box::new(1),Box::new(2));

    pair.destroy();




}