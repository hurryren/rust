
#[allow(dead_code)]
pub fn hello_main(){
    // println!("hello world");
    // hello_world();
    // fmt_print();
    // fmt_debug();
    // fmt_display();
    // fmt_display1();
    // fmt_list();
    fmt_format();


}
#[allow(dead_code)]
fn hello_world(){
    println!("hello world");
}

#[allow(dead_code)]
fn fmt_print(){
    println!("{} days",31);
    println!("{0}, this is {1}. {1}, this is {0}","alice", "bob");

    // 可以使用命名参数
    println!("{subject} {verb} {object}",
                object= 11,
                subject= "thge quick brown fox",
                verb= "jumps over");

    // 可以在 ":" 后面指定特殊的格式
    println!("{} of {:b} pepole kown binary, the other half don't",1,200);

    let pi = 3.1415926;
    println!(" pi is {:.10}",pi);

    #[allow(dead_code)]
    struct structure(i32);

    // println!("this struct '{:?}' won't print...",structure(3));
}

#[allow(dead_code)]
fn fmt_debug(){
    // 这个结构体不能使用 ‘fmt::Display' 或 ’fmt::Debug‘ 来进行打印
    struct UnPrintable(i32);

     // 'derive' 属性会自动创建所需的实现，使这个 ‘struct' 能使用 ’fmt::Debug‘ 打印
    #[derive(Debug)]
    struct DebugPrintable(i32);
    // println!("{:?}",UnPrintable(3));
    println!("{:#?}",DebugPrintable(3));

}

#[allow(dead_code)]
fn fmt_display(){
    // 使用 use 导入 fmt 模块使得 fmt::Display 可用
    use std::fmt;

    // 定义一个结构体，自己为它实现 fmt::Display
    struct Structure(u32);


    // 为了使用 {} 标记，必须手动为类型实现 ’fmt::display' trait

    impl fmt::Display for Structure {
        // 这个 trait 要求 fmt 使用与下面函数完全一致的函数签名
        fn fmt(&self,f:&mut fmt::Formatter) -> fmt::Result{
            // 仅将self的第一个元素写入到给定的输出流 ‘f'。返回 fmt:Result, 此
            // 结果表明操作成功或失败。
            // 注意 write  的用法和println! 相似
            write!(f,"{}",self.0)
        }
    }

    println!("{}",Structure(4));
}

#[allow(dead_code)]
fn fmt_display1(){
    use std::fmt;  // 导入 fmt

    // 带有两个数字的结构体。 推导出 debug 以便与display 的输出进行比较
    #[derive(Debug)]
    struct MinMax(i64,i64);

    // 实现 MinMax 的 Display
    impl fmt::Display for MinMax {
        fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result{
            // 使用 self.number 来表示各个数据
            write!(f,"({}, {})", self.0, self.1)
        }
    }

    // 为了比较， 定义一个含有具体字段的结构体
    #[derive(Debug)]
    struct Point2D{
        x:f64,
        y:f64,
    }

    // 实现 Display
    impl fmt::Display for Point2D{
        fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result{
            write!(f, "x: {}, y: {}",self.x,self.y)
        }
    }

    let minmax = MinMax(0,14);
    println!("compare structures:");
    println!("Display: {}",minmax);
    println!("Debug: {:?}",minmax);

    let big_range = MinMax(-300,300);
    let small_range = MinMax(-3,-3);

    println!("The big range is  {big} and the small is {small}",
                small= small_range,
                big=big_range);

    let point = Point2D {x:3.3,y:7.2};
    println!("Compare points:");
    println!("Display: {}",point);
    println!("Debug: {:?}",point);

    // 报错、  debug 和 display 都被实现了，但是 {:b} 需要fmt::Binary
    // 得到实现
    // println!("What does Point2D look like in binary: {:b}?",point);

}

#[allow(dead_code)]
fn fmt_list(){
    use std::fmt;

    struct List(Vec<i32>);

    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
            // 使用元组的下标获取值，并创建一个 vec 的引用
            let vec = &self.0;
            write!(f, "[")?;

            // 使用 v 对 vec 进行迭代， 并用count 记录迭代次数
            for(count, v ) in vec.iter().enumerate(){
                // 对每个元素加上逗号。
                // 使用 ？ 或者try! 返回错误
                if count != 0 {write!(f,", ")?;}
                write!(f,"{}",v)?;
            }

            // 加上配对中括号，并返回一个 fmt:Result 值
            write!(f, "]")
        }
    }

    let v = List(vec![1,2,3]);
    println!("{}",v);
}

#[allow(dead_code)]
fn fmt_format(){
    use std::fmt::{self, Formatter, Display};

    struct City{
        name: &'static str,
        lat:f32,
        lon:f32,
    }

    impl Display for City{
        // f 是一个缓冲区 (buffer), 此方法必须将格式化后的字符串写入其中
        fn fmt(&self, f:&mut Formatter) -> fmt::Result{
            let lat_c = if self.lat >= 0.0 {'N'} else {'S'};
            let lon_c = if self.lon >= 0.0 {'E'} else {'W'};

            // write! 和 format! 类似，但是会将格式化后的字符串写入缓冲区（即参数f中）
            write!(f,"{}: {:.3}°{} {:.3}°{}",self.name, self.lat.abs(),lat_c,self.lon.abs(),lon_c)
        }
    }
    #[derive(Debug)]
    struct Color{
        red:u8,
        green:u8,
        blue:u8,
    }

    impl Display for Color{
        // f 是一个缓冲区 (buffer), 此方法必须将格式化后的字符串写入其中
        fn fmt(&self, f:&mut Formatter) -> fmt::Result{
            let red:u32 = self.red.into();
            let green:u32 = self.green.into();
            let blue:u32 = self.blue.into();
            let sum:u32 = red << 16 |  green<< 8 | blue;
            write!(f,"RGB ({},{},{}) {:06X}",self.red, self.green,self.blue,sum)
        }
    }

    for city in [
        City {name:"dublin",lat:53.347778,lon:-6.259722},
        City {name:"Oslo",lat:59.95,lon:10.75},
        City {name:"vancouver",lat:49.25,lon:-123.1},
    ].iter() {
        println!("{}",*city);
    }

    for color in [
        Color{red:128,green:255,blue:90},
        Color{red:0,green:3,blue:254},
        Color{red:0,green:0,blue:90},
    ].iter(){
        // println!("{:?}",*color);
        println!("{}",*color);
    }
}

