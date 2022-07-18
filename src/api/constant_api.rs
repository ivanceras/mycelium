//! fetch metadata constant values
use crate::{api::Api, Error};

impl Api {
    pub fn fetch_constant_opaque_value(
        &self,
        module: &str,
        constant_name: &str,
    ) -> Result<Vec<u8>, Error> {
        Ok(self
            .metadata()
            .pallet(module)?
            .constant(constant_name)?
            .value
            .clone())
    }
}
