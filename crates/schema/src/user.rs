use relay_derive::relay_node;

#[relay_node]
pub struct User {
    name: String,
}