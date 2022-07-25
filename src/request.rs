use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    // console,
    Request,
    RequestInit,
    RequestMode,
    Response,
};

use crate::constants::ALLOW_ORIGIN;

pub async fn fetch(
    url: &str,
    headers: Option<HashMap<String, String>>,
) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(url, &opts)?;

    request.headers().set("Accept", "application/json")?;

    if !ALLOW_ORIGIN.is_empty() {
        request
            .headers()
            .set("Access-Control-Allow-Origin", ALLOW_ORIGIN)?;
    }

    if let Some(headers) = headers {
        headers.iter().for_each(|(key, value)| {
            request.headers().set(
                key.as_str(),
                value.as_str(),
            ).unwrap();
        });
    }

    let window = web_sys::window().unwrap();
    let res_obj = JsFuture::from(window.fetch_with_request(&request)).await?;
    let res: Response = res_obj.dyn_into().unwrap();
    let json = JsFuture::from(res.json()?).await?;

    Ok(json)
}
