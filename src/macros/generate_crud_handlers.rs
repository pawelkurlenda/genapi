#[macro_export]
macro_rules! generate_crud_handlers {
    ($entity_name:expr, $($field_name:expr => $field_type:expr),*) => {
        // Generate struct definition
        generate_struct!($entity_name, $($field_name => $field_type),*);

        // Generate CRUD handlers
        mod $entity_name {
            use super::*;
            use actix_web::{web, HttpResponse};

            // Create handler
            #[actix_web::post("/create")]
            pub async fn create(item: web::Json<$entity_name>) -> HttpResponse {
                println!("Creating {:?}", item);
                // Implement your create logic here (e.g., saving to database)
                HttpResponse::Ok().json(item.0)
            }

            // Read handler
            #[actix_web::get("/read/{id}")]
            pub async fn read(path: web::Path<(i32,)>) -> HttpResponse {
                let id = path.into_inner().0;
                println!("Reading {} with id {}", stringify!($entity_name), id);
                // Implement your read logic here (e.g., fetch from database)
                let item = $entity_name {
                    $($field_name: Default::default()),*
                };
                HttpResponse::Ok().json(item)
            }

            // Update handler
            #[actix_web::put("/update/{id}")]
            pub async fn update(path: web::Path<(i32,)>, item: web::Json<$entity_name>) -> HttpResponse {
                let id = path.into_inner().0;
                println!("Updating {} with id {}: {:?}", stringify!($entity_name), id, item);
                // Implement your update logic here (e.g., update in database)
                HttpResponse::Ok().json(item.0)
            }

            // Delete handler
            #[actix_web::delete("/delete/{id}")]
            pub async fn delete(path: web::Path<(i32,)>) -> HttpResponse {
                let id = path.into_inner().0;
                println!("Deleting {} with id {}", stringify!($entity_name), id);
                // Implement your delete logic here (e.g., delete from database)
                HttpResponse::Ok().json(())
            }

            // Generate URLs for CRUD operations
            pub fn urls() -> Vec<String> {
                vec![
                    format!("/{}_api/{}/create", stringify!($entity_name).to_lowercase(), stringify!($entity_name)),
                    format!("/{}_api/{}/read/{{id}}", stringify!($entity_name).to_lowercase(), stringify!($entity_name)),
                    format!("/{}_api/{}/update/{{id}}", stringify!($entity_name).to_lowercase(), stringify!($entity_name)),
                    format!("/{}_api/{}/delete/{{id}}", stringify!($entity_name).to_lowercase(), stringify!($entity_name)),
                ]
            }
        }
    };
}