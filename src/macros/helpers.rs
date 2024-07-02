#[macro_export]
macro_rules! is_endpoint_to_generate {
    ($entity:expr, $a:expr) => {
        $entity.endpoint_types.contains(&$a)
    };
}