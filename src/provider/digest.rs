use hmac::{
    Hmac, Mac,
    digest::{Digest, InvalidLength},
};
use sha2::Sha256;

pub(super) fn hmac_sha256(
    key: impl AsRef<[u8]>,
    data: impl AsRef<[u8]>,
) -> Result<[u8; 32], InvalidLength> {
    let data = Hmac::<Sha256>::new_from_slice(key.as_ref())?
        .chain_update(data.as_ref())
        .finalize()
        .into_bytes()
        .into();
    Ok(data)
}
#[inline(always)]
pub(super) fn hex_sha256(body: impl AsRef<[u8]>) -> String {
    hex::encode(Sha256::digest(body))
}
