use rocket::{
    fairing::{Fairing, Info, Kind}, fs::NamedFile, get, launch, post, response::stream::{Event, EventStream}, routes, serde::json::Json, tokio::{self, sync::broadcast::{channel, Sender}}, Data, Request, Shutdown, State
};
use serde::{Deserialize, Serialize};

#[get("/")]
async fn home() -> NamedFile {
    NamedFile::open("static/index.html").await.unwrap()
}

#[get("/favicon.ico")]
async fn fav() -> NamedFile {
    NamedFile::open("static/favicon.ico").await.unwrap()
}

#[get("/health")]
fn health() -> &'static str {
    "ok"
}

#[get("/events")]
fn events(queue : &State<Sender<Message>>,mut shutdown: Shutdown) -> EventStream![] {
    let mut rx = queue.subscribe();
    EventStream! {
        loop {
            tokio::select! {
                msg = rx.recv() => {
                    match msg {
                        Ok(msg) => yield Event::json(&msg),
                        Err(_) => break,
                    }
                },
                _ = &mut shutdown => {
                    break;
                }
            }
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Message {
    username: String,
    text: String,
}

#[post("/send", data = "<msg>", format = "application/json" )]
fn send(msg: Json<Message>, queue : &State<Sender<Message>>) {
    queue.send(msg.0).unwrap();
}

pub struct LogIp;

#[rocket::async_trait]
impl Fairing for LogIp {
    fn info(&self) -> Info {
        Info { name: "Log client IP", kind: Kind::Request }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        if let Some(ip) = request.client_ip() {
            println!("Client IP: {}", ip);
        }
    }
}

#[launch]
fn rocket() -> _ {
    let (tx,_rx) = channel::<Message>(1024);
    rocket::build().attach(LogIp)
        .manage(tx)
        .mount("/", routes![
            home,
            fav,
        ])
        .mount("/api/v1", routes![
            health,
            events,
            send,
        ])
}
