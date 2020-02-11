

pub mod file_handler {

    use std::fs::File;
    use std::io::{Write, BufReader, BufRead, Error};

    pub fn write(buffer: Vec<i32>) -> Result<()> {
        let mut out_put = File::create("example.txt")?;
        for letter in buffer {
            // make a struct that's a string with a constructor
            write!(out_put, )
        }
        Ok(())
    }

}
