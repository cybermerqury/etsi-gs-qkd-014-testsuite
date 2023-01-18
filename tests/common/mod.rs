// SPDX-FileCopyrightText: © 2023 Merqury Cybersecurity Ltd <info@merqury.eu>
// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

pub mod config;

use config::CONFIG;
use std::{fs::File, io::Read};

pub fn build_client(client_pem_path: &str) -> reqwest::blocking::Client {
    reqwest::blocking::Client::builder()
        .add_root_certificate(load_root_certificate())
        .identity(load_identity(client_pem_path))
        .min_tls_version(reqwest::tls::Version::TLS_1_3)
        .danger_accept_invalid_certs(false)
        .use_rustls_tls()
        .build()
        .unwrap()
}

fn load_root_certificate() -> reqwest::Certificate {
    let mut cert_buf = Vec::new();

    File::open(&CONFIG.root_crt).unwrap().read_to_end(&mut cert_buf).unwrap();

    reqwest::Certificate::from_pem(&cert_buf).unwrap()
}

fn load_identity(pem_path: &str) -> reqwest::Identity {
    let mut buf = Vec::new();
    File::open(pem_path).unwrap().read_to_end(&mut buf).unwrap();

    reqwest::Identity::from_pem(&buf).unwrap()
}
