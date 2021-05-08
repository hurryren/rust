#![allow(dead_code)]
#![allow(overflowing_literals)] // 不显示类型转换溢出警告

pub fn main_value() {
    // immutable1();
    // scope1();
    // type_transfer();
    // type_of_value();
    // type_inference();
    from_trait();
}

fn immutable1() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);
    mutable_binding += 1;
    println!("after mutation: {}", mutable_binding);
}

fn scope1() {
    // 此绑定生存于 main 函数中
    let long_lived_binding = 1;

    // 这是一个代码块，比 main 函数拥有更小的作用域
    {
        // 此绑定只存在于本代码块
        let short_lived_binding = 2;
        println!("inner long: {}", long_lived_binding);

        println!("inner short: {}", short_lived_binding);

        // 此绑定*掩蔽*了外面的绑定
        let long_lived_binding = 5_f32;

        println!("inner long: {}", long_lived_binding);
    }
    // 代码块结束

    // 报错！`short_lived_binding` 在此作用域上不存在
    // println!("outer short: {}", short_lived_binding);
    // 改正 ^ 注释掉这行

    println!("outer long: {}", long_lived_binding);

    // 此绑定同样*掩蔽*了前面的绑定
    let long_lived_binding = 'a';

    println!("outer long: {}", long_lived_binding);
}

fn type_transfer() {
    let decimal = 65.4321_f32;

    // let integer:u8 = decimal; //  不提供意识转换

    let integer:u8 = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}",decimal,integer,character);

    // 当任何类型转换为无符号类型T 时， 会不断加上或者减去（std::T::MAX + 1）
    // 知道值位于新类型 T 的范围内

    // 1000 已经在 u16 的范围内
    println!("1000 as a u16 is: {}", 1000 as u16);

    println!("1000 as a u8 is : {}", 1000 as u8);

    // 转换到有符号类型时，（位操作的）结果就是 先转换到对应的无符号类型
    // 如果 MSB 是 1 ，则该值为负
    println!("128 as i8 is {}", 128 as i8);
}

fn type_of_value(){
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    let i = 1;
    let f = 1.0;

    // size_of_val 返回一个变量所占的字节数
    println!("size of 'x' in bytes: {}",std::mem::size_of_val(&x));
    println!("size of 'y' in bytes: {}",std::mem::size_of_val(&y));
    println!("size of 'z' in bytes: {}",std::mem::size_of_val(&z));
    println!("size of 'i' in bytes: {}",std::mem::size_of_val(&i));
    println!("size of 'f' in bytes: {}",std::mem::size_of_val(&f));
}

fn type_inference(){
    let elem = 5u8;

    // 创建一个空向量
    let mut vec = Vec::new();

    vec.push(elem);

    println!("{:?}",vec);
}

fn from_trait(){
    let my_str = "hello";
    let my_string = String::from(my_str);

    // 为自己的类型定义转换机制
    use std::convert::From;
    #[derive(Debug)]
    struct Number{
        value: i32,
    }

    impl From<i32> for Number {
        fn from(item:i32) -> Self{
            Number{value:item}
        }
    }

    let num = Number::from(30);
    println!("My number is {:?}",num);
}


