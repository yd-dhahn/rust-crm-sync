use futures::{future, Future, Stream};
use hyper::{Body, Response, StatusCode};
use std::collections::HashMap;
use std::sync::mpsc::{ Sender};
use url::form_urlencoded;

pub type BoxFut = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;

pub static NOTFOUND: &[u8] = b"Not Found";
pub static INDEX: &[u8] = b"<h4>===> SYNC API <===</h4>";
pub static MISSING: &[u8] = b"Missing field";
pub static NOTNUMERIC: &[u8] = b"Number field is not numeric";

pub fn build_json_response(res: String) -> BoxFut {
    let body = Body::from(format!("[{}]", res));
    Box::new(future::ok(
        Response::builder()
            .header("Content-Type", "application/json")
            .body(body)
            .unwrap(),
    ))
}

pub fn response_unprocessable(body: &'static [u8]) -> Response<Body> {
    Response::builder()
        .status(StatusCode::UNPROCESSABLE_ENTITY)
        .body(body.into())
        .unwrap()
}

pub fn response_notify(
    body: Body,
    route: String,
    status: StatusCode,
    sender: Sender<(String, u16)>,
) -> BoxFut {
    Box::new(body.concat2().map(move |chunk| {
        let params = form_urlencoded::parse(chunk.as_ref())
            .into_owned()
            .collect::<HashMap<String, String>>();
        let number = if let Some(n) = params.get("number") {
            if let Ok(v) = n.parse::<u16>() {
                v
            } else {
                return response_unprocessable(NOTNUMERIC);
            }
        } else {
            return response_unprocessable(MISSING);
        };
        let _ = sender.send((route, number));

        Response::builder()
            .status(status)
            .body("OK".into())
            .unwrap()
    }))
}
