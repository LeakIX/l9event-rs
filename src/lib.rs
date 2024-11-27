use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const SEVERITY_CRITICAL: &str = "critical";
pub const SEVERITY_HIGH: &str = "high";
pub const SEVERITY_MEDIUM: &str = "medium";
pub const SEVERITY_LOW: &str = "low";
pub const SEVERITY_INFO: &str = "info";

pub const STAGE_OPEN: &str = "open";
pub const STAGE_EXPLORE: &str = "explore";
pub const STAGE_EXFILTRATE: &str = "exfiltrate";

#[derive(Debug, Serialize, Deserialize)]
pub struct L9Event {
    pub event_type: String,
    pub event_source: String,
    pub event_pipeline: Vec<String>,
    pub event_fingerprint: String,
    pub ip: String,
    pub host: String,
    pub reverse: String,
    pub port: String,
    pub mac: String,
    pub vendor: String,
    #[serde(rename = "transport")]
    pub transports: Vec<String>,
    pub protocol: String,
    pub http: L9HttpEvent,
    pub summary: String,
    pub time: DateTime<Utc>,
    pub ssl: L9SSLEvent,
    pub ssh: L9SSHEvent,
    pub service: L9ServiceEvent,
    pub leak: L9LeakEvent,
    pub tags: Vec<String>,
    pub geoip: GeoLocation,
    pub network: Network,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct L9HttpEvent {
    pub root: String,
    pub url: String,
    pub status: i32,
    pub length: i64,
    #[serde(rename = "header")]
    pub headers: HashMap<String, String>,
    pub title: String,
    pub favicon_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct L9ServiceEvent {
    pub credentials: ServiceCredentials,
    pub software: Software,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct L9SSHEvent {
    pub fingerprint: String,
    pub version: i32,
    pub banner: String,
    pub motd: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct L9LeakEvent {
    pub stage: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub severity: String,
    pub dataset: DatasetSummary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct L9SSLEvent {
    pub detected: bool,
    pub enabled: bool,
    pub jarm: String,
    pub cypher_suite: String,
    pub version: String,
    pub certificate: Certificate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatasetSummary {
    pub rows: i64,
    pub files: i64,
    pub size: i64,
    pub collections: i64,
    pub infected: bool,
    pub ransom_notes: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Software {
    pub name: String,
    pub version: String,
    pub operating_system: Option<String>,
    pub modules: Vec<SoftwareModule>,
    pub fingerprint: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoftwareModule {
    pub name: String,
    pub version: String,
    pub fingerprint: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceCredentials {
    pub noauth: bool,
    pub username: String,
    pub password: String,
    pub key: String,
    pub raw: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Certificate {
    pub cn: String,
    pub domain: Vec<String>,
    pub fingerprint: String,
    pub key_algo: String,
    pub key_size: i32,
    pub issuer_name: String,
    pub not_before: DateTime<Utc>,
    pub not_after: DateTime<Utc>,
    pub valid: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeoLocation {
    pub continent_name: Option<String>,
    pub region_iso_code: Option<String>,
    pub city_name: Option<String>,
    pub country_iso_code: Option<String>,
    pub country_name: Option<String>,
    pub region_name: Option<String>,
    pub location: Option<GeoPoint>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeoPoint {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Network {
    pub organization_name: String,
    pub asn: i32,
    pub network: String,
}
