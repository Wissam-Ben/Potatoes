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

pub fn get_seed(random_hexa: String) -> String {
    let mut res = random_hexa;
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

pub fn count_leading_zeros(hashage: Digest) -> u32{
    let mut leading_zeros = 0;
    for octet in hashage.as_slice() { //pour découper

        leading_zeros = leading_zeros + octet.leading_zeros();
        if octet.leading_zeros() < 8 {
            println!("total leading zeros: {}", leading_zeros);
            return leading_zeros;
        }
    }

    println!("total leading zeros: {}", leading_zeros);
    return leading_zeros;
}

pub fn md5hashage(hash_cash_input: MD5HashCashInput) -> MD5HashCashOutput {

    let mut random_hexa = to_hexa(get_random());
    let mut seed = get_seed(random_hexa);
    let mut seed_final: u64 = get_random();

    let mut digest = md5::compute(seed.clone() + &hash_cash_input.message); //

    while count_leading_zeros(digest) < hash_cash_input.complexity  {
        if count_leading_zeros(digest) >= hash_cash_input.complexity {
            println!("ok");
        } else {
            println!("pas bon la seed");
            println!("---------------");
            seed_final = get_random();
            random_hexa = to_hexa(seed_final);
            seed = get_seed(random_hexa.clone());
            digest = md5::compute(seed.clone() + &hash_cash_input.message);
        }
    }

    let result_string: String = format!("{:X}", digest);
    println!("good seed: {}", seed);
    println!("hashage: {}", result_string);

    MD5HashCashOutput { seed: seed_final, hashcode: result_string.to_string()}
}