// SPDX-FileCopyrightText: © 2023 Merqury Cybersecurity Ltd <info@merqury.eu>
// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

use serde::Deserialize;

#[derive(Deserialize)]
pub struct ErrorMessage {
    #[serde(rename = "message")]
    _message: String,
    #[serde(rename = "details")]
    _details: Option<Vec<String>>,
}
