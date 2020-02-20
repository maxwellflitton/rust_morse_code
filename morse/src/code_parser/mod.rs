
pub mod writer {
    mod cipher;

    #[derive(Clone, Debug, PartialEq)]
    pub struct Word {
        pub content: String,
        pub compiled_message: Vec<i32>
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

        pub fn contained_compile(&mut self) {
            let mut buffer = Vec::new();

            for chara in self.content.chars() {
                buffer.push(
                    cipher::check_letter(&chara.to_string())
                );
            }
            self.compiled_message = buffer;
        }

        pub fn display(self) {
            for i in self.compiled_message {
                print!("=>{}", i);
            }
            println!(":");
        }
    }

}

pub mod reader {
    mod cipher;

    pub struct ReceivedMessage {
        pub content: Vec<String>,
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

#[cfg(test)]
mod writer_tests {

    use super::writer;

    #[test]
    fn simple_word_construct() {
        let test_word = writer::Word::new("test");
        assert_eq!("test", test_word.content);
        let test_vec: Vec<i32> = Vec::new();
        assert_eq!(test_vec, test_word.compiled_message);
    }

    #[test]
    fn word_compile() {
        let test_word = writer::Word::new("abc");
        let test_buffer = test_word.compile();
        assert_eq!(1011, test_buffer[0]);
        assert_eq!(11010101, test_buffer[1]);
        assert_eq!(110101101, test_buffer[2]);

        let compare_buffer = vec![1011, 11010101, 110101101];
        assert_eq!(compare_buffer, test_buffer);
    }

    #[test]
    fn word_contained_compile() {
        let mut test_word = writer::Word::new("abc");
        test_word.contained_compile();

        let compare_buffer = vec![1011, 11010101, 110101101];
        assert_eq!(compare_buffer, test_word.compiled_message);
    }
}

#[cfg(test)]
mod reader_tests {

    use super::reader;

    #[test]
    fn received_message_construct() {
        let incoming_message = vec![1011, 11010101, 110101101];
        let test_received_message = reader::ReceivedMessage::new(&incoming_message);
        let expected_message = vec!["a", "b", "c"];
        assert_eq!(expected_message, test_received_message.content);
    }
}
