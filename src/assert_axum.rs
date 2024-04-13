use super::accessor::{BodyAccessor, HeaderAccessor, StatusAccessor};

type AxumResponse = axum::response::Response;

impl StatusAccessor for AxumResponse {
    fn get_status(&self) -> u16 {
        self.status().as_u16()
    }
}

impl HeaderAccessor for AxumResponse {
    fn get_keys(&self) -> Vec<String> {
        self.headers().iter().map(|(k, _)| k.as_str().to_string()).collect::<Vec<_>>()
    }

    fn get_raw_values(&self, key: &str) -> Vec<String> {
        let value = self
            .headers()
            .get(key)
            .and_then(|v| v.to_str().ok())
            .map(str::to_string)
            .unwrap();
        vec![value]
    }
}

impl BodyAccessor for AxumResponse {
    fn get_bytes(&mut self) -> anyhow::Result<Vec<u8>> {
        let body = std::mem::take(self.body_mut());
        let bytes = futures_lite::future::block_on(axum::body::to_bytes(body, usize::MAX))?;
        *self.body_mut() = axum::body::Body::from(bytes.clone());
        Ok(bytes.to_vec())
    }
}
