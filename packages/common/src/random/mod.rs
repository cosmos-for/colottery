pub mod rule;

mod pcg64;

pub mod seed;

use base64ct::{Base64, Encoding};

use crate::error::CommonError;

use self::pcg64::Pcg64;

pub fn pcg64_from_seed(seed: &str) -> Result<Pcg64, CommonError> {
    match Base64::decode_vec(seed) {
        Ok(bytes_vec) => {
            let mut bytes = [0u8; 32];
            bytes.copy_from_slice(bytes_vec.as_slice());
            Ok(Pcg64::from_seed(bytes))
        }
        Err(_err) => Err(CommonError::InvalidSeed {
            seed: seed.to_owned(),
        }),
    }
}
