fn main() {
    println!("Hello, world!");
    let  x:u32 = 0;
    let y = x;
    println!("{} {}",x,y);

    let s1 = String::from("hello");
    let s2 = s1;
    println!("{} [{}",s1,s2);
    // call_fun(x);
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
