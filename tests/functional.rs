// SPDX-FileCopyrightText: © 2023 Merqury Cybersecurity Ltd <info@merqury.eu>
// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

#[macro_use]
extern crate lazy_static;

mod common;
mod models;

use base64::Engine;
use common::config::CONFIG;
use models::{key, status::Status};
use pretty_assertions::assert_eq;
use reqwest::{Method, StatusCode};
use rstest::rstest;
use serde_json::json;

#[rstest]
#[case::using_get(Method::GET)]
#[case::using_post(Method::POST)]
fn successful_key_request_and_retrieval(#[case] request_method: Method) {
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

#[rstest]
#[case::using_get(Method::GET)]
#[case::using_post(Method::POST)]
fn unauthorized_access(#[case] request_method: Method) {
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

#[rstest]
#[case::using_get(Method::GET)]
#[case::using_post(Method::POST)]
fn additional_slave_sae_ids(#[case] request_method: Method) {
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

#[rstest]
#[case::using_get(Method::GET)]
#[case::using_post(Method::POST)]
fn default_values_match_status_reply(#[case] request_method: Method) {
    let status_url =
        format!("{}/{}/status", CONFIG.base_url, CONFIG.slave_sae_id);
    let enc_keys_url =
        format!("{}/{}/enc_keys", CONFIG.base_url, CONFIG.slave_sae_id);
    let client = common::build_client(&CONFIG.master_sae_crt);

    // Request status
    let status_response = client.get(&status_url).send().unwrap();
    // Request a key with the default values
    let enc_keys_response = match request_method {
        Method::GET => {
            client.request(request_method.clone(), enc_keys_url).send().unwrap()
        }
        Method::POST => {
            client.request(request_method.clone(), enc_keys_url).send().unwrap()
        }
        _ => {
            panic!("Only 'GET' and 'POST' methods are supported")
        }
    };

    // Ensure both calls are a success
    assert!(status_response.status().is_success());
    assert!(enc_keys_response.status().is_success());

    // Compare the default number of keys and their size
    let status_body = match status_response.json::<Status>() {
        Ok(val) => val,
        Err(e) => {
            panic!("Invalid '/status' response given. Error: {:?}", e);
        }
    };

    let key_container = match enc_keys_response.json::<key::KeyContainer>() {
        Ok(parsed_body) => parsed_body,
        Err(e) => {
            panic!("Invalid response given. Error: {:?}", e);
        }
    };

    // The default number of keys is 1.
    assert_eq!(key_container.keys.len(), 1);

    let decoded_key = match base64::engine::general_purpose::STANDARD
        .decode(key_container.keys[0].key.as_ref().unwrap())
    {
        Ok(val) => val,
        Err(e) => panic!("Failed to decode key value. Error: {:?}", e),
    };

    assert_eq!(
        // base64 returns a vector of bytes, key_size is in bits, hence the
        // conversion.
        i32::try_from(decoded_key.len()).unwrap() * 8,
        status_body.key_size
    );
}
