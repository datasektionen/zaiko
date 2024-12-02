use serde::Serialize;

#[derive(Serialize)]
struct Supplier {
    name: Option<String>,
    url: Option<String>,
    order_specification: Option<String>,
}
