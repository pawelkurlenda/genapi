#[macro_export]
macro_rules! generate_get_list {
    //($entity:expr, $app:ident) => {
    ($entity:expr) => {

        let entity_name = &($entity.entity);
        let mod_name = format!("{}{}", entity_name, "_get_list");
        let url = format!("/{}");

        quote::quote! {
            pub mod #mod_name {
                use super::*;
                use actix_web::{web, HttpResponse};

                //#[utoipa::path()]
                #[actix_web::get(#url)]
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

        /*quote::quote! {
            $app.service(#mod_name::read)
        }*/
    };
}

#[macro_export]
macro_rules! generate_get_by_id {
    ($entity:expr) => {

        let entity_name = &($entity.entity);
        let mod_name = format!("{}{}", entity_name, "_get_by_id");
        let url = format!("/{}/{}", entity_name, "{id}");

        quote::quote! {
            pub mod #mod_name {
                use super::*;
                use actix_web::{web, HttpResponse};

                //#[utoipa::path()]
                #[actix_web::get(#url)]
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

        /*quote::quote! {
            $app.service(#mod_name::read)
        }*/
    };
}

#[macro_export]
macro_rules! generate_create {
    //($entity:expr, $app:ident) => {
    ($entity:expr) => {

        let entity_name = &($entity.entity);
        let mod_name = format!("{}{}", entity_name, "_create");
        let url = format!("/{}", entity_name);

        quote::quote! {
            pub mod #mod_name {
                use super::*;
                use actix_web::{web, HttpResponse};

                //#[utoipa::path()]
                #[actix_web::post(#url)]
                pub async fn create(item: web::Json<$entity.entity>) -> impl Responder {
                    println!("Creating {:?}", item);
                    // Implement your create logic here (e.g., saving to database)
                    HttpResponse::Ok().json(item.0)
                }
            }
        }

        /*quote::quote! {
            $app.service(#mod_name::create)
        }*/
    };
}

#[macro_export]
macro_rules! generate_update {
    //($entity:expr, $app:ident) => {
    ($entity:expr) => {

        let entity_name = &($entity.entity);
        let mod_name = format!("{}{}", entity_name, "_update");
        let url = format!("/{}/{}", entity_name, "{id}");

        quote::quote! {
            pub mod #mod_name {
                use super::*;
                use actix_web::{web, HttpResponse};

                //#[utoipa::path()]
                #[actix_web::put(#url)]
                pub async fn update(path: web::Path<(i32,)>, item: web::Json<$entity.entity>) -> impl Responder {
                    let id = path.into_inner().0;
                    println!("Updating {} with id {}: {:?}", stringify!($entity.entity), id, item);
                    // Implement your update logic here (e.g., update in database)
                    HttpResponse::Ok().json(item.0)
                }
            }
        }

        /*quote::quote! {
            $app.service(#mod_name::update)
        }*/
    };
}

#[macro_export]
macro_rules! generate_delete {
    //($entity:expr, $app:ident) => {
    ($entity:expr) => {

        let entity_name = &($entity.entity);
        let mod_name = format!("{}{}", entity_name, "_delete");
        let url = format!("/{}/{}", entity_name, "{id}");

        quote::quote! {
            pub mod #mod_name {
                use super::*;
                use actix_web::{web, HttpResponse};

                //#[utoipa::path()]
                #[actix_web::delete(#url)]
                pub async fn delete(path: web::Path<(i32,)>) -> impl Responder {
                    let id = path.into_inner().0;
                    println!("Deleting {} with id {}", stringify!($entity.entity), id);
                    // Implement your delete logic here (e.g., delete from database)
                    HttpResponse::Ok().json(())
                }
            }
        }

        /*quote::quote! {
            $app.service(#mod_name::delete)
        }*/
    };
}
