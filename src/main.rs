use std::io;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug)]
struct Node {
    id: String,
    msg_id: u128,
}

#[derive(Default, Serialize, Deserialize, Debug)]
struct MessageBody {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    msg_type: String,
    msg_id: u128,

    #[serde(default)]
    in_reply_to: u128,

    #[serde(default, skip_serializing)]
    node_id: String,

    #[serde(default)]
    echo: String,
}

#[derive(Default, Serialize, Deserialize, Debug)]
struct Message {
    src: String,
    dest: String,
    body: MessageBody,
}

#[tokio::main]
async fn  main() -> io::Result<()> {
    let mut node: Node = Default::default();

    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        eprint!("Received: {}", buffer);

        let msg: Message = serde_json::from_str(&buffer)?;
        let body = msg.body;

        let mut reply: MessageBody = Default::default();

        match body.msg_type.as_str() {
            "init" => {
                node.id = body.node_id;
                reply.msg_type = "init_ok".to_string();
            },
            "echo" => {
                reply.msg_type = "echo_ok".to_string();
                reply.echo = body.echo;
            },
            _ => {
                eprintln!("Unknown message type: {}", body.msg_type);
                continue;
            }
        }

        node.msg_id += 1;
        reply.msg_id = node.msg_id;

        reply.in_reply_to = body.msg_id;
        let out = Message {
            src: node.id.clone(),
            dest: msg.src,
            body: reply,
        };

        let out_str = serde_json::to_string(&out)?;
        eprintln!("Sending: {}", out_str);
        println!("{}", out_str);
    }
}
