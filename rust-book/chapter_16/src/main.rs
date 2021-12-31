fn main() {
    {
        /* */
        println!();
        println!();
        println!("--------- creating a new thread with spawn ---------");
        use std::thread;
        use std::time::Duration;

        thread::spawn(||{
            for i in 1..10{
                println!("hi number {} for the spawned thread!",i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5{
            println!("hi number {} for the main thread!",i);
            thread::sleep(Duration::from_millis(1));
        }

    }

    {
        /* */
        println!();
        println!();
        println!("--------- waiting for all threads to finish using join handles ---------");
        use std::thread;
        use std::time::Duration;

        let handle = thread::spawn(||{
            for i in 1..10{
                println!("hi number {} for the spawned thread!",i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5{
            println!("hi number {} for the main thread!",i);
            thread::sleep(Duration::from_millis(1));
        }

        handle.join().unwrap();


    }

    {
        /* */
        println!();
        println!();
        println!("---------  ---------");

    }

    {
        /* */
        println!();
        println!();
        println!("---------  ---------");

    }


    {
        /* */
        println!();
        println!();
        println!("---------  ---------");

    }


    {
        /* */
        println!();
        println!();
        println!("---------  ---------");

    }


    {
        /* */
        println!();
        println!();
        println!("---------  ---------");

    }


    {
        /* */
        println!();
        println!();
        println!("---------  ---------");

    }


    {
        /* */
        println!();
        println!();
        println!("---------  ---------");

    }


    {
        /* */
        println!();
        println!();
        println!("---------  ---------");

    }


}
