## Keysafer

A small rust project to manage password and mnemonics
Encrypted Passwords are stored in src/encrypted_tokens/password.txt
Encrypted Mnemonics are stored in src/encrypted_tokens/XXX.txt

## Author

ZJY

## APIS

1. Get secret  
   GET `/keysafer/secret`
1. Get account passwords  
   GET `/keysafer/password`
1. Generate encrypted password by plaintext password  
   GET `/keysafer/password/encrypt`
1. Recover plaintext password by encrypted password(Only when the local file is lost then we need to recover from encrypted password(stored in git))  
   GET `/keysafer/password/recover`
1. Generate encrypted wallet mnemonics by plaintext mnemonics(When creating a new wallet and adding mnenomics)  
   GET `/keysafer/mnemonic/encrypt/`
1. Recover plaintext wallet mnemonics by encrypted mnemonics(Only when the local file is lost then we need to recover from encrypted mnemonics(stored in git))  
   GET `/keysafer/mnemonic/recover`
1. Get the mnemonics  
   GET `/keysafer/mnemonic/get/{id}`
1. Get cipher by plaintext  
   GET `/keysafer/encrypt`

## Tips:

1. Everytime onboarding a new mnemonic, do the following  
   a. Put plainetext mnemonic tokens into a new file in `src/tokens/(XXX.txt)`  
   b. Add the name into constant::MNEMONIC_LIST and update the length  
   c. Call `/keysafer/mnemonic/encrypt/` to backup  
   d. Push to Github for remote backup

2. Everytime onboarding a new password, do the following  
   a. Add a new line in `src/tokens/password.txt`  
   b. Call `/keysafer/password/encrypt` to backup  
   c. Push to Github for remote backup

3. When pulling from Github, do the following to recover all tokens  
   a. Create a new folder `src/tokens`  
   b. Add the secret into `src/tokens/secret.txt`  
   c. Call `/keysafer/password/recover` to recover all passwords  
   d. Call `/keysafer/mnemonic/recover` to recover all mnemonics

## How to run

```
cargo build
cargo run
```
