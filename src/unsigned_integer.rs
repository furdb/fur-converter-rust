use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::{char::from_digit, error::Error};

#[derive(serde::Deserialize)]
struct EncodeParams {
    pub data: String,
    pub size: String,
}

#[get("/unsigned_integer/encode")]
pub async fn encode(req: HttpRequest) -> Result<impl Responder, Box<dyn Error>> {
    let params = web::Query::<EncodeParams>::from_query(req.query_string()).unwrap();

    let mut data = params.data.parse::<u128>()?;
    let size = params.size.parse::<u128>()?;

    let mut bin = String::new();
    while data > 0 {
        bin.push(from_digit((data % 2) as u32, 10).unwrap());
        data /= 2;
    }

    while bin.len() < size as usize {
        bin.push('0');
    }

    bin = bin.chars().rev().collect::<String>();

    Ok(HttpResponse::Ok().body(bin))
}

#[derive(serde::Deserialize)]
struct DecodeParams {
    pub binary: String,
}

#[get("/unsigned_integer/decode")]
pub async fn decode(req: HttpRequest) -> Result<impl Responder, Box<dyn Error>> {
    let params = web::Query::<DecodeParams>::from_query(req.query_string()).unwrap();

    let binary = params.binary.clone();
    let decimal: u128 = 0;

    for i in binary.len()..=0 {
        let cur_add = if binary[i as usize] == '1' { 10 } else { 0 };
    }

    Ok(HttpResponse::Ok().body("LALALA"))
}
