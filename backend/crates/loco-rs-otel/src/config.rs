use serde::Deserialize;
use std::collections::HashMap;

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
    #[allow(clippy::upper_case_acronyms)]
    HTTP(OtelTransportHttp),
}

#[derive(Deserialize)]
pub(crate) struct OtelTransportHttp {
    pub(crate) url: String,
    #[serde(default)]
    pub(crate) headers: HashMap<String, String>,
}
