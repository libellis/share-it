use std::env;

use dotenv::dotenv;
use actix_web::{web, HttpServer, App, Responder, Error as AWError, HttpResponse, Result};
use actix_web::http::header;
use actix_session::{CookieSession, Session};
use serde::Deserialize;
use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken,
    RedirectUrl, Scope, TokenResponse, TokenUrl,
};

struct OauthState {
    client: BasicClient,
}

fn index(session: Session) -> HttpResponse {
    let login = session.get::<String>("login").unwrap();
    let link = if login.is_some() { "logout" } else { "login" };

    let html = format!(
        r#"<html>
        <head><title>Vimeo Oauth2</title></head>
        <body>
            {} <a href="/{}">{}</a>
        </body>
    </html>"#,
        login.unwrap_or("".to_string()),
        link,
        link
    );

    HttpResponse::Ok().body(html)
}

async fn login(
    session: Session,
    state: web::Data<OauthState>
) -> HttpResponse {
    // Generate the authorization URL to which we'll redirect the user.
    let (authorize_url, csrf_state) = state.client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("private".to_string()))
        .url();

    session.set("csrf_state", csrf_state.secret()).unwrap();

    HttpResponse::Found()
        .header(header::LOCATION, authorize_url.to_string())
        .finish()
}

#[derive(Deserialize)]
pub struct OauthResponse {
    code: String,
    state: String,
}

async fn auth_listener(
    session: Session,
    state: web::Data<OauthState>,
    web::Query(req): web::Query<OauthResponse>,
) -> HttpResponse {
    let code = AuthorizationCode::new(req.code.clone());
    let sent_state_str = session.get::<String>("csrf_state").unwrap();
    let should_bail = if let Some(sent_state) = sent_state_str {
        if sent_state == req.state {
            false
        } else {
            true
        }
    } else { true };
    if should_bail {
        return HttpResponse::BadRequest().body("CSRF Attack detected. States did not match during oauth validation process.");
    }

    let token_res = state
        .client
        .exchange_code(code)
        .request(http_client);

    if let Ok(token) = token_res {
        let scopes = if let Some(scopes_vec) = token.scopes() {
            scopes_vec
                .iter()
                .map(|comma_separated| comma_separated.split(','))
                .flatten()
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };

        session.set("login", token.access_token().secret()).unwrap();

        // TODO: Actually return the scopes.
        HttpResponse::Ok().body(format!(r#"<!DOCTYPE html>
                               <html lang="en">
                                 <head>
                                   <meta charset="utf-8">
                                   <title>Vimeo Access Token</title>
                                 </head>
                                 <body>
                                    Vimeo access token:
                                    <pre>{}</pre>
                                    <a href="/">Home</a>
                                 </body>
                               </html>
                               "#, token.access_token().secret()))
    } else {
        HttpResponse::InternalServerError().body("We didn't get a token back from vimeo, using the code from the oauth process.")
    }
}

async fn logout(session: Session) -> HttpResponse {
    session.remove("login");
    HttpResponse::Found()
        .header(header::LOCATION, "/".to_string())
        .finish()
}


#[actix_rt::main]
async fn main() {
    dotenv().ok();

    let addr = match std::env::var("SERVER_HOST") {
        Ok(host) => host,
        Err(_) => "0.0.0.0:8080".to_string(),
    };

    HttpServer::new(|| {
        let vimeo_client_id = ClientId::new(
            env::var("VIMEO_CLIENT_ID").expect("Missing the VIMEO_CLIENT_ID environment variable."),
        );
        let vimeo_client_secret = ClientSecret::new(
            env::var("VIMEO_CLIENT_SECRET")
                .expect("Missing the VIMEO_CLIENT_SECRET environment variable."),
        );
        let auth_url = AuthUrl::new("https://api.vimeo.com/oauth/authorize".to_string())
            .expect("Invalid authorization endpoint URL");
        let token_url = TokenUrl::new("https://api.vimeo.com/oauth/access_token".to_string())
            .expect("Invalid token endpoint URL");

        let client = BasicClient::new(
            vimeo_client_id,
            Some(vimeo_client_secret),
            auth_url,
            Some(token_url),
        )
        .set_redirect_url(
            RedirectUrl::new("http://localhost:8080/auth".to_string()).expect("Invalid redirect URL"),
        );

        App::new()
            .data(OauthState {
                client,
            })
            // TODO: Make this actual prod ready
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .route("/", web::get().to(index))
            .route("/login", web::get().to(login))
            .route("/logout", web::get().to(logout))
            .route("/auth", web::get().to(auth_listener))
        })
        .bind(&addr)
        .expect("Can not bind to port 8080")
        .run()
        .await
        .unwrap();
}