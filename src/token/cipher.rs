use crate::token::aes_cipher::AesCipher;
use base64::{engine::general_purpose, Engine};
use prost::Message;

pub struct TokenCipher {}

impl TokenCipher {
    pub fn encrypt_with_iv<T: Message>(src: &T, key: &str, iv: &[u8]) -> String {
        let mut prost_encoded = Vec::new();
        Message::encode(src, &mut prost_encoded).expect("Failed to encode");
        let data = AesCipher::encrypt_with_iv(&prost_encoded, key, iv);
        let base64_encoded = &general_purpose::STANDARD.encode(data);

        base64_encoded.to_owned()
    }

    pub fn decrypt_with_iv<T: Message + Default>(
        src: &str,
        key: &str,
        iv: &[u8],
    ) -> Result<T, String> {
        let base64_decoded = &general_purpose::STANDARD.decode(src);

        if let Err(err) = base64_decoded {
            return Err(format!("{}", err));
        }

        let base64_decoded = base64_decoded.as_ref().unwrap();
        let decrypted = AesCipher::decrypt_with_iv(base64_decoded, key, iv);

        let Ok(decrypted) = decrypted else {
            return Err(decrypted.unwrap_err());
        };

        let prost_decoded: Result<T, prost::DecodeError> = Message::decode(&decrypted[..]);

        match prost_decoded {
            Err(err) => Err(format!("{}", err)),
            Ok(data) => Ok(data),
        }
    }

    pub fn encrypt<T: Message>(src: &T, key: &str) -> String {
        let mut prost_encoded = Vec::new();
        Message::encode(src, &mut prost_encoded).expect("Failed to encode");
        let data = AesCipher::encrypt(&prost_encoded, key);
        let base64_encoded = &general_purpose::STANDARD.encode(data);

        base64_encoded.to_owned()
    }

    pub fn decrypt<T: Message + Default>(src: &str, key: &str) -> Result<T, String> {
        let base64_decoded = &general_purpose::STANDARD.decode(src);

        if let Err(err) = base64_decoded {
            return Err(format!("{}", err));
        }

        let base64_decoded = base64_decoded.as_ref().unwrap();
        let decrypted = AesCipher::decrypt(base64_decoded, key);

        let Ok(decrypted) = decrypted else {
            return Err(decrypted.unwrap_err());
        };

        let prost_decoded: Result<T, prost::DecodeError> = Message::decode(&decrypted[..]);

        match prost_decoded {
            Err(err) => Err(format!("{}", err)),
            Ok(data) => Ok(data),
        }
    }
}
