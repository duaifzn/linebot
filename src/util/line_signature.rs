use hmac::{Hmac, Mac};
use sha2::Sha256;
use base64::encode;

pub fn create_signature(secret: String, body: String) ->String{
    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .expect("HMAC can take key of any size");
    mac.update(body.as_bytes());

    let result = mac.finalize();
    let code_bytes = result.into_bytes();
    encode(code_bytes)
}