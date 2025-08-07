use tauri::{AppHandle, command, Runtime};

use crate::models::*;
use crate::{BiometricExt, Result};

#[command]
pub(crate) async fn status<R: Runtime>(
    app: AppHandle<R>,
) -> Result<Status> {
    app.biometric().status()
}

#[command]
pub(crate) async fn authenticate<R: Runtime>(
    reason: String, 
    options: AuthOptions,
    app: AppHandle<R>,
) -> Result<()> {
    app.biometric().authenticate(reason, options)
}

#[command]
pub(crate) async fn has_data<R: Runtime>(
    options: DataOptions,
    app: AppHandle<R>,
) -> Result<bool> {
    app.biometric().has_data(options)
}

#[command]
pub(crate) async fn get_data<R: Runtime>(
    options: GetDataOptions,
    app: AppHandle<R>,
) -> Result<DataOptions> {
    app.biometric().get_data(options)
}

#[command]
pub(crate) async fn set_data<R: Runtime>(
    options: SetDataOptions,
    app: AppHandle<R>,
) -> Result<()> {
    app.biometric().set_data(options)
}

#[command]
pub(crate) async fn remove_data<R: Runtime>(
    options: RemoveDataOptions,
    app: AppHandle<R>,
) -> Result<()> {
    app.biometric().remove_data(options)
}
