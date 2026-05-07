use std::cell::{RefCell};
use std::error::Error as StdError;
use std::rc::{Rc};


fn go_base64_encode(data: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    let mut i = 0;
    while i < data.len() {
        let b0 = data[i];
        let b1 = if i + 1 < data.len() { data[i + 1] } else { 0 };
        let b2 = if i + 2 < data.len() { data[i + 2] } else { 0 };

        out.push(TABLE[(b0 >> 2) as usize] as char);
        out.push(TABLE[(((b0 & 0x03) << 4) | (b1 >> 4)) as usize] as char);
        if i + 1 < data.len() {
            out.push(TABLE[(((b1 & 0x0f) << 2) | (b2 >> 6)) as usize] as char);
        } else {
            out.push('=');
        }
        if i + 2 < data.len() {
            out.push(TABLE[(b2 & 0x3f) as usize] as char);
        } else {
            out.push('=');
        }

        i += 3;
    }
    out
}

fn go_base64_value(byte: u8) -> Option<u8> {
    match byte {
        b'A'..=b'Z' => Some(byte - b'A'),
        b'a'..=b'z' => Some(byte - b'a' + 26),
        b'0'..=b'9' => Some(byte - b'0' + 52),
        b'+' => Some(62),
        b'/' => Some(63),
        _ => None,
    }
}

fn go_base64_decode(input: &str) -> Result<Vec<u8>, String> {
    let bytes = input.as_bytes();
    if bytes.len() % 4 != 0 {
        return Err(format!("illegal base64 data at input byte {}", bytes.len()));
    }

    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let c0 = go_base64_value(bytes[i])
            .ok_or_else(|| format!("illegal base64 data at input byte {}", i))?;
        let c1 = go_base64_value(bytes[i + 1])
            .ok_or_else(|| format!("illegal base64 data at input byte {}", i + 1))?;
        let pad2 = bytes[i + 2] == b'=';
        let pad3 = bytes[i + 3] == b'=';
        let c2 = if pad2 {
            0
        } else {
            go_base64_value(bytes[i + 2])
                .ok_or_else(|| format!("illegal base64 data at input byte {}", i + 2))?
        };
        let c3 = if pad3 {
            0
        } else {
            go_base64_value(bytes[i + 3])
                .ok_or_else(|| format!("illegal base64 data at input byte {}", i + 3))?
        };

        out.push((c0 << 2) | (c1 >> 4));
        if !pad2 {
            out.push((c1 << 4) | (c2 >> 2));
        }
        if !pad3 {
            out.push((c2 << 6) | c3);
        }

        i += 4;
    }
    Ok(out)
}

fn main() {
    let mut data = Rc::new(RefCell::new(Some("Hello, World!".to_string())));
    let mut encoded = Rc::new(RefCell::new(Some(go_base64_encode(&*(Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap()).as_bytes().to_vec())))).borrow().as_ref().unwrap()))));
    println!("{} {}", "Encoded:".to_string(), { let __v = (*encoded.borrow().as_ref().unwrap()).clone(); __v });

    let (mut decoded, _) = { match go_base64_decode(&*encoded.borrow().as_ref().unwrap()) { Ok(v) => (Rc::new(RefCell::new(Some(v))), Rc::new(RefCell::new(None))), Err(e) => (Rc::new(RefCell::new(Some(Vec::<u8>::new()))), Rc::new(RefCell::new(Some(Box::<dyn StdError>::from(e))))) } };
    println!("{} {}", "Decoded:".to_string(), (*Rc::new(RefCell::new(Some(String::from_utf8((*decoded.borrow().as_ref().unwrap()).clone()).unwrap()))).borrow().as_ref().unwrap()));
}