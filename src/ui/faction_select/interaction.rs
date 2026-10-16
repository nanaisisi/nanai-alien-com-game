use bevy::prelude::*;
use bevy::ui_widgets::Button;

use super::types::*;
use crate::faction::types::PlayerFaction;
use crate::map::settings::MapConfig;
use crate::state::AppState;
use crate::ui::theme::UiTheme;

pub type FactionSelectInteractionQuery<'world, 'state> = Query<
    'world,
    'state,
    (
        Option<&'static bevy::picking::hover::Hovered>,
        Option<&'static bevy::ui::Pressed>,
        &'static SelectAction,
        &'static mut BorderColor,
        &'static mut BackgroundColor,
    ),
    With<Button>,
>;

pub type FactionSelectActionQuery<'world, 'state> = Query<
    'world,
    'state,
    (Option<&'static bevy::ui::Pressed>, &'static SelectAction),
    With<Button>,
>;

pub type DetailTitleTextQuery<'world, 'state> = Query<
    'world,
    'state,
    (&'static mut Text, &'static mut TextColor),
    (With<DetailTitleText>, Without<DetailDescText>),
>;

pub type DetailDescTextQuery<'world, 'state> =
    Query<'world, 'state, &'static mut Text, (With<DetailDescText>, Without<DetailTitleText>)>;

pub type EnvDescTextQuery<'world, 'state> = Query<
    'world,
    'state,
    &'static mut Text,
    (
        With<EnvDescText>,
        Without<DetailTitleText>,
        Without<DetailDescText>,
        Without<SeedDisplayText>,
    ),
>;

pub type SeedTextQuery<'world, 'state> = Query<
    'world,
    'state,
    &'static mut Text,
    (
        With<SeedDisplayText>,
        Without<DetailTitleText>,
        Without<DetailDescText>,
        Without<EnvDescText>,
    ),
>;

