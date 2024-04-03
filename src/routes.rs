use crate::{
    core::{general_v1_server_time, prod, user},
    crisis,
};
use axum::{
    http::Uri,
    routing::{get, post},
    Router,
};
use reqwest::StatusCode;

pub fn routes() -> Router {
    Router::new()
        .nest("/app", app_routes())
        .nest("/config/prod", config_routes())
        .nest("/crisis", crisis_routes())
        .nest("/crisisV2", crisis_v2_routes())
        .nest("/user", user_routes())
        .merge(misc_routes())
        .fallback(fallback)
}

fn app_routes() -> Router {
    Router::new().route("/v1/config", get(user::app_v1_config))
}

fn config_routes() -> Router {
    Router::new()
        .route("/official/Android/version", get(prod::prod_android_version))
        .route("/official/network_config", get(prod::prod_network_config))
        .route("/official/remote_config", get(prod::prod_remote_config))
        .route("/official/refresh_config", get(prod::prod_refresh_config))
        .route("/announce_meta/Android/announcement.meta.jsons", get(prod::prod_announcement))
        .route("/announce_meta/Android/preannouncement.meta.json", get(prod::prod_pre_announcement))
}

fn crisis_routes() -> Router {
    Router::new().route("/crisis", get(|| async { "Crisis" }))
}

fn crisis_v2_routes() -> Router {
    Router::new()
        .route("/getInfo", post(crisis::crisis_v2_get_info))
        .route("/battleStart", post(crisis::crisis_v2_battle_start))
}

fn user_routes() -> Router {
    Router::new()
        .route("/auth", post(user::user_auth))
        .route("/agreement", get(user::user_agreement))
        .route("/info/v1/basic", get(user::info_v1_basic))
        .route("/changeSecretary", post(user::user_change_secretary))
}

fn misc_routes() -> Router {
    Router::new()
        .route("/general/v1/server_time", get(general_v1_server_time))
        .route("/u8/user/auth/v1/agreement_version", get(user::agreement_version))
}

async fn fallback(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("ERROR: {} not found", uri))
}