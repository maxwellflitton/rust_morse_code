

pub fn check_letter(character: &str) -> i32 {

    match character {
        "a" => 1011,
        "b" => 11010101,
        "c" => 110101101,
        "d" => 110101,
        "e" => 1,
        "f" => 10101101,
        "g" => 1101101,
        "h" => 1010101,
        "i" => 101,
        "j" => 1011011011,
        "k" => 1101011,
        "l" => 10110101,
        "m" => 11011,
        "n" => 1101,
        "o" => 11011011,
        "p" => 101101101,
        "q" => 1101101011,
        "r" => 101101,
        "s" => 10101,
        "t" => 11,
        "u" => 101011,
        "v" => 10101011,
        "w" => 1011011,
        "x" => 110101011,
        "y" => 1101011011,
        "z" => 110110101,
        " " => 110011,
        _ => 11001100
    }
}
