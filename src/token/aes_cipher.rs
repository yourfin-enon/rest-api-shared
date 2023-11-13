use libaes::Cipher;
use sha2::{Digest, Sha512};

pub struct AesCipher {}

impl AesCipher {
    pub fn encrypt(src: &[u8], key: &str) -> Vec<u8> {
        let mut hasher = Sha512::new();
        hasher.update(key);
        let key_hash = hasher.finalize();
        let mut aes_key = [0; 24];
        aes_key.copy_from_slice(&key_hash[..24]);

        let mut iv = vec![0u8; 16];
        iv.copy_from_slice(&src[..16]);

        let cipher = Cipher::new_192(&aes_key);
        let mut encrypted = cipher.cbc_encrypt(&iv, src);

        let mut data: Vec<u8> = Vec::with_capacity(iv.len() + encrypted.len());
        data.append(&mut iv);
        data.append(&mut encrypted);

        data
    }

    pub fn decrypt(src: &[u8], key: &str) -> Result<Vec<u8>, String> {
        let mut hasher = Sha512::new();
        hasher.update(key);
        let key_hash = hasher.finalize();
        const KEY_LEN: usize = 24;

        if key_hash.len() < KEY_LEN {
            return Err(format!("Key hash len can't be less than {}", KEY_LEN));
        }

        let mut aes_key = [0; KEY_LEN];
        aes_key.copy_from_slice(&key_hash[..KEY_LEN]);

        const IV_LEN: usize = 16;

        if src.len() < IV_LEN {
            return Err(format!("Src array len can't be less than {}", IV_LEN));
        }

        let mut iv = vec![0u8; IV_LEN];
        iv.copy_from_slice(&src[..IV_LEN]);

        let cipher = Cipher::new_192(&aes_key);
        let decrypted = cipher.cbc_decrypt(&iv, &src[16..]);

        Ok(decrypted)
    }
}
