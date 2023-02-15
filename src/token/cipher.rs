use base64::{engine::general_purpose, Engine};
use libaes::Cipher;
use prost::Message;
use sha2::{Digest, Sha512};

pub struct TokenCipher {}

impl TokenCipher {
    pub fn encrypt<T: Message>(src: &T, key: &str) -> String {
        let mut prost_encoded = Vec::new();
        prost::Message::encode(src, &mut prost_encoded).expect("Failed to encode");

        let mut iv: [u8; 16] = [0; 16];
        iv.copy_from_slice(&prost_encoded[..16]);

        // calculate key
        let mut hasher = Sha512::new();
        hasher.update(key);
        let key_hash = hasher.finalize();
        let mut aes_key = [0; 24];
        aes_key.copy_from_slice(&key_hash[..24]);

        let cipher = Cipher::new_192(&aes_key);
        let encrypted = cipher.cbc_encrypt(&iv, &prost_encoded);
        let mut data: Vec<u8> = vec![0; iv.len() + encrypted.len()];
        data[..16].copy_from_slice(&iv);
        data[16..].copy_from_slice(&encrypted);

        let base64_encoded = &general_purpose::STANDARD.encode(data);

        base64_encoded.to_owned()
    }

    pub fn decrypt<T: Message + Default>(src: &str, key: &str) -> Result<T, String> {
        let decoded_token = &general_purpose::STANDARD.decode(src);

        if let Err(err) = decoded_token {
            return Err(format!("{}", err));
        }

        let decoded_token = decoded_token.as_ref().unwrap();
        let mut iv: [u8; 16] = [0; 16];
        iv.copy_from_slice(&decoded_token[..16]);

        let mut hasher = Sha512::new();
        hasher.update(key);
        let key_hash = hasher.finalize();
        let mut aes_key = [0; 24];
        aes_key.copy_from_slice(&key_hash[..24]);

        let cipher = Cipher::new_192(&aes_key);
        let decrypted = cipher.cbc_decrypt(&iv, &decoded_token[16..]);

        let result: Result<T, prost::DecodeError> = prost::Message::decode(&decrypted[..]);

        match result {
            Err(err) => return Err(format!("{}", err)),
            Ok(data) => return Ok(data),
        }
    }
}
