
pub mod console_input {

    use std::io::{stdin,stdout,Write};

    pub fn get_input() -> String {
        let mut s = String::new();
        print!("Please enter some text: ");
        let _=stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        if let Some('\n')=s.chars().next_back() {
            s.pop();
        }
        if let Some('\r')=s.chars().next_back() {
            s.pop();
        }
        return s
    }
}
