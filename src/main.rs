use anyhow::Result;
use std::{env, sync::Arc};
use tera::{Context, Tera};
// use oauth2::AuthUrl;
mod google;
mod greetings;
use serde::Serialize;

use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Extension, Router,
};

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

const HOME: &str = "http://localhost:8000";

#[derive(Serialize)]
struct HtmlData {
    user: String,
    authenticated: bool,
}

type Templates = Arc<Tera>;

// fn main() -> Result<()> {
//     println!("Hello, world!");
//     let v = env::var("USER").expect("$USER is not set");
//
//     let authorize_url = env::var("AUTH_CALLBACK").expect("AUTH_CALLBACK is not set");
//     println!("{}", v);
//     greetings::say_hi(&v);
//
//     // testing
//     // let url = AuthUrl::new(authorize_url.to_string())?;
//     // print_type_of(&url);
//     // google::client(&HOME, &authorize_url)?;
//     google::client(&authorize_url, &HOME)?;
//     Ok(())
// }

pub fn get_router() -> Router {
    let mut tera = Tera::default();
    tera.add_raw_templates(vec![("index", include_str!("../templates/index.html"))])
        .unwrap();

    Router::new()
        .route("/", get(get_user_var))
        .route("/html", get(index_html))
        .layer(Extension(Arc::new(tera)))
}
async fn get_user_var() -> String {
    let u = env::var("USER").expect("$USER is not set");
    format!("User running this service: {:?}", u)
}

async fn index_html(Extension(templates): Extension<Templates>) -> impl IntoResponse {
    let mut context = Context::new();
    let data = HtmlData {
        user: String::from("test"),
        authenticated: true,
    };
    context.insert("data", &data);
    Html(templates.render("index", &context).unwrap())
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    // let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    println!("Spinning server up on http://0.0.0.0:3000");
    // let app = Router::new().route("/", get(get_user_var));
    let app = get_router();

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
