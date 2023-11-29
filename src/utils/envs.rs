use std::env;
use std::str::FromStr;

use super::b64::b64u_decode;

pub fn get_env(name: &'static str) -> Result<String> {
    dotenvy::dotenv().ok();
    env::var(name).map_err(|_| Error::MissingEnv(name))
}

#[allow(dead_code)]
pub fn get_env_parse<T: FromStr>(name: &'static str) -> Result<T> {
    let val = get_env(name)?;
    val.parse::<T>().map_err(|_| Error::WrongFormat(name))
}

#[allow(dead_code)]
pub fn get_env_b64u_as_u8s(name: &'static str) -> Result<Vec<u8>> {
    b64u_decode(&get_env(name)?).map_err(|_| Error::WrongFormat(name))
}

// region:    --- Error
pub type Result<T> = core::result::Result<T, Error>;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    MissingEnv(&'static str),
    WrongFormat(&'static str),
}

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate

// endregion: --- Error
