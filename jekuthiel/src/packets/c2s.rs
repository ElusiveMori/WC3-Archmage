#![allow(dead_code)]

use bytes::*;
use super::*;

type E = LittleEndian;

const BNET_HEADER: u8 = 0xff;
const BNET_HEADER_LENGTH: u16 = 4;
fn new_packet(id: PacketID, length: usize) -> BytesMut {
    let mut buf = BytesMut::with_capacity(BNET_HEADER_LENGTH as usize + length);
    buf.put(BNET_HEADER);
    buf.put(id as u8);
    buf.put_u16::<E>(BNET_HEADER_LENGTH + length as u16);
    buf
}

fn recalc_length(buf: &mut BytesMut) {
    let length = buf.len() as u16;
    buf[2] = (length & 0x00ff) as u8;
    buf[3] = (length >> 8) as u8;
}

fn null() -> Bytes {
    new_packet(PacketID::NULL, 0).freeze()
}

fn stop_adv() -> Bytes {
    new_packet(PacketID::STOPADV, 0).freeze()
}

const GETADVLISTEX_DATA: [u8; 15] = [255, 3, 0, 0, 255, 3, 0, 0, 0, 0, 0, 1, 0, 0, 0];
fn get_adv_list_ex(game_name: &[u8]) -> Bytes {
    // allocate 3 additional bytes for null-terminators
    let capacity = GETADVLISTEX_DATA.len() + game_name.len() + 3;
    let mut buf = new_packet(PacketID::GETADVLISTEX, capacity);
    buf.put(Bytes::from_static(&GETADVLISTEX_DATA));
    buf.put(game_name);

    // put in null terminators
    for _ in 0..3 {
        buf.put(0u8)
    }

    buf.freeze()
}

fn enter_chat() -> Bytes {
    let mut buf = new_packet(PacketID::ENTERCHAT, 2);
    buf.put(0u8);
    buf.put(0u8);
    buf.freeze()
}

pub enum JoinChannelFlag {
    NoCreate = 0x00,
    FirstJoin = 0x01,
    ForcedJoin = 0x02,
} 

fn join_channel(flag: JoinChannelFlag, channel: &[u8]) -> Bytes {
    let mut buf = new_packet(PacketID::JOINCHANNEL, 4 + channel.len() + 1);
    buf.put_u32::<E>(flag as u32);
    buf.put(channel);
    buf.put(0u8);
    buf.freeze()
}

fn chat_command(message: &[u8]) -> Bytes {
    let mut buf = new_packet(PacketID::CHATCOMMAND, message.len() + 1);
    buf.put(message);
    buf.put(0u8);
    buf.freeze()
}

#[derive(Copy, Clone)]
enum StartAdvEx3GameState {
    Private = 0x01,
    Full = 0x02,
    NotEmpty = 0x04,
    InProgress = 0x08,
    Replay = 0x80
}

fn start_adv_ex3(state: &[StartAdvEx3GameState], 
                since_creation: u32, 
                game_type: u16, 
                sub_game_type: u16,
                version: u32, 
                ladder_type: u32,
                game_name: &[u8],
                game_password: &[u8],
                game_statstring: &[u8]) -> Bytes {
    let state = state.iter().fold(0, |acc, &x| acc | x as u32);
    let mut buf = new_packet(PacketID::STARTADVEX3, 4 + 4 + 2 + 2 + 4 + 4 + game_name.len() + game_password.len() + game_statstring.len() + 3);

    buf.put_u32::<E>(state);
    buf.put_u32::<E>(since_creation);
    buf.put_u16::<E>(game_type);
    buf.put_u16::<E>(sub_game_type);
    buf.put_u32::<E>(version);
    buf.put_u32::<E>(ladder_type);
    buf.put(game_name);
    buf.put(0u8);
    buf.put(game_password);
    buf.put(0u8);
    buf.put(game_statstring);
    buf.put(0u8);

    buf.freeze()
}

fn ping(value: u32) -> Bytes {
    let mut buf = new_packet(PacketID::PING, 4);
    buf.put_u32::<E>(value);
    buf.freeze()
}

fn net_game_port(port: u16) -> Bytes {
    let mut buf = new_packet(PacketID::NETGAMEPORT, 2);
    buf.put_u16::<E>(port);
    buf.freeze()
}

struct AuthInfo {
    protocol_id: u32,
    platform_code: u32,
    product_code: u32,
    version_byte: u32,
    language_code: u32,
    local_ip: u32,
    time_zone_bias: u32,
    mpq_locale_id: u32,
    user_language_id: u32,
    country_abbr: String,
    country: String
}

const PROTOCOL_ID: [u8; 4] = [0, 0, 0, 0];
const PLATFORM_ID: [u8; 4] = [54, 56, 88, 73]; // IX86
const PRODUCT_ID: [u8; 4] = [80, 88, 51, 87]; // W3XP
const LANGUAGE: [u8; 4] = [83, 85, 110, 101]; // enUS
const LOCAL_IP: [u8; 4] = [127, 0, 0, 1];
const TIMEZONE_BIAS: [u8; 4] = [60, 0, 0, 0];
fn auth_info(version: u8, locale_id: u32, country: &[u8], country_abbr: &[u8]) -> Bytes {
    let mut buf = new_packet(PacketID::AUTHINFO, 4 * 9 + country.len() + country_abbr.len() + 2);
    let version: [u8; 4] = [version, 0, 0, 0];

    buf.put_slice(&PROTOCOL_ID);
    buf.put_slice(&PLATFORM_ID);
    buf.put_slice(&PRODUCT_ID);
    buf.put_slice(&version);
    buf.put_slice(&LANGUAGE);
    buf.put_slice(&LOCAL_IP);
    buf.put_slice(&TIMEZONE_BIAS);
    buf.put_u32::<E>(locale_id);
    buf.put_u32::<E>(locale_id);
    buf.put(country_abbr);
    buf.put(0u8);
    buf.put(country);
    buf.put(0u8);

    buf.freeze()
}

fn auth_check(client_token: u32, 
              exe_version: u32, 
              exe_hash: u32, 
              roc_key: &[u8], 
              tft_key: &[u8], 
              exe_info: &[u8],
              owner_name: &[u8]) -> Bytes {
    let mut buf = new_packet(PacketID::AUTHCHECK, 4 * 5 + roc_key.len() + tft_key.len() + exe_info.len() + owner_name.len() + 2);
    buf.put_u32::<E>(client_token);
    buf.put_u32::<E>(exe_version);
    buf.put_u32::<E>(exe_hash);
    // amount of keys
    buf.put_u32::<E>(2u32);
    // spawn flag
    buf.put_u32::<E>(0u32);
    buf.put(roc_key);
    buf.put(tft_key);
    buf.put(exe_info);
    buf.put(0u8);
    buf.put(owner_name);
    buf.put(0u8);
    buf.freeze()
}

fn account_logon(client_key: u8, username: &[u8]) -> Bytes {
    let mut buf = new_packet(PacketID::AUTHACCOUNTLOGON, 1 + username.len() + 1);
    buf.put(client_key);
    buf.put(username);
    buf.put(0u8);
    buf.freeze()
}

fn account_logon_proof(proof: &[u8]) -> Bytes {
    let mut buf = new_packet(PacketID::AUTHACCOUNTLOGONPROOF, proof.len());
    buf.put(proof);
    buf.freeze()
}