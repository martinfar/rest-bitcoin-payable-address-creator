
extern crate secp256k1;
extern crate crypto;
extern crate rand;
extern crate rust_base58;
extern crate bech32;

use secp256k1::Secp256k1;
use rand::rngs::OsRng;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use hex::FromHex;
use serde::{Deserialize, Serialize};

mod create_address;



#[derive(Deserialize)]
struct AddressRequest {
    address_type: String,
    network: String,
    script: Option<String>,
}

#[derive(Serialize)]
struct AddressResponse {
    address: String,
    public_key: String,
}

async fn generate_address(info: web::Json<AddressRequest>) -> impl Responder {
    let secp256k1 = Secp256k1::new();
    let mut rng = OsRng::new().expect("OsRng");
    let (_secret_key, public_key) = secp256k1.generate_keypair(&mut rng);
    let serialized_public_key = public_key.serialize();

    let network = match info.network.as_str() {
        "testnet" => create_address::Network::Testnet,
        _ => create_address::Network::Mainnet,
    };

    let address = match info.address_type.as_str() {
        "p2pkh" => create_address::BitcoinAddress::p2pkh(&serialized_public_key, network),
        "p2wpkh" => create_address::BitcoinAddress::p2wpkh(&serialized_public_key, network),
        "p2sh" => {
            let script = parse_script(&info.script.as_ref().unwrap());
            create_address::BitcoinAddress::p2sh(&script, network)
        },
        "p2wsh" => {
            let script = parse_script(&info.script.as_ref().unwrap());
            create_address::BitcoinAddress::p2wsh(&script, network)
        },
        _ => return HttpResponse::BadRequest().body("Invalid address type"),
    };

    let response = AddressResponse {
        address: address.to_string(),
        public_key: hex::encode(serialized_public_key),
    };

    HttpResponse::Ok().json(response)
}

fn parse_script(script: &str) -> Vec<u8> {
    script.split(',')
        .filter_map(|s| hex::decode(s).ok())
        .flatten()
        .collect()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/generate", web::post().to(generate_address))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}




