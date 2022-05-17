#[macro_use]
extern crate log;

extern crate pretty_env_logger;

use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use counter::{get_contract_id, setup_provider_and_wallet, tx_params, Counter};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
struct Increment {
    success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct GetCount {
    success: bool,
    count: u32,
}

async fn increment_count(req: HttpRequest) -> Result<HttpResponse, Error> {
    let resp = Increment { success: true };
    let state = req.app_data::<web::Data<Mutex<Counter>>>().unwrap();
    let contract = match state.lock() {
        Ok(c) => c,
        Err(e) => {
            error!("Could not get state: {}", e);
            return Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(serde_json::to_string(&resp).unwrap()));
        }
    };

    let result = contract
        .increment_counter(1)
        .tx_params(tx_params())
        .call()
        .await
        .unwrap();

    debug!("{:?}", result);

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&resp).unwrap()))
}

async fn get_count(req: HttpRequest) -> Result<HttpResponse, Error> {
    let resp = GetCount {
        success: true,
        count: 1u32,
    };

    let state = req.app_data::<web::Data<Mutex<Counter>>>().unwrap();
    let contract = state.lock().unwrap();

    let result = contract
        .get_count()
        .tx_params(tx_params())
        .call()
        .await
        .unwrap();

    debug!("{:?}", result);

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&resp).unwrap()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    let (provider, wallet) = setup_provider_and_wallet().await;
    let contract_id: String = get_contract_id(&provider, &wallet).await;
    info!("Using contract at {}", contract_id);
    let contract: Counter = Counter::new(contract_id, provider, wallet);

    let state = web::Data::new(Mutex::new(contract));

    info!("Starting server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/counter")
                    .app_data(web::JsonConfig::default().limit(1024))
                    .route(web::post().to(increment_count))
                    .route(web::get().to(get_count)),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::body::to_bytes;
    use actix_web::dev::Service;
    use actix_web::{http, test, web, App};

    #[actix_web::test]
    async fn test_increment_count() {
        let app = test::init_service(
            App::new().service(web::resource("/counter").route(web::post().to(increment_count))),
        )
        .await;

        let req = test::TestRequest::post().uri("/counter").to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let body_bytes = to_bytes(resp.into_body()).await.unwrap();
        assert_eq!(body_bytes, r##"{"success":true}"##);
    }

    #[actix_web::test]
    async fn test_get_count() {
        let app = test::init_service(
            App::new().service(web::resource("/counter").route(web::get().to(get_count))),
        )
        .await;

        let req = test::TestRequest::get().uri("/counter").to_request();
        let resp = app.call(req).await.unwrap();
        let body_bytes = to_bytes(resp.into_body()).await.unwrap();
        assert_eq!(body_bytes, r##"{"success":true,"count":1}"##);
    }
}
