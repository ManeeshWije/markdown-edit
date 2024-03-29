use std::env;

use anyhow::Context;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{ErrorResponse, IntoResponse, Redirect},
    routing::get,
    Router,
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use oauth2::{
    basic::BasicClient, AuthorizationCode, CsrfToken, PkceCodeChallenge, Scope, TokenResponse,
};
use oauth2::{reqwest::async_http_client, PkceCodeVerifier};
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};

use crate::db::user_queries;
use crate::utils::constants::{
    COOKIE_AUTH_CODE_VERIFIER, COOKIE_AUTH_CSRF_STATE, COOKIE_AUTH_SESSION, SESSION_DURATION,
};

// What we get back from Google
#[derive(Default, Debug, serde::Serialize, serde::Deserialize)]
struct GoogleUser {
    sub: String,
    name: String,
    email: String,
}

// What we send to Google
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct AuthRequest {
    code: String,
    state: String,
}

pub fn google_auth_router(pool: sqlx::PgPool) -> Router {
    Router::new()
        .route("/logout", get(logout))
        .route("/google/login", get(login))
        .route("/google/callback", get(callback))
        .with_state(pool)
}

fn get_oauth_client() -> Result<BasicClient, anyhow::Error> {
    let client_id = ClientId::new(
        std::env::var("GOOGLE_CLIENT_ID")
            .context("Missing the GOOGLE_CLIENT_ID environment variable")?,
    );

    let client_secret = ClientSecret::new(
        std::env::var("GOOGLE_CLIENT_SECRET")
            .context("Missing the GOOGLE_CLIENT_SECRET environment variable")?,
    );

    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
        .context("Invalid authorization endpoint URL")?;
    let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())
        .context("Invalid token endpoint URL")?;

    let base_url = std::env::var("BASE_URL").context("Failed to get app base url")?;
    let redirect_url = RedirectUrl::new(format!("{base_url}/auth/google/callback"))
        .context("Invalid redirect url")?;

    let client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
        .set_redirect_uri(redirect_url);

    Ok(client)
}

async fn login() -> Result<impl IntoResponse, StatusCode> {
    let client = get_oauth_client().map_err(|err| {
        eprintln!("Failed to create google auth client: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

    let (authorize_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.profile".to_string(),
        ))
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.email".to_string(),
        ))
        .set_pkce_challenge(pkce_code_challenge)
        .url();

    // Set csrf and code verifier cookies, these are short lived cookies
    let cookie_max_age = cookie::time::Duration::minutes(5);
    let csrf_cookie: Cookie =
        Cookie::build((COOKIE_AUTH_CSRF_STATE, csrf_state.secret().to_owned()))
            .http_only(true)
            .path("/")
            .same_site(SameSite::Lax)
            .max_age(cookie_max_age)
            .into();

    let code_verifier: Cookie = Cookie::build((
        COOKIE_AUTH_CODE_VERIFIER,
        pkce_code_verifier.secret().to_owned(),
    ))
    .http_only(true)
    .path("/")
    .same_site(SameSite::Lax)
    .max_age(cookie_max_age)
    .into();

    let cookies = CookieJar::new().add(csrf_cookie).add(code_verifier);

    Ok((cookies, Redirect::to(authorize_url.as_str())))
}

