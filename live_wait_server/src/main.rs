#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[cfg(test)]
mod tests;

use std::collections::VecDeque;
use std::path::Path;
use std::path::PathBuf;
use std::sync::RwLock;

use chrono::prelude::Utc;
use rocket::response::NamedFile;
use rocket::State;
use rocket_contrib::json::Json;

use futures_util::stream::Stream;
use live_wait_server::sse;
use tokio::sync::broadcast;

use live_wait_server::Student;

struct WaitQueue(RwLock<VecDeque<Student>>);

fn send_broadcast(queue: State<WaitQueue>, subqueue: State<'_, broadcast::Sender<Vec<Student>>>) {
    let reader = queue.0.read().unwrap();
    let mut to_send = vec![];
    for s in reader.iter() {
        to_send.push(s.clone());
    }
    let _ = subqueue.send(to_send);
}

#[put("/add?<event>")]
fn add(
    event: String,
    queue: State<WaitQueue>,
    subqueue: State<'_, broadcast::Sender<Vec<Student>>>,
) {
    queue.0.write().unwrap().push_back(Student {
        name: event,
        comment: String::from(""),
        join_time: Utc::now(),
    });
    send_broadcast(queue, subqueue);
}

#[put("/push", format = "json", data = "<joinstudent>")]
fn push(
    joinstudent: Json<Student>,
    queue: State<WaitQueue>,
    subqueue: State<'_, broadcast::Sender<Vec<Student>>>,
) {
    let mut student = joinstudent.0;
    student.join_time = Utc::now();
    queue.0.write().unwrap().push_back(student);
    send_broadcast(queue, subqueue);
}

#[get("/pop")]
fn pop(queue: State<WaitQueue>, subqueue: State<'_, broadcast::Sender<Vec<Student>>>) -> String {
    let name = queue.0.write().unwrap().pop_front().unwrap().name;
    send_broadcast(queue, subqueue);
    name
}

#[put("/leave?<event>")]
fn leave(
    event: String,
    queue: State<WaitQueue>,
    subqueue: State<'_, broadcast::Sender<Vec<Student>>>,
) {
    {
        let mut q = queue.0.write().unwrap();
        q.retain(|x| x.name != event);
    }
    send_broadcast(queue, subqueue);
}

#[get("/sse")]
async fn sse(
    wq: State<'_, WaitQueue>,
    queue: State<'_, broadcast::Sender<Vec<Student>>>,
) -> sse::SSE2<impl Stream<Item = sse::Event>> {
    // Subscribe to messages
    let mut subscription = queue.subscribe();

    // Create the SSE stream
    // TODO: Only need async_stream here because subscription does not implement Stream
    let stream = async_stream::stream! {
        loop {
            match subscription.recv().await {
                Ok(student_vec) => {
                    let data = serde_json::to_string(&student_vec).unwrap();
                    yield sse::Event::new(Some("message".into()), Some(data.into()), None);
                }
                Err(broadcast::RecvError::Closed) => break,
                Err(broadcast::RecvError::Lagged(_)) => {
                    yield sse::Event::new(Some("behind".into()), None, None);
                }
            }
        }
    };

    send_broadcast(wq, queue);

    sse::from_stream(stream)
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
        .manage(broadcast::channel::<Vec<Student>>(1024).0)
}

fn main() {
    rocket().launch().unwrap();
}
