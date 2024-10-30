use std::error::Error;
use std::fmt::Display;

pub trait AnyhowExt<T> {
    fn map_err_string(self) -> Result<T, String>;
}

impl<T> AnyhowExt<T> for anyhow::Result<T> {
    fn map_err_string(self) -> Result<T, String> {
        self.map_err(|e| format!("{e}"))
    }
}

pub trait ResultExt<T> {
    fn map_err_string(self) -> Result<T, String>;
}

impl<T, E> ResultExt<T> for Result<T, E>
where
    E: Error + Display,
{
    fn map_err_string(self) -> Result<T, String> {
        self.map_err(|e| format!("{e}"))
    }
}
