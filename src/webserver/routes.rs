use axum::{
    body::Body, extract::{Extension, Path}, response::{Html, Response}, routing::get, Router
};
use std::{fs, sync::Arc};
use super::{error::AppError, WebserverState};

pub fn create_router(state: Arc<WebserverState>) -> Router {
    Router::new()
        .route("/bracket", get(render_bracket))
        .route("/assets/*path", get(serve_asset))
        .route("/team/:team", get(render_team))
        .route("/scoreboard", get(render_scoreboard))
        .route("/rounds", get(render_rounds))
        .layer(Extension(state))
}

pub async fn render_bracket(
    Extension(webserver_state): Extension<Arc<WebserverState>>,
) -> Result<Html<String>, AppError> {
    // Lock the context to safely access it
    let mut context = webserver_state.context.lock().unwrap();
    let state = webserver_state.shared_state.lock().unwrap();
    
    populate_context(&mut context, &state);

    match webserver_state.tera.render("bracket.html", &context) {
        Ok(rendered) => Ok(Html(rendered)),
        Err(e) => {
            eprintln!("Failed to render template: {}", e);
            Err(AppError::TemplateError)
        }
    }
}

pub async fn render_team(
    Path(team_number): Path<usize>,
    Extension(webserver_state): Extension<Arc<WebserverState>>,
) -> Result<Html<String>, AppError> {
    let mut context = webserver_state.context.lock().unwrap();
    let state = webserver_state.shared_state.lock().unwrap();

    populate_context(&mut context, &state);

    let team_index = if team_number == 1 {
        state.current_match.team1
    } else {
        state.current_match.team2
    };
    
    let team = team_index
        .and_then(|index| state.division.teams.get(index))
        .ok_or(AppError::NotFound)?;

    context.insert("team", team);

    match webserver_state.tera.render("team.html", &context) {
        Ok(rendered) => Ok(Html(rendered)),
        Err(e) => {
            eprintln!("Failed to render template: {}", e);
            Err(AppError::TemplateError)
        }
    }
}

pub async fn render_scoreboard(
    Extension(webserver_state): Extension<Arc<WebserverState>>,
) -> Result<Html<String>, AppError> {
    let mut context = webserver_state.context.lock().unwrap();
    let state = webserver_state.shared_state.lock().unwrap();

    populate_context(&mut context, &state);
    
    match webserver_state.tera.render("scoreboard.html", &context) {
        Ok(rendered) => Ok(Html(rendered)),
        Err(e) => {
            eprintln!("Failed to render template: {}", e);
            Err(AppError::TemplateError)
        }
    }
}

pub async fn render_rounds(
    Extension(webserver_state): Extension<Arc<WebserverState>>,
) -> Result<Html<String>, AppError> {
    let mut context = webserver_state.context.lock().unwrap();
    let state = webserver_state.shared_state.lock().unwrap();

    populate_context(&mut context, &state);

    match webserver_state.tera.render("rounds.html", &context) {
        Ok(rendered) => Ok(Html(rendered)),
        Err(e) => {
            eprintln!("Failed to render template: {}", e);
            Err(AppError::TemplateError)
        }
    }
}

pub async fn serve_asset(Path(path): Path<String>) -> Result<Response, AppError> {
    if path.contains("..") {
        return Err(AppError::NotFound);
    }

    // Match path end with whether it is text or binary
    let content_type = get_content_type(path.as_str());
    let body = if content_type.starts_with("text") {
        Body::from(fs::read_to_string(format!("assets/{}", path))?)
    } else {
        Body::from(fs::read(format!("assets/{}", path))?)
    };

    let mut response = Response::new(body);
    response.headers_mut().insert("Content-Type", content_type.parse().unwrap());
    Ok(response)
}

fn populate_context(context: &mut tera::Context, state: &crate::AppState) {
    context.insert("assets", &state.assets_hashmap());
    context.insert("role_icons", &state.roles_hashmap());
    context.insert("character_icons", &state.characters_hashmap());
    context.insert("gamemode_icons", &state.gamemodes_hashmap());
    context.insert("teams", &state.division.teams);
    context.insert("team_count", &state.team_names().len());
    context.insert("bracket", &state.division.bracket);
    context.insert("bracket_stage_count", &state.bracket_stage_count());
    context.insert("bracket_visibilities", &state.bracket_visibilities());
    context.insert("team1", &state.current_match.team1.map(|i| &state.division.teams[i]));
    context.insert("team2", &state.current_match.team2.map(|i| &state.division.teams[i]));
    context.insert("team1_score", &state.current_match.team1_score());
    context.insert("team2_score", &state.current_match.team2_score());
    context.insert("rounds", &state.current_match.rounds);
}

fn get_content_type(path: &str) -> &'static str {
    match path.split('.').last() {
        Some("css") => "text/css",
        Some("js") => "text/javascript",
        Some("png") => "image/png",
        Some("jpg") => "image/jpeg",
        Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("svg") => "image/svg+xml",
        _ => "application/octet-stream",
    }
}
