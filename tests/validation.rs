// SPDX-FileCopyrightText: © 2023 Merqury Cybersecurity Ltd <info@merqury.eu>
// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

#[macro_use]
extern crate lazy_static;

mod common;
mod models;

use base64::Engine;
use common::config::CONFIG;
use models::{error_message::ErrorMessage, key, status::Status};
use pretty_assertions::assert_eq;
use reqwest::{blocking::Response, Method, StatusCode};
use rstest::rstest;
use serde_json::json;
use uuid::Uuid;

#[rstest]
#[case::zero_key_size("0")]
#[case::negative_key_size("-8")]
#[case::alphanumeric_key_size("abc01")]
fn validate_key_size(#[case] key_size: &str) {
    let client = common::build_client(&CONFIG.master_sae_crt);
    let url = format!("{}/{}/enc_keys", CONFIG.base_url, CONFIG.slave_sae_id);
    let mut responses: Vec<Response> = Vec::new();

    responses
        .push(client.get(&url).query(&[("size", key_size)]).send().unwrap());

    let json_body = match key_size.parse::<i64>() {
        Ok(numeric_key_size) => {
            json!({ "size": numeric_key_size })
        }
        Err(_) => {
            json!({ "size": key_size })
        }
    };

    responses.push(client.post(&url).json(&json_body).send().unwrap());

    for response in responses {
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let resp_body = response.text().unwrap();

        assert!(
            serde_json::from_str::<ErrorMessage>(&resp_body).is_ok(),
            "Invalid error message format returned. Response: {}",
            resp_body
        );
    }
}

#[rstest]
#[case::zero_requested_keys("0")]
#[case::negative_number_of_keys_requested("-8")]
#[case::alphanumeric_number_of_requested_keys("abc01")]
fn validate_num_keys(#[case] num_keys: &str) {
    let client = common::build_client(&CONFIG.master_sae_crt);
    let url = format!("{}/{}/enc_keys", CONFIG.base_url, CONFIG.slave_sae_id);
    let mut responses: Vec<Response> = Vec::new();

    responses
        .push(client.get(&url).query(&[("number", num_keys)]).send().unwrap());

    let json_body = match num_keys.parse::<i64>() {
        Ok(numeric_num_keys) => {
            json!({ "number": numeric_num_keys })
        }
        Err(_) => {
            json!({ "number": num_keys })
        }
    };

    responses.push(client.post(url).json(&json_body).send().unwrap());

    for response in responses {
        assert!(
            !response.status().is_success(),
            "Success returned on invalid request"
        );
        assert!(
            serde_json::from_str::<ErrorMessage>(&response.text().unwrap())
                .is_ok(),
            "Invalid error message format returned"
        );
    }
}

#[rstest]
#[case::empty_additional_sae_id_supplied(vec!["additional_sae_1234", " "])]
#[case::invalid_additional_sae_id_supplied(vec!["additional_sae_1234", "additional_sae_1234"])]
#[case::duplicate_additional_sae_id_with_slave(vec![CONFIG.slave_sae_id.as_str()])]
#[case::duplicate_additional_sae_id_with_master(vec![CONFIG.master_sae_id.as_str()])]
#[case::empty_sae_id_list(vec![])]
fn additional_sae_ids(#[case] additional_slave_sae_ids: std::vec::Vec<&str>) {
    let client = common::build_client(&CONFIG.master_sae_crt);
    let url = format!("{}/{}/enc_keys", CONFIG.base_url, CONFIG.slave_sae_id);

    let response = client
        .post(url)
        .json(&json!({
            "additional_slave_SAE_IDs": additional_slave_sae_ids
        }))
        .send()
        .unwrap();

    assert!(
        !response.status().is_success(),
        "Success returned on invalid request"
    );
    assert!(
        serde_json::from_str::<ErrorMessage>(&response.text().unwrap()).is_ok(),
        "Invalid error message format returned"
    );
}

#[test]
fn empty_sae_id_in_path() {
    // NOTE: This cannot be confirmed, because the error response of an actual
    // bad request, and when an entry is not found is the same. Suggest to
    // update the standard to return 404 when a key is not found.
    // The test can be updated such that it first gets a key and then calls the
    // endpoint, but that is more of a functional test, than validation test.
    let client = common::build_client(&CONFIG.master_sae_crt);
    let enc_keys_url = format!("{}/ /enc_keys", CONFIG.base_url);
    let dec_keys_url = format!("{}/ /dec_keys", CONFIG.base_url);
    let sample_key_id = Uuid::new_v4();
    let mut responses: Vec<Response> = Vec::new();

    responses.push(client.get(&enc_keys_url).send().unwrap());
    responses.push(client.post(&enc_keys_url).send().unwrap());

    responses.push(
        client
            .get(&dec_keys_url)
            .query(&[("key_ID", sample_key_id)])
            .send()
            .unwrap(),
    );
    responses.push(
        client
            .post(&dec_keys_url)
            .json(&json!({"key_IDs": [{"key_ID": sample_key_id}]}))
            .send()
            .unwrap(),
    );

    for response in responses {
        assert_eq!(
            response.status(),
            StatusCode::BAD_REQUEST,
            "Expected BAD_REQUEST, returned status: {}",
            &response.status()
        );

        let response_text = response.text().unwrap();

        assert!(
            serde_json::from_str::<ErrorMessage>(&response_text).is_ok(),
            "Invalid error message format returned. Response: {}",
            &response_text
        );
    }
}

