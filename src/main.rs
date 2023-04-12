use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use serde_json::Value;
use serde_json::json;
mod get_address;



fn main() {


   let listener
   =TcpListener::bind("192.168.0.171:7878").unwrap();
    for stream in listener.incoming(){
    let stream=stream.unwrap();
    handle_connection(stream);
    
}
}
fn handle_connection(mut stream:TcpStream){
    let mut buffer=[0;1024];
    let  streamio=stream.read(&mut buffer).unwrap();
    
    let get=b"GET / HTTP/1.1\r\n";
    
    
    

let ( status,filename)=
if buffer.starts_with(get){

    ("HTTP/1.1 200 OK","index.html")
    

}
else{
    ("HTTP/1.1 404 error", "404.html")

};
let content= fs::read_to_string(filename).unwrap();

let responce =format!
("{}\r\nContent-Length: {} {}",
  status,
content.len(),
content

);
println!("{}",responce);
let res= get_address1();
stream.write(responce.as_bytes()).unwrap();
stream.flush().unwrap();
   
}
fn get_address1 ()->String{
        let res: Result<Vec<get_address::Todo>, reqwest::Error> = get_address::get_address();
        
        let json_object: Value = match res {
            Ok(Todo)  =>json!(Todo),
            Err(err) => json!({ "status": "error", "message": err.to_string() }),
        };
           
        let json_string = serde_json::to_string(&json_object).unwrap();
        println!("{}",json_string);
        serde_json::to_string(&json_object).expect("Failed to serialize object to JSON")
    }