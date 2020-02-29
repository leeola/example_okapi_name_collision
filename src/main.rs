#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use]
extern crate rocket;
use rocket_contrib::json::Json;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

pub mod handler_one {
    use super::*;
    #[derive(serde::Serialize, schemars::JsonSchema)]
    pub struct Response {
        reply: String,
    }
    #[rocket_okapi::openapi]
    #[get("/one")]
    pub fn handler_one() -> Json<Response> {
        Json(Response {
            reply: "show me the docs!".to_string(),
        })
    }
}
pub mod handler_two {
    use super::*;
    #[derive(serde::Serialize, schemars::JsonSchema)]
    pub struct Response {
        different_schema: String,
    }
    #[rocket_okapi::openapi]
    #[get("/two")]
    pub fn handler_two() -> Json<Response> {
        Json(Response {
            different_schema: "show me the docs!".to_string(),
        })
    }
}

fn get_docs() -> SwaggerUIConfig {
    SwaggerUIConfig {
        url: Some("/my_resource/openapi.json".to_string()),
        urls: None,
    }
}
fn main() {
    rocket::ignite()
        .mount(
            "/my_resource",
            rocket_okapi::routes_with_openapi![handler_one::handler_one, handler_two::handler_two],
        )
        .mount("/swagger", make_swagger_ui(&get_docs()))
        .launch();
}
