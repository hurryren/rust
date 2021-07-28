use std::fmt::format;

/*
    Rust's standard library includes a number of very useful data structures
    called collections. Most other data types represent one specific value,
    but collections can contain multiple values. Unlike the built-in array
    and tuple types, the data these collections point to is stored on the
    heap, which means the amount of data does not need to be known at compile
    time and can grow or shrink as the program runs. Each kind of collection
    has different capabilities and costs, and choosing an appropriate one for
    your current situation is a skill you'll develop over time.

    a vector allows you to store a variable number of values next to each other
    a string is a collection of characters. We've mentioned the string type
    previously,
    a hash map allows you to associate a value with a particular key. It;s a
    particular implementation of the more general data structure called a map.
*/


fn main() {

    {
        /* */
        println!();
        println!();
        println!("--------- creating a new vector ---------");
        let  v:Vec<i32> = Vec::new();
        println!("{:?}",v);
        let mut v = vec![1,2,3];
        println!("{:?}",v);
        v.push(5);
        println!("{:?}",v);

    }

    {
        /* */
        println!();
        println!();
        println!("--------- reading elements of vectors ---------");
        let v = vec![1,2,3,4,5];

        let third:&i32 = &v[2];
        println!("The third element is {}",&third);

        match v.get(2) {
            Some(third) => println!("The third element is {}",third),
            None => println!("There is no third element"),
        }

    }

    {
        /* */
        println!();
        println!();
        println!("--------- iterating over the values in a vector ---------");
        let v = vec![1,2,3,4,5];
        for i in &v{
            println!("{}",i);
        }

        let mut v = vec![1,2,3,4,5];
        for i in &mut v{
            *i += 50;
        }

        for i in &v{
            println!("{}",i);
        }

    }

    {
        /*
        the columns contain integers, some floating-point numbers, and sme string.
        we can define an enum whose variants will hold the different value types,
        and then all the enum variants will be considered the same type: that of
        the enum. Then we can create a vector that holds enum and so, ultimately,
        holds different types.
        */
        println!();
        println!();
        println!("--------- useing an enum to store multiple types ---------");
        #[derive(Debug)]
        enum SpreadsheetCall{
            Int(i32),
            Float(f64),
            Text(String),
        }

        let row = vec![
            SpreadsheetCall::Int(3),
            SpreadsheetCall::Text(String::from("Blue")),
            SpreadsheetCall::Float(10.02),
        ];

        for i in &row{
            println!("{:?}",i);
        }

    }

    {
        /* */
        println!();
        println!();
        println!("--------- creating a new string ---------");
        let mut s = String::new();
        let data = "initial contents";

        let s = data.to_string();
        let s = "initial contenst".to_string();

        let hello = String::from("السلام عليكم");
        let hello = String::from("Dobrý den");
        let hello = String::from("Hello");
        let hello = String::from("שָׁלוֹם");
        let hello = String::from("नमस्ते");
        let hello = String::from("こんにちは");
        let hello = String::from("안녕하세요");
        let hello = String::from("你好");
        let hello = String::from("Olá");
        let hello = String::from("Здравствуйте");
        let hello = String::from("Hola");
    }


    {
        /*
        a string can grow in size and its contns can change, just like the contens
        of a Vec<T>, if you push more data into it, in addition, you can conveniently
        use the + operator or the format! macro to concatenate string values
        */
        println!();
        println!();
        println!("--------- updating a string ---------");
        let mut s = String::from("foo");
        println!("{}",s);
        // s.push_str("\n");
        s.push_str("bar");
        println!("{}",s);

        let mut s1 = String::from("foo");
        let  s2 = "bar";
        s1.push_str(s2);
        println!("s1 is {}",s1);
        println!("s2 is {}",s2);

    }

    {
        /*
        the push method takees a single character as a parameter and adds it to the String.
        */
        println!();
        println!();
        println!("---------  ---------");
        let mut s = String::from("lo");
        s.push('l');
        println!("{}",s);
    }



    {
        /* */
        println!();
        println!();
        println!("--------- concatention with the + operator or the format! macro ---------");
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2;
        // note s1 has been moved here and can longer be used

        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
        let s = s1 + "-" + &s2 + "-" +&s3;


        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
        let s = format!("{}-{}-{}",s1,s2,s3);
    }

    {
        /*
        A String is a wrapper over a Vec<u8>;
        in utf-8, bucause each Unicode scalar value in that string takes 2 buytes of storage.
        Therefore, an index into the string's bytes will not always correlate to a valid unicode
        scalar value.
        */
        println!();
        println!();
        println!("--------- indexing into string ---------");
        let hello = String::from("hola"); // len = 4 bytes

        let hello = String::from("Здравствуйте"); // len = 24 bytes


    }

    {
        /* */
        println!();
        println!();
        println!("--------- creating a new hash map ---------");
        use std::collections::HashMap;

        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"),10);
        scores.insert(String::from("yellow"),50);

        let teams = vec![String::from("Blue"),String::from("Yellow")];
        let initial_scores = vec![10,50];

        let mut scores: HashMap<_,_> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    }


    {
        /* */
        println!();
        println!();
        println!("--------- hash maps and ownership ---------");
        use std::collections::HashMap;

        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");

        let mut map = HashMap::new();
        map.insert(field_name, field_value);
        // field_name and field_value are invalid at this point, try using them and
        // see waht compiler error you get
        // println!("{}",field_name);

    }


    {
        /* */
        println!();
        println!();
        println!("---------  ---------");

    }



}
