use crate::{ui::components::common::Spinner, utils::HERETIC_URL};
use leptos::*;
use leptos_icons::*;
use leptos_router::*;

#[component]
pub fn TopNav() -> impl IntoView {
  let is_routing = use_context::<ReadSignal<bool>>().expect("Missing is_routing");

  view! {
    <div class="navbar bg-base-100 shadow-xl">
      <div class="flex-1">
        <A href="" class="btn btn-ghost normal-case text-xl">
          <Icon icon=Icon::from(HiIcon::HiCircleStackOutlineLg) class="w-8 h-8 mr-1"/>
          <span>"Torrents-CSV"</span>
        </A>
      </div>
      <div class="flex-none gap-2">
        <Form method="GET" action="search">
          <div class="form-control">
            <div class="join">

              <input
                type="text"
                name="q"
                placeholder="Search..."
                class="input input-bordered w-24 md:w-auto join-item"
              />
              <Show
                when=is_routing
                fallback=|| {
                    view! {
                      <button type="submit" class="btn btn-square join-item">
                        <Icon
                          icon=Icon::from(HiIcon::HiMagnifyingGlassOutlineLg)
                          class="w-5 h-5 join-item"
                        />
                      </button>
                    }
                }
              >

                <button disabled class="btn btn-square join-item">
                  <Spinner/>
                </button>
              </Show>
            </div>
          </div>
        </Form>
      </div>
    </div>
  }
}

#[component]
pub fn Footer() -> impl IntoView {
  view! {
    <footer class="sticky top-[100vh] footer mt-10 p-10 bg-neutral text-neutral-content">
      <p class="inline-block text-xl">
        <b>"Torrents-csv"</b>
        " by "
        <a href=HERETIC_URL class="link">
          "Heretic"
        </a>
        ". Made with "
        <a href="https://www.rust-lang.org" class="link">
          "Rust"
        </a>
        ", "
        <a href="https://actix.rs" class="link">
          "Actix"
        </a>
        ", and "
        <a href="https://leptos.dev" class="link">
          "Leptos"
        </a>
        "."
      </p>
    </footer>
  }
}
