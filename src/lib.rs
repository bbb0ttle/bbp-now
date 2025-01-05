use extism_pdk::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, ToBytes, FromBytes)]
#[encoding(Json)]
struct Payload {
    pub content: String,
}

#[host_fn("extism:host/user")]
extern "ExtismHost" {
    fn toast(content: String);
}

// start with something simple
#[plugin_fn]
pub fn greet(name: String) -> FnResult<String> {
    unsafe {
        let _ = toast(format!("Got you, {}!", name));
    };
    let c = format!("Hey, *{}*!", name);
    Ok(markdown::to_html(&c))
}

// use json data for inputs and outputs
#[derive(FromBytes, Deserialize, PartialEq, Debug)]
#[encoding(Json)]
struct Add {
    left: i32,
    right: i32,
}
#[derive(ToBytes, Serialize, PartialEq, Debug)]
#[encoding(Json)]
struct Sum {
    value: i32,
}

#[plugin_fn]
pub fn http_get(Json(req): Json<HttpRequest>) -> FnResult<Memory> {
    trace!("HTTP Request: {:?}", req);
    info!("Request to: {}", req.url);

    let api = "https://api.bbki.ng/movies";

    let req_new = HttpRequest{
        url: (&api).to_string(),
        method: Some("GET".to_string()),
        headers: BTreeMap::new(),
    };

    let res = http::request::<()>(&req_new, None)?;
    Ok(res.into_memory())
}

#[plugin_fn]
pub fn add(input: Add) -> FnResult<Sum> {
    Ok(Sum {
        value: input.left + input.right,
    })
}
