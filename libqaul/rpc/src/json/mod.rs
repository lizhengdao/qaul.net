//! Json enveloping module
//!
//! This code is mostly used in `libqaul-ws` and `libqaul-http`, and
//! wraps the transactions of libqaul-rpc in json envelopes that are
//! nicer to work with for web tools.

mod generator;
mod parser;

use libqaul::{users::UserAuth, Identity};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::BTreeMap;

pub type JsonMap = BTreeMap<String, JsonValue>;

/// A struct wrapper for UserAuth
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JsonAuth {
    id: Identity,
    token: String,
}

impl From<JsonAuth> for UserAuth {
    fn from(ja: JsonAuth) -> Self {
        Self(ja.id, ja.token)
    }
}

/// A json specific request envelope
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RequestEnv {
    /// The request ID
    pub id: String,
    /// Auth data for the request
    pub auth: Option<JsonAuth>,
    /// An optional page selector
    pub page: Option<String>,
    /// Operation method
    pub method: String,
    /// Request scope
    pub kind: String,
    /// The rest of the nested data
    ///
    /// We keep this as a map because we need to inject the auth
    /// information into it later on, because the API expects it to be
    /// in eath RPC struct, while the json interface likes it in the
    /// envelope
    #[serde(default)]
    pub data: JsonMap,
}

/// Just print an example request envelope for reference.  You can see
/// this by running `cargo test json::print_sample_req -- --nocapture`
#[test]
fn print_sample_req() {
    use libqaul::{helpers::Tag, messages::MsgQuery};
    println!(
        "{}",
        serde_json::to_string_pretty(&RequestEnv {
            id: "1".into(),
            auth: Some(JsonAuth {
                id: Identity::random(),
                token: Identity::random().to_string()
            }),
            page: Some("1".into()),
            method: "query".into(),
            kind: "messages".into(),
            data: {
                let mut map = JsonMap::new();
                map.insert("service".into(), JsonValue::String("net.qaul.chat".into()));
                map.insert(
                    "query".into(),
                    serde_json::to_value(
                        MsgQuery::new()
                            .tag(Tag::new("room-id", Identity::random().as_bytes().to_vec())),
                    )
                    .unwrap(),
                );

                map
            },
        })
        .unwrap()
    );
}

/// A json specific repsonse envelope
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResponseEnv {
    /// Response ID, same as request ID
    pub id: String,
    /// Mirrored auth token
    #[serde(skip)]
    pub auth: Option<JsonAuth>,
    /// Request method
    pub method: String,
    /// Request scope
    pub kind: String,
    /// Optional object count
    #[serde(skip)]
    pub total: Option<usize>,
    /// Optional pagination info
    #[serde(skip)]
    pub next: Option<String>,
    /// Response data
    pub data: JsonMap,
}
