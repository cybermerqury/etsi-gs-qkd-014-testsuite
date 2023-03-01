# Description

This project provides a host of tests that verify compliance to the
[ETSI GS QKD 014 v1.1.1](https://www.etsi.org/deliver/etsi_gs/QKD/001_099/014/01.01.01_60/gs_QKD014v010101p.pdf)
standard.

# Test description

The test suite provides two sets of tests: functional and validation tests.

## Functional tests

Functional tests (located at `tests/functional.rs`) contain all the tests
required to ensure that the web service is functioning as expected.
For example, among other tests, the web service is tested to ensure that the
correct number of keys as requested is returned.

## Validation tests

Validation tests (located at `tests/validation.rs`) are tests that ensures that
the web service is able to handle incorrect data and return the appropriate
error messages.

# Using the test suite

A `makefile` is provided in the root directory of this project that contains
commands showing how to build and run the tests.
The `makefile` is assuming that all the certificates are located in the `certs`
directory.
However, the `makefile` can be modified to suit your needs.

The provided `makefile` has three targets
| Target name            | Description                              |
|------------------------|------------------------------------------|
| `build`                | Compiles and builds the tests.           |
| `run_tests`            | Runs *all* the tests in this test suite. |
| `run_functional_tests` | Runs the functional tests *only*.        |
| `run_validation_tests` | Runs the validation tests *only*.        |

## Running the tests

The simplest way to run all the tests is to use the provided `makefile` and
running the `run_tests` target.

## Tests and SAEs

The tests require that the user supplies three different Secure Application
Entity (SAE) certificates and their associated name.
These SAE certificates are used by the tests to mimic a request coming from an
actual SAE.

## Environment variables

Environment variables are used to set user specific values required by the
tests.

| Environment variable                     | Description                                                            |
|------------------------------------------|------------------------------------------------------------------------|
ETSI_014_TEST_SUITE_BASE_URL               | Base URL of the server to test.                                        |
ETSI_014_TEST_SUITE_TLS_ROOT_CRT           | Path to the root certificate.                                          |
ETSI_014_TEST_SUITE_MASTER_SAE_ID          | Name of the master SAE ID.                                             |
ETSI_014_TEST_SUITE_TLS_MASTER_SAE_CERT    | Path to the certificate to associate with the master SAE ID.           |
ETSI_014_TEST_SUITE_SLAVE_SAE_ID           | Name of the slave SAE ID.                                              |
ETSI_014_TEST_SUITE_TLS_SLAVE_SAE_CERT     | Path to the certificate to associate with the slave SAE ID.            |
ETSI_014_TEST_SUITE_ADD_SLAVE_SAE_ID       | Name of the additional slave SAE ID.                                   |
ETSI_014_TEST_SUITE_TLS_ADD_SLAVE_SAE_CERT | Path to the certificate to associate with the additional slave SAE ID. |

# Certificate generation

A comprehensive guide on how to generate authentication certificates can be
found in the ETSI GS QKD 014 reference implementation [repository](https://github.com/cybermerqury/etsi-gs-qkd-014-referenceimplementation/blob/main/README.md#certificates).

# License

© 2023 Merqury Cybersecurity Ltd.

This project is licensed under the [PolyForm Noncommercial License
1.0.0](https://polyformproject.org/licenses/noncommercial/1.0.0) that prohibits
commercial use of this product.
If you would like to use this product in a commercial environment, kindly
contact us on [info@merqury.eu](mailto:info@merqury.eu).

# Acknowledgements

This software has been developed in the project EQUO (European QUantum
ecOsystems) which is funded by the European Commission in the Digital Europe
Programme under the grant agreement No 101091561.
