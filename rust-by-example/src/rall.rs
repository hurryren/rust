/*
    rust 的变量不只是在栈中保存书： 它们也占用资源，比如 Box<T> 占用堆
    中的内存。Rust强制实行RALL(Resource Acquisition initialization,
    资源获取即初始化)， 所以任何对象在离开作用域时， 它的析构函数就被调用，
    然后它占用的资源就被释放。

    这中行为避免了资源泄露，不用手动释放内存或者代行内存泄漏
*/

/*
    rust 中的析构函数是通过Frop trait 提供的。当资源离开作用域，就调用析构函数。
    无需为每种类型都实现FDrop trait，只要为需要自己的析构函数逻辑的类型实现即可。
*/

pub fn main_func(){
    // example_func();
    destroy_func();

}

fn example_func(){
    fn create_box(){
        // 在堆上分配一个整型数据
        let _box1 = Box::new(3i32);

        // _box1 在这里被销毁，内存被释放
    }

    let _box2 = Box::new(5i32);

    // 嵌套作用域
    {
        // 在堆上分配一个整型数据
        let _box3 = Box::new(4i32);

        // _box3 在这里被销毁

    }
    for _ in 0u32..1_000{
        create_box();
    }
    println!("end...");

    // box2 在这里被销毁，内存被释放
}

fn destroy_func(){
    struct ToDrop;

    impl Drop for ToDrop {
        fn drop(&mut self){
            println!("ToDrop is being droped");
        }
    }

    let x = ToDrop;
    println!("Made a Todrop");
}