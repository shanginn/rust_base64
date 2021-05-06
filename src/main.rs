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
    let bytes_count = bytes.len();
    let full_chunks_count = bytes_count / 3 * 3;
    let left_bytes_count = bytes_count - full_chunks_count;

    let mut result = String::new();

    for chunk in bytes[..full_chunks_count].chunks(3) {
        let index1 = chunk[0] >> 2;
        let index2 = (chunk[0] << 6 | chunk[1] >> 2) >> 2;
        let index3 = (chunk[1] << 4 | chunk[2] >> 4) >> 2;
        let index4 = chunk[2] & 0b0011_1111;

        result.push(alphabet[index1 as usize]);
        result.push(alphabet[index2 as usize]);
        result.push(alphabet[index3 as usize]);
        result.push(alphabet[index4 as usize]);
    }

    if left_bytes_count > 0 {
        let left_bytes = &bytes[full_chunks_count..];
        let index1 = left_bytes[0] >> 2;
        result.push(alphabet[index1 as usize]);

        if left_bytes_count == 1 {
            let index2 = (left_bytes[0] << 6) >> 2;

            result.push(alphabet[index2 as usize]);
            result.push('=');
        }

        if left_bytes_count == 2 {
            let index2 = (left_bytes[0] << 6 | left_bytes[1] >> 2) >> 2;
            let index3 = (left_bytes[1] << 4) >> 2;

            result.push(alphabet[index2 as usize]);
            result.push(alphabet[index3 as usize]);
        }

        result.push('=');
    }

    result
}