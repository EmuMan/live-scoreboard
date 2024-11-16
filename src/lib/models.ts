export interface Settings {
    event_name: string;
    round_count: number;
    bracket_stage_count: number;
    gamemodes: Gamemode[];
    roles: Role[];
    characters: Character[];
}

export interface Gamemode {
    name: string;
    icon: string | null;
    maps: Map[];
}

export interface Map {
    name: string;
    image: string | null;
}

export interface Role {
    name: string;
    icon: string | null;
}

export interface Character {
    name: string;
    image: string | null;
}

export interface Resources {
    images: ResourcePair[];
    strings: ResourcePair[];
}

export interface ResourcePair {
    name: string;
    value: string;
}

export interface Division {
    teams: Team[];
    bracket: (Matchup | null)[][];
}

export interface Matchup {
    team1: number | null;
    team2: number | null;
    team1_score: number;
    team2_score: number;
    completed: boolean;
}

export interface Match {
    rounds: Round[];
    team1: number | null;
    team2: number | null;
    swap_scoreboard: boolean;
}

export interface Round {
    gamemode: number | null;
    map: number | null;
    team1_score: number;
    team2_score: number;
    completed: boolean;
}

export interface Player {
    name: string;
    role: number | null;
    character: number | null;
}

export interface Team {
    name: string;
    icon: string | null;
    players: Player[];
}
