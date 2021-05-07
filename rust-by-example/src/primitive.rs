#[allow(dead_code)]
pub fn main_primitive(){
    // compound_type();
    // operator1();
    tuple1();

}

#[allow(dead_code)]
fn compound_type(){
    let logical:bool = true;

    let a_float:f64 = 1.0;
    let an_integer = 5i32;

    let default_float = 3.0;
    let default_integer = 7;

    let mut inferred_type:i64 = 12;
    inferred_type = 19999999999;
    println!("{}",inferred_type);
}

#[allow(dead_code)]
fn operator1(){
    println!("1 + 2 = {}",1u32 + 2);

    println!("1 - 2 = {}",1i32 - 2);

    // 布尔运算
    println!("true AND false is {}",true &&  false );
    println!("true OR false is {}",true ||  false );
    println!("NOT true is {}",!true );

    // 位运算
    println!("0011 and 0101 is {:04b}", 0b00111u32 & 0b0101);
    println!("0011 or 0101 is {:04b}", 0b00111u32 | 0b0101);
    println!("0011 xor 0101 is {:04b}", 0b00111u32 ^ 0b0101);
    println!("1 << 5 is {}",1u32 << 5);
    println!("0x80 >> 2 is 0x {:x}", 0x80u32 >> 2);
}


#[allow(dead_code)]
fn tuple1(){
    fn reverse(pair: (i32, bool)) -> (bool,i32){
        let (integer, boolean) = pair;
        (boolean,integer)
    }

    #[derive(Debug)]
    struct Matrix(f32,f32,f32,f32);

    let long_tuple = (1u8,2u16,3u32,4u64,
        -1i8,-2i8,-3i8,-4i8,
        0.1f32,0.2f64,
    'a',true);

    // 通过元组的下标访问具体的值
    println!("long tuple first value: {}",long_tuple.0);
    println!("long tuple second value: {}",long_tuple.1);

    // 元组也可以充当元组的元素
    let tuple_of_tuples = ((1u8,2u16,2u32),(4u64,-1i8), -2i16);
    println!("tuple of tuples: {:?}",tuple_of_tuples);
    println!("tuple of tuples 0.1: {:?}",tuple_of_tuples.0.1);

    // 很长的元组无法打印
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);

    let pair = (1, true);
    println!("pair is: {:?}",pair);
    println!("the reversed pair is: {:?}",reverse(pair));

    // 创建单元素元组需要一个额外的逗号，这是为了和被括号包括的字面量区分
    println!(" one element tuple: {:?}",(5u32,));
    println!(" just an integer: {:?}",(5u32));

    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);

}