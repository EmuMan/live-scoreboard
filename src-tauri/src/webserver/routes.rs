use axum::{
    body::Body, extract::{Extension, Path}, response::{Html, Response}, routing::get, Router
};
use tera::Context;
use std::{fs, sync::Arc};
use std::error::Error;
use super::{error::AppError, WebserverState};

use crate::{SaveData, fs::from_relative_path};

pub fn create_router(webserver_state: Arc<WebserverState>) -> Router {
    Router::new()
        .route("/bracket", get(render_bracket))
        .route("/assets/*path", get(serve_asset))
        .route("/team/:team", get(render_team))
        .route("/scoreboard", get(render_scoreboard))
        .route("/rounds", get(render_rounds))
        .route("/waiting", get(render_waiting))
        .layer(Extension(webserver_state))
}

pub async fn render_bracket(
    Extension(webserver_state): Extension<Arc<WebserverState>>,
) -> Result<Html<String>, AppError> {
    // Lock the context to safely access it
    let mut context = Context::new();
    let state = webserver_state.shared_state.lock().unwrap();
    
    populate_context(&mut context, &state.data);

    match webserver_state.tera.render("bracket.html", &context) {
        Ok(rendered) => Ok(Html(rendered)),
        Err(e) => {
            eprintln!("Failed to render template: {:?}", e);
            Err(AppError::TemplateError)
        }
    }
}

pub async fn render_team(
    Path(team_number): Path<usize>,
    Extension(webserver_state): Extension<Arc<WebserverState>>,
) -> Result<Html<String>, AppError> {
    let mut context = Context::new();
    let state = webserver_state.shared_state.lock().unwrap();
    
    populate_context(&mut context, &state.data);

    let team_index = if team_number == 1 {
        state.data.current_match.team1
    } else {
        state.data.current_match.team2
    };
    
    let team = team_index
        .and_then(|index| state.data.division.teams.get(index))
        .ok_or(AppError::NotFound)?;

    context.insert("team", team);

    match webserver_state.tera.render("team.html", &context) {
        Ok(rendered) => Ok(Html(rendered)),
        Err(e) => {
            eprintln!("Failed to render template: {:?}", e.source());
            Err(AppError::TemplateError)
        }
    }
}

pub async fn render_scoreboard(
    Extension(webserver_state): Extension<Arc<WebserverState>>,
) -> Result<Html<String>, AppError> {
    let mut context = Context::new();
    let state = webserver_state.shared_state.lock().unwrap();
    
    populate_context(&mut context, &state.data);
    
    match webserver_state.tera.render("scoreboard.html", &context) {
        Ok(rendered) => Ok(Html(rendered)),
        Err(e) => {
            eprintln!("Failed to render template: {:?}", e.source());
            Err(AppError::TemplateError)
        }
    }
}

pub async fn render_rounds(
    Extension(webserver_state): Extension<Arc<WebserverState>>,
) -> Result<Html<String>, AppError> {
    let mut context = Context::new();
    let state = webserver_state.shared_state.lock().unwrap();
    
    populate_context(&mut context, &state.data);

    match webserver_state.tera.render("rounds.html", &context) {
        Ok(rendered) => Ok(Html(rendered)),
        Err(e) => {
            eprintln!("Failed to render template: {:?}", e.source());
            Err(AppError::TemplateError)
        }
    }
}

pub async fn render_waiting(
    Extension(webserver_state): Extension<Arc<WebserverState>>,
) -> Result<Html<String>, AppError> {
    let mut context = Context::new();
    let state = webserver_state.shared_state.lock().unwrap();
    
    populate_context(&mut context, &state.data);

    match webserver_state.tera.render("waiting.html", &context) {
        Ok(rendered) => Ok(Html(rendered)),
        Err(e) => {
            eprintln!("Failed to render template: {:?}", e.source());
            Err(AppError::TemplateError)
        }
    }
}

pub async fn serve_asset(
    Path(path): Path<String>,
    Extension(webserver_state): Extension<Arc<WebserverState>>,
) -> Result<Response, AppError> {
    if path.contains("..") {
        return Err(AppError::NotFound);
    }

    let state = webserver_state.shared_state.lock().unwrap();

    let Some(base_path) = state.get_base_path() else {
        eprintln!("Failed to get base path");
        return Err(AppError::NotFound);
    };

    let path = path.trim_start_matches("/");
    let path = format!("assets/{}", path);
    let path = from_relative_path(&base_path, &path);

    // Match path end with whether it is text or binary
    let content_type = get_content_type(path.as_str());
    let body = if content_type.starts_with("text") {
        Body::from(fs::read_to_string(path)?)
    } else {
        Body::from(fs::read(path)?)
    };

    let mut response = Response::new(body);
    response.headers_mut().insert("Content-Type", content_type.parse().unwrap());
    Ok(response)
}

fn populate_context(context: &mut tera::Context, data: &SaveData) {
    context.insert("images", &data.images_hashmap());
    context.insert("strings", &data.strings_hashmap());
    context.insert("gamemodes", &data.settings.gamemodes);
    context.insert("roles", &data.settings.roles);
    context.insert("characters", &data.settings.characters);
    context.insert("teams", &data.division.teams);
    context.insert("team_count", &data.team_names().len());
    context.insert("bracket", &data.division.bracket);
    context.insert("bracket_stage_count", &data.settings.bracket_stage_count);
    context.insert("rounds", &data.current_match.rounds);
    context.insert("event_name", &data.settings.event_name);
    context.insert("team1", &data.current_match.team1.map(|i| &data.division.teams[i]));
    context.insert("team2", &data.current_match.team2.map(|i| &data.division.teams[i]));
    context.insert("team1_score", &data.current_match.team1_score());
    context.insert("team2_score", &data.current_match.team2_score());
    context.insert("swap_scoreboard", &data.current_match.swap_scoreboard);

    for image in &data.resources.images {
        let name = format!("image_{}", image.name);
        context.insert(&name, &image.value);
    }

    for string in &data.resources.strings {
        let name = format!("string_{}", string.name);
        context.insert(&name, &string.value);
    }
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
