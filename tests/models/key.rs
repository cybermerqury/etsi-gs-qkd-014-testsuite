// SPDX-FileCopyrightText: © 2023 Merqury Cybersecurity Ltd <info@merqury.eu>
// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct KeyContainer {
    #[serde(rename(serialize = "key_IDs", deserialize = "keys"))]
    pub keys: Vec<KeyContainerElement>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct KeyContainerElement {
    #[serde(rename = "key_ID")]
    pub key_id: uuid::Uuid,
    #[serde(skip_serializing)]
    pub key: Option<String>,
}
