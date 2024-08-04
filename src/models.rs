use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub round_count: usize,
    pub gamemodes: Vec<String>,
    pub maps: Vec<String>,
    pub roles: Vec<String>,
    pub heroes: Vec<String>,
}

impl Settings {
    pub fn new(
        round_count: usize,
        gamemodes: Vec<String>,
        maps: Vec<String>,
        roles: Vec<String>,
        heroes: Vec<String>,
    ) -> Self {
        Self {
            round_count,
            gamemodes,
            maps,
            roles,
            heroes,
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new(5, Vec::new(), Vec::new(), Vec::new(), Vec::new())
    }
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Match {
    pub rounds: Vec<Round>,
    pub team1: Option<usize>,
    pub team2: Option<usize>,
    pub score1: usize,
    pub score2: usize,
    pub winner: Option<usize>,
}

impl Match {
    pub fn new(
        rounds: Vec<Round>,
        team1: Option<usize>,
        team2: Option<usize>,
        score1: usize,
        score2: usize,
        winner: Option<usize>
    ) -> Self {
        Self {
            rounds,
            team1,
            team2,
            score1,
            score2,
            winner,
        }
    }
}

impl Default for Match {
    fn default() -> Self {
        Self::new(Vec::new(), None, None, 0, 0, None)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl Default for Player {
    fn default() -> Self {
        Self::new("New Player", "(none)", "(none)")
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

    pub fn player_info(&self) -> Vec<Vec<String>> {
        self.players
            .iter()
            .map(|player| vec![player.name.clone(), player.role.clone(), player.hero.clone()])
            .collect()
    }
}

impl Default for Team {
    fn default() -> Self {
        Self::new("New Team", Vec::new())
    }
}
