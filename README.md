# Restful Bitcoin Address Creator 

## Overview

This API allows you to generate various types of Bitcoin addresses. 
It supports P2PKH, P2WPKH, P2SH, and P2WSH address types for both mainnet and testnet networks.

## Run 
```shell
cargo build
cd target/debug/
./rest_bitcoin_payable_address_creator
```
```shell
curl --location 'http://127.0.0.1:8080/generate' \
--header 'Content-Type: application/json' \
--data '{
  "address_type": "p2sh",
  "network": "testnet",
  "script": "00,14"
}'
```

## Base URL
```
http://127.0.0.1:8080
```

## Endpoints

### Generate Bitcoin Address

Generates a new Bitcoin address based on the provided parameters.

- **URL**: `/generate`
- **Method**: `POST`
- **Content-Type**: `application/json`

#### Request Body

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| address_type | string | Yes | The type of address to generate. Valid values: "p2pkh", "p2wpkh", "p2sh", "p2wsh" |
| network | string | Yes | The Bitcoin network to use. Valid values: "mainnet", "testnet" |
| script | string | No | Required for p2sh and p2wsh address types. A comma-separated list of hexadecimal values representing the script |

#### Response

| Field | Type | Description |
|-------|------|-------------|
| address | string | The generated Bitcoin address |
| public_key | string | The public key associated with the address (in hexadecimal format) |

#### Examples

1. Generating a P2PKH address on mainnet:

Request:
```json
{
  "address_type": "p2pkh",
  "network": "mainnet"
}
```

Response:
```json
{
  "address": "1BvBMSEYstWetqTFn5Au4m4GFg7xJaNVN2",
  "public_key": "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798"
}
```

2. Generating a P2SH address on testnet with a custom script:

Request:
```json
{
  "address_type": "p2sh",
  "network": "testnet",
  "script": "00,14,79be667ef9dcbbac55a06295ce870b07029bfcdb2"
}
```

Response:
```json
{
  "address": "2N7asrqCHGsLX2ynwzGGKq7eyER6QXMxtMj",
  "public_key": "0379be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798"
}
```

## Error Handling

The API uses standard HTTP response codes to indicate the success or failure of requests.

- 200 OK: The request was successful.
- 400 Bad Request: The request was invalid or cannot be served. An error message will be provided in the response body.
- 500 Internal Server Error: The server encountered an unexpected condition that prevented it from fulfilling the request.

Error Response Body:
```json
{
  "error": "Description of the error"
}
```

## Rate Limiting

Currently, there are no rate limits imposed on the API. However, please use the API responsibly.

## Security

This API is intended for development and testing purposes. It does not implement authentication or encryption. Do not use it to generate addresses for storing real funds without proper security measures in place.

## Changelog

- v0.0.1 (2024-04-01): Initial release of the Bitcoin Address Generator API.

For any questions or issues, please contact the API maintainer.