// SPDX-License-Identifier: MIT
extern crate openssl;
extern crate base64;
extern crate split_keyvals;

use openssl::rsa::{Rsa, Padding};
use openssl::symm::Cipher;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;
use std::collections::HashMap;

fn main() {

    // IMP: use a std::iter::Map struct for this instead of a generic <T, T> struct

    //let env_key = "RUST_BACKTRACE";
    //env::set_var(env_key, "0"); // use '1' for short version (not full)

    // IMPORTANT: argument nth(0) is self: target\debug\rsa2_encode.exe and is ignored
    let arguments: Vec<String> = env::args().collect();
    // prepare argument for consumption
    
    let mut temp_args: Vec<split_keyvals::KeyVal<String, String>> = Vec::new(); 
    for a in &arguments {
        let curr_kvp = split_keyvals::split_str_str(String::from(a), '=');
        match curr_kvp {
            Ok(x) => temp_args.push(x),        
            Err(e) => (), // ignoring - error handling in logic below
        }       
    }
    
    let mut args2 = HashMap::new(); 
    for a in &temp_args {
        args2.insert(&a.key, &a.val);
    }
    println!("args2.len: {}", args2.len());
    match args2.get(&"mode".to_string()) {
        Some(md) => println!("mode={}", &md),
        _ => println!("no mode found"),
    }
        
    // display help
    if arguments.len() == 1 || arguments[1] == String::from("help") {
        println!("Command Line Tool '{}' possible arguments:\n    mode=|keygen|encode|decode|", arguments[0]);
        println!("    passphrase=|secret passphrase|");
        println!("    message=|text to encode|hash to decode|");
        println!("    key_file=|recipient's public key for encrypting|recipient private key for decrypting|");
        println!("Examples:");
        let pass_phrase = r#""My secret passphrase""#;        
        println!("1. keygen example: mode=keygen passphrase={}", &pass_phrase);        
        let msg = r#""Hello world""#; 
        println!("2. encode example: mode=encode message={} keyfile=alice_pubKey.pem", msg);
        let hash = "hEcvkNCLoc7SGsHleuylWjssi+pt9I6URV8aH1jyTaTMey9bte3lI2LDrD946ANx1OcKwrmku3ef90+ARiJRU3lvnyK5d4xvJ0vO7WNz0vIxbiTejtUEj1NVwaPzugxxQ/jAqekDvXTKDJyKaLhfbRTJhYS5PeXs4gwafBNu/+Z4SObcxfQnXB8ITR12cX+CDek+fDxWzqDg+AJLBeCET2PKZMUQvqGtFmQ/VZqoI9pWSlgN3TP5Zje1IMGYN0DK0kKxs8sDckIuxuG/Lk3clXSgEuwpwohgX42VrAAzzzQrMpkn1zj7G/o/hpmQGDL59u41A6nXKdrWEhEhMdlNxg==";
        println!("3. decode example: mode=decode keyfile=bob.privKey.pem passphrase= {} message={}", pass_phrase, hash);        
        std::process::exit(-100);
    }

    // consume incoming args
    let mut mode = "";
    let mut message = "";
    let mut key_file = "";
    let mut pass_phrase = "";
    for a in &temp_args {        
        if &a.key == "keyfile" { 
            key_file = &a.val;            
        } else if &a.key == "message" {
            message = &a.val;
        } else if &a.key == "passphrase" {
            pass_phrase = &a.val;
        } else if &a.key == "mode" {
            mode = &a.val;
        }
    }
    //println!("\n\nmode: {}, keyfile: {}, message: {}, passphrase: {}", mode, key_file, message, pass_phrase);

    // 3 modes for using this tool exist: keygen, encode, decode
    if mode  == "keygen" {
        if pass_phrase.is_empty() {
            println!("passphrase is empty. cannot run keygen.");
            std::process::exit(-100);
        }
        println!("beginning key generation...\n");
    
        let rsa = Rsa::generate(2048).unwrap(); // 1024 is default
        let private_key: Vec<u8> = rsa.private_key_to_pem_passphrase(Cipher::aes_128_cbc(), pass_phrase.as_bytes()).unwrap();
        let public_key: Vec<u8> = rsa.public_key_to_pem().unwrap();
    
        /* Open a file in write-only mode, (destroys existing file if present) 
         * returns `io::Result<File>` */    
         let path = Path::new("privKey.pem");
         let display = path.display();
         let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };       
        // Write the `private key` string to `file`, returns `io::Result<()>`
        match file.write_all(&private_key) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }
        
        //  Write the `public key` string to `file`, returns `io::Result<()>`
        let path = Path::new("pubKey.pem");
        let display = path.display();
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };
        // Write the `public key` string to `file`, returns `io::Result<()>`
        match file.write_all(&public_key) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }        
    } else if mode == "encode" {
        if message.is_empty() {
            println!("\nmessage value is empty. cannot encode.");
            std::process::exit(-100);
        } else if key_file.is_empty() {
            println!("\nkeyfile value is empty. cannot encode.");
            std::process::exit(-100);
        }
        println!("\nbegin encode...\n");
        // read in contents of pem file
        let mut encrypt_key_file = File::open(&key_file).expect("Can't open the file.");
        let mut contents = String::new();
        encrypt_key_file.read_to_string(&mut contents).expect("Unable to read file.");
        let rsa = Rsa::public_key_from_pem(contents.as_bytes()).unwrap();
        let mut buf: Vec<u8> = vec![0; rsa.size() as usize];
        // Encrypt with public key
        let _ = rsa.public_encrypt(message.as_bytes(), &mut buf, Padding::PKCS1).unwrap();
        // to base 64:
        println!("encrypted messsage:\n{}", base64::encode(&buf));
    } else if mode == "decode" {
        if message.is_empty() {
            println!("\nmessage value is empty. cannot decode.");
            std::process::exit(-100);
        } else if key_file.is_empty() {
            println!("\nkeyfile value is empty. cannot decode.");
            std::process::exit(-100);
        } else if pass_phrase.is_empty() {
            println!("\npassphrase value is empty. cannot decode.");
            std::process::exit(-100);
        }        
        println!("\nbegin decode...\n");
        
        // prepare hash for decryption
        let hash_bytes = base64::decode(&message).unwrap();
        // read in contents of pem file
        let mut decrypt_key_file = File::open(&key_file).expect("Can't open the file."); 
        let mut buf2 = Vec::new();
        decrypt_key_file.read_to_end(&mut buf2);
        // Decrypt with private key        
        let rsa = Rsa::private_key_from_pem_passphrase(&buf2, &pass_phrase.as_bytes()).unwrap();
        let mut buf: Vec<u8> = vec![0; rsa.size() as usize];
        let _ = rsa.private_decrypt(&hash_bytes, &mut buf, Padding::PKCS1).unwrap();
        println!("decrypted message:\n{}", String::from_utf8(buf).unwrap());
    } else {
        println!("action not recognized: {}", arguments[1]);
        std::process::exit(-100);
        //error handler
    }
}
