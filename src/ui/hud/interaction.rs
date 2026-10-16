use bevy::prelude::*;
use bevy::ui_widgets::Button;

use crate::faction::{FactionId, FactionManager, FactionResources, PlayerFaction};
use crate::state::AppState;
use crate::ui::theme::UiTheme;

use super::types::{END_TURN_HOVER, END_TURN_NORMAL, END_TURN_PRESSED, HudAction, HudLabel};

type ButtonInteractionQuery<'world, 'state> = Query<
    'world,
    'state,
    (
        Option<&'static bevy::picking::hover::Hovered>,
        Option<&'static bevy::ui::Pressed>,
        &'static mut BackgroundColor,
        &'static mut BorderColor,
        &'static HudAction,
    ),
    With<Button>,
>;

pub fn hud_button_interaction_system(mut query: ButtonInteractionQuery) {
    let surfaces = UiTheme::surfaces();
    let standard_btn = UiTheme::button(false);

    for (hovered, pressed, mut bg_color, mut border_color, action) in &mut query {
        let is_pressed = pressed.is_some();
        let is_hovered = hovered.is_some();

        match action {
            HudAction::EndTurn => {
                if is_pressed {
                    *bg_color = BackgroundColor(END_TURN_PRESSED);
                    *border_color = BorderColor::all(Color::WHITE);
                } else if is_hovered {
                    *bg_color = BackgroundColor(END_TURN_HOVER);
                    *border_color = BorderColor::all(Color::srgb(0.5, 0.95, 0.9));
                } else {
                    *bg_color = BackgroundColor(END_TURN_NORMAL);
                    *border_color = BorderColor::all(surfaces.accent());
                }
            }
            HudAction::OpenCity => {
                if is_pressed {
                    *bg_color = BackgroundColor(standard_btn.pressed());
                    *border_color = BorderColor::all(Color::WHITE);
                } else if is_hovered {
                    *bg_color = BackgroundColor(Color::srgb(0.18, 0.38, 0.48));
                    *border_color = BorderColor::all(Color::WHITE);
                } else {
                    *bg_color = BackgroundColor(Color::srgba(0.12, 0.28, 0.38, 0.85));
                    *border_color = BorderColor::all(surfaces.accent());
                }
            }
            HudAction::OpenDiplomacy => {
                if is_pressed {
                    *bg_color = BackgroundColor(standard_btn.pressed());
                    *border_color = BorderColor::all(Color::WHITE);
                } else if is_hovered {
                    *bg_color = BackgroundColor(Color::srgb(0.16, 0.35, 0.44));
                    *border_color = BorderColor::all(Color::WHITE);
                } else {
                    *bg_color = BackgroundColor(Color::srgba(0.10, 0.25, 0.32, 0.85));
                    *border_color = BorderColor::all(surfaces.accent());
                }
            }
            HudAction::OpenMenu => {
                if is_pressed {
                    *bg_color = BackgroundColor(standard_btn.pressed());
                    *border_color = BorderColor::all(surfaces.accent());
                } else if is_hovered {
                    *bg_color = BackgroundColor(standard_btn.hovered());
                    *border_color = BorderColor::all(surfaces.accent());
                } else {
                    *bg_color = BackgroundColor(standard_btn.normal());
                    *border_color = BorderColor::all(surfaces.border());
                }
            }
        }
    }
}

type ButtonActionQuery<'world, 'state> =
    Query<'world, 'state, (Option<&'static bevy::ui::Pressed>, &'static HudAction), With<Button>>;

use bevy::ecs::system::SystemParam;