#[test]
fn identical_sae_ids() {
    // NOTE: This cannot be confirmed, because the error response of an actual
    // bad request, and when an entry is not found is the same. Suggest to
    // update the standard to return 404 when a key is not found.
    // The test can be updated such that it first gets a key and then calls the
    // endpoint, but that is more of a functional test, than validation test.
    let client = common::build_client(&CONFIG.master_sae_crt);
    let enc_keys_url =
        format!("{}/{}/enc_keys", CONFIG.base_url, CONFIG.master_sae_id);
    let dec_keys_url =
        format!("{}/{}/dec_keys", CONFIG.base_url, CONFIG.master_sae_id);
    let sample_key_id = Uuid::new_v4();
    let mut responses: Vec<Response> = Vec::new();

    responses.push(client.get(&enc_keys_url).send().unwrap());
    responses.push(client.post(&enc_keys_url).send().unwrap());

    responses.push(
        client
            .get(&dec_keys_url)
            .query(&[("key_ID", sample_key_id)])
            .send()
            .unwrap(),
    );
    responses.push(
        client
            .post(&dec_keys_url)
            .json(&json!({"key_IDs": [{"key_ID": sample_key_id}]}))
            .send()
            .unwrap(),
    );

    for response in responses {
        assert_eq!(
            response.status(),
            StatusCode::BAD_REQUEST,
            "Expected BAD_REQUEST, returned status: {}",
            &response.status()
        );

        let response_text = response.text().unwrap();

        assert!(
            serde_json::from_str::<ErrorMessage>(&response_text).is_ok(),
            "Invalid error message format returned. Response: {}",
            &response_text
        );
    }
}

#[test]
fn key_id() {
    let client = common::build_client(&CONFIG.slave_sae_crt);
    let url = format!("{}/{}/dec_keys", CONFIG.base_url, CONFIG.master_sae_id);
    let invalid_key_id = "non-uuid";
    let mut responses: Vec<Response> = Vec::new();

    responses.push(
        client.get(&url).query(&[("key_ID", invalid_key_id)]).send().unwrap(),
    );

    responses.push(
        client
            .post(&url)
            .json(&json!({"key_IDs": [{"key_ID": invalid_key_id}]}))
            .send()
            .unwrap(),
    );

    for response in responses {
        assert_eq!(
            response.status(),
            StatusCode::BAD_REQUEST,
            "Expected BAD_REQUEST, returned status: {}",
            &response.status()
        );

        let response_text = response.text().unwrap();

        assert!(
            serde_json::from_str::<ErrorMessage>(&response_text).is_ok(),
            "Invalid error message format returned. Response: {}",
            &response_text
        );
    }
}

#[rstest]
#[case::using_get(Method::GET)]
#[case::using_post(Method::POST)]
fn num_keys_requested_equals_returned(#[case] request_method: Method) {
    let client = common::build_client(&CONFIG.master_sae_crt);
    let url = format!("{}/{}/enc_keys", CONFIG.base_url, CONFIG.slave_sae_id);
    let num_keys = 5;

    // Request a key
    let enc_keys_response = match request_method {
        Method::GET => client
            .request(request_method, url)
            .query(&[("number", num_keys)])
            .send()
            .unwrap(),
        Method::POST => client
            .request(request_method, url)
            .json(&json!({ "number": num_keys }))
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

    assert_eq!(returned_keys.keys.len(), num_keys);
}

#[rstest]
#[case::using_get(Method::GET)]
#[case::using_post(Method::POST)]
fn key_body(#[case] request_method: Method) {
    let client = common::build_client(&CONFIG.master_sae_crt);
    let url = format!("{}/{}/enc_keys", CONFIG.base_url, CONFIG.slave_sae_id);
    let num_keys = 3;
    let key_size_bits = 1024;
    let key_size_bytes = key_size_bits / 8;

    // Request a key
    let enc_keys_response = match request_method {
        Method::GET => client
            .request(request_method, url)
            .query(&[("number", 1), ("size", key_size_bits)])
            .send()
            .unwrap(),
        Method::POST => client
            .request(request_method, url)
            .json(&json!({ "number": num_keys, "size": key_size_bits}))
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

    for key in &returned_keys.keys {
        let decoding_result = base64::engine::general_purpose::STANDARD
            .decode(key.key.as_ref().unwrap());

        assert!(decoding_result.is_ok());
        assert_eq!(decoding_result.unwrap().len(), key_size_bytes);
    }
}

#[test]
fn status() {
    let client = common::build_client(&CONFIG.master_sae_crt);
    let url = format!("{}/{}/status", CONFIG.base_url, CONFIG.slave_sae_id);

    let response = client.get(&url).send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.text().unwrap();

    let parsed_reply = match serde_json::from_str::<Status>(&body) {
        Ok(val) => val,
        Err(e) => {
            panic!("Malformed JSON body returned. Error:'{:?}'", e)
        }
    };

    assert_eq!(parsed_reply.master_sae_id, CONFIG.master_sae_id);
    assert_eq!(parsed_reply.slave_sae_id, CONFIG.slave_sae_id);
}
