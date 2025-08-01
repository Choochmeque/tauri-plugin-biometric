use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<Biometric<R>> {
  Ok(Biometric(app.clone()))
}

/// Access to the biometric APIs.
pub struct Biometric<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Biometric<R> {
    pub fn status(&self) -> crate::Result<Status> {
        Err(crate::Error::from(std::io::Error::new(std::io::ErrorKind::Other, "Biometric is not supported on desktop platforms")))
    }

    pub fn authenticate(&self, reason: String, options: AuthOptions) -> crate::Result<()> {
        Err(crate::Error::from(std::io::Error::new(std::io::ErrorKind::Other, "Biometric is not supported on desktop platforms")))
    }
}