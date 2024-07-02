use std::fs;
use actix_web::{App, HttpServer};
use utoipa::OpenApi;
//use utoipa_swagger_ui::SwaggerUi;
use crate::models::entity_definitions::EntityDefinition;

mod macros;
mod models;


//#[derive(OpenApi)]
//#[openapi(paths(get_model))]
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

    for entity in &entity_definitions {
        generate_crud_handlers_2!(entity);
        //let struct_definition = generate_struct!(entity);
    }

    //let api_doc = ApiDoc::openapi();

    Ok(())

    /*HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", api_doc.clone()))
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await*/
}


// Define entity definitions here (or read from JSON as before)
//generate_crud_handlers!("User", "id" => i32, "username" => String, "email" => String);
//generate_crud_handlers!("Product", "id" => i32, "name" => String, "price" => f64);
