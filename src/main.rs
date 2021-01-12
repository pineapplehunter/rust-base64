extern crate base64;

use base64::{decode, decode_string};
use base64::{encode, encode_string};

fn main() {
    let v = vec![1, 2, 3, 4, 5];

    println!("before           :{:?}", v);
    let encoded = encode(&v);
    println!("encoded          :{:X?}", encoded);
    println!(
        "encoded as string:{}",
        String::from_utf8(encoded.clone()).unwrap()
    );
    let decoded = decode(&encoded);
    println!("after   :{:?}", decoded);

    println!("-----------------------------");

    let s = "Hello, World!".into();

    println!("before  :{}", s);
    let encoded = encode_string(&s);
    println!("encoded :{:X?}", encoded);
    let decoded = decode_string(&encoded);
    println!("after   :{:?}", decoded);
}
