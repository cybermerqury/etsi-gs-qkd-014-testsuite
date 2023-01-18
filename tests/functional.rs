// SPDX-FileCopyrightText: © 2023 Merqury Cybersecurity Ltd <info@merqury.eu>
// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0
#[macro_use]
extern crate lazy_static;

mod common;
mod models;

use common::config::CONFIG;
use models::key;
use reqwest::{Method, StatusCode};
use serde_json::json;
use test_case::test_case;

#[test_case(Method::GET; "Using GET")]
#[test_case(Method::POST; "Using POST")]
fn successful_key_request_and_retrieval(request_method: Method) {
    let enc_keys_url =
        format!("{}/{}/enc_keys", CONFIG.base_url, CONFIG.slave_sae_id);
    let dec_keys_url =
        format!("{}/{}/dec_keys", CONFIG.base_url, CONFIG.master_sae_id);
    let master_client = common::build_client(&CONFIG.master_sae_crt);
    let slave_client = common::build_client(&CONFIG.slave_sae_crt);

    // Request a key
    let enc_keys_response = match request_method {
        Method::GET => master_client
            .request(request_method.clone(), enc_keys_url)
            .query(&[("number", 1)])
            .send()
            .unwrap(),
        Method::POST => master_client
            .request(request_method.clone(), enc_keys_url)
            .send()
            .unwrap(),
        _ => {
            panic!("Only 'GET' and 'POST' methods are supported")
        }
    };

    assert!(enc_keys_response.status().is_success());

    let returned_keys = match enc_keys_response.json::<key::KeyContainer>() {
        Ok(parsed_body) => parsed_body,
        Err(e) => {
            panic!("Invalid response given. Error: {:?}", e);
        }
    };

    // Request the key retrieved based on its id
    let dec_keys_response = match request_method {
        Method::GET => {
            assert_eq!(returned_keys.keys.len(), 1);

            slave_client
                .request(request_method, dec_keys_url)
                .query(&[("key_ID", returned_keys.keys[0].key_id)])
                .send()
                .unwrap()
        }
        Method::POST => slave_client
            .request(request_method, dec_keys_url)
            .json(&returned_keys)
            .send()
            .unwrap(),
        _ => {
            panic!("Only 'GET' and 'POST' methods are supported")
        }
    };

    assert!(dec_keys_response.status().is_success());

    let retrieved_key_by_id =
        match dec_keys_response.json::<key::KeyContainer>() {
            Ok(parsed_body) => parsed_body,
            Err(e) => {
                panic!("Invalid response given. Error: {:?}", e);
            }
        };

    assert_eq!(retrieved_key_by_id, returned_keys);
}

#[test_case(Method::GET; "Using GET")]
#[test_case(Method::POST; "Using POST")]
fn unauthorized_access(request_method: Method) {
    let enc_keys_url =
        format!("{}/{}/enc_keys", CONFIG.base_url, CONFIG.slave_sae_id);
    let dec_keys_url =
        format!("{}/{}/dec_keys", CONFIG.base_url, CONFIG.master_sae_id);
    let master_client = common::build_client(&CONFIG.master_sae_crt);
    let unauthorized_client = common::build_client(&CONFIG.add_slave_sae_crt);

    // Request a key
    let enc_keys_response = match request_method {
        Method::GET => master_client
            .request(request_method.clone(), enc_keys_url)
            .query(&[("number", 1)])
            .send()
            .unwrap(),
        Method::POST => master_client
            .request(request_method.clone(), enc_keys_url)
            .json(&json!({"number": 1}))
            .send()
            .unwrap(),
        _ => {
            panic!("Only 'GET' and 'POST' methods are supported")
        }
    };

    assert!(enc_keys_response.status().is_success());

    let key = match enc_keys_response.json::<key::KeyContainer>() {
        Ok(parsed_body) => parsed_body.keys.get(0).unwrap().clone(),
        Err(e) => {
            panic!("Invalid response given. Error: {:?}", e);
        }
    };

    // Request the key using the id from an unauthorized SAE
    let dec_keys_response = match request_method {
        Method::GET => unauthorized_client
            .request(request_method, dec_keys_url)
            .query(&[("key_ID", key.key_id)])
            .send()
            .unwrap(),
        Method::POST => unauthorized_client
            .request(request_method, dec_keys_url)
            .json(&json!({ "key_IDs": [key] }))
            .send()
            .unwrap(),
        _ => {
            panic!("Only 'GET' and 'POST' methods are supported")
        }
    };

    assert_eq!(dec_keys_response.status(), StatusCode::UNAUTHORIZED);
}

#[test_case(Method::GET; "Using GET")]
#[test_case(Method::POST; "Using POST")]
fn additional_slave_sae_ids(request_method: Method) {
    let enc_keys_url =
        format!("{}/{}/enc_keys", CONFIG.base_url, CONFIG.slave_sae_id);
    let dec_keys_url =
        format!("{}/{}/dec_keys", CONFIG.base_url, CONFIG.master_sae_id);
    let master_client = common::build_client(&CONFIG.master_sae_crt);
    let additional_slave_client =
        common::build_client(&CONFIG.add_slave_sae_crt);

    let enc_keys_response = master_client
        .post(enc_keys_url)
        .json(&json!({"number": 1, "additional_slave_SAE_IDs": [CONFIG.add_slave_sae_id]}))
        .send()
        .unwrap();

    assert!(enc_keys_response.status().is_success());

    let key = match enc_keys_response.json::<key::KeyContainer>() {
        Ok(parsed_body) => parsed_body.keys.get(0).unwrap().clone(),
        Err(e) => {
            panic!("Invalid response given. Error: {:?}", e);
        }
    };

    // Request the key retrieved based on its id
    let dec_keys_response = match request_method {
        Method::GET => additional_slave_client
            .request(request_method, dec_keys_url)
            .query(&[("key_ID", key.key_id)])
            .send()
            .unwrap(),
        Method::POST => additional_slave_client
            .request(request_method, dec_keys_url)
            .json(&json!({ "key_IDs": [key] }))
            .send()
            .unwrap(),
        _ => {
            panic!("Only 'GET' and 'POST' methods are supported")
        }
    };

    assert!(dec_keys_response.status().is_success());

    let retrieved_key_by_id =
        match dec_keys_response.json::<key::KeyContainer>() {
            Ok(parsed_body) => parsed_body.keys.get(0).unwrap().clone(),
            Err(e) => {
                panic!("Invalid response given. Error: {:?}", e);
            }
        };

    assert_eq!(retrieved_key_by_id, key);
}
