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

