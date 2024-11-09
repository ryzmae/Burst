use anyhow::Result;
use base64;

pub trait Base64 {
    fn encode(&self, data: &[u8]) -> Result<String>;
    fn decode(&self, data: &str) -> Result<Vec<u8>>;
}

pub struct Base64Impl {}

impl Base64Impl {
    pub fn new() -> Self {
        Base64Impl {}
    }

    pub fn encode(&self, data: &[u8]) -> Result<String> {
        Ok(base64::encode(data))
    }

    pub fn decode(&self, data: &str) -> Result<Vec<u8>> {
        Ok(base64::decode(data)?)
    }
}

impl Base64 for Base64Impl {
    fn encode(&self, data: &[u8]) -> Result<String> {
        Ok(base64::encode(data))
    }

    fn decode(&self, data: &str) -> Result<Vec<u8>> {
        Ok(base64::decode(data)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_encode() {
        let base64 = Base64Impl::new();
        let data = b"hello world";
        let encoded = base64.encode(data).unwrap();
        assert_eq!(encoded, "aGVsbG8gd29ybGQ=");
    }

    #[test]
    fn test_base64_decode() {
        let base64 = Base64Impl::new();
        let data = base64::encode(b"hello world");
        let decoded = base64.decode(&data).unwrap();
        assert_eq!(decoded, b"hello world");
    }
}
