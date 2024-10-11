use axum::{
    extract::{Json, State},
    response::Html,
    routing::{get, post},
    Router,
};
use b64::FromBase64;
use serde::{Deserialize, Serialize};
use std::{env, fs, process, process::Command, sync::Arc};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct OurState {
    password: String,
}

#[tokio::main]
pub async fn main() {
    tracing_subscriber::fmt::init();

    let url = env::args().nth(1).unwrap();
    let password = env::args().nth(2).unwrap();

    let state = Arc::new(OurState { password });

    let app = Router::new()
        .route("/", get(get_root))
        .route("/killswitch", get(get_killswitch))
        .route("/play", post(post_play))
        .route("/tts", post(post_tts))
        .layer(CorsLayer::very_permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(Arc::clone(&state));

    let listener = tokio::net::TcpListener::bind(&url).await.unwrap();
    eprintln!("Listening at {}", &url);
    axum::serve(listener, app).await.unwrap();
}

async fn get_root() -> Html<String> {
    let html = include_str!("../index.html");

    let url = env::args().nth(1).unwrap();

    let html = html.replace("/*URL*/", &format!(" = \"{}\"", url));
    Html(html)
}

// Just in case a bug occurs...
async fn get_killswitch() {
    process::exit(1)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayData {
    password: String,
    data: String,
    ext: String,
}

async fn post_play(State(state): State<Arc<OurState>>, Json(req): Json<PlayData>) {
    // side channel attacks go brr
    if state.password == req.password {
        let audio = req.data.from_base64().unwrap();
        let id = Uuid::new_v4();
        let file_name = format!("/tmp/{}.{}", id, req.ext);
        fs::write(&file_name, audio).unwrap();
        Command::new("ffplay")
            .arg("-nodisp")
            .arg("-autoexit")
            .arg(&file_name)
            .output()
            .unwrap();
        fs::remove_file(file_name).unwrap();
    } else {
        eprintln!("No Auth");
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TTSData {
    password: String,
    data: String,
}

async fn post_tts(State(state): State<Arc<OurState>>, Json(req): Json<TTSData>) {
    if state.password == req.password {
        Command::new("espeak").arg(req.data).spawn().unwrap();
    } else {
        eprintln!("No Auth");
    }
}
