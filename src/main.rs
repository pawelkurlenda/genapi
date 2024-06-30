use std::fs;
use crate::models::entity_definitions::EntityDefinition;

mod macros;
mod models;


macro_rules! generate_struct {
    ($entity:expr) => {
        {
            let entity_name = &($entity.entity);
            println!("{}", entity_name);

            for field in $entity.fields.iter() {
                println!("{}", &field.name);
            }

            let fields = $entity.fields.iter().map(|f| {
                let f_n = &f.name;
                let f_t = &f.field_type;

                quote::quote! {
                    pub #f_n: #f_t,
                }
            });

            // Correct struct implementation

            quote::quote! {
                #[derive(Debug, Serialize, Deserialize)]
                struct #entity_name {
                    #(#fields)*
                }
            }
        }
    };
}

fn main() {
    let paths = fs::read_dir("./generated_models").unwrap();

    let mut entity_definitions: Vec<EntityDefinition> = Vec::new();

    for path in paths {
        let path = path.unwrap().path();
        let entity_definition = EntityDefinition::new(&path);
        entity_definitions.push(entity_definition);
    }

    for entity in &entity_definitions {
        let struct_definition = generate_struct!(entity);
    }

    println!("Hello, world!");
}


// Define entity definitions here (or read from JSON as before)
//generate_crud_handlers!("User", "id" => i32, "username" => String, "email" => String);
//generate_crud_handlers!("Product", "id" => i32, "name" => String, "price" => f64);
