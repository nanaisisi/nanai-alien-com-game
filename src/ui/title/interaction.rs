use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::ui_widgets::Button;

use super::types::*;
use crate::state::AppState;
use crate::ui::theme::UiTheme;

pub fn title_keyboard_navigation_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut focus: ResMut<TitleMenuFocus>,
    mut next_state: ResMut<NextState<AppState>>,
    mut settings: ResMut<crate::ui::settings::GameSettings>,
    mut exit_events: bevy::ecs::message::MessageWriter<AppExit>,
    debug_state: Option<Res<crate::ui::debug_console::DebugConsoleState>>,
) {
    if let Some(ref debug) = debug_state
        && (debug.is_open || debug.show_warning_modal)
    {
        return;
    }

    let count = MENU_ACTIONS.len();
    let current = focus.selected_index.unwrap_or(0);

    if keys.just_pressed(KeyCode::ArrowUp) || keys.just_pressed(KeyCode::KeyW) {
        focus.selected_index = Some(if current == 0 { count - 1 } else { current - 1 });
    } else if keys.just_pressed(KeyCode::ArrowDown) || keys.just_pressed(KeyCode::KeyS) {
        focus.selected_index = Some((current + 1) % count);
    }

    if (keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::Space))
        && let Some(selected) = focus.selected_index
        && selected < count
    {
        execute_title_action(
            MENU_ACTIONS[selected],
            &mut settings,
            &mut next_state,
            &mut exit_events,
        );
    }
}

pub type TitleButtonInteractionQuery<'world, 'state> = Query<
    'world,
    'state,
    (
        &'static Interaction,
        &'static TitleMenuButton,
        &'static MenuButtonAction,
    ),
    With<Button>,
>;

pub fn button_interaction_system(
    interaction_query: TitleButtonInteractionQuery,
    mut focus: ResMut<TitleMenuFocus>,
) {
    // マウスホバーされた場合はフォーカスインデックスを更新
    for (interaction, menu_btn, _) in &interaction_query {
        if *interaction == Interaction::Hovered || *interaction == Interaction::Pressed {
            focus.selected_index = Some(menu_btn.0);
        }
    }
}

/// フォーカス状態（キーボード選択・マウスホバー）に応じてボタンの枠線と背景色を同期更新
pub fn update_title_focus_highlight_system(
    focus: Res<TitleMenuFocus>,
    mut query: Query<(&TitleMenuButton, &mut BorderColor, &mut BackgroundColor), With<Button>>,
) {
    let standard_btn = UiTheme::button(false);
    let surfaces = UiTheme::surfaces();

    for (menu_btn, mut border_color, mut bg_color) in &mut query {
        let is_selected = focus.selected_index == Some(menu_btn.0);

        if is_selected {
            *bg_color = BackgroundColor(standard_btn.hovered());
            *border_color = BorderColor::all(surfaces.accent());
        } else {
            *bg_color = BackgroundColor(standard_btn.normal());
            *border_color = BorderColor::all(Color::srgb(0.25, 0.38, 0.50));
        }
    }
}

pub fn execute_title_action(
    action: MenuButtonAction,
    settings: &mut ResMut<crate::ui::settings::GameSettings>,
    next_state: &mut ResMut<NextState<AppState>>,
    exit_events: &mut bevy::ecs::message::MessageWriter<AppExit>,
) {
    match action {
        MenuButtonAction::NewGame => {
            info!("Transitioning to Faction Selection...");
            next_state.set(AppState::FactionSelect);
        }
        MenuButtonAction::LoadGame => {
            info!("Load Game clicked (WIP)");
        }
        MenuButtonAction::Settings => {
            info!("Transitioning to Settings...");
            settings.return_state = AppState::Title;
            next_state.set(AppState::Settings);
        }
        MenuButtonAction::Exit => {
            info!("Exiting Game...");
            exit_events.write(AppExit::Success);
        }
    }
}

pub fn button_action_system(
    interaction_query: TitleButtonInteractionQuery,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut settings: ResMut<crate::ui::settings::GameSettings>,
    mut next_state: ResMut<NextState<AppState>>,
    mut exit_events: bevy::ecs::message::MessageWriter<AppExit>,
    debug_state: Option<Res<crate::ui::debug_console::DebugConsoleState>>,
) {
    if let Some(ref debug) = debug_state
        && (debug.is_open || debug.show_warning_modal)
    {
        return;
    }

    let mut triggered_action = None;

    // 1. Interaction::Pressed による押下検知
    for (interaction, _, action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            triggered_action = Some(*action);
            break;
        }
    }

    // 2. マウスクリック時にホバー中のボタンがあれば実行
    if triggered_action.is_none() && mouse_button.just_pressed(MouseButton::Left) {
        for (interaction, _, action) in &interaction_query {
            if *interaction == Interaction::Hovered {
                triggered_action = Some(*action);
                break;
            }
        }
    }

    if let Some(action) = triggered_action {
        execute_title_action(action, &mut settings, &mut next_state, &mut exit_events);
    }
}
