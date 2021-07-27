

fn main() {

    {
        /* */
        println!();
        println!();
        println!("--------- match with Option<T> ---------");
        fn plus_one(x:Option<i32>)->Option<i32>{
            match x{
                None => None,
                Some(i) => Some(i+1),
            }
        }
        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);
        println!("six {:?}, None {:?}",six,none);


    }

    {
        /* */
        println!();
        println!();
        println!("--------- The _ placeholder ---------");
        let some_u8_value = 3u8;
        match some_u8_value{
            1 => println!("one"),
            3 => println!("three"),
            5 => println!("five"),
            7 => println!("seven"),
            _ => (),
        }

    }

    {
        /*
        the if let syntax lets you combine if and let into a less verbose way
        to handle values that amtch one pattern while ignoring the reset.
        */
        println!();
        println!();
        println!("--------- Concise control flow with if let ---------");
        let some_u8_value = Some(3u8);
        match some_u8_value{
            Some(3) => println!("match: three"),
            _ => (),
        }
        if let Some(3) = some_u8_value{
            println!("if let: three");
        }

    }


    {
        /* */
        println!();
        println!();
        println!("---------  ---------");

    }




}
