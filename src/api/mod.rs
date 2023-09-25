use crate::errors::AppError;
use leptos::Serializable;
use serde::Serialize;
use serde_json::Value;

pub mod search;

#[cfg(not(feature = "ssr"))]
fn endpoint() -> String {
  let location = web_sys::window().unwrap().location();
  let hostname = location.hostname().unwrap();
  let port = location.port().unwrap();

  // If the port ends in 3000, its testing, and you need to use the 8902 port for API calls
  if port == "3000" {
    format!("http://{hostname}:8902")
  } else {
    location.origin().unwrap()
  }
}

#[cfg(feature = "ssr")]
fn endpoint() -> String {
  std::env::var("TORRENTS_CSV_ENDPOINT").unwrap_or("http://0.0.0.0:8902".to_string())
}

#[cfg(not(feature = "ssr"))]
pub async fn api_wrapper<Response, Form>(form: &Form) -> Result<Response, AppError>
where
  Response: Serializable,
  Form: Serialize,
{
  let abort_controller = web_sys::AbortController::new().ok();
  let abort_signal = abort_controller.as_ref().map(|a| a.signal());

  let json = gloo_net::http::Request::get(&build_fetch_query(endpoint(), form))
    .abort_signal(abort_signal.as_ref())
    .send()
    .await?
    .text()
    .await?;

  // abort in-flight requests if the Scope is disposed
  // i.e., if we've navigated away from this page
  leptos::on_cleanup(move || {
    if let Some(abort_controller) = abort_controller {
      abort_controller.abort()
    }
  });

  // Return the error response json as an error
  Response::de(&json).map_err(|_| AppError::APIError {
    error: json_deser_err(&json),
  })
}

/// Used if you hit a deser error, which usually means a LemmyAPI error
/// Of type {error: string}
fn json_deser_err(json: &str) -> String {
  serde_json::from_str(json)
    .map(|v: Value| v["error"].as_str().unwrap_or("Unknown").to_string())
    .unwrap_or("Unknown".to_string())
}

#[cfg(feature = "ssr")]
pub async fn api_wrapper<Response, Form>(form: &Form) -> Result<Response, AppError>
where
  Response: Serializable,
  Form: Serialize,
{
  let client = reqwest::Client::new();

  let json = client
    .get(&build_fetch_query(endpoint(), form))
    .send()
    .await?
    .text()
    .await?;

  // Return the error response json as an error
  Response::de(&json).map_err(|_| AppError::APIError {
    error: json_deser_err(&json),
  })
}

fn build_fetch_query<T: Serialize>(path: String, form: T) -> String {
  let form_str = serde_urlencoded::to_string(&form).unwrap_or_default();
  format!("{path}/service/search?{form_str}")
}
