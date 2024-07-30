use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub name: String,
    pub path: String,
}

impl Asset {
    pub fn new(name: &str, path: &str) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Division {
    pub name: String,
    pub teams: Vec<Team>,
    pub bracket: Vec<Vec<Option<usize>>>,
}

impl Division {
    pub fn new(name: &str, teams: Vec<Team>, bracket: Option<Vec<Vec<Option<usize>>>>) -> Self {
        let bracket = match bracket {
            Some(bracket) => bracket,
            None => vec![
                vec![None, None, None, None, None, None, None, None],
                vec![None, None, None, None],
                vec![None, None],
                vec![None],
            ],
        };
        Self {
            name: name.to_string(),
            teams,
            bracket,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub role: String,
    pub hero: String,
}

impl Player {
    pub fn new(name: &str, role: &str, hero: &str) -> Self {
        Self {
            name: name.to_string(),
            role: role.to_string(),
            hero: hero.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    pub name: String,
    pub players: Vec<Player>,
}

impl Team {
    pub fn new(name: &str, players: Vec<Player>) -> Self {
        Self {
            name: name.to_string(),
            players,
        }
    }
}
