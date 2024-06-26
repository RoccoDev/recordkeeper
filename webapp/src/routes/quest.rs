use crate::components::page::{PageControls, PageOrganizer};
use crate::components::quest::QuestRow;
use crate::data::Data;
use crate::lang::Text;
use crate::save::SaveContext;
use game_data::IdInt;
use ybc::{Table, Tile};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct TableProps {
    pub start: IdInt,
    pub end: IdInt,
}

const PAGES_PER_VIEW: usize = 2;
const ROWS_PER_PAGE: usize = 12;

#[function_component]
pub fn Quests() -> Html {
    let save = use_context::<SaveContext>().unwrap();
    let data = use_context::<Data>().unwrap();

    let quests = &data.game().quests;
    let is_dlc4 = save.get().get_save().is_dlc4();
    let start = quests.start(is_dlc4);
    let end = quests.end(is_dlc4);

    let page = use_state(|| 0);
    let page_organizer =
        PageOrganizer::<PAGES_PER_VIEW>::new(ROWS_PER_PAGE, *page, (end + 1 - start) as usize);

    html! {
        <>
            <Tile classes="mb-2">
                {for page_organizer.current_bounds.into_iter().map(|(s, e)| html! {
                    <Tile>
                        <TablePage start={start + s as u32} end={start + e as u32} />
                    </Tile>
                })}
            </Tile>

            <PageControls<PAGES_PER_VIEW> organizer={page_organizer} state={page} />
        </>
    }
}

#[function_component]
fn TablePage(props: &TableProps) -> Html {
    html! {
        <Table classes={classes!("is-fullwidth")}>
            <thead>
                <tr>
                    <th><Text path="quest_id" /></th>
                    <th><Text path="quest_name" /></th>
                    <th><Text path="quest_status" /></th>
                    <th><Text path="quest_actions" /></th>
                </tr>
            </thead>

            <tbody>
                {for (props.start..=props.end).map(|index| {
                    html!(<QuestRow id={index} />)
                })}
            </tbody>
        </Table>
    }
}
