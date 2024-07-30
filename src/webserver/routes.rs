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
        .layer(Extension(state))
}

pub async fn render_bracket(
    Extension(webserver_state): Extension<Arc<WebserverState>>,
) -> Result<Html<String>, AppError> {
    // Lock the context to safely access it
    let mut context = webserver_state.context.lock().unwrap();
    let state = webserver_state.shared_state.lock().unwrap();
    
    let teams_display_list: Vec<String> = state
        .division
        .teams
        .iter()
        .map(|team| team.name.clone())
        .collect();
    
    let mut first_round = 0;
    for round in &state.division.bracket {
        if round.iter().any(|team| team.is_some()) {
            break;
        }
        first_round += 1;
    }

    context.insert("teams", &teams_display_list);
    context.insert("numTeams", &teams_display_list.len());
    context.insert("bracket", &state.division.bracket);
    context.insert("firstRound", &first_round);

    // Render the template
    let rendered = webserver_state
        .tera
        .render("bracket.html", &context);
    match rendered {
        Ok(rendered) => Ok(Html(rendered)),
        Err(e) => {
            eprintln!("Failed to render template: {}", e);
            Err(AppError::TemplateError)
        }
    }
}

pub async fn render_team(
    Path(team): Path<String>,
    Extension(webserver_state): Extension<Arc<WebserverState>>,
) -> Result<Html<String>, AppError> {
    let mut context = webserver_state.context.lock().unwrap();
    let state = webserver_state.shared_state.lock().unwrap();

    let team = state
        .division
        .teams
        .iter()
        .find(|t| t.name == team)
        .ok_or(AppError::NotFound)?;

    let players_display_list: Vec<Vec<String>> = team
        .players
        .iter()
        .map(|player| vec![
            player.name.clone(),
            player.role.clone(),
            player.hero.clone()
        ])
        .collect();

    context.insert("team", &team.name);
    context.insert("players", &players_display_list);

    let rendered = webserver_state
        .tera
        .render("team.html", &context);
    match rendered {
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
