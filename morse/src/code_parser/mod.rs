
pub mod writer {
    mod cipher;

    pub struct Word {
        content: String,
        compiled_message: Vec<i32>
    }

    impl Word {

        pub fn new(new_content: &str) -> Word {
            Word {content: String::from(new_content), compiled_message: Vec::new()}
        }

        pub fn compile(self) -> Vec<i32> {
            let mut buffer = Vec::new();
            // convert String into vector
            for chara in self.content.chars() {
                buffer.push(
                    cipher::check_letter(&chara.to_string())
                );
            }
            return buffer
        }
    }

}

pub mod reader {
    mod cipher;

    pub struct ReceivedMessage {
        content: Vec<String>,
    }

    impl ReceivedMessage {

        pub fn new(message_buffer: &Vec<i32>) -> ReceivedMessage {
            let mut buffer = Vec::new();

            for number in message_buffer {
                buffer.push(cipher::check_letter(&number))
            }
            ReceivedMessage {content: buffer}
        }

        pub fn display(self) {
            println!(":");
            for chara in self.content {
                print!("{}", chara);
            }
            println!(":");
        }

    }
}

