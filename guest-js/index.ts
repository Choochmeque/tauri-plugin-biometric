// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

import { invoke } from '@tauri-apps/api/core'

export enum BiometryType {
  None = 0,
  // Apple TouchID or Android fingerprint
  TouchID = 1,
  // Apple FaceID or Android face authentication
  FaceID = 2,
  // Android iris authentication
  Iris = 3
}

export interface Status {
  isAvailable: boolean
  biometryType: BiometryType
  error?: string
  errorCode?:
    | 'appCancel'
    | 'authenticationFailed'
    | 'invalidContext'
    | 'notInteractive'
    | 'passcodeNotSet'
    | 'systemCancel'
    | 'userCancel'
    | 'userFallback'
    | 'biometryLockout'
    | 'biometryNotAvailable'
    | 'biometryNotEnrolled'
}

export interface AuthOptions {
  allowDeviceCredential?: boolean
  cancelTitle?: string

  // iOS options
  fallbackTitle?: string

  // android options
  title?: string
  subtitle?: string
  confirmationRequired?: boolean
}

export interface DataOptions {
  uid: string
  name: string
}

export interface DataResponse {
  uid: string
  name: string
  data: string
}

export interface GetDataOptions {
  uid: string
  name: string
  reason: string
}

export interface SetDataOptions {
  uid: string
  name: string
  data: string
}

export type RemoveDataOptions = DataOptions

/**
 * Checks if the biometric authentication is available.
 * @returns a promise resolving to an object containing all the information about the status of the biometry.
 */
export async function checkStatus(): Promise<Status> {
  return await invoke('plugin:biometric|status')
}

/**
 * Prompts the user for authentication using the system interface (touchID, faceID or Android Iris).
 * Rejects if the authentication fails.
 *
 * ```javascript
 * import { authenticate } from "@tauri-apps/plugin-biometric";
 * await authenticate('Open your wallet');
 * ```
 * @param reason
 * @param options
 * @returns
 */
export async function authenticate(
  reason: string,
  options: AuthOptions
): Promise<void> {
  await invoke('plugin:biometric|authenticate', {
    reason: reason,
    options: options
  })
}

/**
 * Checks if secure data exists for the given uid and name.
 * @param options The options containing uid and name to check
 * @returns A promise resolving to true if data exists, false otherwise
 */
export async function hasData(options: DataOptions): Promise<boolean> {
  return await invoke('plugin:biometric|has_data', { options })
}

/**
 * Gets secure data associated with biometric authentication.
 * @param options The options containing uid, name, and reason for authentication
 * @returns A promise resolving to the DataOptions containing the retrieved data
 */
export async function getData(options: GetDataOptions): Promise<DataResponse> {
  return await invoke('plugin:biometric|get_data', { options })
}

/**
 * Sets secure data that requires biometric authentication to access.
 * @param options The options containing uid, name, and data to store
 * @returns A promise that resolves when the data is stored
 */
export async function setData(options: SetDataOptions): Promise<void> {
  await invoke('plugin:biometric|set_data', { options })
}

/**
 * Removes secure data associated with biometric authentication.
 * @param options The options containing uid and name of the data to remove
 * @returns A promise that resolves when the data is removed
 */
export async function removeData(options: RemoveDataOptions): Promise<void> {
  await invoke('plugin:biometric|remove_data', { options })
}
