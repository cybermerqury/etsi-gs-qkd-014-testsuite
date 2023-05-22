# SPDX-FileCopyrightText: © 2023 Merqury Cybersecurity Ltd <info@merqury.eu>
# SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

.PHONY: default build run_tests

ROOT_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))

default: build

build:
	@cargo build --tests

run_tests:
	@                                                                        \
	ETSI_014_TEST_SUITE_BASE_SERVER_URL=https://localhost:8443/api/v1/keys   \
	ETSI_014_TEST_SUITE_BASE_CLIENT_URL=https://localhost:8443/api/v1/keys   \
	ETSI_014_TEST_SUITE_TLS_ROOT_CRT=$(ROOT_DIR)/certs/root.crt              \
	ETSI_014_TEST_SUITE_MASTER_SAE_ID=sae_001                                \
	ETSI_014_TEST_SUITE_TLS_MASTER_SAE_CERT=$(ROOT_DIR)/certs/sae_001.pem    \
	ETSI_014_TEST_SUITE_SLAVE_SAE_ID=sae_002                                 \
	ETSI_014_TEST_SUITE_TLS_SLAVE_SAE_CERT=$(ROOT_DIR)/certs/sae_002.pem     \
	ETSI_014_TEST_SUITE_ADD_SLAVE_SAE_ID=sae_003                             \
	ETSI_014_TEST_SUITE_TLS_ADD_SLAVE_SAE_CERT=$(ROOT_DIR)/certs/sae_003.pem \
	cargo test

run_functional_tests:
	@                                                                        \
	ETSI_014_TEST_SUITE_BASE_SERVER_URL=https://localhost:8443/api/v1/keys   \
	ETSI_014_TEST_SUITE_BASE_CLIENT_URL=https://localhost:8443/api/v1/keys   \
	ETSI_014_TEST_SUITE_TLS_ROOT_CRT=$(ROOT_DIR)/certs/root.crt              \
	ETSI_014_TEST_SUITE_MASTER_SAE_ID=sae_001                                \
	ETSI_014_TEST_SUITE_TLS_MASTER_SAE_CERT=$(ROOT_DIR)/certs/sae_001.pem    \
	ETSI_014_TEST_SUITE_SLAVE_SAE_ID=sae_002                                 \
	ETSI_014_TEST_SUITE_TLS_SLAVE_SAE_CERT=$(ROOT_DIR)/certs/sae_002.pem     \
	ETSI_014_TEST_SUITE_ADD_SLAVE_SAE_ID=sae_003                             \
	ETSI_014_TEST_SUITE_TLS_ADD_SLAVE_SAE_CERT=$(ROOT_DIR)/certs/sae_003.pem \
	cargo test --test functional

run_validation_tests:
	@                                                                        \
	ETSI_014_TEST_SUITE_BASE_SERVER_URL=https://localhost:8443/api/v1/keys   \
	ETSI_014_TEST_SUITE_BASE_CLIENT_URL=https://localhost:8443/api/v1/keys   \
	ETSI_014_TEST_SUITE_TLS_ROOT_CRT=$(ROOT_DIR)/certs/root.crt              \
	ETSI_014_TEST_SUITE_MASTER_SAE_ID=sae_001                                \
	ETSI_014_TEST_SUITE_TLS_MASTER_SAE_CERT=$(ROOT_DIR)/certs/sae_001.pem    \
	ETSI_014_TEST_SUITE_SLAVE_SAE_ID=sae_002                                 \
	ETSI_014_TEST_SUITE_TLS_SLAVE_SAE_CERT=$(ROOT_DIR)/certs/sae_002.pem     \
	ETSI_014_TEST_SUITE_ADD_SLAVE_SAE_ID=sae_003                             \
	ETSI_014_TEST_SUITE_TLS_ADD_SLAVE_SAE_CERT=$(ROOT_DIR)/certs/sae_003.pem \
	cargo test --test validation
