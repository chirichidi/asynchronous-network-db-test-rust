use std::collections::HashMap;
use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};
use std::sync::{Arc, Mutex};
use bytes::Bytes;

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:6379").await.unwrap();
    println!("Listening");

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = listener.accept().await.unwrap();

        let db = db.clone();

        println!("Accepted");
        tokio::spawn(async move  {
            process(socket, db).await;
        });
    }

    //TODO ivl 에선 ftp binlog download(E) 를 tokio::spawn 으로 시작해서 내부 로직 플로우(TL) 하면 될 듯
    //TODO TL task 도 분리해서 병렬 처리하고 싶으면 tokio::spawn 여러쌍 생성후 채널로 파이프라이닝하면 될 듯
}

async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Set};

    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        connection.write_frame(&response).await.unwrap();
    }
}