async fn callback(
    cookies: CookieJar,
    State(pool): State<sqlx::PgPool>,
    Query(query): Query<AuthRequest>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let code = query.code;
    let state = query.state;
    let stored_state = cookies.get(COOKIE_AUTH_CSRF_STATE);
    let stored_code_verifier = cookies.get(COOKIE_AUTH_CODE_VERIFIER);

    let (Some(csrf_state), Some(code_verifier)) = (stored_state, stored_code_verifier) else {
        return Err(ErrorResponse::from(StatusCode::BAD_REQUEST));
    };

    if csrf_state.value() != state {
        return Err(ErrorResponse::from(StatusCode::BAD_REQUEST));
    }

    let client = get_oauth_client().map_err(|err| {
        eprintln!("Failed to create google auth client: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let code = AuthorizationCode::new(code);
    let pkce_code_verifier = PkceCodeVerifier::new(code_verifier.value().to_owned());

    let token_response = client
        .exchange_code(code)
        .set_pkce_verifier(pkce_code_verifier)
        .request_async(async_http_client)
        .await
        .map_err(|err| {
            eprintln!("Failed to get token response: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Get the Google user info
    let google_user = reqwest::Client::new()
        .get("https://www.googleapis.com/oauth2/v3/userinfo")
        .bearer_auth(token_response.access_token().secret())
        .send()
        .await
        .map_err(|err| {
            eprintln!("Failed to get user info: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .json::<GoogleUser>()
        .await
        .map_err(|err| {
            eprintln!("Failed to parse user info: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Check if the user exists and create a new user if they don't
    let account_email = google_user.email.clone().to_string();
    let existing_user = user_queries::fetch_user_by_email(&pool, account_email.as_str())
        .await
        .context("Failed to get user")
        .map_err(|err| {
            eprintln!("Failed to get user: {}", err);
            err
        });

    let user = match existing_user {
        Ok(user) => user,
        Err(_) => {
            let new_user = user_queries::create_user(
                &pool,
                uuid::Uuid::new_v4(),
                google_user.name.as_str(),
                account_email.as_str(),
            )
            .await
            .map_err(|err| {
                eprintln!("Failed to create user: {}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

            new_user
        }
    };

    // check if the user session exists already and is valid. if not create a new one
    let user_session = user_queries::fetch_user_session_by_user_uuid(&pool, user.uuid)
        .await
        .context("Failed to get user session")
        .map_err(|err| {
            eprintln!("Failed to get user session: {}", err);
            err
        });

    let user_session = match user_session {
        Ok(user_session) => user_session,
        Err(_) => {
            let new_user_session =
                user_queries::create_user_session(&pool, user.uuid, SESSION_DURATION)
                    .await
                    .map_err(|err| {
                        eprintln!("Failed to create user session: {}", err);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })?;

            new_user_session
        }
    };

    // Remove code_verifier and csrf_state cookies
    let mut remove_csrf_cookie = Cookie::new(COOKIE_AUTH_CSRF_STATE, "");
    remove_csrf_cookie.set_path("/");
    remove_csrf_cookie.make_removal();

    let mut remove_code_verifier = Cookie::new(COOKIE_AUTH_CODE_VERIFIER, "");
    remove_code_verifier.set_path("/");
    remove_code_verifier.make_removal();

    let session_cookie: Cookie =
        Cookie::build((COOKIE_AUTH_SESSION, user_session.uuid.to_string()))
            .same_site(SameSite::Lax)
            .http_only(true)
            .path("/")
            .max_age(cookie::time::Duration::milliseconds(
                SESSION_DURATION.as_millis() as i64,
            ))
            .into();

    let cookies = CookieJar::new()
        .add(remove_csrf_cookie)
        .add(remove_code_verifier)
        .add(session_cookie);

    Ok((
        cookies,
        Redirect::to(env::var("CLIENT_URL").unwrap().as_str()).into_response(),
    ))
}

pub async fn logout(
    mut cookies: CookieJar,
    State(pool): State<sqlx::PgPool>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let session_cookie = cookies.get(COOKIE_AUTH_SESSION);

    let Some(session_cookie) = session_cookie else {
        return Err(ErrorResponse::from(StatusCode::UNAUTHORIZED));
    };

    user_queries::delete_user_session(
        &pool,
        uuid::Uuid::parse_str(session_cookie.value()).unwrap(),
    )
    .await
    .map_err(|_| ErrorResponse::from(StatusCode::INTERNAL_SERVER_ERROR))?;

    let mut remove_session_cookie = Cookie::new(COOKIE_AUTH_SESSION, "");
    remove_session_cookie.set_path("/");
    remove_session_cookie.make_removal();

    cookies = cookies.add(remove_session_cookie);

    Ok((
        cookies,
        Redirect::to(env::var("CLIENT_URL").unwrap().as_str()).into_response(),
    ))
}
