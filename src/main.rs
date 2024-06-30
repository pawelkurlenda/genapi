mod macros;
mod models;

fn main() {
    println!("Hello, world!");
}



// Define entity definitions here (or read from JSON as before)
generate_crud_handlers!("User", "id" => i32, "username" => String, "email" => String);
generate_crud_handlers!("Product", "id" => i32, "name" => String, "price" => f64);
