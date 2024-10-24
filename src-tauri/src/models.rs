use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveData {
    pub settings: Settings,
    pub division: Division,
    pub assets: Vec<Asset>,
    pub current_match: Match,
}

impl SaveData {
    pub fn new(settings: Settings, division: Division, assets: Vec<Asset>, current_match: Match) -> Self {
        Self {
            settings,
            division,
            assets,
            current_match,
        }
    }

    pub fn default() -> Self {
        Self {
            settings: Settings::default(),
            division: Division::default(),
            assets: Vec::new(),
            current_match: Match::default(),
        }
    }

    pub fn team_names(&self) -> Vec<String> {
        self.division.teams.iter().map(|team| team.name.clone()).collect()
    }

    pub fn assets_hashmap(&self) -> std::collections::HashMap<String, String> {
        self.assets.iter().map(|asset| (asset.name.clone(), asset.path.clone())).collect()
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
            self.current_match.rounds.push(Round::default());
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Settings {
    pub event_name: String,
    pub round_count: usize,
    pub gamemodes: Vec<Gamemode>,
    pub roles: Vec<Role>,
    pub characters: Vec<Character>,
}

impl Settings {
    pub fn new(
        event_name: &str,
        round_count: usize,
        gamemodes: Vec<Gamemode>,
        roles: Vec<Role>,
        characters: Vec<Character>,
    ) -> Self {
        Self {
            event_name: event_name.to_string(),
            round_count,
            gamemodes,
            roles,
            characters,
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new("New Event", 5, Vec::new(), Vec::new(), Vec::new())
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
    pub image: Option<String>,
}

impl Map {
    pub fn new(name: &str, image: Option<String>) -> Self {
        Self {
            name: name.to_string(),
            image,
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
    pub image: Option<String>,
}

impl Character {
    pub fn new(name: &str, image: Option<String>) -> Self {
        Self {
            name: name.to_string(),
            image,
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
    pub teams: Vec<Team>,
    pub bracket: Vec<Vec<Matchup>>,
}

impl Division {
    pub fn new(
        teams: Vec<Team>,
        bracket: Option<Vec<Vec<Matchup>>>
    ) -> Self {
        let bracket = match bracket {
            Some(bracket) => bracket,
            None => vec![
                vec![Matchup::default(); 4],
                vec![Matchup::default(); 2],
                vec![Matchup::default()],
            ],
        };
        Self {
            teams,
            bracket,
        }
    }
}

impl Default for Division {
    fn default() -> Self {
        Self::new(Vec::new(), None)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Matchup {
    pub team1: Option<usize>,
    pub team2: Option<usize>,
    pub team1_score: usize,
    pub team2_score: usize,
    pub winner: Option<usize>,
}

impl Matchup {
    pub fn new(
        team1: Option<usize>,
        team2: Option<usize>,
        team1_score: usize,
        team2_score: usize,
        winner: Option<usize>
    ) -> Self {
        Self {
            team1,
            team2,
            team1_score,
            team2_score,
            winner,
        }
    }

    pub fn is_filled(&self) -> bool {
        self.team1.is_some() && self.team2.is_some()
    }
}

impl Default for Matchup {
    fn default() -> Self {
        Self::new(None, None, 0, 0, None)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Match {
    pub rounds: Vec<Round>,
    pub team1: Option<usize>,
    pub team2: Option<usize>,
    pub swap_scoreboard: bool,
}

impl Match {
    pub fn new(
        rounds: Vec<Round>,
        team1: Option<usize>,
        team2: Option<usize>,
        swap_scoreboard: bool,
    ) -> Self {
        Self {
            rounds,
            team1,
            team2,
            swap_scoreboard,
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
        Self::new(Vec::new(), None, None, false)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Round {
    pub gamemode: Option<usize>,
    pub map: Option<usize>,
    pub team1_score: usize,
    pub team2_score: usize,
    pub completed: bool,
}

impl Round {
    pub fn new(
        gamemode: Option<usize>,
        map: Option<usize>,
        team1_score: usize,
        team2_score: usize,
        completed: bool
    ) -> Self {
        Self {
            gamemode,
            map,
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
    pub role: Option<usize>,
    pub character: Option<usize>,
}

impl Player {
    pub fn new(name: &str, role: Option<usize>, character: Option<usize>) -> Self {
        Self {
            name: name.to_string(),
            role,
            character,
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new("New Player", None, None)
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
}

impl Default for Team {
    fn default() -> Self {
        Self::new("New Team", None, Vec::new())
    }
}
