extern crate core;
use std::net::TcpStream;
mod hash_cash_challenge;
use std::io::{Write, Read, stdin, stdout};
use serde_json::from_str;
use std::str::from_utf8;
use rand::Rng;
use shared::{ChallengeAnswer, ChallengeResult, Message, PublicLeaderBoard, PublicPlayer, Subscribe, SubscribeResult};
use shared::Challenge::{MD5HashCash, MonstrousMaze};


fn get_entry() -> String {
    let mut buf = String::new();

    stdin().read_line(&mut buf);
    buf.replace("\n", "").replace("\r", "")
}

fn exchange_with_server(mut stream: TcpStream) {
    let stdout = std::io::stdout();
    let mut io = stdout.lock();
    let mut buf = &mut [0; 3];

    println!("Enter 'quit' when you want to leave");

    shared::send(&mut stream, Message::Hello);
    let response = shared::receive(&mut stream);
    let res = from_utf8(&response);
    match res {
        Ok(resp) => {
            println!("{}", resp);
        }
        Err(err) => {
            println!("{}", err);
        }
    }

    shared::send(&mut stream, Message::Subscribe(Subscribe { name: "Wissm".to_string() }));
    let response = shared::receive(&mut stream);
    let res = from_utf8(&response);
    match res {
        Ok(resp) => {
            println!("{}", resp);
        }
        Err(err) => {
            println!("{}", err);
        }
    }

    loop {
        let response = shared::receive(&mut stream);
        let res = from_utf8(&response);
        match res {
            Ok(resp) => {
                let response = serde_json::from_str(resp);
                match response {
                    Ok(res) => {
                        match res {
                            Message::EndOfGame(..) => {
                                break;
                            }
                            Message::Challenge(response) => {
                                match response {
                                    MD5HashCash(md5_hash_cash_input) => {
                                        shared::send(&mut stream, Message::ChallengeResult(
                                            ChallengeResult {
                                                answer: ChallengeAnswer::MD5HashCash {
                                                    0: hash_cash_challenge::md5hashage(md5_hash_cash_input),
                                                },
                                                next_target: "{\"Challenge\":{\"MD5HashCash\":{\"complexity\":5,\"message\":\"Hello\"}}}".to_string()
                                            }
                                        ));
                                        let response = shared::receive(& mut stream);
                                        let res = from_utf8(&response);
                                        match res {
                                            Ok(resp) => {
                                                println!("{}", resp);
                                            }
                                            Err(err) => {
                                                println!("{}", err);
                                            }
                                        }

                                    },
                                    MonstrousMaze(monstrous_maze_input) => {
                                        break;
                                    }
                                }
                            }
                            Message::SubscribeResult(res) => {
                                match res {
                                    SubscribeResult::Ok => {}
                                    SubscribeResult::Err(..) => {
                                        break;
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    Err(err) => {
                        println!("{}", err);
                    }
                }
                println!("Réponse du serveur : {:?}", buf);
            }
            Err(err) => {
                println!("{}", err);
            }
        }
        let response= from_utf8(&response).unwrap();
        println!("{}", response);
    }
}

fn main() {
    println!("Tentative de connexion au serveur...");
    let stream = std::net::TcpStream::connect("127.0.0.1:7878");
    match stream {
        Ok(stream) => {
            println!("Connexion au serveur réussie !");
            exchange_with_server(stream);
        }
        Err(e) => {
            println!("La connexion au serveur a échoué : {}", e);
        }
    }
}