pub mod routes {
    pub mod auth;
    pub mod game_server;
    pub mod meta;

    use crate::state::ServerState;
    use roa::router::{get, post, Router};

    pub fn build_router() -> Router<ServerState> {
        Router::new()
            .gate(roa::query::query_parser)
            /* meta */
            .on("/", get(meta::index))
            .on("/version", get(meta::version))
            /* auth */
            .on("/totplogin", post(auth::totp_login))
            .on("/challenge/new", post(auth::challenge_start))
            .on("/challenge/verify", post(auth::challenge_finish))
            /* game-server api, not for the end user */
            .on("/gs/boot", post(game_server::boot))
            .on("/gs/verify", post(game_server::verify_token))
    }
}