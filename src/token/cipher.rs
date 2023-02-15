use base64::{engine::general_purpose, Engine};
use libaes::Cipher;
use prost::Message;
use sha2::{Digest, Sha512};

pub struct TokenCipher {}

impl TokenCipher {
    pub fn encrypt<T: Message>(src: &T, key: &str) -> String {
        let mut prost_encoded = Vec::new();
        prost::Message::encode(src, &mut prost_encoded).expect("Failed to encode");

        let mut hasher = Sha512::new();
        hasher.update(key);
        let key_hash = hasher.finalize();
        let mut aes_key = [0; 24];
        aes_key.copy_from_slice(&key_hash[..24]);

        let mut iv = vec![0u8; 16];
        iv.copy_from_slice(&prost_encoded[..16]);

        let cipher = Cipher::new_192(&aes_key);
        let mut encrypted = cipher.cbc_encrypt(&iv, &prost_encoded);

        let mut data: Vec<u8> = Vec::with_capacity(iv.len() + encrypted.len());
        data.append(&mut iv);
        data.append(&mut encrypted);

        let base64_encoded = &general_purpose::STANDARD.encode(data);

        base64_encoded.to_owned()
    }

    pub fn decrypt<T: Message + Default>(src: &str, key: &str) -> Result<T, String> {
        let base64_decoded = &general_purpose::STANDARD.decode(src);

        if let Err(err) = base64_decoded {
            return Err(format!("{}", err));
        }

        let base64_decoded = base64_decoded.as_ref().unwrap();

        let mut hasher = Sha512::new();
        hasher.update(key);
        let key_hash = hasher.finalize();
        let mut aes_key = [0; 24];
        aes_key.copy_from_slice(&key_hash[..24]);

        let mut iv = vec![0u8; 16];
        iv.copy_from_slice(&base64_decoded[..16]);

        let cipher = Cipher::new_192(&aes_key);
        let decrypted = cipher.cbc_decrypt(&iv, &base64_decoded[16..]);

        let prost_decoded: Result<T, prost::DecodeError> = prost::Message::decode(&decrypted[..]);

        match prost_decoded {
            Err(err) => return Err(format!("{}", err)),
            Ok(data) => return Ok(data),
        }
    }
}
