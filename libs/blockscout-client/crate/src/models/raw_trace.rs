/*
 * BlockScout API
 *
 * API for BlockScout web app
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: you@your-company.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RawTrace {
    #[serde(rename = "action")]
    pub action: models::RawTraceAction,
    #[serde(rename = "subtraces")]
    pub subtraces: i32,
    #[serde(rename = "traceAddress")]
    pub trace_address: Vec<i32>,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<models::RawTraceResult>,
}

impl RawTrace {
    pub fn new(
        action: models::RawTraceAction,
        subtraces: i32,
        trace_address: Vec<i32>,
        r#type: String,
    ) -> RawTrace {
        RawTrace {
            action,
            subtraces,
            trace_address,
            r#type,
            error: None,
            result: None,
        }
    }
}
