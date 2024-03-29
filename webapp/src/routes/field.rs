use strum::{EnumIter, IntoEnumIterator};
use ybc::{Tabs, Tile};
use yew::prelude::*;

use crate::{
    components::field::{
        env::Environment,
        location::LocationsPage,
        player::{FieldStats, PlayerLoc, ShipLoc},
    },
    lang::Text,
};

#[derive(Clone, PartialEq, Copy, EnumIter)]
pub enum FieldTab {
    Player,
    Locations,
    Colonies,
    Maps,
}

#[function_component]
pub fn FieldPage() -> Html {
    let tab = use_state(|| FieldTab::Player);

    let update_tab = |t| {
        let tab_state = tab.clone();
        Callback::from(move |_: MouseEvent| {
            tab_state.set(t);
        })
    };

    html! {
        <>
            <Tabs classes={classes!("is-toggle", "is-centered")}>
                {for FieldTab::iter().map(|t| {
                    let classes = if t == *tab { classes!("is-active") } else { classes!() };
                    html!(<li class={classes}><a onclick={update_tab(t)}><Text path={t.lang()} /></a></li>)
                })}
            </Tabs>

            {match *tab {
                FieldTab::Player => html!(<TabPlayer />),
                FieldTab::Locations => html!(<LocationsPage />),
                FieldTab::Colonies => html!(),
                FieldTab::Maps => html!(),
            }}
        </>
    }
}

#[function_component]
fn TabPlayer() -> Html {
    html! {
        <>
            <Tile>
                <Tile classes={classes!("is-parent")}>
                    <PlayerLoc />
                </Tile>
                <Tile classes={classes!("is-parent")}>
                    <ShipLoc />
                </Tile>
            </Tile>
            <Tile>
                <Tile classes={classes!("is-parent")}>
                    <FieldStats />
                </Tile>
                <Tile classes={classes!("is-parent")}>
                    <Environment />
                </Tile>
            </Tile>
        </>
    }
}

impl FieldTab {
    fn lang(&self) -> String {
        let id = match self {
            FieldTab::Player => "player",
            FieldTab::Locations => "locations",
            FieldTab::Colonies => "colonies",
            FieldTab::Maps => "maps",
        };
        format!("field_tab_{id}")
    }
}
