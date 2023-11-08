use std::{env, net::IpAddr};

use actix_web::{post, App, HttpRequest, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use remote_core::{
    add_or_update_client, get_commands_queue_for_ip, update_commands_for_ip, Ping, Pong,
};

fn get_client_ip(request: HttpRequest) -> String {
    request
        .peer_addr()
        .map(|socket_addr| {
            // Extract the IP address without the port
            let ip: IpAddr = socket_addr.ip();
            ip.to_string()
        })
        .unwrap_or_else(|| "<unknown>".to_string())
}

fn is_token_valid(token: &String) -> bool {
    let config_token = env::var("TOKEN").expect("Config token must be set");
    config_token.eq(token)
}

#[post("/ping")]
pub async fn ping(request: HttpRequest, model: actix_web::web::Json<Ping>) -> impl Responder {
    let ip = get_client_ip(request);
    println!("Ping {}: {:?}", ip, model);

    if !is_token_valid(&model.token) {
        println!("Tokens don't match");
        return HttpResponse::Unauthorized().body("You shall not pass!");
    }

    let commands = get_commands_queue_for_ip(&ip);
    add_or_update_client(&ip);

    return HttpResponse::Ok().json(commands);
}

#[post("/pong")]
pub async fn pong(request: HttpRequest, model: actix_web::web::Json<Pong>) -> impl Responder {
    let ip = get_client_ip(request);
    println!("Pong {}: {:?}", ip, model);

    if !is_token_valid(&model.token) {
        println!("Tokens don't match");
        return HttpResponse::Unauthorized().body("You shall not pass!");
    }

    add_or_update_client(&ip);
    update_commands_for_ip(ip, model.into_inner().commands);

    return HttpResponse::Ok().json(true);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    println!("Remote Central Server running");

    HttpServer::new(|| App::new().service(ping).service(pong))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
