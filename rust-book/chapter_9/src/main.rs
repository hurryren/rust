use core::panic;

/*
Error handling
    Rust groups errors into tow major categories: recoverable and
unrecoverable errors. For a recoverble error, such as a file not
found error,it's reasonable to report the problem to the user and
retry the operation. Unrecoverable errors are always symptoms of bugs,
like trying to access a location beyond the end of an array.

*/


fn main() {

    {
        /* */
        println!();
        println!();
        println!("--------- unrecoverable errors with panic ---------");

    }




    {
        /* */
        println!();
        println!();
        println!("--------- Recoverable errors with result ---------");
        use std::fs::File;
        use std::io::ErrorKind;

        let f = File::open("hello.txt");

        let f = match f{
            Ok(file) => file,
            Err(error) =>  match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt"){
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}",e),
                },
                other_error => {
                    panic!("problem opening the file: {:?}", other_error);
                }
            },
        };

        let f  = File::open("hello.txt").unwrap_or_else(|error|{
            if error.kind() == ErrorKind::NotFound{
                File::create("hello.txt").unwrap_or_else(|error|{
                    panic!("problem creating the file: {:?}",error);
                })
            }else{
                panic!("problem opening the file: {:?}",error);
            }
        });

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
