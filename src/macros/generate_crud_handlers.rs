use crate::{generate_create, generate_delete, generate_get_by_id, generate_get_list, generate_struct, generate_update};
#[macro_export]
macro_rules! generate_crud_handlers_2 {
    //($entity:expr, $app:ident) => {
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

                let entity_name = &($entity.entity);
                let mod_name = format!("{}{}", entity_name, "_get_list");

                /*quote::quote! {
                    $app.service(#mod_name::read)
                }*/
            }
            else if a.to_str() == "GET_LIST" {
                //println!("GET_LIST, {:?}", a);
                //generate_get_list!($entity, $app);
            }
            else if a.to_str() == "CREATE" {
                //println!("CREATE, {:?}", a);
                //generate_create!($entity, $app);
            }
            else if a.to_str() == "UPDATE" {
                //println!("UPDATE, {:?}", a);
                //generate_update!($entity, $app);
            }
            else if a.to_str() == "DELETE" {
                //println!("DELETE, {:?}", a);
                //generate_delete!($entity, $app);
            }
        }
    };
}

#[macro_export]
macro_rules! generate_crud_handlers_new {
    ($entity:expr, $app:expr) => {

        let entity_name = &($entity.entity);

        pub mod entity_name {
            use super::*;
            use actix_web::{web, HttpResponse};

            //pub fn configure(cfg: &mut web::ServiceConfig) {
                let mut scope = web::scope(stringify!(entity_name).to_lowercase().as_str());

                if $entity.endpoint_types.contains(&"CREATE".to_string()) {
                    #[actix_web::post("/create")]
                    async fn create(item: web::Json<super::entity_name>) -> HttpResponse {
                        HttpResponse::Ok().json(item.0)
                    }
                    scope = scope.route("/create", web::post().to(create));
                }

                if $entity.endpoint_types.contains(&"GET_BY_ID".to_string()) {
                    #[actix_web::get("/read/{id}")]
                    async fn read(path: web::Path<(i32,)>) -> HttpResponse {
                        let id = path.into_inner().0;
                        HttpResponse::Ok().body(format!("Read entity {} with id {}", stringify!(entity_name), id))
                    }
                    scope = scope.route("/read/{id}", web::get().to(read));
                }

                if $entity.endpoint_types.contains(&"GET_LIST".to_string()) {
                    #[actix_web::get("/list")]
                    async fn list() -> HttpResponse {
                        HttpResponse::Ok().body(format!("List of {}", stringify!(entity_name)))
                    }
                    scope = scope.route("/list", web::get().to(list));
                }

                if $entity.endpoint_types.contains(&"UPDATE".to_string()) {
                    #[actix_web::put("/update/{id}")]
                    async fn update(path: web::Path<(i32,)>, item: web::Json<super::entity_name>) -> HttpResponse {
                        let id = path.into_inner().0;
                        HttpResponse::Ok().json(item.0)
                    }
                    scope = scope.route("/update/{id}", web::put().to(update));
                }

                if $entity.endpoint_types.contains(&"DELETE".to_string()) {
                    #[actix_web::delete("/delete/{id}")]
                    async fn delete(path: web::Path<(i32,)>) -> HttpResponse {
                        let id = path.into_inner().0;
                        HttpResponse::Ok().body(format!("Deleted entity {} with id {}", stringify!(entity_name), id))
                    }
                    scope = scope.route("/delete/{id}", web::delete().to(delete));
                }

                cfg.service(scope);
            //}

            /*impl crate::Configurable for $entity {
                fn configure(cfg: &mut web::ServiceConfig) {
                    configure(cfg);
                }
            }*/
        }
    };
}