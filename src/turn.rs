use bevy::prelude::*;

use crate::faction::FactionResources;
use crate::state::AppState;

/// ターン進行リクエストを通知するメッセージ（イベント）
#[derive(Message, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AdvanceTurnMessage;

/// ターン進行およびゲームループ管理用プラグイン
pub struct TurnPlugin;

impl Plugin for TurnPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<AdvanceTurnMessage>().add_systems(
            Update,
            handle_turn_advance_messages.run_if(in_state(AppState::InGame)),
        );
    }
}

/// ターン進行メッセージを受信し、各リソース計算やターン加算を実行するシステム
fn handle_turn_advance_messages(
    mut messages: MessageReader<AdvanceTurnMessage>,
    mut resources: ResMut<FactionResources>,
) {
    for _ in messages.read() {
        advance_turn(&mut resources);
    }
}

/// ターン進行の基本処理（ターン数インクリメントおよび各種毎ターン資源産出の加算）
pub fn advance_turn(resources: &mut FactionResources) {
    resources.turn += 1;
    resources.energy += resources.energy_per_turn;
    resources.production += resources.production_per_turn;
    resources.science += resources.science_per_turn;
    resources.food += resources.food_per_turn;
    info!("Advancing to Turn {}", resources.turn);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advance_turn() {
        let mut resources = FactionResources::default();
        let initial_turn = resources.turn;
        let initial_energy = resources.energy;

        advance_turn(&mut resources);

        assert_eq!(resources.turn, initial_turn + 1);
        assert_eq!(resources.energy, initial_energy + resources.energy_per_turn);
    }
}
