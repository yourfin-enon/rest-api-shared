use libaes::Cipher;
use sha2::{Digest, Sha512};

pub struct AesCipher {}

impl AesCipher {
    pub fn encrypt(src: &Vec<u8>, key: &str) -> Vec<u8> {
        let mut hasher = Sha512::new();
        hasher.update(key);
        let key_hash = hasher.finalize();
        let mut aes_key = [0; 24];
        aes_key.copy_from_slice(&key_hash[..24]);

        let mut iv = vec![0u8; 16];
        iv.copy_from_slice(&src[..16]);

        let cipher = Cipher::new_192(&aes_key);
        let mut encrypted = cipher.cbc_encrypt(&iv, &src);

        let mut data: Vec<u8> = Vec::with_capacity(iv.len() + encrypted.len());
        data.append(&mut iv);
        data.append(&mut encrypted);

        data
    }

    pub fn decrypt(src: &Vec<u8>, key: &str) -> Vec<u8> {
        let mut hasher = Sha512::new();
        hasher.update(key);
        let key_hash = hasher.finalize();
        let mut aes_key = [0; 24];
        aes_key.copy_from_slice(&key_hash[..24]);

        let mut iv = vec![0u8; 16];
        iv.copy_from_slice(&src[..16]);

        let cipher = Cipher::new_192(&aes_key);
        let decrypted = cipher.cbc_decrypt(&iv, &src[16..]);

        decrypted
    }
}
