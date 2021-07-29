use std::fmt::format;

fn main() {
    {
        /* */
        println!();
        println!();
        println!("--------- removing duplication by extracting a function ---------");
        let number_list = vec![34, 50, 25, 100, 65];
        let mut largest = number_list[0];
        for number in number_list {
            if number > largest {
                largest = number;
            }
        }
        println!("The largest number is {}", largest);

        let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
        let mut largest = number_list[0];
        for number in number_list {
            if number > largest {
                largest = number;
            }
        }
        println!("The largest number is {}", largest);

        {
            fn largest(list: &[i32]) -> i32 {
                let mut largest = list[0];
                for &item in list {
                    if item > largest {
                        largest = item;
                    }
                }
                largest
            }

            let number_list = vec![34, 50, 25, 100, 65];

            let result = largest(&number_list);
            println!("The largest number is {}", result);

            let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

            let result = largest(&number_list);
            println!("The largest number is {}", result);
        }
    }

    {
        /* */
        println!();
        println!();
        println!("--------- in method definitions ---------");
        struct Point<T, U> {
            x: T,
            y: U,
        }

        impl<T, U> Point<T, U> {
            fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
                Point {
                    x: self.x,
                    y: other.y,
                }
            }
        }

        let p1 = Point { x: 5, y: 10.4 };
        let p2 = Point { x: "Hello", y: 'c' };

        let p3 = p1.mixup(p2);
        println!("P3.x = {} , p3.y = {}", p3.x, p3.y);
    }

    {
        /*
        Rust accomplishes this by performing monomorphization of the code that
        is using generics at compile time. Monomorphization is the process of
        turning generic code into specific code by filling the concrete types
        that are used when compiled.

        */
        println!();
        println!();
        println!("--------- perfomance of code using generics ---------");


    }


    {
        /*
        a type's behavior consists of the methods we can call on that type. Different
        types share the same behavior if we can call the same methods on all of those
        types. Trait definitions are a way to group method signatures togeter to define
        a set of behaviors necessary to accomplish some purpose
        */
        println!();
        println!();
        println!("--------- define a trait ---------");

    }

    {
        /* */
        println!();
        println!();
        println!("--------- implementing a trait on a type ---------");
        pub trait Summary{
            fn summarize(&self) ->String;
        }

        pub struct NewsArticle{
            pub headline:String,
            pub location:String,
            pub author:String,
            pub content:String,
        }

        impl Summary for NewsArticle{
            fn summarize(&self) ->String {
                format!("{}, by {} ({})",self.headline,self.author,self.location)
            }
        }

        pub struct Tweet{
            pub username:String,
            pub content:String,
            pub replay:String,
            pub retweet:String,
        }

        impl Summary for Tweet {
            fn summarize(&self) ->String {
                format!("{}: {}",self.username,self.content)
            }
        }

        let tweet = Tweet{
            username:String::from("horse_eboks"),
            content:String::from("of course, as you probably already know, people",),
            replay:String::from("null"),
            retweet:String::from("null"),
        };
        println!("1 new tweet: {}", tweet.summarize());

    }



}
