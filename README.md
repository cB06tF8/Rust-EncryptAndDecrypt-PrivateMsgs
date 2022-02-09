# Rust-EncryptAndDecrypt-PrivateMsgs
Rust console app that uses pem files to encrypt/decrypt private messages

## Dependencies
Dependent on the split_keyvals crate: 

https://github.com/cB06tF8/split_keyvals

## Requirements
You will need to generate RSA pem files for the participants. The users will need each other's public pems for encrypting and their 
own private key and passphrase to decrypt. More information can be found in various places online, such as these:

https://www.webdevsplanet.com/post/how-to-generate-rsa-private-and-public-keys

https://vesicae.wnpctv.org/generate-rsa-private-key-in-pem-format/

## Other Information
This app was built as a learning exercise. As such, it is not very robust but gets the job done. When run without any arguments, it 
should display a 'help' menu (shown below) for how to use the tool.

mode=|keygen|encode|decode|
passphrase=|secret passphrase|
message=|text to encode|hash to decode|
key_file=|recipient's public key for encrypting|recipient private key for decrypting|
### Examples:
1. keygen example: mode=keygen passphrase="My secret passphrase"
2. encode example: mode=encode message="Hello world" keyfile=alice_pubKey.pem
3. decode example: mode=decode keyfile=bob.privKey.pem passphrase= "My secret passphrase" message=hEcvkNCLoc7SGsHleuylWjssi+pt9I6URV8aH1jyTaTMey9bte3lI2LDrD946ANx1OcKwrmku3ef90+ARiJRU3lvnyK5d4xvJ0vO7WNz0vIxbiTejtUEj1NVwaPzugxxQ/jAqekDvXTKDJyKaLhfbRTJhYS5PeXs4gwafBNu/+Z4SObcxfQnXB8ITR12cX+CDek+fDxWzqDg+AJLBeCET2PKZMUQvqGtFmQ/VZqoI9pWSlgN3TP5Zje1IMGYN0DK0kKxs8sDckIuxuG/Lk3clXSgEuwpwohgX42VrAAzzzQrMpkn1zj7G/o/hpmQGDL59u41A6nXKdrWEhEhMdlNxg==
