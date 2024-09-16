use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use hex;
use rand::Rng;
use std::str;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

/// 将hex字符串转换为字节数组
fn hex_to_bytes(hex_str: &str) -> Vec<u8> {
    hex::decode(hex_str).expect("Decoding failed")
}

/// 将字节数组转换为hex字符串
fn bytes_to_hex(bytes: &[u8]) -> String {
    hex::encode(bytes).to_uppercase()
}

/// AES加密函数
fn encrypt(plain_text: &str, key_seed: Option<&str>, charset: &str) -> Option<String> {
    let key = generate_key(key_seed);
    let iv = rand::thread_rng().gen::<[u8; 16]>(); // 生成随机IV

    let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();
    let encrypted_bytes = cipher.encrypt_vec(plain_text.as_bytes());

    // 返回16进制格式
    let mut result = bytes_to_hex(&iv);
    result.push_str(&bytes_to_hex(&encrypted_bytes));
    Some(result)
}

/// AES解密函数
fn decrypt(encrypted_text: &str, key_seed: Option<&str>, charset: &str) -> Option<String> {
    let key = generate_key(key_seed);
    let (iv_hex, encrypted_hex) = encrypted_text.split_at(32); // 前32个hex字符是IV
    let iv = hex_to_bytes(iv_hex);
    let encrypted_bytes = hex_to_bytes(encrypted_hex);

    let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();
    let decrypted_bytes = cipher.decrypt_vec(&encrypted_bytes).unwrap();

    // 将字节数组转换回字符串
    Some(String::from_utf8(decrypted_bytes).unwrap())
}

/// 根据种子生成密钥
fn generate_key(key_seed: Option<&str>) -> Vec<u8> {
    let default_seed = "qazwsxedcrfv1234567890*#"; // 默认的种子
    let seed = key_seed.unwrap_or(default_seed);
    let mut key = vec![0u8; 16];
    let seed_bytes = seed.as_bytes();
    key[..seed_bytes.len().min(16)].copy_from_slice(&seed_bytes[..seed_bytes.len().min(16)]);
    key
}
#[derive(Debug)]
enum User {
    App(i64)
}
fn main() {
    let key_seed: Option<i64> = Some(41);
    let user = User::App(1);
    
    match user {
        User::App(i) => println!("{}", i),
    }

    let a = key_seed.map(User::App);
    println!("{:?}", a);
}
// field in this variant
//
// /*
//
//  field `0` is never read
//   --> src/main.rs:59:9
//    |
// 59 |     App(i64)
//    |     --- ^^^
//    |     |
//    |     field in this variant
//    |
//    = note: `User` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
// help: consider changing the field to be of unit type to suppress this warning while preserving the field numbering, or remove the field
// */