use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub(crate) struct RootSettings {
    pub(crate) otel: Otel,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub(crate) enum Otel {
    Enabled(OtelConfig),
    Disabled,
}

#[derive(Deserialize)]
pub(crate) struct OtelConfig {
    pub(crate) common: OtelCommon,
}

#[derive(Deserialize)]
pub(crate) struct OtelCommon {
    pub(crate) transport: OtelTransport,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub(crate) enum OtelTransport {
    HTTP(OtelTransportHttp),
}

#[derive(Deserialize)]
pub(crate) struct OtelTransportHttp {
    pub(crate) url: String,
    #[serde(default)]
    pub(crate) headers: HashMap<String, String>,
}
