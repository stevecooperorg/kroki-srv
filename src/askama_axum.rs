pub use askama::*;
use axum::http;
pub use axum::response::{IntoResponse, Response};
use http::StatusCode;

pub fn into_response<T: Template>(t: &T) -> Response {
    match t.render() {
        Ok(body) => {
            let headers = [(
                http::header::CONTENT_TYPE,
                http::HeaderValue::from_static(T::MIME_TYPE),
            )];

            (headers, body).into_response()
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
//
// pub trait TemplateResponse: Template {
//     fn resp(t: &Self) -> Response {
//         match t.render() {
//             Ok(body) => {
//                 let headers = [(
//                     http::header::CONTENT_TYPE,
//                     http::HeaderValue::from_static(Self::MIME_TYPE),
//                 )];
//
//                 (headers, body).into_response()
//             }
//             Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
//         }
//     }
// }
