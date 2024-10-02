use std::sync::{Arc, Mutex, OnceLock};
use tokio::runtime::Runtime;
use serde::{Deserialize, Serialize};

pub mod webserver;
pub mod ui;
pub mod models;
pub mod fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveData {
    pub settings: models::Settings,
    pub division: models::Division,
    pub assets: Vec<models::Asset>,
    pub current_match: models::Match,
}

impl SaveData {
    pub fn new(settings: models::Settings, division: models::Division, assets: Vec<models::Asset>, current_match: models::Match) -> Self {
        Self {
            settings,
            division,
            assets,
            current_match,
        }
    }

    pub fn default() -> Self {
        Self {
            settings: models::Settings::default(),
            division: models::Division::default(),
            assets: Vec::new(),
            current_match: models::Match::default(),
        }
    }

    pub fn team_names(&self) -> Vec<String> {
        self.division.teams.iter().map(|team| team.name.clone()).collect()
    }

    pub fn assets_hashmap(&self) -> std::collections::HashMap<String, String> {
        self.assets.iter().map(|asset| (asset.name.clone(), asset.path.clone())).collect()
    }

    pub fn roles_hashmap(&self) -> std::collections::HashMap<String, Option<String>> {
        self.settings.roles.iter().map(|role| (role.name.clone(), role.icon.clone())).collect()
    }

    pub fn characters_hashmap(&self) -> std::collections::HashMap<String, Option<String>> {
        self.settings.characters.iter().map(|character| (character.name.clone(), character.icon.clone())).collect()
    }

    pub fn gamemodes_hashmap(&self) -> std::collections::HashMap<String, Option<String>> {
        self.settings.gamemodes.iter().map(|gamemode| (gamemode.name.clone(), gamemode.icon.clone())).collect()
    }

    pub fn bracket_stage_count(&self) -> usize {
        let mut stage_count: usize = 3;
        for stage in &self.division.bracket {
            if stage.iter().any(|matchup| matchup.is_filled()) {
                break;
            }
            stage_count -= 1;
        }
        stage_count
    }

    pub fn bracket_visibilities(&self) -> Vec<Vec<bool>> {
        let mut visibilities = Vec::new();
        for stage in &self.division.bracket {
            let mut stage_visibilities = Vec::new();
            for matchup in stage {
                stage_visibilities.push(matchup.is_filled());
            }
            visibilities.push(stage_visibilities);
        }
        visibilities
    }

    pub fn correct_rounds_to_count(&mut self) {
        while self.settings.round_count < self.current_match.rounds.len() {
            self.current_match.rounds.pop();
        }
        while self.settings.round_count > self.current_match.rounds.len() {
            self.current_match.rounds.push(models::Round::default());
        }
    }
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub loaded_config: Option<std::path::PathBuf>,
    pub data: SaveData,
}

type SharedState = Arc<Mutex<AppState>>;

impl AppState {
    pub fn new(
        loaded_config: Option<std::path::PathBuf>,
        data: SaveData,
    ) -> Self {
        Self {
            loaded_config,
            data,
        }
    }

    pub fn new_shared(
        loaded_config: Option<std::path::PathBuf>,
        data: SaveData,
    ) -> SharedState {
        Arc::new(Mutex::new(Self::new(loaded_config, data)))
    }

    pub fn get_base_path(&self) -> Option<std::path::PathBuf> {
        self.loaded_config.as_ref()
            .map(|path| fs::remove_file_from_path(path))
    }
}

pub fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| {
        Runtime::new().expect("Setting up tokio runtime needs to succeed.")
    })
}
