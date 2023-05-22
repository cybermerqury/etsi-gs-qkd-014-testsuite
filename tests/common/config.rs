// SPDX-FileCopyrightText: © 2023 Merqury Cybersecurity Ltd <info@merqury.eu>
// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

use std::env;

static ENV_BASE_SERVER_URL: &str = "ETSI_014_TEST_SUITE_BASE_SERVER_URL";
static ENV_BASE_CLIENT_URL: &str = "ETSI_014_TEST_SUITE_BASE_CLIENT_URL";
static ENV_TLS_ROOT_CRT: &str = "ETSI_014_TEST_SUITE_TLS_ROOT_CRT";
static ENV_MASTER_SAE_ID: &str = "ETSI_014_TEST_SUITE_MASTER_SAE_ID";
static ENV_TLS_MASTER_SAE_CERT: &str =
    "ETSI_014_TEST_SUITE_TLS_MASTER_SAE_CERT";
static ENV_TLS_SLAVE_SAE_CERT: &str = "ETSI_014_TEST_SUITE_TLS_SLAVE_SAE_CERT";
static ENV_SLAVE_SAE_ID: &str = "ETSI_014_TEST_SUITE_SLAVE_SAE_ID";
static ENV_TLS_ADD_SLAVE_SAE_CERT: &str =
    "ETSI_014_TEST_SUITE_TLS_ADD_SLAVE_SAE_CERT";
static ENV_ADD_SLAVE_SAE_ID: &str = "ETSI_014_TEST_SUITE_ADD_SLAVE_SAE_ID";

pub struct Config {
    pub base_server_url: String,
    pub base_client_url: String,
    pub root_crt: String,
    pub master_sae_id: String,
    pub master_sae_crt: String,
    pub slave_sae_id: String,
    pub slave_sae_crt: String,
    pub add_slave_sae_id: String,
    pub add_slave_sae_crt: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            base_server_url: Self::extract_string_value(ENV_BASE_SERVER_URL),
            base_client_url: Self::extract_string_value(ENV_BASE_CLIENT_URL),
            root_crt: Self::extract_string_value(ENV_TLS_ROOT_CRT),
            master_sae_id: Self::extract_string_value(ENV_MASTER_SAE_ID),
            master_sae_crt: Self::extract_string_value(ENV_TLS_MASTER_SAE_CERT),
            slave_sae_id: Self::extract_string_value(ENV_SLAVE_SAE_ID),
            slave_sae_crt: Self::extract_string_value(ENV_TLS_SLAVE_SAE_CERT),
            add_slave_sae_id: Self::extract_string_value(ENV_ADD_SLAVE_SAE_ID),
            add_slave_sae_crt: Self::extract_string_value(
                ENV_TLS_ADD_SLAVE_SAE_CERT,
            ),
        }
    }

    fn extract_string_value(var_name: &str) -> String {
        match env::var(var_name) {
            Ok(val) => val,
            Err(e) => {
                panic!(
                    "Environment variable '{}' not set. Error: {:?}",
                    var_name, e
                )
            }
        }
    }
}

lazy_static! {
    pub static ref CONFIG: Config = Config::new();
}
