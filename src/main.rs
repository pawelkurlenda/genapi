use std::fs;
use actix_web::{App, get, HttpResponse, HttpServer, Responder, web};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::models::entity_definitions::EntityDefinition;
use actix_web::web::ServiceConfig;

mod macros;
mod models;


//#[derive(OpenApi)]
//#[openapi(paths(product::delete))]
struct ApiDoc;

/*#[macro_export]
macro_rules! aaa {
    ($entity:expr) => {
        let paths = fs::read_dir("./generated_models").unwrap();

        let mut entity_definitions: Vec<EntityDefinition> = Vec::new();

        for path in paths {
            let path = path.unwrap().path();
            let entity_definition = EntityDefinition::new(&path);
            entity_definitions.push(entity_definition);

            println!("{}", entity_definition.entity);
        }
    };
}

aaa!(1);*/

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let paths = fs::read_dir("./generated_models").unwrap();

    let mut entity_definitions: Vec<EntityDefinition> = Vec::new();

    for path in paths {
        let path = path.unwrap().path();
        let entity_definition = EntityDefinition::new(&path);
        entity_definitions.push(entity_definition);
    }

    for entity in &entity_definitions {
        generate_crud_handlers_2!(entity);
        //let struct_definition = generate_struct!(entity);
    }

    //Ok(());
    HttpServer::new(move || {
        let mut app = App::new();

        app = app.service(hello);

        for entity in &entity_definitions {

            generate_crud_handlers_2!(entity);

            let mod_name = format!("{}{}", entity.entity, "_get_by_id");

            quote::quote! {
                app = app.service(crate::User_get_by_id::read);
                println!("{}{}", entity.entity, "_get_by_id");
            };

            //generate_crud_handlers_2!(entity, app);

            //generate_crud_handlers_new!(entity, app);

            //app = app.configure();
            /*    .service(
                web::scope()
                    .route()
            );*/
        }

        app
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[get("/asd")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
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
