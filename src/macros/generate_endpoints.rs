#[macro_export]
macro_rules! generate_get_list {
    ($entity:expr) => {
        // Generate struct definition
        //generate_struct!($entity);

        let entity_name = &($entity.entity);
        let mod_name = format!("{}{}", entity_name, "_get_list");

        println!("generate_get_list");
        println!("{}", mod_name);

        // Generate CRUD handlers
        quote::quote! {
            mod #mod_name {
                use super::*;
                use actix_web::{web, HttpResponse};

                // Read handler
                #[utoipa::path()]
                #[actix_web::get("/")]
                pub async fn read(path: web::Path<(i32,)>) -> impl Responder {
                    let id = path.into_inner().0;
                    println!("Reading {} with id {}", stringify!($entity.entity), id);
                    // Implement your read logic here (e.g., fetch from database)
                    /*let item = $entity.entity {
                        $($field_name: Default::default()),*
                    };*/
                    //HttpResponse::Ok().json(item)
                    HttpResponse::Ok()
                }
            }
        }
    };
}

#[macro_export]
macro_rules! generate_get_by_id {
    ($entity:expr) => {
        // Generate struct definition
        //generate_struct!($entity);

        let entity_name = &($entity.entity);
        let mod_name = format!("{}{}", entity_name, "_get_by_id");

        println!("generate_get_by_id");
        println!("{}", mod_name);

        // Generate CRUD handlers
        quote::quote! {
            mod #mod_name {
                use super::*;
                use actix_web::{web, HttpResponse};

                // Read handler
                #[utoipa::path()]
                #[actix_web::get("/{id}")]
                pub async fn read(path: web::Path<(i32,)>) -> impl Responder {
                    let id = path.into_inner().0;
                    println!("Reading {} with id {}", stringify!($entity.entity), id);
                    // Implement your read logic here (e.g., fetch from database)
                    /*let item = $entity.entity {
                        $($field_name: Default::default()),*
                    };*/
                    //HttpResponse::Ok().json(item)
                    HttpResponse::Ok()
                }
            }
        }
    };
}

#[macro_export]
macro_rules! generate_create {
    ($entity:expr) => {
        // Generate struct definition
        //generate_struct!($entity);

        let entity_name = &($entity.entity);
        let mod_name = format!("{}{}", entity_name, "_create");

        println!("generate_create");
        println!("{}", mod_name);

        // Generate CRUD handlers
        quote::quote! {
            mod #mod_name {
                use super::*;
                use actix_web::{web, HttpResponse};

                // Create handler
                #[utoipa::path()]
                #[actix_web::post("/")]
                pub async fn create(item: web::Json<$entity.entity>) -> impl Responder {
                    println!("Creating {:?}", item);
                    // Implement your create logic here (e.g., saving to database)
                    HttpResponse::Ok().json(item.0)
                }
            }
        }
    };
}

#[macro_export]
macro_rules! generate_update {
    ($entity:expr) => {
        // Generate struct definition
        //generate_struct!($entity);

        let entity_name = &($entity.entity);
        let mod_name = format!("{}{}", entity_name, "_update");

        println!("generate_update");
        println!("{}", mod_name);

        // Generate CRUD handlers
        quote::quote! {
            mod #mod_name {
                use super::*;
                use actix_web::{web, HttpResponse};

                // Update handler
                #[utoipa::path()]
                #[actix_web::put("/{id}")]
                pub async fn update(path: web::Path<(i32,)>, item: web::Json<$entity.entity>) -> impl Responder {
                    let id = path.into_inner().0;
                    println!("Updating {} with id {}: {:?}", stringify!($entity.entity), id, item);
                    // Implement your update logic here (e.g., update in database)
                    HttpResponse::Ok().json(item.0)
                }
            }
        }
    };
}

#[macro_export]
macro_rules! generate_delete {
    ($entity:expr) => {
        // Generate struct definition
        //generate_struct!($entity);

        let entity_name = &($entity.entity);
        let mod_name = format!("{}{}", entity_name, "_delete");

        println!("generate_delete");
        println!("{}", mod_name);

        // Generate CRUD handlers
        quote::quote! {
            mod #mod_name {
                use super::*;
                use actix_web::{web, HttpResponse};

                // Delete handler
                #[utoipa::path()]
                #[actix_web::delete("/{id}")]
                pub async fn delete(path: web::Path<(i32,)>) -> impl Responder {
                    let id = path.into_inner().0;
                    println!("Deleting {} with id {}", stringify!($entity.entity), id);
                    // Implement your delete logic here (e.g., delete from database)
                    HttpResponse::Ok().json(())
                }
            }
        }
    };
}
