// using structs to structure related data
/*
    Structs let you create custom typs that are meaningful for your domain. By using
    structs, you can keep associated pieces of data connected to each other and name
    each piece to make your code clear, Methods let you specify the behavior that
    instances of your structs have, and associated functions let you namesapce
    functionslity that is particular to your struct without having an instance available


*/



#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self,other:&Rectangle) ->bool{
        self.width > other.width && self.height > other.height
    }

    fn square(size:u32) ->Rectangle{
        Rectangle{
            width:size,
            height:size,
        }
    }
}

fn main() {
    {
        println!();
        println!();
        println!("--------- struct method ---------");
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        println!("the area of the rectangle is {} square pixels.",rect1.area());
    }

    {
        println!();
        println!();
        println!("--------- methods with more parameters ---------");
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        let rect2 = Rectangle {
            width: 10,
            height: 40,
        };
        let rect3 = Rectangle {
            width: 60,
            height: 40,
        };
        println!("can rect1 hold rect2? {}",rect1.can_hold(&rect2));
        println!("can rect1 hold rect3? {}",rect1.can_hold(&rect3));

    }

    {
        /*
            Another usefulfeature of impl blocks is that we're allowed to define
            functions within impl blocks that don't take self as a parameter. These are
            called associated functions because they're associated with the struct. They're
            still functions, not methods, because they don't have an instance of the struct to
            work with. You've already used the String::from assiciated functions

            Associated functions are often used for constructors that will return a new
            instance of the struct. for example, we could provide an associateed functin that
            wouldhave one dimension parameter and use that as both width and height,
            thus making it easier to create a square Rectangle rather than having to specify the
            same value twice.

        */
        println!();
        println!();
        println!("---------  ---------");

    }

    {
        /* */
        println!();
        println!();
        println!("---------  ---------");

    }


}
