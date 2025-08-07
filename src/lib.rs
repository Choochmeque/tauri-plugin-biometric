// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Biometric;
#[cfg(mobile)]
use mobile::Biometric;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`], [`tauri::WebviewWindow`], [`tauri::Webview`] and [`tauri::Window`] to access the biometric APIs.
pub trait BiometricExt<R: Runtime> {
    fn biometric(&self) -> &Biometric<R>;
}

impl<R: Runtime, T: Manager<R>> crate::BiometricExt<R> for T {
    fn biometric(&self) -> &Biometric<R> {
        self.state::<Biometric<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("biometric")
        .invoke_handler(tauri::generate_handler![
            commands::status,
            commands::authenticate,
            commands::has_data,
            commands::get_data,
            commands::set_data,
            commands::remove_data,
        ])
        .setup(|app, api| {
            #[cfg(mobile)]
            let biometric = mobile::init(app, api)?;
            #[cfg(desktop)]
            let biometric = desktop::init(app, api)?;
            app.manage(biometric);
            Ok(())
        })
        .build()
}
