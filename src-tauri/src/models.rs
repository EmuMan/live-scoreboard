use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveData {
    pub settings: Settings,
    pub division: Division,
    pub resources: Resources,
    pub current_match: Match,
}

impl SaveData {
    pub fn new(settings: Settings, division: Division, resources: Resources, current_match: Match) -> Self {
        Self {
            settings,
            division,
            resources,
            current_match,
        }
    }

    pub fn default() -> Self {
        Self {
            settings: Settings::default(),
            division: Division::default(),
            resources: Resources::default(),
            current_match: Match::default(),
        }
    }

    pub fn team_names(&self) -> Vec<String> {
        self.division.teams.iter().map(|team| team.name.clone()).collect()
    }

    pub fn images_hashmap(&self) -> std::collections::HashMap<String, String> {
        self.resources.images.iter().map(|asset| (asset.name.clone(), asset.value.clone())).collect()
    }

    pub fn strings_hashmap(&self) -> std::collections::HashMap<String, String> {
        self.resources.strings.iter().map(|asset| (asset.name.clone(), asset.value.clone())).collect()
    }

    pub fn correct_rounds_to_count(&mut self) {
        while self.settings.round_count < self.current_match.rounds.len() {
            self.current_match.rounds.pop();
        }
        while self.settings.round_count > self.current_match.rounds.len() {
            self.current_match.rounds.push(Round::default());
        }
    }

    pub fn correct_bracket_to_count(&mut self) {
        while self.settings.bracket_stage_count < self.division.bracket.len() {
            self.division.bracket.remove(0);
        }
        while self.settings.bracket_stage_count > self.division.bracket.len() {
            self.division.bracket.insert(0, 
                vec![None; 2_usize.pow(self.division.bracket.len() as u32)]);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Settings {
    pub event_name: String,
    pub round_count: usize,
    pub bracket_stage_count: usize,
    pub gamemodes: Vec<Gamemode>,
    pub roles: Vec<Role>,
    pub characters: Vec<Character>,
}

impl Settings {
    pub fn new(
        event_name: &str,
        round_count: usize,
        bracket_stage_count: usize,
        gamemodes: Vec<Gamemode>,
        roles: Vec<Role>,
        characters: Vec<Character>,
    ) -> Self {
        Self {
            event_name: event_name.to_string(),
            round_count,
            bracket_stage_count,
            gamemodes,
            roles,
            characters,
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new("New Event", 5, 3, Vec::new(), Vec::new(), Vec::new())
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
pub struct Resources {
    pub images: Vec<ResourcePair>,
    pub strings: Vec<ResourcePair>,
}

impl Resources {
    pub fn new(images: Vec<ResourcePair>, strings: Vec<ResourcePair>) -> Self {
        Self {
            images,
            strings,
        }
    }
}

impl Default for Resources {
    fn default() -> Self {
        Self::new(Vec::new(), Vec::new())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ResourcePair {
    pub name: String,
    pub value: String,
}

impl ResourcePair {
    pub fn new(name: &str, value: &str) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Division {
    pub teams: Vec<Team>,
    pub bracket: Vec<Vec<Option<Matchup>>>,
}

impl Division {
    pub fn new(
        teams: Vec<Team>,
        bracket: Option<Vec<Vec<Option<Matchup>>>>
    ) -> Self {
        let bracket = match bracket {
            Some(bracket) => bracket,
            None => vec![
                vec![None; 4],
                vec![None; 2],
                vec![None],
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
    pub completed: bool,
}

impl Matchup {
    pub fn new(
        team1: Option<usize>,
        team2: Option<usize>,
        team1_score: usize,
        team2_score: usize,
        completed: bool,
    ) -> Self {
        Self {
            team1,
            team2,
            team1_score,
            team2_score,
            completed,
        }
    }

    pub fn is_filled(&self) -> bool {
        self.team1.is_some() && self.team2.is_some()
    }
}

impl Default for Matchup {
    fn default() -> Self {
        Self::new(None, None, 0, 0, false)
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
