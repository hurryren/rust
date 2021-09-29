



fn main() {
    {
        /*
            1. Dereference a raw pointer
            2. Call an unsafe function or method
            3. Access or modify a mutable static variable
            4. Implement an unsafe trait
            5. Access fields of Unions

        It's important to understand that unsafe doesn't turn off the borrow
        checker or disable any other of Rust's safety checks: if you use a
        reference in unsafe code, it will still be checked. The unsafe keyword
        only gives you access to these five features that then noe checked by
        compiler fo memory safety.
        */
        println!();
        println!();
        println!("--------- unsafe superpowers ---------");

    }

    {
        /*
        raw pointers:
            1. Are allowed to ignore the borrowing rules by having both immutable
            and mutable pointers or multiple mutable pointers to the same location
            2. Aren't guaranteed to point valid memory
            3. Are allowed to be null
            4. Don't implement any automatic cleanup

        */
        println!();
        println!();
        println!("--------- Dereferencing a raw pointer ---------");
        let mut num = 5;
        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;
        // Note that don't include the unsafe keyword in this code. We can create
        // raw pointers in safe code; we just can't dereference raw pointers outside
        // an unsafe block

        unsafe {
            println!("r1 is: {}",*r1);
            println!("r2 is: {}",*r2);
        }

    }

    {
        /* */
        println!();
        println!();
        println!("--------- calling an unsafe function or method ---------");
        unsafe fn dangerous(){};

        unsafe {
            dangerous();
        }
        // we must call the dagerous function within a separate unsafe block,
        // if we try to call dangerous without the unsafe block, we'll get an error

    }


    {
        /* */
        println!();
        println!();
        println!("--------- creating a safe abstration over unsafe code ---------");
        let mut v = vec![1,2,3,4,5,6];
        let r = &mut v[..];
        let (a,b) = r.split_at_mut(3);

        assert_eq!(a,&mut [1,2,3]);
        assert_eq!(b,&mut [4,5,6]);

        {
            use std::slice;
            fn split_at_mut_1(slice: &mut [i32], mid:usize) -> (&mut [i32], &mut [i32]){
                let len = slice.len();
                let ptr = slice.as_mut_ptr();

                assert!(mid<=len);

                unsafe{(
                    slice::from_raw_parts_mut(ptr,mid),
                    slice::from_raw_parts_mut(ptr.add(mid),len-mid),
                )}
            }
        }

    }


    {
        /* */
        println!();
        println!();
        println!("--------- using extern function to call external code ---------");

        extern "C"{
            fn abs(input:i32)->i32;
        }

        unsafe{
            println!("Absolute value of -3 according to c: {}",abs(-3));
        }

    }


    {
        /* */
        println!();
        println!();
        println!("--------- calling rust fucntions from other languages ---------");
        #[no_mangle]
        pub extern "C" fn call_from_c(){
            println!("Just called a rust function from c!");
        }

    }

    {
        /*
        static variables are similar to constants. Static variables can only store
        references with the 'static lifetime, which means the rust compiler can figure
        out the lifetime and we aren't required to annotate it explicitly. Accessing
        an immutable static variable is safe.
        */
        println!();
        println!();
        println!("--------- accessing or modifying a mutable static variable ---------");

        static mut COUNTER:u32=0;

        fn add_to_count(inc:u32){
            unsafe {
                COUNTER += inc;
            }
        }

        add_to_count(3);
        unsafe{
            println!("COUNTER: {}",COUNTER);
        }


    }


    {
        /* */
        println!();
        println!();
        println!("---------  ---------");

    }



}
