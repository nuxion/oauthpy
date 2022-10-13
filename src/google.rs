use anyhow::Result;
use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    Scope, TokenResponse, TokenUrl,
};
use std::env;
// use url::Url;
const AUTHORIZE_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";

// const DRIVE_SCOPE: &str = "https://www.googleapis.com/auth/drive.metadata.readonly";
const EMAIL_SCOPE: &str = "https://www.googleapis.com/auth/userinfo.email";
const PROFILE_SCOPE: &str = "https://www.googleapis.com/auth/userinfo.profile";

pub fn client(authorized_url: &str, redirect_url: &str) -> Result<()> {
    // Create an OAuth2 client by specifying the client ID, client secret, authorization URL and
    // token URL.
    let client_id = env::var("GOOGLE_CLIENTID").expect("$GOOGLE_CLIENTID is not set");
    let client_secret = env::var("GOOGLE_SECRET").expect("$GOOGLE_SECRET is not set");
    let client = BasicClient::new(
        ClientId::new(client_id.to_string()),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new(AUTHORIZE_URL.to_string())?,
        Some(TokenUrl::new(AUTHORIZE_URL.to_string())?),
    )
    // Set the URL the user will be redirected to after the authorization process.
    .set_redirect_uri(RedirectUrl::new(authorized_url.to_string())?);

    // Generate a PKCE challenge.
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the full authorization URL.
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        .add_scope(Scope::new(EMAIL_SCOPE.to_string()))
        .add_scope(Scope::new(PROFILE_SCOPE.to_string()))
        // Set the PKCE code challenge.
        // .set_pkce_challenge(pkce_challenge)
        .url();

    // This is the URL you should redirect the user to, in order to trigger the authorization
    // process.
    println!("Browse to: \n\n\t{}\n", auth_url);

    // Once the user has been redirected to the redirect URL, you'll have access to the
    // authorization code. For security reasons, your code should verify that the `state`
    // parameter returned by the server matches `csrf_state`.

    // Now you can trade it for an access token.
    /*let token_result = client
        .exchange_code(AuthorizationCode::new(
            "some authorization code".to_string(),
        ))
        // Set the PKCE code verifier.
        .set_pkce_verifier(pkce_verifier)
    .request(http_client)?;
     */
    Ok(())
}
