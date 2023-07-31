use chrono::NaiveDateTime;
use format_num::format_num;
use serde::{Deserialize, Serialize};

pub const REPO_URL: &str = "https://git.torrents-csv.ml/heretic/torrents-csv-server";
pub const HERETIC_URL: &str = "https://git.torrents-csv.ml/heretic";
pub const ANDROID_APP_REPO_URL: &str = "https://git.torrents-csv.ml/heretic/torrents-csv-android";

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Torrent {
  pub infohash: String,
  pub name: String,
  pub size_bytes: u64,
  pub created_unix: u32,
  pub seeders: u32,
  pub leechers: u32,
  pub completed: Option<u32>,
  pub scraped_date: u32,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct SearchQuery {
  pub q: String,
  pub page: Option<usize>,
}

pub fn pretty_date(unix: u32) -> String {
  let naive = NaiveDateTime::from_timestamp_opt(unix.into(), 0);
  naive
    .map(|d| d.format("%Y-%m-%d").to_string())
    .unwrap_or("Now".to_string())
}

pub fn pretty_num(num: u32) -> String {
  // Annoying workaround because rust doesn't have pretty num crate
  if num > 100 {
    format_num!(".3s", num)
  } else {
    num.to_string()
  }
}

pub fn magnet_link(torrent: &Torrent) -> String {
  format!(
    "magnet:?xt=urn:btih:{}&dn={}",
    torrent.infohash, torrent.name
  )
}
