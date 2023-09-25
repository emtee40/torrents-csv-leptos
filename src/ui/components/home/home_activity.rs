use crate::utils::{ANDROID_APP_REPO_URL, REPO_URL};
use leptos::*;

// This is helpful:
// https://github.com/leptos-rs/leptos/blob/main/examples/hackernews/src/routes/stories.rs

#[component]
pub fn HomeActivity() -> impl IntoView {
  view! {
    <main class="container mx-auto mt-6 px-4">
      <IntroText/>
    </main>
  }
}

#[component]
fn IntroText() -> impl IntoView {
  view! {
    <div class="flex flex-col space-y-4 text-xl">
      <p>
        <a href=REPO_URL class="link link-primary">
          "Torrents.csv"
        </a>
        " is a "
        <i>"collaborative"</i>
        " git repository of torrents, consisting of a single, searchable "
        <code>"torrents.csv"</code>
        " file. Its initially populated with a January
        2017 backup of the pirate bay, and new torrents are periodically added
        from various torrents sites. It comes with a self-hostable webserver,
        a command line search, and a folder scanner to add torrents."
      </p>
      <p>
        <a href=REPO_URL class="link link-primary">
          "Torrents.csv"
        </a>
        " will only store torrents with at
        least one seeder to keep the file small, will be periodically purged of
        non-seeded torrents, and sorted by seeders descending."
      </p>
      <p>
        <a href=ANDROID_APP_REPO_URL class="link link-primary">
          "Torrents-csv-android app"
        </a>
      </p>
      <p>
        "API: "
        <code>
          "https://torrents-csv.ml/service/search?q=[QUERY]&size=[NUMBER_OF_RESULTS]&page=[PAGE]"
        </code>
      </p>
      <p>
        "To request more torrents, or add your own, go " <a href=REPO_URL class="link link-primary">
          "here"
        </a>
      </p>
    </div>
  }
}
