/*

*/

pub fn main_func(){
    // example_func();
    part_move();

}

fn example_func(){
    // 此函数取得堆分配的内存的所有权
    fn destroy_box(c: Box<i32>){
        println!("Destroying a box that contains {}",c);

        // C 被销毁且内存得到释放
    }

    // 栈分配的整型
    let x = 5u32;

    // 将 x 复制到 y -- 不存在资源移动
    let y = x;

    println!("x is {}, y is {}",x,y);

    //  a 是一个指向堆分配到恶整数的指针
    let a =Box::new(5i32);

    println!("a contains: {}",a);

    // 移动 a 到 b
    let b = a;
    /*
        把 a 的指针地址复制到 b. 现在来给你这都指向
        同一个堆分配的数据。但是现在 b 拥有它
    */
    // 此函数从 b 中取得堆分配的内存的所有权
    destroy_box(b);

    // 此时堆内存已经被释放，这个操作会导致解引用已释放的内存，这是编译器禁止的。
    // println!("b contains: {}",b);

}

fn part_move(){
    #[derive(Debug)]
    struct Person{
        name :String,
        age: u8,
    }

    let person = Person{
        name: String::from("Alice"),
        age: 20,
    };

    // name 从person中移走，但 age 只是引用
    let Person{name, ref age} = person;

    println!("The person's age is {}",age);
    println!("The person's name is {}",name);

    // 报错
    // println!("the person struct is {:?}",person);

    // person 不能用，但是 person.age 因为没有被移动而可以继续使用
    println!("The person's age from person struct is {}",person.age);
}