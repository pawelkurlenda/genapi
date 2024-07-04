use std::fs;
use actix_web::{App, HttpServer, web};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::models::entity_definitions::EntityDefinition;
use actix_web::web::ServiceConfig;

mod macros;
mod models;


//#[derive(OpenApi)]
//#[openapi(paths(product::delete))]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let paths = fs::read_dir("./generated_models").unwrap();

    let mut entity_definitions: Vec<EntityDefinition> = Vec::new();

    for path in paths {
        let path = path.unwrap().path();
        let entity_definition = EntityDefinition::new(&path);
        entity_definitions.push(entity_definition);
    }

    /*for entity in &entity_definitions {
        generate_crud_handlers_2!(entity);
        //let struct_definition = generate_struct!(entity);
    }*/

    HttpServer::new(move || {
        let mut app = App::new();

        for entity in &entity_definitions {

            generate_crud_handlers_3!(entity);

            let config_route = entity.get_routes();

            let mut scope = web::scope(&config_route.scope);

            for config_route_route in config_route.routes.iter() {
                //scope = scope.route(config_route_route, );
                //scope = scope.service(); // todo pass endpoint function
            }

            app = app.service(scope);
        }

        app
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

pub fn configure_app(mut app: App, entity_definitions: &Vec<EntityDefinition>) -> App {
    for entity in entity_definitions.iter() {
        generate_struct!(entity);
        generate_crud_handlers_3!(entity, entity.endpoints);
        /*match entity.entity.as_str() {
            "User" => {
                generate_struct!(entity);
                generate_crud_handlers_3!(entity, entity.endpoints);
                app = app.configure(User::configure);
            },
            "Product" => {
                generate_struct!(Product, id: i32, name: String, price: f64);
                generate_crud_handlers_3!(Product, entity.endpoints);
                app = app.configure(Product::configure);
            },
            _ => {
                eprintln!("Unknown entity type: {}", entity.entity);
            }
        }*/
    }
    app
}

/*pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/cache")
            .route("/keys", web::get().to(cache_keys_get_handler))
            .route("/{cache_key}", web::get().to(cache_get_handler))
            .route("/{cache_key}", web::put().to(cache_put_handler))
            .route("/{cache_key}", web::delete().to(cache_delete_handler))
    );
}*/

/*pub fn configure_routes(mut app: App, entity: &EntityDefinition) -> actix_web::App {
    match entity.entity.as_str() {
        "User" => {
            generate_struct!(User, id: i32, username: String, email: String);
            generate_crud_handlers!(User, entity.endpoints);
            app.configure(User::configure)
        },
        "Product" => {
            generate_struct!(Product, id: i32, name: String, price: f64);
            generate_crud_handlers!(Product, entity.endpoints);
            app.configure(Product::configure)
        },
        _ => {
            eprintln!("Unknown entity type: {}", entity.entity);
            app
        }
    }

    app
}*/

// Define entity definitions here (or read from JSON as before)
//generate_crud_handlers!("User", "id" => i32, "username" => String, "email" => String);
//generate_crud_handlers!("Product", "id" => i32, "name" => String, "price" => f64);
