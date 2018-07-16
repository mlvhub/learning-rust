extern crate actix;
extern crate actix_web;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate failure;
extern crate reqwest;

use actix_web::{server, App, HttpRequest, HttpResponse, Responder, http, error};

#[derive(Fail, Debug)]
enum AppError {
   #[fail(display="internal request error")]
   InternalRequestError,
   #[fail(display="internal error")]
   InternalError,
   #[fail(display="bad request")]
   BadClientData,
   #[fail(display="timeout")]
   Timeout,
}

impl error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
       match *self {
          AppError::InternalRequestError => HttpResponse::new(
              http::StatusCode::INTERNAL_SERVER_ERROR),
          AppError::InternalError => HttpResponse::new(
              http::StatusCode::INTERNAL_SERVER_ERROR),
          AppError::BadClientData => HttpResponse::new(
              http::StatusCode::BAD_REQUEST),
          AppError::Timeout => HttpResponse::new(
              http::StatusCode::GATEWAY_TIMEOUT),
       }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Quote {
    id: String,
    value: String,
    icon_url: String
}
impl Responder for Quote {
    type Item = HttpResponse;
    type Error = actix_web::Error;

    fn respond_to<S>(self, _req: &HttpRequest<S>) -> Result<HttpResponse, actix_web::Error> {
        let body = serde_json::to_string(&self)?;

        // Create response and set content type
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
    }
}

fn get_quote() -> Result<Quote, reqwest::Error> {
    let request_url = "https://api.chucknorris.io/jokes/random";

    let client = reqwest::Client::new();
    let mut response = client.get(request_url).send()?;

    let quote: Quote = response.json()?;
    println!("{:?}", quote);

    Ok(quote)
}

fn index(_req: HttpRequest) -> Result<Quote, AppError> {
    get_quote().map_err(|_| AppError::InternalRequestError)
}

fn main() {
    let sys = actix::System::new("example");

    server::new(
        || App::new()
            .resource("/", |r| r.method(http::Method::GET).f(index)))
        .bind("127.0.0.1:8088").unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8088");
    let _ = sys.run();
}
