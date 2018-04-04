extern crate irc;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

use irc::client::prelude::*;
use reqwest::{Client, Url};
use serde_json::{from_str, Value};

fn main() {
    let config = Config {
        nickname: Some("taigabot".to_owned()),
        server: Some("irc.snoonet.org".to_owned()),
        channels: Some(vec!["##taigacult sonic".to_owned()]),
        ..Config::default()
    };

    let mut reactor = IrcReactor::new().unwrap();
    let client = reactor.prepare_client_and_connect(&config).unwrap();
    client.identify().unwrap();

    reactor.register_client_with_handler(client, |client, message| {
        let req_client = Client::new();

        if let Command::PRIVMSG(ref target, ref msg) = message.command {
            if msg.starts_with("!/taiga") {
                let mut value =
                    get_post("https://reddit.com/r/taiga/random.json", &req_client).unwrap();
                while value[0]["data"]["children"][0]["data"]["is_self"]
                    .as_bool()
                    .unwrap()
                {
                    value =
                        get_post("https://reddit.com/r/taiga/random.json", &req_client).unwrap();
                }

                send_link(
                    value[0]["data"]["children"][0]["data"]["url"]
                        .as_str()
                        .unwrap(),
                    client,
                    target,
                );
            } else if msg.starts_with("!/toradora") {
                let mut value =
                    get_post("https://reddit.com/r/toradora/random.json", &req_client).unwrap();
                while value[0]["data"]["children"][0]["data"]["is_self"]
                    .as_bool()
                    .unwrap()
                {
                    value =
                        get_post("https://reddit.com/r/toradora/random.json", &req_client).unwrap();
                }

                send_link(
                    value[0]["data"]["children"][0]["data"]["url"]
                        .as_str()
                        .unwrap(),
                    client,
                    target,
                );
            }
        }
        Ok(())
    });

    reactor.run().unwrap();
}

fn get_post(url: &str, client: &reqwest::Client) -> Result<Value, reqwest::Error> {
    let request: String = client
        .get(Url::parse(&url).unwrap())
        .send()?
        .text()
        .unwrap();
    let value: Value = from_str(request.as_str()).unwrap();
    Ok(value)
}

fn send_link(link: &str, client: &irc::client::IrcClient, target: &String) {
    client.send_privmsg(target, &link).unwrap();
}
