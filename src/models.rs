use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Settings {
    pub round_count: usize,
    pub gamemodes: Vec<Gamemode>,
    pub roles: Vec<Role>,
    pub characters: Vec<Character>,
}

impl Settings {
    pub fn new(
        round_count: usize,
        gamemodes: Vec<Gamemode>,
        roles: Vec<Role>,
        characters: Vec<Character>,
    ) -> Self {
        Self {
            round_count,
            gamemodes,
            roles,
            characters,
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new(5, Vec::new(), Vec::new(), Vec::new())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Gamemode {
    pub name: String,
    pub icon: Option<String>,
    pub maps: Vec<Map>,
}

impl Gamemode {
    pub fn new(name: &str, icon: Option<String>, maps: Vec<Map>) -> Self {
        Self {
            name: name.to_string(),
            icon,
            maps,
        }
    }
}

impl Default for Gamemode {
    fn default() -> Self {
        Self::new("New Gamemode", None, Vec::new())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Map {
    pub name: String,
    pub icon: Option<String>,
}

impl Map {
    pub fn new(name: &str, icon: Option<String>) -> Self {
        Self {
            name: name.to_string(),
            icon,
        }
    }
}

impl Default for Map {
    fn default() -> Self {
        Self::new("New Map", None)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Role {
    pub name: String,
    pub icon: Option<String>,
}

impl Role {
    pub fn new(name: &str, icon: Option<String>) -> Self {
        Self {
            name: name.to_string(),
            icon,
        }
    }
}

impl Default for Role {
    fn default() -> Self {
        Self::new("New Role", None)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Character {
    pub name: String,
    pub icon: Option<String>,
}

impl Character {
    pub fn new(name: &str, icon: Option<String>) -> Self {
        Self {
            name: name.to_string(),
            icon,
        }
    }
}

impl Default for Character {
    fn default() -> Self {
        Self::new("New Character", None)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Division {
    pub name: String,
    pub teams: Vec<Team>,
    pub bracket: Vec<Vec<Option<usize>>>,
}

impl Division {
    pub fn new(
        name: &str,
        teams: Vec<Team>,
        bracket: Option<Vec<Vec<Option<usize>>>>
    ) -> Self {
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

impl Default for Division {
    fn default() -> Self {
        Self::new("New Division", Vec::new(), None)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Match {
    pub rounds: Vec<Round>,
    pub team1: Option<usize>,
    pub team2: Option<usize>,
}

impl Match {
    pub fn new(
        rounds: Vec<Round>,
        team1: Option<usize>,
        team2: Option<usize>,
    ) -> Self {
        Self {
            rounds,
            team1,
            team2,
        }
    }

    pub fn team1_score(&self) -> usize {
        self.rounds.iter().filter(|round| round.team1_score > round.team2_score).count()
    }

    pub fn team2_score(&self) -> usize {
        self.rounds.iter().filter(|round| round.team2_score > round.team1_score).count()
    }
}

impl Default for Match {
    fn default() -> Self {
        Self::new(Vec::new(), None, None)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Round {
    pub gamemode: Option<String>,
    pub map: Option<String>,
    pub team1_score: usize,
    pub team2_score: usize,
    pub completed: bool,
}

impl Round {
    pub fn new(
        gamemode: Option<&str>,
        map: Option<&str>,
        team1_score: usize,
        team2_score: usize,
        completed: bool
    ) -> Self {
        Self {
            gamemode: gamemode.map(|gamemode| gamemode.to_string()),
            map: map.map(|map| map.to_string()),
            team1_score,
            team2_score,
            completed,
        }
    }
}

impl Default for Round {
    fn default() -> Self {
        Self::new(None, None, 0, 0, false)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Player {
    pub name: String,
    pub role: String,
    pub character: String,
}

impl Player {
    pub fn new(name: &str, role: &str, character: &str) -> Self {
        Self {
            name: name.to_string(),
            role: role.to_string(),
            character: character.to_string(),
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new("New Player", "(none)", "(none)")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Team {
    pub name: String,
    pub icon: Option<String>,
    pub players: Vec<Player>,
}

impl Team {
    pub fn new(name: &str, icon: Option<String>, players: Vec<Player>) -> Self {
        Self {
            name: name.to_string(),
            icon,
            players,
        }
    }

    pub fn player_info(&self) -> Vec<Vec<String>> {
        self.players
            .iter()
            .map(|player| vec![player.name.clone(), player.role.clone(), player.character.clone()])
            .collect()
    }
}

impl Default for Team {
    fn default() -> Self {
        Self::new("New Team", None, Vec::new())
    }
}
