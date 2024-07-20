#[derive(Clone)]
pub struct Bracket {
    pub division: Division,

}

#[derive(Clone)]
pub struct Division {
    pub name: String,
    pub teams: Vec<Team>,
}

impl Division {
    pub fn new(name: &str, teams: Vec<Team>) -> Self {
        Self {
            name: name.to_string(),
            teams,
        }
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
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
