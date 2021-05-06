use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Err");

    println!("{}", string_to_base64(buffer));
}

fn string_to_base64(string: String) -> String {
    let alphabet: [char; 64] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
        'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
        'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
        'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f',
        'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
        'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
        'w', 'x', 'y', 'z', '0', '1', '2', '3',
        '4', '5', '6', '7', '8', '9', '+', '/',
    ];

    let bytes = string.as_bytes();

    let mut prev: u8 = 0b0000_0000;
    let mut index;
    let mut offset;
    let mut counter = 0;
    let mut result = String::new();

    for (i, byte) in bytes.iter().enumerate() {
        counter = i % 3;
        offset = (counter + 1) * 2;
        index = ((byte >> offset - 2) | prev) >> 2;
        prev = byte << (8 - offset);

        result.push(alphabet[index as usize]);

        if counter == 2 {
            result.push(alphabet[(prev >> 2) as usize]);

            prev = 0b0000_0000;
        }
    }

    if prev != 0b0000_0000 {
        result.push(alphabet[(prev >> 2) as usize]);
    }

    for _i in 0..(2 - counter) {
        result.push('=');
    }

    result
}