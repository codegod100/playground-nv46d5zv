use simple_logger::SimpleLogger;
use serde::{Deserialize,Serialize};
use serde_json::{ json};

#[derive(Debug)]
enum Message {
    Ping,
    Command { name: String, arg: String },
}

impl Message {
    fn read(&self) -> String {
			log::info!("Incoming! {:?}",self);
        match self {
            Message::Command { name, arg } => match name.as_str() {
                "delete" => return format!("Deleting {arg}"),
                _ => {
                    let msg = format!("I don't understand command: '{name}'");
                    if arg == "" {
                        return msg;
                    }
                    return format!("{msg} or its argument: {arg}");
                }
            },
            Message::Ping => return format!("Pong!"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct JSON {
	raw: String
}

fn main() {
    SimpleLogger::new().init().unwrap();

		let data = json!({
			"name": "foo",
			"this": ["will", "be", "json"]
		});
		log::info!("{}",data);

		let str = r#"
		{
			"raw":"json string"
		}"#;
		let parsed: JSON = serde_json::from_str(str).unwrap();
		log::info!("{:?}", parsed);

    let user = format!("V");
    let mut m = Message::Command {
        name: format!("delete"),
        arg: format!("{user}'s files"),
    };
    log::info!("{}", m.read());
    log::info!("{}", Message::Ping.read());
    m = Message::Command {
        name: format!("random"),
        arg: String::from("woo"),
    };
    log::info!("{}", m.read())
}
