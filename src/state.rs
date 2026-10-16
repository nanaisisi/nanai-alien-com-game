use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Title,
    FactionSelect,
    InGame,
    PauseMenu,
    Settings,
}

impl AppState {
    /// コマンドライン引数（例: `--state in-game`, `-s settings` 等）から起動ステートを決定
    pub fn from_args() -> Self {
        let args: Vec<String> = std::env::args().collect();
        for (i, arg) in args.iter().enumerate() {
            if (arg == "--state" || arg == "-s") && i + 1 < args.len() {
                match args[i + 1].to_lowercase().as_str() {
                    "faction-select" | "faction" => return Self::FactionSelect,
                    "in-game" | "ingame" | "game" => return Self::InGame,
                    "pause" | "pausemenu" => return Self::PauseMenu,
                    "settings" | "setting" => return Self::Settings,
                    "title" => return Self::Title,
                    other => {
                        eprintln!(
                            "[WARN] 不明なステート引数: '{other}'. デフォルト (Title) を使用します。"
                        );
                    }
                }
            }
        }
        Self::Title
    }
}
