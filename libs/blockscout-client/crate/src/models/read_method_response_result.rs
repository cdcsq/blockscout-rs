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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReadMethodResponseResult {
    ExtendedRevertReasonAsMap(models::ExtendedRevertReasonAsMap),
    Error(models::Error),
    DecodedInput(models::DecodedInput),
    CodeAndMessage(models::CodeAndMessage),
    OutputAndNames(models::OutputAndNames),
}

impl Default for ReadMethodResponseResult {
    fn default() -> Self {
        Self::ExtendedRevertReasonAsMap(Default::default())
    }
}
