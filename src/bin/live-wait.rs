#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;

use std::io::Cursor;
use std::collections::VecDeque;
use std::path::PathBuf;
use std::path::Path;
use std::sync::RwLock;

use rocket::request::Request;
use rocket::response::NamedFile;
use rocket::response::Response;
use rocket::State;
use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket_contrib::json::Json;
use chrono::prelude::Utc;

use live_wait::Student;

struct WaitQueue(RwLock<VecDeque<Student>>);

impl Responder<'static> for &WaitQueue {
    fn respond_to(self, _: &Request) -> Result<Response<'static>, Status> {
        let data = &self.0.read().unwrap();
        let data = data.iter().collect::<Vec<&Student>>();
        let datavec = serde_json::to_string(&data).unwrap(); 
        let datavec = format!("data: {}\n\n", datavec);
        Response::build()
            .header(ContentType::new("text", "event-stream"))
            // implement something for Read that keeps open and reflects the queue
            .streamed_body(Cursor::new(datavec))
            .ok()
    }
}

#[put("/add?<event>")]
fn add(event: String, queue: State<WaitQueue>) {
    queue.0.write().unwrap()
        .push_back(Student{name: event, comment: String::from(""), join_time: Utc::now()});
}

#[put("/push", format = "json", data = "<joinstudent>")]
fn push(joinstudent: Json<Student>, queue: State<WaitQueue>) {
    let mut student = joinstudent.0;
    student.join_time = Utc::now();
    queue.0.write().unwrap().push_back(student);
}

#[get("/pop")]
fn pop(queue: State<WaitQueue>) -> String {
    queue.0.write().unwrap().pop_front().unwrap().name
}

#[put("/leave?<event>")]
fn leave(event: String, queue: State<WaitQueue>) {
    let mut q = queue.0.write().unwrap();
    q.retain(|x| x.name != event);
}

#[get("/sse")]
fn sse(queue: State<WaitQueue>) -> &WaitQueue {
    queue.inner()
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
        .mount("/", routes![index, add, leave, files, sse, push, pop])
        .manage(WaitQueue(RwLock::new(VecDeque::new())))
}

fn main() {
    rocket().launch();
}
