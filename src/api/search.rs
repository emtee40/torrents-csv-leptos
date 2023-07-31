use crate::{
  api::api_wrapper,
  errors::AppError,
  utils::{SearchQuery, Torrent},
};
use leptos::Scope;

pub async fn search(cx: Scope, form: &SearchQuery) -> Result<Vec<Torrent>, AppError> {
  api_wrapper::<Vec<Torrent>, SearchQuery>(cx, form).await
}
