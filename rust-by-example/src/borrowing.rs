use std::boxed;

/*
    多数情况下，希望能够访问数据，同时不取得所有权。
    为实现这点，Rust使用了借用机制。对象可以通过引用（&T）来 传递，
    从而取代通过值传递。

    编译器通过借用检查静态的保证了引用总是指向有效的对象。
    也就是说，存在引用指向一个对象时，该对象不能被销毁。
*/

pub fn main_func(){
    // example_func();
    mutable_func();

}

fn example_func(){
    fn eat_box_i32(boxed_i32:Box<i32>){
        println!("Destroying box that contains {}",boxed_i32);
    }

    // 此函数借用了一个 i32 类型
    fn borrow_i32(borrowed_i32: &i32){
        println!("This int is : {}",borrowed_i32);
    }

    // 创建一个 box 的 i32 类型
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;


    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        // 取得一个对 box 中数据的引用
        let _ref_to_i32: &i32 = &boxed_i32;

        // eat_box_i32(boxed_i32);

        // 在 _ref_to_i32 里面的值被销毁后，尝试借用 _ref_to_i32
        borrow_i32(_ref_to_i32);
    }

    // boxed_i32 现在可以将所有全交给 eat_i32 并被销毁
    // 能够销毁时因为已经不存在对 boxed_i32 的引用
    eat_box_i32(boxed_i32);


}


fn mutable_func(){
    #[derive(Clone, Copy)]
    struct Book{
        // & static str 是一个对分配在只读内存区的字符串的引用
        author : &'static str,
        title: &'static str,
        year: u32,
    }

    // 此函数接受一个对 book类型的引用
    fn borrow_book(book: &Book){
        println!(" I immutably borrowed {} - {} edition", book.title, book.year);
    }

    // 此函数接受一个对可变 book 类型的引用， 他把年份 year 改为2014
    fn new_edition(book: &mut Book){
        book.year = 2014;
        println!("I mutabley borrowed {} - {} edition", book.title, book.year);
    }


    // 创建一个名为immutabook 的不可变 book示例
    let immutabook = Book{
        // 字符串字面量拥有 &‘static str 类型
        author: "Douglas Hofstadter",
        title: "Godel, echadj, skldjh",
        year: 1979,
    };

    // 创建一个 immutabook 的可变拷贝，命名为 mutabook
    let mut mutabook = immutabook;

    // 不可变的借用一个不可变对象
    borrow_book(&immutabook);

    // 不可变的借用一个可变对象
    borrow_book(&mutabook);

    // 可变的借用一个可变对象
    new_edition(&mut mutabook);

    // 不能可变的借用一个不可变对象
    // new_edition(&mut immutabook);

}
