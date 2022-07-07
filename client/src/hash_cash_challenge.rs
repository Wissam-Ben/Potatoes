extern crate core;

use std::char::from_digit;
use std::ops::Sub;
use md5::Digest;
use rand::Rng;
use shared::*;

pub fn get_random() -> u64 {
    let mut rng = rand::thread_rng();
    let n1: u64 = rng.gen();
    n1
}

pub fn to_hexa(value: u64) -> String {
    let mut res : String = format!("{:02X}", value);
    res
}

pub fn get_seed(randomHexa: String) -> String {
    let mut res = randomHexa;
    let mut zeros = "0".to_string();
    let mut count = 0;
    while res.len() + count < 15 {
        zeros.push_str("0");
        count = count + 1;
    }
    let mut seed = zeros + &res;
    println!("{}", seed);
    seed
}

pub fn countLeadingZeros(hashage:Digest) -> u32{
    let mut leadingZeros = 0;
    for octet in hashage.as_slice() { //pour découper

        leadingZeros = leadingZeros + octet.leading_zeros();
        if octet.leading_zeros() < 8 {
            println!("total leading zeros: {}", leadingZeros);
            return leadingZeros;
        }
    }

    println!("total leading zeros: {}", leadingZeros);
    return leadingZeros;
}

pub fn md5hashage(hashcashInput: MD5HashCashInput) -> MD5HashCashOutput {

    let mut random_hexa = to_hexa(get_random());
    let mut seed = get_seed(random_hexa);
    let mut seedf : u64 = get_random();

    let mut digest = md5::compute(seed.clone() + &hashcashInput.message); //

    while countLeadingZeros(digest) < hashcashInput.complexity  {
        if countLeadingZeros(digest) >= hashcashInput.complexity {
            println!("ok");
        } else {
            println!("pas bon la seed");
            println!("---------------");
            seedf = get_random();
            random_hexa = to_hexa(seedf);
            seed = get_seed(random_hexa.clone());
            digest = md5::compute(seed.clone() + &hashcashInput.message);
        }
    }

    let result_string: String = format!("{:X}", digest);
    println!("good seed: {}", seed);
    println!("hashage: {}", result_string);

    MD5HashCashOutput { seed: seedf, hashcode: result_string.to_string()}
}