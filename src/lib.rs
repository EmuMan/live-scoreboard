use std::sync::{Arc, Mutex, OnceLock};
use tokio::runtime::Runtime;
use serde::{Deserialize, Serialize};

pub mod webserver;
pub mod ui;
pub mod models;
pub mod fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub division: models::Division,
    pub assets: Vec<models::Asset>,
    pub current_match: models::Match,
}

type SharedState = Arc<Mutex<AppState>>;

impl AppState {
    pub fn new(division: models::Division, assets: Vec<models::Asset>) -> Self {
        Self {
            division,
            assets,
            current_match: models::Match::new(),
        }
    }

    pub fn new_shared(division: models::Division, assets: Vec<models::Asset>) -> SharedState {
        Arc::new(Mutex::new(Self::new(division, assets)))
    }

    pub fn team_names(&self) -> Vec<String> {
        self.division.teams.iter().map(|team| team.name.clone()).collect()
    }

    pub fn team_names_model(&self) -> gtk::StringList {
        let mut team_names_with_none = vec![""];
        let team_names = self.team_names();
        let mut team_names: Vec<&str> = team_names.iter().map(|team| team.as_str()).collect();
        team_names_with_none.append(&mut team_names);
        gtk::StringList::new(&team_names_with_none)
    }

    pub fn assets_hashmap(&self) -> std::collections::HashMap<String, String> {
        self.assets.iter().map(|asset| (asset.name.clone(), asset.path.clone())).collect()
    }

    pub fn bracket_visibilities(&self) -> Vec<Vec<usize>> {
        let mut visibilities: Vec<Vec<usize>> = Vec::new();
        let mut old_round_visibility: Vec<usize> = Vec::new();
        
        for round in &self.division.bracket {
            let mut round_visibility = Vec::new();
            for (i, team) in round.iter().enumerate() {
                let mut visibility = if team.is_some() { 2 } else { 0 };
                if visibility == 0 &&
                    (*old_round_visibility.get(i * 2).unwrap_or(&0) != 0 ||
                    *old_round_visibility.get(i * 2 + 1).unwrap_or(&0) != 0) {
                    visibility = 1;
                }
                round_visibility.push(visibility);
            }
            visibilities.push(round_visibility.clone());
            old_round_visibility = round_visibility;
        }

        visibilities
    }

    pub fn bracket_first_round(&self) -> usize {
        let mut first_round = 0;
        for round in &self.division.bracket {
            if round.iter().any(|team| team.is_some()) {
                break;
            }
            first_round += 1;
        }
        first_round
    }
}

pub fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| {
        Runtime::new().expect("Setting up tokio runtime needs to succeed.")
    })
}
