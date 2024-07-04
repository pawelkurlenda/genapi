use crate::{generate_create, generate_delete, generate_get_by_id, generate_get_list, generate_struct, generate_update};
#[macro_export]
macro_rules! generate_crud_handlers_2 {
    ($entity:expr) => {
        // Generate struct definition
        generate_struct!($entity);

        let entity_name = &($entity.entity);

        for a in $entity.endpoint_types.iter() {
            /*println!("{}", is_endpoint_to_generate!($entity, a));

            if is_endpoint_to_generate!($entity, a) {
                println!("OK, {:?}", a);
            }
            else {
                println!("NOT, {:?}", a);
            }*/

            // todo figute it out how to remove comparing to string or enum (idea - generate_endpoints! on enum in EndpointType)

            if a.to_str() == "GET_BY_ID" {
                //println!("GET_BY_ID, {:?}", a);
                generate_get_by_id!($entity);
            }
            else if a.to_str() == "GET_LIST" {
                //println!("GET_LIST, {:?}", a);
                generate_get_list!($entity);
            }
            else if a.to_str() == "CREATE" {
                //println!("CREATE, {:?}", a);
                generate_create!($entity);
            }
            else if a.to_str() == "UPDATE" {
                //println!("UPDATE, {:?}", a);
                generate_update!($entity);
            }
            else if a.to_str() == "DELETE" {
                //println!("DELETE, {:?}", a);
                generate_delete!($entity);
            }
        }

        /*if contains_endpoint!($entity.endpoint_types, "CREATE") {
            println!("{}", entity_name);
            println!("CreateCreate");
            //generate_create!($entity)
        }*/

        /*
        generate_get_by_id!($entity)
        generate_get_list!($entity)
        generate_create!($entity)
        generate_update!($entity)
        generate_delete!($entity)
        */

        // Generate CRUD handlers
        quote::quote! {
            mod #entity_name {
                use super::*;
                use actix_web::{web, HttpResponse};

                // Create handler
                //
                //generate_create!($entity)
                //
                if endpoints.contains(EndpointType::Create) {
                    println!("CreateCreate");
                    generate_create!($entity)
                }
                /*#[actix_web::post("/create")]
                pub async fn create(item: web::Json<$entity.entity>) -> HttpResponse {
                    println!("Creating {:?}", item);
                    // Implement your create logic here (e.g., saving to database)
                    HttpResponse::Ok().json(item.0)
                }*/

                // Read handler
                //
                //generate_get_by_id!($entity)
                //
                if endpoints.contains(EndpointType::GetById) {
                    println!("GetByIdGetById");
                    generate_get_by_id!($entity)
                }
                /*#[actix_web::get("/read/{id}")]
                pub async fn read(path: web::Path<(i32,)>) -> HttpResponse {
                    let id = path.into_inner().0;
                    println!("Reading {} with id {}", stringify!($entity.entity), id);
                    // Implement your read logic here (e.g., fetch from database)
                    /*let item = $entity.entity {
                        $($field_name: Default::default()),*
                    };*/
                    //HttpResponse::Ok().json(item)
                    HttpResponse::Ok()
                }*/

                // Read list handler
                //
                //generate_get_list!($entity)
                //
                if endpoints.contains(EndpointType::GetList) {
                    println!("GetListGetList");
                    generate_get_list!($entity)
                }
                /*#[actix_web::get("/read/{id}")]
                pub async fn read(path: web::Path<(i32,)>) -> HttpResponse {
                    let id = path.into_inner().0;
                    println!("Reading {} with id {}", stringify!($entity.entity), id);
                    // Implement your read logic here (e.g., fetch from database)
                    /*let item = $entity.entity {
                        $($field_name: Default::default()),*
                    };*/
                    //HttpResponse::Ok().json(item)
                    HttpResponse::Ok()
                }*/

                // Update handler
                //
                //generate_update!($entity)
                //
                if endpoints.contains(EndpointType::Update) {
                    println!("UpdateUpdate");
                    generate_update!($entity)
                }
                /*#[actix_web::put("{id}")]
                pub async fn update(path: web::Path<(i32,)>, item: web::Json<$entity.entity>) -> HttpResponse {
                    let id = path.into_inner().0;
                    println!("Updating {} with id {}: {:?}", stringify!($entity.entity), id, item);
                    // Implement your update logic here (e.g., update in database)
                    HttpResponse::Ok().json(item.0)
                }*/

                // Delete handler
                //
                //generate_delete!($entity)
                //
                if endpoints.contains(EndpointType::Delete) {
                    println!("DeleteDelete");
                    generate_delete!($entity)
                }
                /*#[actix_web::delete("/delete/{id}")]
                pub async fn delete(path: web::Path<(i32,)>) -> HttpResponse {
                    let id = path.into_inner().0;
                    println!("Deleting {} with id {}", stringify!($entity.entity), id);
                    // Implement your delete logic here (e.g., delete from database)
                    HttpResponse::Ok().json(())
                }*/

                // Generate URLs for CRUD operations
                /*pub fn urls() -> Vec<String> {
                    vec![
                        format!("/{}_api/{}/create", stringify!($entity_name).to_lowercase(), stringify!($entity_name)),
                        format!("/{}_api/{}/read/{{id}}", stringify!($entity_name).to_lowercase(), stringify!($entity_name)),
                        format!("/{}_api/{}/update/{{id}}", stringify!($entity_name).to_lowercase(), stringify!($entity_name)),
                        format!("/{}_api/{}/delete/{{id}}", stringify!($entity_name).to_lowercase(), stringify!($entity_name)),
                    ]
                }*/
            }
        }
    };
}