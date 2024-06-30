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