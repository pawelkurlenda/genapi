#[macro_export]
macro_rules! generate_get_by_id {
    ($entity:expr) => {
        // Generate struct definition
        //generate_struct_2!($entity);

        let entity_name = &($entity.entity);

        // Generate CRUD handlers
        quote::quote! {

            // Read handler
            #[actix_web::get("/{id}")]
            pub async fn read(path: web::Path<(i32,)>) -> HttpResponse {
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
    };
}

#[macro_export]
macro_rules! generate_get_list {
    ($entity:expr) => {
        // Generate struct definition
        //generate_struct_2!($entity);

        let entity_name = &($entity.entity);

        // Generate CRUD handlers
        quote::quote! {

            // Read handler
            #[actix_web::get("/{id}")]
            pub async fn read(path: web::Path<(i32,)>) -> HttpResponse {
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
    };
}

#[macro_export]
macro_rules! generate_create {
    ($entity:expr) => {
        // Generate struct definition
        //generate_struct_2!($entity);

        let entity_name = &($entity.entity);

        // Generate CRUD handlers
        quote::quote! {

            // Create handler
            #[actix_web::post("/create")]
            pub async fn create(item: web::Json<$entity.entity>) -> HttpResponse {
                println!("Creating {:?}", item);
                // Implement your create logic here (e.g., saving to database)
                HttpResponse::Ok().json(item.0)
            }
        }
    };
}

#[macro_export]
macro_rules! generate_update {
    ($entity:expr) => {
        // Generate struct definition
        //generate_struct_2!($entity);

        let entity_name = &($entity.entity);

        // Generate CRUD handlers
        quote::quote! {

            // Update handler
            #[actix_web::put("{id}")]
            pub async fn update(path: web::Path<(i32,)>, item: web::Json<$entity.entity>) -> HttpResponse {
                let id = path.into_inner().0;
                println!("Updating {} with id {}: {:?}", stringify!($entity.entity), id, item);
                // Implement your update logic here (e.g., update in database)
                HttpResponse::Ok().json(item.0)
            }
        }
    };
}

#[macro_export]
macro_rules! generate_delete {
    ($entity:expr) => {
        // Generate struct definition
        //generate_struct_2!($entity);

        let entity_name = &($entity.entity);

        // Generate CRUD handlers
        quote::quote! {

            // Delete handler
            #[actix_web::delete("/delete/{id}")]
            pub async fn delete(path: web::Path<(i32,)>) -> HttpResponse {
                let id = path.into_inner().0;
                println!("Deleting {} with id {}", stringify!($entity.entity), id);
                // Implement your delete logic here (e.g., delete from database)
                HttpResponse::Ok().json(())
            }
        }
    };
}
