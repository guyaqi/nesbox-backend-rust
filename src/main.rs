use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use listenfd::ListenFd;
use std::fs;
use std::ffi::OsStr;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Deserialize,Serialize)]
pub struct ListData {
    // s: u32,
    list: Vec<String>
}

fn getList() -> ListData {
    let game_root = "../nesbox-games";
    let paths = fs::read_dir(game_root).unwrap();
    let mut v: Vec<String> = vec![];
    for path in paths {
        let p = path.unwrap().path();
        let p = p.file_name().unwrap();
        if p == OsStr::new(".git") {
            continue
        };
        if p == OsStr::new("README.md") {
            continue
        };
        let p = p.to_str().unwrap();
        
        v.push(String::from(p));
    }
    ListData {
        list: v
    }
}

async fn index() -> impl Responder {
    
    let list = getList();
    let res = serde_json::to_string(&list).unwrap();
    print!("res");
    HttpResponse::Ok().body(res)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();
    let mut server =  HttpServer::new(|| {
        App::new().route("/api/nesbox-games", web::get().to(index))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:3000")?
    };
    server.run().await
}