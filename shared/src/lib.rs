use std::io::{Read, Write};
use std::net::TcpStream;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Hello,
    Welcome(Welcome),
    Subscribe(Subscribe),
    SubscribeResult(SubscribeResult),
    PublicLeaderBoard(PublicLeaderBoard),
    ChallengeResult(ChallengeResult),
    Challenge(Challenge),
    RoundSummary(RoundSummary),
    EndOfGame(EndOfGame),

}

#[derive(Debug, Serialize, Deserialize)]
pub enum Challenge {
    MD5HashCash(MD5HashCashInput),
    MonstrousMaze(MonstrousMazeInput),
}

fn serialize(message : Message) -> String {
    let json = serde_json::to_string(&message);
    return json.unwrap();
}

pub fn send(stream: &mut TcpStream, message : Message){
    let json = serialize(message);
    let length = json.len() as u32;
    let length = length.to_be_bytes();
    stream.write_all(&length).unwrap();
    stream.write_all(&json.as_bytes()).unwrap();
}

pub fn receive(stream: &mut TcpStream) -> Vec<u8> {
    let mut line:[u8 ; 4] = [0 ; 4];
    match stream.read_exact(&mut line) {
        Ok(_) => {
            let length = u32::from_be_bytes(line) as usize;
            let mut line : Vec<u8> = vec![0u8; length];
            match stream.read_exact(&mut line) {
                Ok(_) => {
                    return line;
                },
                Err(e) => {
                    panic!("Failed to receive line: {}", e);
                }
            }
        },
        Err(e) => {
            panic!("Failed to receive line: {}", e);
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Hello;

impl Hello {
    pub fn init() -> Hello {
        return Hello;
    }
    pub fn to_string() -> String {
        return serde_json::to_string(&Hello).unwrap();
    }
    pub fn from_str(s: &str) -> Hello {
        if s == "Hello" {
            return Hello::init();
        }
        panic!("Expected 'Hello', got '{}'", s);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Welcome {
    pub version: u8,
}

impl Welcome {
    pub fn init(version: u8) -> Welcome {
        return Welcome { version };
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subscribe {
    pub name: String
}

impl Subscribe {
    pub fn init(name: String) -> Subscribe {
        return Subscribe { name };
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SubscribeError {
    AlreadyRegistered,
    InvalidName
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SubscribeResult {
    Ok,
    Err(SubscribeError),
}

impl SubscribeResult {
    pub fn init(is_ok: bool, error: SubscribeError) -> SubscribeResult {
        if is_ok {
            return SubscribeResult::Ok;
        }
        return SubscribeResult::Err(error);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicPlayer {
    pub name: String,
    pub stream_id: String,
    pub score: i32,
    pub steps: u32,
    pub is_active: bool,
    pub total_used_time: f64,
}

impl PublicPlayer {
    pub fn init(name: String, stream_id: String, score: i32, steps: u32, is_active: bool, total_used_time: f64) -> PublicPlayer {
        return PublicPlayer { name, stream_id, score, steps, is_active, total_used_time}
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicLeaderBoard {
    #[serde(rename(serialize = "PublicLeaderBoard", deserialize = "PublicLeaderBoard"))]
    pub players: Vec<PublicPlayer>,
}

impl PublicLeaderBoard {
    pub fn init(players: Vec<PublicPlayer>) -> PublicLeaderBoard {
        return PublicLeaderBoard { players };
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MD5HashCashInput {
    pub complexity: u32,
    pub message: String
}

impl MD5HashCashInput {
    pub fn init(complexity: u32, message: String) -> MD5HashCashInput {
        return MD5HashCashInput { complexity, message };
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MonstrousMazeInput {
    pub grid: String,
    pub endurance: u8
}

impl MonstrousMazeInput {
    pub fn init(grid: String, endurance: u8) -> MonstrousMazeInput {
        return MonstrousMazeInput { grid, endurance };
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ChallengeType {
    MD5HashCash(MD5HashCashInput),
    MonstrousMaze(MonstrousMazeInput)
}

impl ChallengeType {
    pub fn initMD5HashCashInput(complexity: u32, message: String) -> ChallengeType {
        return ChallengeType::MD5HashCash(MD5HashCashInput::init(complexity,message));
    }
    pub fn initMonstrousMazeInput(grid: String, endurance: u8) -> ChallengeType {
        return ChallengeType::MonstrousMaze(MonstrousMazeInput::init(grid,endurance));
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MD5HashCashOutput {
    pub seed: u64,
    pub hashcode: String
}

impl MD5HashCashOutput {
    pub fn init(seed: u64, hashcode: String) -> MD5HashCashOutput {
        return MD5HashCashOutput { seed, hashcode };
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MonstrousMazeOutput {
    pub path: String
}

impl MonstrousMazeOutput {
    pub fn init(path: String) -> MonstrousMazeOutput{
        return MonstrousMazeOutput { path };
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ChallengeAnswer {
    MD5HashCash(MD5HashCashOutput),
    MonstrousMaze(MonstrousMazeOutput)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChallengeResult {
    pub answer: ChallengeAnswer,
    pub next_target: String
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ChallengeValue {
    Unreachable,
    Timeout,
    BadResult {
        used_time: f64,
        next_target: String
    },
    Ok {
        used_time: f64,
        next_target: String
    }
}

impl ChallengeValue {
    pub fn initUnreachable() -> ChallengeValue {
        return ChallengeValue::Unreachable;
    }
    pub fn initTimeout() -> ChallengeValue {
        return ChallengeValue::Timeout;
    }
    pub fn initBadResult(used_time: f64, next_target: String) -> ChallengeValue {
        return ChallengeValue::BadResult { used_time, next_target };
    }
    pub fn initOk(used_time: f64, next_target: String) -> ChallengeValue {
        return ChallengeValue::Ok { used_time, next_target };
    }

}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReportedChallengeResult {
    pub name: String,
    pub value: ChallengeValue,
}

impl ReportedChallengeResult {
    pub fn init(name: String, value: ChallengeValue) -> ReportedChallengeResult {
        return ReportedChallengeResult { name, value };
    }

}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoundSummaryAnswer {
    pub challenge: String,
    pub chain: Vec<ReportedChallengeResult>
}

impl RoundSummaryAnswer {
    pub fn init(challenge: String, chain: Vec<ReportedChallengeResult>) -> RoundSummaryAnswer {
        return RoundSummaryAnswer { challenge, chain }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoundSummary {
    #[serde(rename(serialize = "RoundSummary", deserialize = "RoundSummary"))]
    pub answer: RoundSummaryAnswer
}

impl RoundSummary {
    pub fn init(challenge: String, chain: Vec<ReportedChallengeResult>) -> RoundSummary {
        return RoundSummary { answer: RoundSummaryAnswer { challenge, chain } }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EndOfGameAnswer {
    pub leader_board: Vec<PublicPlayer>
}

impl EndOfGameAnswer {
    pub fn init(leader_board: Vec<PublicPlayer>) -> EndOfGameAnswer {
        return EndOfGameAnswer { leader_board }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EndOfGame {
    #[serde(rename(serialize = "EndOfGame", deserialize = "EndOfGame"))]
    pub answer:  EndOfGameAnswer
}

impl EndOfGame {
    pub fn init(leader_board :  Vec<PublicPlayer>) -> EndOfGame {
        return EndOfGame { answer: EndOfGameAnswer { leader_board } }
    }
}