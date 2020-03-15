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
fn push(event: String, queue: State<WaitQueue>) {
    queue.0.write().unwrap().push_back(event);
}

#[get("/pop")]
fn pop(queue: State<WaitQueue>) -> Option<String> {
    queue.0.write().unwrap().pop_front()
}

#[put("/leave?<event>")]
fn leave(event: String, queue: State<WaitQueue>) {
    let mut q = queue.0.write().unwrap();
    q.retain(|x| x != &event);
}

#[get("/sse")]
fn sse(queue: State<WaitQueue>) -> SSEresp {
    let q = &queue.0;
    let data =  serde_json::to_string(&q.read().unwrap().iter().collect::<Vec<&String>>()).unwrap(); 
    SSEresp { data: format!("data: {}\n\n", data) }
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("client/dist/").join(file)).ok()
}

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("client/dist/index.html")).ok()
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, leave, files, sse, push, pop])
        .manage(WaitQueue(Arc::new(RwLock::new(VecDeque::new()))))
}

fn main() {
    rocket().launch();
}
