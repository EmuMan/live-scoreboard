use std::sync::{Arc, Mutex, OnceLock};
use tokio::runtime::Runtime;
use serde::{Deserialize, Serialize};

pub mod webserver;
pub mod ui;
pub mod models;
pub mod fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub settings: models::Settings,
    pub division: models::Division,
    pub assets: Vec<models::Asset>,
    pub current_match: models::Match,
}

type SharedState = Arc<Mutex<AppState>>;

impl AppState {
    pub fn new(
        settings: models::Settings,
        division: models::Division,
        assets: Vec<models::Asset>,
        current_match: models::Match
    ) -> Self {
        Self {
            settings,
            division,
            assets,
            current_match,
        }
    }

    pub fn new_shared(
        settings: models::Settings,
        division: models::Division,
        assets: Vec<models::Asset>,
        current_match: models::Match
    ) -> SharedState {
        Arc::new(Mutex::new(Self::new(settings, division, assets, current_match)))
    }

    pub fn team_names(&self) -> Vec<String> {
        self.division.teams.iter().map(|team| team.name.clone()).collect()
    }

    pub fn assets_hashmap(&self) -> std::collections::HashMap<String, String> {
        self.assets.iter().map(|asset| (asset.name.clone(), asset.path.clone())).collect()
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

pub fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| {
        Runtime::new().expect("Setting up tokio runtime needs to succeed.")
    })
}
