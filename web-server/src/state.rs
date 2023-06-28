//! Server state

use std::collections::HashMap;
use tide::{Body, Request, Response, StatusCode, Redirect};
use tide::http::headers::HeaderName;
//use surf::Client;

/// Runtime state of the server
#[derive(Debug, Clone)]
pub struct State {
    //// Http client
    //pub http_client: Client,
    ///// Proxy /docs/* to a different URL
    //pub docs_proxy: Option<String>
}

///// Handles proxying
////
///// This is not production-grade, just for proxying get requests during development.
// pub async fn handle_get_proxy<S>(client: &Client, location: S, req: &Request<State>) -> tide::Result where S: AsRef<str> {
//     let path = req.url().path();
//     let location = location.as_ref();
//     let proxy_path = format!("{location}{path}");
//     log::info!("Proxying {path} to {proxy_path}");
    
//     let mut proxy_resp = client.get(proxy_path).send().await?;
//     let mut resp = Response::new(proxy_resp.status());

//     proxy_resp.iter().for_each(|(key, value)| {
//         //if let Ok(key) = HeaderName::from_string(key.to_string()) {
//             //if let Ok(value) = value.to_str() {
//                 resp.insert_header(key, value);
//            // }
//         //}
//     });


//     resp.set_body(Body::from_bytes(proxy_resp.body_bytes().await?.to_vec()));

//     Ok(resp)
    
// }