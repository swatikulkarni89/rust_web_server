use serde::{Serialize,Deserialize};
#[derive(Debug,Serialize,Deserialize)]

pub struct Todo{

    first_name: String,
    last_name: String,
    active: i8,
    email: String,
    address_id: i32,
    address: String,
    address2: Option<String>,
    city_id: i16,
    postal_code:String
}
 #[tokio::main]
pub async fn get_address() ->Result<Vec<Todo>,reqwest::Error>{

let todos:Vec<Todo>=reqwest::Client::new()
.get("http://localhost:8080/api/user/list/customerAddressService1/33")
.basic_auth("user", Some("password"))
.send()
.await?
.json()
.await?;

println!("{:#?}",todos);
Ok(todos)
}