#[derive(SystemParam)]
pub struct HudActionContext<'w, 's> {
    pub resources: Res<'w, FactionResources>,
    pub settings: ResMut<'w, crate::ui::settings::GameSettings>,
    pub next_state: ResMut<'w, NextState<AppState>>,
    pub commands: Commands<'w, 's>,
    pub asset_server: Res<'w, AssetServer>,
    pub player_faction: Res<'w, PlayerFaction>,
    pub faction_mgr: Res<'w, FactionManager>,
    pub modal_state: ResMut<'w, crate::ui::diplomacy::DiplomacyModalState>,
    pub city_modal_state: ResMut<'w, crate::ui::city::CityModalState>,
    pub outposts_query: Query<'w, 's, (Entity, &'static crate::faction::FactionOutpost)>,
    pub selected_tile: Res<'w, crate::map::interaction::SelectedTile>,
    pub turn_messages: MessageWriter<'w, crate::turn::AdvanceTurnMessage>,
}

pub fn hud_button_action_system(query: ButtonActionQuery, ctx: HudActionContext) {
    let HudActionContext {
        resources,
        mut settings,
        mut next_state,
        mut commands,
        asset_server,
        player_faction,
        faction_mgr,
        mut modal_state,
        mut city_modal_state,
        outposts_query,
        selected_tile,
        mut turn_messages,
    } = ctx;
    for (pressed, action) in &query {
        if pressed.is_some() {
            match action {
                HudAction::EndTurn => {
                    turn_messages.write(crate::turn::AdvanceTurnMessage);
                }
                HudAction::OpenCity => {
                    city_modal_state.is_open = !city_modal_state.is_open;
                    if city_modal_state.is_open {
                        let mut target_entity = None;
                        if let Some(coord) = selected_tile.0
                            && let Some((e, _)) = outposts_query
                                .iter()
                                .find(|(_, o)| o.coord == coord && o.faction == player_faction.0)
                        {
                            target_entity = Some(e);
                        }
                        if target_entity.is_none()
                            && let Some((e, _)) = outposts_query
                                .iter()
                                .find(|(_, o)| o.faction == player_faction.0)
                        {
                            target_entity = Some(e);
                        }
                        city_modal_state.target_outpost_entity = target_entity;
                        if let Some(target_e) = target_entity
                            && let Ok((_, outpost)) = outposts_query.get(target_e)
                        {
                            crate::ui::city::spawn_city_modal(
                                &mut commands,
                                &asset_server,
                                outpost,
                                &resources,
                            );
                        } else {
                            city_modal_state.is_open = false;
                        }
                    }
                }
                HudAction::OpenDiplomacy => {
                    modal_state.is_open = !modal_state.is_open;
                    if modal_state.is_open {
                        if modal_state.selected_target.is_none() {
                            modal_state.selected_target = FactionId::ALL
                                .iter()
                                .copied()
                                .find(|&f| f != player_faction.0);
                        }
                        crate::ui::diplomacy::spawn_diplomacy_modal(
                            &mut commands,
                            &asset_server,
                            player_faction.0,
                            &modal_state,
                            &faction_mgr,
                        );
                    }
                }
                HudAction::OpenMenu => {
                    info!("Opening Pause Menu...");
                    settings.return_state = AppState::InGame;
                    next_state.set(AppState::PauseMenu);
                }
            }
        }
    }
}

pub fn handle_keyboard_shortcuts(
    mut action_events: MessageReader<crate::map::input::InGameActionEvent>,
    mut settings: ResMut<crate::ui::settings::GameSettings>,
    mut next_state: ResMut<NextState<AppState>>,
    mut turn_messages: MessageWriter<crate::turn::AdvanceTurnMessage>,
) {
    for event in action_events.read() {
        match event.0 {
            crate::map::input::InGameAction::ForceEndTurn => {
                info!("Shift+Enter: Forced Turn Advance");
                turn_messages.write(crate::turn::AdvanceTurnMessage);
            }
            crate::map::input::InGameAction::EndTurn => {
                turn_messages.write(crate::turn::AdvanceTurnMessage);
            }
            crate::map::input::InGameAction::OpenPauseMenu => {
                info!("ESC pressed: Opening Pause Menu...");
                settings.return_state = AppState::InGame;
                next_state.set(AppState::PauseMenu);
            }
            _ => {}
        }
    }
}

pub fn update_hud_display_system(
    resources: Res<FactionResources>,
    mut query: Query<(&mut Text, &HudLabel)>,
) {
    if !resources.is_changed() {
        return;
    }

    for (mut text, label) in &mut query {
        match label {
            HudLabel::Turn => {
                **text = format!("TURN {:02}", resources.turn);
            }
            HudLabel::Energy => {
                **text = format!("{} (+{})", resources.energy, resources.energy_per_turn);
            }
            HudLabel::Production => {
                **text = format!(
                    "{} (+{})",
                    resources.production, resources.production_per_turn
                );
            }
            HudLabel::Science => {
                **text = format!("{} (+{})", resources.science, resources.science_per_turn);
            }
            HudLabel::Food => {
                **text = format!("{} (+{})", resources.food, resources.food_per_turn);
            }
        }
    }
}
