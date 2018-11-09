// Copyright (c) Microsoft. All rights reserved.

/*
 * DeviceProvisioningService_DeviceRuntimeClient
 *
 * API for device runtime operations with the Azure IotHub Device Provisioning Service
 *
 * OpenAPI spec version: 2017-11-15
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use std::default::Default;

/// [`DeviceRegistration`] : Device registration.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceRegistration {
    #[serde(
        rename = "registrationId",
        skip_serializing_if = "Option::is_none"
    )]
    registration_id: Option<String>,
    #[serde(rename = "tpm", skip_serializing_if = "Option::is_none")]
    tpm: Option<TpmAttestation>,
}

impl DeviceRegistration {
    /// Device registration.
    pub fn new() -> Self {
        DeviceRegistration {
            registration_id: None,
            tpm: None,
        }
    }

    pub fn set_registration_id(&mut self, registration_id: String) {
        self.registration_id = Some(registration_id);
    }

    pub fn with_registration_id(mut self, registration_id: String) -> Self {
        self.registration_id = Some(registration_id);
        self
    }

    pub fn registration_id(&self) -> Option<&str> {
        self.registration_id.as_ref().map(AsRef::as_ref)
    }

    pub fn reset_registration_id(&mut self) {
        self.registration_id = None;
    }

    pub fn set_tpm(&mut self, tpm: TpmAttestation) {
        self.tpm = Some(tpm);
    }

    pub fn with_tpm(mut self, tpm: TpmAttestation) -> Self {
        self.tpm = Some(tpm);
        self
    }

    pub fn tpm(&self) -> Option<&TpmAttestation> {
        self.tpm.as_ref()
    }

    pub fn reset_tpm(&mut self) {
        self.tpm = None;
    }
}

impl Default for DeviceRegistration {
    fn default() -> Self {
        DeviceRegistration::new()
    }
}

/// [`TpmAttestation`] : Attestation via TPM.

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TpmAttestation {
    #[serde(rename = "endorsementKey")]
    endorsement_key: String,
    #[serde(
        rename = "storageRootKey",
        skip_serializing_if = "Option::is_none"
    )]
    storage_root_key: Option<String>,
}

impl TpmAttestation {
    /// Attestation via TPM.
    pub fn new(endorsement_key: String) -> Self {
        TpmAttestation {
            endorsement_key,
            storage_root_key: None,
        }
    }

    pub fn set_endorsement_key(&mut self, endorsement_key: String) {
        self.endorsement_key = endorsement_key;
    }

    pub fn with_endorsement_key(mut self, endorsement_key: String) -> Self {
        self.endorsement_key = endorsement_key;
        self
    }

    pub fn endorsement_key(&self) -> &String {
        &self.endorsement_key
    }

    pub fn set_storage_root_key(&mut self, storage_root_key: String) {
        self.storage_root_key = Some(storage_root_key);
    }

    pub fn with_storage_root_key(mut self, storage_root_key: String) -> Self {
        self.storage_root_key = Some(storage_root_key);
        self
    }

    pub fn storage_root_key(&self) -> Option<&str> {
        self.storage_root_key.as_ref().map(AsRef::as_ref)
    }

    pub fn reset_storage_root_key(&mut self) {
        self.storage_root_key = None;
    }
}

/// [`TpmRegistrationResult`] : TPM registration result.

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TpmRegistrationResult {
    #[serde(
        rename = "authenticationKey",
        skip_serializing_if = "Option::is_none"
    )]
    authentication_key: Option<String>,
}

impl TpmRegistrationResult {
    /// TPM registration result.
    pub fn new() -> Self {
        TpmRegistrationResult {
            authentication_key: None,
        }
    }

    pub fn set_authentication_key(&mut self, authentication_key: String) {
        self.authentication_key = Some(authentication_key);
    }

    pub fn with_authentication_key(mut self, authentication_key: String) -> Self {
        self.authentication_key = Some(authentication_key);
        self
    }

    pub fn authentication_key(&self) -> Option<&str> {
        self.authentication_key.as_ref().map(AsRef::as_ref)
    }

    pub fn reset_authentication_key(&mut self) {
        self.authentication_key = None;
    }
}

impl Default for TpmRegistrationResult {
    fn default() -> Self {
        TpmRegistrationResult::new()
    }
}

/// [`RegistrationOperationStatus`] : Registration operation status.

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegistrationOperationStatus {
    /// Operation ID.
    #[serde(rename = "operationId")]
    operation_id: String,
    /// Device enrollment status.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    status: Option<String>,
    /// Device registration status.
    #[serde(
        rename = "registrationState",
        skip_serializing_if = "Option::is_none"
    )]
    registration_state: Option<DeviceRegistrationResult>,
}

impl RegistrationOperationStatus {
    /// Registration operation status.
    pub fn new(operation_id: String) -> Self {
        RegistrationOperationStatus {
            operation_id,
            status: None,
            registration_state: None,
        }
    }

    pub fn set_operation_id(&mut self, operation_id: String) {
        self.operation_id = operation_id;
    }

    pub fn with_operation_id(mut self, operation_id: String) -> Self {
        self.operation_id = operation_id;
        self
    }

    pub fn operation_id(&self) -> &String {
        &self.operation_id
    }

    pub fn set_status(&mut self, status: String) {
        self.status = Some(status);
    }

    pub fn with_status(mut self, status: String) -> Self {
        self.status = Some(status);
        self
    }

    pub fn status(&self) -> Option<&str> {
        self.status.as_ref().map(AsRef::as_ref)
    }

    pub fn reset_status(&mut self) {
        self.status = None;
    }

    pub fn set_registration_state(&mut self, registration_state: DeviceRegistrationResult) {
        self.registration_state = Some(registration_state);
    }

    pub fn with_registration_state(mut self, registration_state: DeviceRegistrationResult) -> Self {
        self.registration_state = Some(registration_state);
        self
    }

    pub fn registration_state(&self) -> Option<&DeviceRegistrationResult> {
        self.registration_state.as_ref()
    }

    pub fn reset_registration_state(&mut self) {
        self.registration_state = None;
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceRegistrationResult {
    /// TPM registration result.
    #[serde(rename = "tpm", skip_serializing_if = "Option::is_none")]
    tpm: Option<TpmRegistrationResult>,
    /// X509 registration result.
    #[serde(skip_deserializing)]
    x509: Option<String>,
    /// Registration ID.
    #[serde(rename = "registrationId")]
    registration_id: String,
    /// Registration create date time (in UTC).
    #[serde(
        rename = "createdDateTimeUtc",
        skip_serializing_if = "Option::is_none"
    )]
    created_date_time_utc: Option<String>,
    /// Assigned Azure IoT Hub.
    #[serde(
        rename = "assignedHub",
        skip_serializing_if = "Option::is_none"
    )]
    assigned_hub: Option<String>,
    /// Device ID.
    #[serde(rename = "deviceId", skip_serializing_if = "Option::is_none")]
    device_id: Option<String>,
    /// Enrollment status.
    #[serde(rename = "status")]
    status: String,
    /// Error code.
    #[serde(
        rename = "errorCode",
        skip_serializing_if = "Option::is_none"
    )]
    error_code: Option<i32>,
    /// Error message.
    #[serde(
        rename = "errorMessage",
        skip_serializing_if = "Option::is_none"
    )]
    error_message: Option<String>,
    /// Last updated date time (in UTC).
    #[serde(
        rename = "lastUpdatedDateTimeUtc",
        skip_serializing_if = "Option::is_none"
    )]
    last_updated_date_time_utc: Option<String>,
    /// The entity tag associated with the resource.
    #[serde(rename = "etag", skip_serializing_if = "Option::is_none")]
    etag: Option<String>,
}

impl DeviceRegistrationResult {
    /// Device registration result.
    pub fn new(registration_id: String, status: String) -> Self {
        DeviceRegistrationResult {
            tpm: None,
            x509: None,
            registration_id,
            created_date_time_utc: None,
            assigned_hub: None,
            device_id: None,
            status,
            error_code: None,
            error_message: None,
            last_updated_date_time_utc: None,
            etag: None,
        }
    }

    pub fn set_tpm(&mut self, tpm: TpmRegistrationResult) {
        self.tpm = Some(tpm);
    }

    pub fn with_tpm(mut self, tpm: TpmRegistrationResult) -> Self {
        self.tpm = Some(tpm);
        self
    }

    pub fn tpm(&self) -> Option<&TpmRegistrationResult> {
        self.tpm.as_ref()
    }

    pub fn reset_tpm(&mut self) {
        self.tpm = None;
    }

    pub fn set_registration_id(&mut self, registration_id: String) {
        self.registration_id = registration_id;
    }

    pub fn with_registration_id(mut self, registration_id: String) -> Self {
        self.registration_id = registration_id;
        self
    }

    pub fn registration_id(&self) -> &String {
        &self.registration_id
    }

    pub fn set_created_date_time_utc(&mut self, created_date_time_utc: String) {
        self.created_date_time_utc = Some(created_date_time_utc);
    }

    pub fn with_created_date_time_utc(mut self, created_date_time_utc: String) -> Self {
        self.created_date_time_utc = Some(created_date_time_utc);
        self
    }

    pub fn created_date_time_utc(&self) -> Option<&str> {
        self.created_date_time_utc.as_ref().map(AsRef::as_ref)
    }

    pub fn reset_created_date_time_utc(&mut self) {
        self.created_date_time_utc = None;
    }

    pub fn set_assigned_hub(&mut self, assigned_hub: String) {
        self.assigned_hub = Some(assigned_hub);
    }

    pub fn with_assigned_hub(mut self, assigned_hub: String) -> Self {
        self.assigned_hub = Some(assigned_hub);
        self
    }

    pub fn assigned_hub(&self) -> Option<&str> {
        self.assigned_hub.as_ref().map(AsRef::as_ref)
    }

    pub fn reset_assigned_hub(&mut self) {
        self.assigned_hub = None;
    }

    pub fn set_device_id(&mut self, device_id: String) {
        self.device_id = Some(device_id);
    }

    pub fn with_device_id(mut self, device_id: String) -> Self {
        self.device_id = Some(device_id);
        self
    }

    pub fn device_id(&self) -> Option<&str> {
        self.device_id.as_ref().map(AsRef::as_ref)
    }

    pub fn reset_device_id(&mut self) {
        self.device_id = None;
    }

    pub fn set_status(&mut self, status: String) {
        self.status = status;
    }

    pub fn with_status(mut self, status: String) -> Self {
        self.status = status;
        self
    }

    pub fn status(&self) -> &String {
        &self.status
    }

    pub fn set_error_code(&mut self, error_code: i32) {
        self.error_code = Some(error_code);
    }

    pub fn with_error_code(mut self, error_code: i32) -> Self {
        self.error_code = Some(error_code);
        self
    }

    pub fn error_code(&self) -> Option<i32> {
        self.error_code
    }

    pub fn reset_error_code(&mut self) {
        self.error_code = None;
    }

    pub fn set_error_message(&mut self, error_message: String) {
        self.error_message = Some(error_message);
    }

    pub fn with_error_message(mut self, error_message: String) -> Self {
        self.error_message = Some(error_message);
        self
    }

    pub fn error_message(&self) -> Option<&str> {
        self.error_message.as_ref().map(AsRef::as_ref)
    }

    pub fn reset_error_message(&mut self) {
        self.error_message = None;
    }

    pub fn set_last_updated_date_time_utc(&mut self, last_updated_date_time_utc: String) {
        self.last_updated_date_time_utc = Some(last_updated_date_time_utc);
    }

    pub fn with_last_updated_date_time_utc(mut self, last_updated_date_time_utc: String) -> Self {
        self.last_updated_date_time_utc = Some(last_updated_date_time_utc);
        self
    }

    pub fn last_updated_date_time_utc(&self) -> Option<&str> {
        self.last_updated_date_time_utc.as_ref().map(AsRef::as_ref)
    }

    pub fn reset_last_updated_date_time_utc(&mut self) {
        self.last_updated_date_time_utc = None;
    }

    pub fn set_etag(&mut self, etag: String) {
        self.etag = Some(etag);
    }

    pub fn with_etag(mut self, etag: String) -> Self {
        self.etag = Some(etag);
        self
    }

    pub fn etag(&self) -> Option<&str> {
        self.etag.as_ref().map(AsRef::as_ref)
    }

    pub fn reset_etag(&mut self) {
        self.etag = None;
    }
}
