use crate::{
  api::search::search,
  ui::components::common::Spinner,
  utils::{magnet_link, pretty_date, pretty_num, SearchQuery, Torrent},
};
use human_bytes::human_bytes;
use leptos::*;
use leptos_icons::*;
use leptos_meta::Title;
use leptos_router::{use_query_map, A};

#[component]
pub fn SearchActivity() -> impl IntoView {
  let query = use_query_map();
  let q = move || query.with(|q| q.get("q").cloned()).unwrap_or_default();
  let page = move || {
    query
      .with(|q| q.get("page").and_then(|page| page.parse::<usize>().ok()))
      .unwrap_or(1)
  };

  let torrents = create_resource(
    move || (q(), page()),
    move |(q, page)| async move {
      let form = SearchQuery {
        q,
        page: Some(page),
      };
      search(&form).await.ok()
    },
  );

  let err_msg = " Error loading.";

  let title = move || {
    if q().is_empty() {
      "Search".to_string()
    } else {
      q()
    }
  };

  view! {
    <Title text=title/>
    <main class="container mx-auto mt-6 px-4">
      <Suspense fallback=move || {
          view! { <Spinner/> }
      }>
        {move || {
            torrents
                .get()
                .map(|res| match res {
                    None => {
                        view! { <div>{err_msg}</div> }
                    }
                    Some(torrents) => {
                        if !torrents.is_empty() {
                            view! {
                              <div>
                                <TorrentListings torrents=torrents.into()/>
                                <div class="join grid grid-cols-2">
                                  <Show when=move || { page() > 1 } fallback=|| view! { "" }>
                                    <A
                                      class="join-item btn btn-outline"
                                      href=move || { format!("?q={}&page={}", q(), page() - 1) }
                                    >
                                      "Prev"
                                    </A>
                                  </Show>
                                  <A
                                    class="join-item btn btn-outline"
                                    href=move || { format!("?q={}&page={}", q(), page() + 1) }
                                  >
                                    "Next"
                                  </A>
                                </div>
                              </div>
                            }
                        } else {
                            view! { <div class="text-lg">"No results."</div> }
                        }
                    }
                })
        }}

      </Suspense>
    </main>
  }
}

#[component]
fn TorrentListings(torrents: MaybeSignal<Vec<Torrent>>) -> impl IntoView {
  view! {
    <For each=torrents key=|t| t.infohash.clone() let:child>
      <TorrentListing torrent=child/>
    </For>
  }
}

#[component]
fn TorrentListing(torrent: Torrent) -> impl IntoView {
  let magnet = magnet_link(&torrent);
  let bytes = human_bytes(torrent.size_bytes as f64);
  let seeders = pretty_num(torrent.seeders);
  let created = pretty_date(torrent.created_unix);

  let row_c = "inline-flex items-center";
  let icon_c = "w-4 h-4 mr-2";

  view! {
    <div class="card card-compact shadow-xl bg-base-100 mb-3">
      <div class="card-body">
        <a href=magnet class="link card-title break-all">
          {torrent.name}
        </a>
        <div class="flex justify-between text-lg">
          <div class=row_c>
            <Icon icon=Icon::from(HiIcon::HiArrowDownTrayOutlineLg) class=icon_c/>
            <span class="text-primary">{seeders}</span>
          </div>
          <div class=row_c>
            <Icon icon=Icon::from(HiIcon::HiCircleStackOutlineLg) class=icon_c/>
            <span>{bytes}</span>
          </div>
          <div class=row_c>
            <Icon icon=Icon::from(AiIcon::AiCalendarOutlined) class=icon_c/>
            <span>{created}</span>
          </div>
        </div>
      </div>
    </div>
  }
}
