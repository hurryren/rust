pub fn run(){
    string();
    pointer();
}

fn string(){
    let data: String = "hello".into();

    let s1: &str = &data;
    let s2: &str = &data;
    let s3: &str = &data;

    dbg!(&s1 as *const _);
    dbg!(&s2 as *const _);
    dbg!(&s3 as *const _);

    dbg!(s1.as_bytes() as *const _);
    dbg!(s2.as_bytes() as *const _);
    dbg!(s3.as_bytes() as *const _);
}

static MAX: u32 = 0;

fn foo() {}

fn pointer(){
    let hello = "hello world".to_string();
    let data = Box::new(1);

    // string literals 指向 RODATA 地址
    println!("RODATA: {:p}", "hello world!");
    // static 变量在 DATA section
    println!("DATA (static var): {:p}", &MAX);
    // function 在 TEXT
    println!("TEXT (function): {:p}", foo as *const ());
    // String 结构体分配在栈上，所以其引用指向一个栈地址
    println!("STACK (&hello): {:p}", &hello);
    // 需要通过解引用获取其堆上数据，然后取其引用
    println!("HEAP (&*hello): {:p}", &*hello);
    // Box 实现了 Pointer trait 无需额外解引用
    println!("HEAP (box impl Pointer) {:p} {:p}", data, &*data);
}