#[macro_export]
macro_rules! generate_struct {
    ($entity_name:expr, $($field_name:expr => $field_type:expr),*) => {
        #[derive(Debug, Serialize, Deserialize)]
        struct $entity_name {
            $(
                $field_name: $field_type,
            )*
        }
    };
}

#[macro_export]
macro_rules! generate_struct_2 {
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