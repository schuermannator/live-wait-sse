#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate crossbeam;

#[cfg(test)] mod tests;

use std::io::Cursor;
use std::collections::VecDeque;
use std::path::PathBuf;
use std::path::Path;
use std::sync::{Arc, RwLock};

use rocket::request::Request;
use rocket::response::NamedFile;
use rocket::response::Response;
use rocket::State;
use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use crossbeam::queue::SegQueue;
use serde::{Serialize, Deserialize};

//#[derive(Serialize, Deserialize)]
//struct WaitQueue(SegQueue<String>);
//#[derive(Debug)]
//struct WaitQueue(VecDeque<String>);
struct WaitQueue(Arc<RwLock<VecDeque<String>>>);

//struct SSEresp(String);
struct SSEresp {
    data: String,
}

impl Responder<'static> for SSEresp {
    fn respond_to(self, _: &Request) -> Result<Response<'static>, Status> {
        Response::build()
            .header(ContentType::new("text", "event-stream"))
            .sized_body(Cursor::new(self.data))
            .ok()
    }
}

#[put("/push?<event>")]
//fn push(event: String, queue: VecDeque<String>) {
fn push(event: String, queue: State<WaitQueue>) {
    queue.0.write().unwrap().push_back(event);
    //queue.0.push(event);
}

#[get("/pop")]
fn pop(queue: State<WaitQueue>) -> Option<String> {
    queue.0.write().unwrap().pop_front()
    //queue.0.pop().ok()
}

#[put("/leave?<event>")]
fn leave(event: String, queue: State<WaitQueue>) {
    let mut q = queue.0.write().unwrap();
    q.retain(|x| x != &event);
}

#[get("/sse")]//, format = "text/event-stream")]
fn sse(queue: State<WaitQueue>) -> SSEresp {
    //let ser = serde_json::to_string(&queue.0).unwrap();
    //queue.0.push_back(String::from("HHI"));
    //queue.0.push_back(String::from("HI"));
    let q = &queue.0;
    let data =  serde_json::to_string(&q.read().unwrap().iter().collect::<Vec<&String>>()).unwrap(); 

    //let q = &queue;
    println!("data vec: {}\n\n", data);
    //let data = r#"{ "res": ["test1", "test2"]}"#;
    SSEresp { data: format!("data: {}\n\n", data) }
    //SSEresp(String::from(format!("data: {}\n\n", 45)))
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("client/dist/").join(file)).ok()
}

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("client/dist/index.html")).ok()
}


//#[get("/sse", format = "text/event-stream")]
//fn sse() {
//    let mut resp = Response::new();
//    let sse_content_type = ContentType::new("text", "event-stream");
//
//    resp.set_header(sse_content_type);
//    resp.adjoin_raw_header("Cache-Control", "no-cache");
//    resp.adjoin_raw_header("Connection", "keep-alive");
//
//    resp.set_sized_body(Cursor::new("idk"));
//    resp.finalize();
//    resp.ok()
//}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, leave, files, sse, push, pop])
        //.manage(WaitQueue(SegQueue::new()))
        //.manage(WaitQueue(VecDeque::new()))
        .manage(WaitQueue(Arc::new(RwLock::new(VecDeque::new()))))
}

fn main() {
    rocket().launch();
}
