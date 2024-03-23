
struct Error {
    message: String,
}
// 1. Main
pub fn read(data: &Vec<u8>) -> i32{
    // Datatype Signature
    const STRING: u8 = b'+';
    const ERROR: u8 = b'-';
    const INTEGER: u8 = b':';
    const BINARY: u8 = b'$';
    const ARRAY: u8 = b'*';
    const DICT: u8 = b'%';

    // Response Codes
    const OK: i32 = 200;
    const ERR: i32 = 400;

    if data.get(0).is_none() {
        return ERR;
    }

    let signature = data[0];
    let remaining_raw = remove_trailing_crlf(&data[1..]);
    let mut remaining = String::from_utf8_lossy(&remaining_raw).to_string();


    match signature {
        STRING => {
            let mut rstring = remaining;
            OK
        },
        ERROR => {
            let rerror = Error{message:remaining};
            OK
        },
        INTEGER => {
            let mut rnum: i32;
            match remaining.parse::<i32>(){
                Ok(num) => rnum=num,
                Err(_) => return ERR,
            }
            OK
        },
        _ => ERR,
        
    }
    


}

// 2. Helper

fn remove_trailing_crlf(data: &[u8]) -> Vec<u8> {
    let mut result = data.to_vec();
    if result.ends_with(b"\r\n") {
        result.truncate(result.len() - 2);
    }
    return result;
}