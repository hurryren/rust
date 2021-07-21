use std::usize;

fn main() {
    println!("Hello, world!");
    let  x:u32 = 0;
    let y = x;
    println!("{} {}",x,y);

    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{} [{}",s1,s2);
    // call_fun(x);

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1={}, s2={}",s1,s2);

    /*
here are some of the types that implement Copy:
    1. All the integer types, such as u32
    2. The boolean type, bool, with values true and false
    3. All the floating point types, such as f64
    4. he character type, char
    5. Tuples, if they only contain types that also implemnt cipy.
        For example, (i32,i32) implements copy, but (i32, String) does not

    */


    /*
    The semantics for passing a value to a function are similar to those for assigning a value
    to a variable. Passing a variable to a function will move or copy, just as assignment does.
    */
    println!("------------");
    println!("Ownership and funtions");
    let s = String::from("hello");  //  s comes into scope
    takes_ownership(s);         //
                                            // and so is no longer value here

    let x = 5;
    makes_copy(x);

    println!("------------");
    println!("return values and scope");
    let s1 = gives_owenership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);


    println!("------------");
    println!("reference and borrowing");
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of ‘{}’ is {}", s1, len);
    {
        println!("------------");
        println!("the slice type");
        let mut s = String::from("nihao, hah");
        let b= first_word(&s);
        println!("{}",b);
        s.clear(); //this empties the string, makeing it equal to ""
        // word still has the valie 5 here, but there's no more string that
        // we could meaninfully use the value 5 with. word is now totally invalid
    }

    {
        println!("------------");
        println!("string slice");
        // a string slice is a reference to part of a string, and it looks likes this
        let s = String::from("helloworld ");
        let hello = &s[0..5];
        let world = &s[6..11];
        let slice = first_word_in_slice(&s);
        println!("{}",slice);
    }

}

fn first_word_in_slice(s:&String)-> &str{
    let bytes = s.as_bytes();

    for (i,&item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}


fn first_word(s:&String) ->usize{
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate(){
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn calculate_length(s:&String) ->usize{
    s.len()
}// here, s goes out of scope, but because it does not
// have ownership of what it refers to, nothing happends

fn gives_owenership() -> String{
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string:String) -> String{
    a_string
}
fn takes_ownership(some_string:String){
    println!("{}",some_string);
}

fn makes_copy(some_integer:i32){
    println!("{}",some_integer);
}

fn call_fun(mut num:u32){
    print!(" {} ",num);
    num = num+1;
    if num < 100 {
        call_fun(num);
    }

}

/*
Ownership rules
    . Each value in rust has a variable that's called its owner.
    . There can only be one owner at a time.
    . When the owner goes out of scope, the value will be dropped.






*/
