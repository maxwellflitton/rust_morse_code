

pub mod file_handler {

    use std::fs;
    use std::io::Error;

    pub fn write(compiled_message: String) -> Result<(), Error> {
        let new_data: &str = compiled_message.as_str();
        fs::write("message.txt", new_data).expect("Unable to write file");
        Ok(())
    }

}

#[cfg(test)]
mod file_handler_tests {

    use super::file_handler;

    #[test]
    fn write() {
        // need to look into mocks
        assert_eq!(1, 1);
    }

}
