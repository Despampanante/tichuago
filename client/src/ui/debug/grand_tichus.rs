use crate::global::state::AppContext;
use tichuago_common::{PublicGameStage, TichuCallStatus};
use yew::prelude::*;

#[function_component(AllGrandTichus)]
pub fn all_grand_tichu() -> Html {
    let app_context = use_context::<AppContext>().expect("AppContext not found");
    let app_state = &*app_context.app_reducer_handle;

    match &app_state.game_state {
        Some(game_state) => html! {{
            for game_state
                .participants
                .iter()
                .map(|user| html!{
                  <UserGrandTichu
                    user_id={user.user_id.clone()}
                    display_name={user.display_name.clone()}
                  />
                })
        }},
        None => html! {<> </>},
    }
}

#[derive(Properties, PartialEq)]
pub struct UserGrandTichuProps {
    user_id: String,
    display_name: String,
}

#[function_component(UserGrandTichu)]
pub fn user_grand_tichu(props: &UserGrandTichuProps) -> Html {
    let app_context = use_context::<AppContext>().expect("AppContext not found");
    let app_state = &*app_context.app_reducer_handle;

    let grand_tichu_call_status = if let Some(game_state) = &app_state.game_state {
        let grand_tichus = match &game_state.stage {
            PublicGameStage::GrandTichu(grand_tichu_state) => &grand_tichu_state.grand_tichus,
            PublicGameStage::Trade(trade) => &trade.grand_tichus,
            PublicGameStage::Play(play) => &play.grand_tichus,
            _ => {
                return html! {
                    <p>{&format!("Grand Tichu Call Status for {}: n/a \n", props.display_name)}</p>
                }
            }
        };

        match grand_tichus.iter().find(|user_id_with_tichu_call_status| {
            *user_id_with_tichu_call_status.user_id == *props.user_id
        }) {
            Some(user_id_with_tichu_call_status) => {
                match user_id_with_tichu_call_status.tichu_call_status {
                    TichuCallStatus::Undecided => "Undecided",
                    TichuCallStatus::Called => "Called",
                    TichuCallStatus::Declined => "Declined",
                    TichuCallStatus::Achieved => "Achieved",
                    TichuCallStatus::Failed => "Failed",
                }
            }
            None => "n/a",
        }
    } else {
        "n/a"
    };
    html! {
            <p>{&format!("Grand Tichu Call Status for {}: {} \n", props.display_name, grand_tichu_call_status)}</p>
    }
}