pub type FactionCardsQuery<'world, 'state> = Query<
    'world,
    'state,
    (&'static FactionCard, &'static mut BorderColor, &'static mut BackgroundColor),
    (Without<EnvButton>, Without<SizeButton>),
>;

pub type EnvButtonsQuery<'world, 'state> = Query<
    'world,
    'state,
    (&'static EnvButton, &'static mut BorderColor, &'static mut BackgroundColor),
    (Without<FactionCard>, Without<SizeButton>),
>;

pub type SizeButtonsQuery<'world, 'state> = Query<
    'world,
    'state,
    (&'static SizeButton, &'static mut BorderColor, &'static mut BackgroundColor),
    (Without<FactionCard>, Without<EnvButton>),
>;

pub type DetailPanelBorderQuery<'world, 'state> = Query<
    'world,
    'state,
    &'static mut BorderColor,
    (
        With<DetailPanelRoot>,
        Without<FactionCard>,
        Without<EnvButton>,
        Without<SizeButton>,
    ),
>;

pub fn faction_select_button_system(
    mut interaction_query: FactionSelectInteractionQuery,
    selected_menu: Res<SelectedFactionMenu>,
    map_config: Res<MapConfig>,
) {
    let surfaces = UiTheme::surfaces();

    for (hovered, pressed, action, mut border_color, mut bg_color) in &mut interaction_query {
        let is_hover_or_press = hovered.is_some() || pressed.is_some();
        let is_pressed = pressed.is_some();
        let is_hovered = hovered.is_some();

        match action {
            SelectAction::Choose(f) => {
                let is_active = *f == selected_menu.faction;
                if is_hover_or_press {
                    *border_color = BorderColor::all(f.accent_color());
                    *bg_color = BackgroundColor(Color::srgba(0.20, 0.30, 0.42, 0.9));
                } else if is_active {
                    *border_color = BorderColor::all(f.primary_color());
                    *bg_color = BackgroundColor(Color::srgba(0.15, 0.22, 0.32, 0.9));
                } else {
                    *border_color = BorderColor::all(surfaces.border());
                    *bg_color = BackgroundColor(surfaces.card());
                }
            }
            SelectAction::ChooseEnvironment(env) => {
                let is_active = *env == map_config.environment;
                if is_hover_or_press {
                    *border_color = BorderColor::all(env.theme_color());
                    *bg_color = BackgroundColor(Color::srgba(0.22, 0.32, 0.45, 0.9));
                } else if is_active {
                    *border_color = BorderColor::all(env.theme_color());
                    *bg_color = BackgroundColor(Color::srgba(0.18, 0.26, 0.38, 0.9));
                } else {
                    *border_color = BorderColor::all(surfaces.border());
                    *bg_color = BackgroundColor(surfaces.card());
                }
            }
            SelectAction::ChooseSize(sz) => {
                let is_active = *sz == map_config.size;
                if is_hover_or_press {
                    *border_color = BorderColor::all(surfaces.accent());
                    *bg_color = BackgroundColor(Color::srgba(0.25, 0.40, 0.50, 0.9));
                } else if is_active {
                    *border_color = BorderColor::all(surfaces.accent());
                    *bg_color = BackgroundColor(Color::srgba(0.20, 0.35, 0.45, 0.9));
                } else {
                    *border_color = BorderColor::all(surfaces.border());
                    *bg_color = BackgroundColor(surfaces.card());
                }
            }
            SelectAction::RerollSeed => {
                if is_hover_or_press {
                    *border_color = BorderColor::all(surfaces.accent());
                    *bg_color = BackgroundColor(Color::srgba(0.18, 0.28, 0.38, 0.9));
                } else {
                    *border_color = BorderColor::all(surfaces.border());
                    *bg_color = BackgroundColor(surfaces.card());
                }
            }
            SelectAction::Confirm => {
                if is_pressed {
                    *bg_color = BackgroundColor(Color::srgb(0.25, 0.65, 0.70));
                } else if is_hovered {
                    *bg_color = BackgroundColor(Color::srgb(0.18, 0.48, 0.55));
                } else {
                    *bg_color = BackgroundColor(Color::srgb(0.12, 0.35, 0.40));
                }
            }
            SelectAction::Back => {
                if is_hover_or_press {
                    *border_color = BorderColor::all(surfaces.accent());
                    *bg_color = BackgroundColor(Color::srgb(0.15, 0.24, 0.35));
                } else {
                    *border_color = BorderColor::all(surfaces.border());
                    *bg_color = BackgroundColor(surfaces.card());
                }
            }
        }
    }
}

use bevy::ecs::system::SystemParam;

#[derive(SystemParam)]
pub struct FactionSelectStateParams<'w> {
    pub selected_menu: ResMut<'w, SelectedFactionMenu>,
    pub player_faction: ResMut<'w, PlayerFaction>,
    pub map_config: ResMut<'w, MapConfig>,
    pub next_state: ResMut<'w, NextState<AppState>>,
}

#[derive(SystemParam)]
pub struct FactionSelectUiQueries<'w, 's> {
    pub title_query: DetailTitleTextQuery<'w, 's>,
    pub desc_query: DetailDescTextQuery<'w, 's>,
    pub env_desc_query: EnvDescTextQuery<'w, 's>,
    pub seed_text_query: SeedTextQuery<'w, 's>,
    pub cards_query: FactionCardsQuery<'w, 's>,
    pub env_btn_query: EnvButtonsQuery<'w, 's>,
    pub size_btn_query: SizeButtonsQuery<'w, 's>,
    pub panel_query: DetailPanelBorderQuery<'w, 's>,
}

pub fn faction_select_action_system(
    query: FactionSelectActionQuery,
    mut state: FactionSelectStateParams,
    mut ui: FactionSelectUiQueries,
) {
    let FactionSelectStateParams {
        ref mut selected_menu,
        ref mut player_faction,
        ref mut map_config,
        ref mut next_state,
    } = state;
    for (pressed, action) in &query {
        if pressed.is_none() {
            continue;
        }

        match action {
            SelectAction::Choose(faction) => {
                selected_menu.faction = *faction;

                if let Ok((mut text, mut text_color)) = ui.title_query.single_mut() {
                    **text = format!("{} ({})", faction.formal_title(), faction.name_ja());
                    *text_color = TextColor(faction.accent_color());
                }
                if let Ok(mut text) = ui.desc_query.single_mut() {
                    **text = faction.description().to_string();
                }

                if let Ok(mut panel_border) = ui.panel_query.single_mut() {
                    *panel_border = BorderColor::all(faction.primary_color());
                }

                let surfaces = UiTheme::surfaces();

                // カード枠のスタイル更新
                for (card, mut border_color, mut bg_color) in &mut ui.cards_query {
                    let is_active = card.0 == *faction;
                    if is_active {
                        *border_color = BorderColor::all(card.0.primary_color());
                        *bg_color = BackgroundColor(Color::srgba(0.15, 0.22, 0.32, 0.9));
                    } else {
                        *border_color = BorderColor::all(surfaces.border());
                        *bg_color = BackgroundColor(surfaces.card());
                    }
                }
            }
            SelectAction::ChooseEnvironment(env) => {
                map_config.environment = *env;

                if let Ok(mut text) = ui.env_desc_query.single_mut() {
                    **text = env.description().to_string();
                }

                let surfaces = UiTheme::surfaces();
                for (btn, mut border_color, mut bg_color) in &mut ui.env_btn_query {
                    let is_active = btn.0 == *env;
                    if is_active {
                        *border_color = BorderColor::all(btn.0.theme_color());
                        *bg_color = BackgroundColor(Color::srgba(0.18, 0.26, 0.38, 0.9));
                    } else {
                        *border_color = BorderColor::all(surfaces.border());
                        *bg_color = BackgroundColor(surfaces.card());
                    }
                }
            }
            SelectAction::ChooseSize(sz) => {
                map_config.size = *sz;

                let surfaces = UiTheme::surfaces();
                for (btn, mut border_color, mut bg_color) in &mut ui.size_btn_query {
                    let is_active = btn.0 == *sz;
                    if is_active {
                        *border_color = BorderColor::all(surfaces.accent());
                        *bg_color = BackgroundColor(Color::srgba(0.20, 0.35, 0.45, 0.9));
                    } else {
                        *border_color = BorderColor::all(surfaces.border());
                        *bg_color = BackgroundColor(surfaces.card());
                    }
                }
            }
            SelectAction::RerollSeed => {
                let display_seed = map_config.reroll_seed();

                if let Ok(mut text) = ui.seed_text_query.single_mut() {
                    **text = format!("🎲 SEED: {}", display_seed);
                }
            }
            SelectAction::Confirm => {
                player_faction.0 = selected_menu.faction;
                next_state.set(AppState::InGame);
            }
            SelectAction::Back => {
                next_state.set(AppState::Title);
            }
        }
    }
}
