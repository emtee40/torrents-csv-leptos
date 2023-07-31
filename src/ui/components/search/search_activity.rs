use crate::{
  api::search::search,
  utils::{magnet_link, pretty_date, pretty_num, SearchQuery, Torrent},
};
use human_bytes::human_bytes;
use leptos::*;
use leptos_heroicons::size_24::outline::{ArrowDownTray, Calendar, CircleStack};
use leptos_router::use_query_map;

#[component]
pub fn SearchActivity(cx: Scope) -> impl IntoView {
  let query = use_query_map(cx);
  let q = move || query.with(|q| q.get("q").cloned()).unwrap_or_default();
  let page = move || {
    query
      .with(|q| q.get("page").and_then(|page| page.parse::<usize>().ok()))
      .unwrap_or(1)
  };

  let torrents = create_resource(
    cx,
    move || (q(), page()),
    move |(q, page)| async move {
      let form = SearchQuery {
        q,
        page: Some(page),
      };
      search(cx, &form).await.ok()
    },
  );

  let err_msg = " Error loading.";

  view! { cx,
    <main class="container mx-auto mt-6">
      <Suspense fallback=|| {
          view! { cx, "Loading..." }
      }>
        {move || {
            torrents
                .read(cx)
                .map(|res| match res {
                    None => {
                        view! { cx, <div>{err_msg}</div> }
                    }
                    Some(torrents) => {

                        view! { cx,
                          <div>
                            <TorrentListings torrents=torrents.into()/>
                          </div>
                        }
                    }
                })
        }}

      </Suspense>
    </main>
  }
}

#[component]
fn TorrentListings(cx: Scope, torrents: MaybeSignal<Vec<Torrent>>) -> impl IntoView {
  let no_results = torrents().is_empty().then_some("No results.");
  view! { cx,
    <For
      each=torrents
      key=|t| t.infohash.clone()
      view=move |cx, torrent| {
          view! { cx, <TorrentListing torrent=torrent/> }
      }
    />

    <div class="text-lg">{no_results}</div>
  }
}

#[component]
fn TorrentListing(cx: Scope, torrent: Torrent) -> impl IntoView {
  let magnet = magnet_link(&torrent);
  let bytes = human_bytes(torrent.size_bytes as f64);
  let seeders = pretty_num(torrent.seeders);
  let created = pretty_date(torrent.created_unix);

  let row_c = "inline-flex items-center";
  let icon_c = "w-4 h-4 mr-2";

  view! { cx,
    <div class="card card-compact shadow-xl bg-base-100 mb-3">
      <div class="card-body">
        <a href=magnet class="link card-title truncate">
          {torrent.name}
        </a>
        <div class="flex justify-between text-lg">
          <div class=row_c>
            <ArrowDownTray class=icon_c/>
            <span class="text-primary">{seeders}</span>
          </div>
          <div class=row_c>
            <CircleStack class=icon_c/>
            <span>{bytes}</span>
          </div>
          <div class=row_c>
            <Calendar class=icon_c/>
            <span>{created}</span>
          </div>
        </div>
      </div>
    </div>
  }
}
