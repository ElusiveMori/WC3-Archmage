#![allow(dead_code)]

use bytes::*;
use super::*;

pub struct GetAdvListEx {
    count: u32,
    status: GetAdvListExStatus
}

pub struct GetAdvListExItem {
    game_settings: u32,
    language_id: u32,
    address_family: u16,
    address_port: u16,
    host_ip: u32,
    game_status: u32,
    elapsed_time: u32,
    game_name: Vec<u8>,
    game_password: Vec<u8>,
    game_statstring: Vec<u8>,
}

pub enum GetAdvListExStatus {
    OK(Vec<GetAdvListExItem>),
    EMPTY(u32)
}

pub struct EnterChat {
    unique_name: Vec<u8>,
    statstring: Vec<u8>,
    account_name: Vec<u8>
}

#[derive(Copy, Clone)]
pub enum ChatEventID {
    ShowUser = 0x01,
    Join = 0x02,
    Leave = 0x03,
    Whisper = 0x04,
    Talk = 0x05,
    Broadcast = 0x06,
    Channel = 0x07,
    Userflags = 0x09,
    WhisperSent = 0x0A,
    ChannelFull = 0x0D,
    ChannelDoesNotExist = 0x0E,
    ChannelRestricted = 0x0F,
    Info = 0x12,
    Error = 0x13,
    Ignore = 0x15,
    Accept = 0x16,
    Emote = 0x17,
    Invalid = 0xFF
}

impl ChatEventID {
    pub fn from_id(id: u32) -> Self {
        match id {
            0x01 => ChatEventID::ShowUser,
            0x02 => ChatEventID::Join,
            0x03 => ChatEventID::Leave,
            0x04 => ChatEventID::Whisper,
            0x05 => ChatEventID::Talk,
            0x06 => ChatEventID::Broadcast,
            0x07 => ChatEventID::Channel,
            0x09 => ChatEventID::Userflags,
            0x0A => ChatEventID::WhisperSent,
            0x0D => ChatEventID::ChannelFull,
            0x0E => ChatEventID::ChannelDoesNotExist,
            0x0F => ChatEventID::ChannelRestricted,
            0x12 => ChatEventID::Info,
            0x13 => ChatEventID::Error,
            0x15 => ChatEventID::Ignore,
            0x16 => ChatEventID::Accept,
            0x17 => ChatEventID::Emote,
            _ => ChatEventID::Invalid
        }
    }
}

pub struct ChatEvent {
    event_id: ChatEventID,
    user_flags: u32,
    ping: u32,
    username: Vec<u8>,
    text: Vec<u8>
}

#[derive(Copy, Clone)]
pub enum StartAdvEx3Status {
    Ok = 0x00,
    Failed = 0x01,
    Invalid = 0xFF
}

impl StartAdvEx3Status {
    fn from_id(id: u32) -> Self {
        match id {
            0 => StartAdvEx3Status::Ok,
            1 => StartAdvEx3Status::Failed,
            _ => StartAdvEx3Status::Invalid
        }
    }
}

pub struct StartAdvEx3 {
    status: StartAdvEx3Status
}

pub struct Ping {
    value: u32
}

pub struct AuthInfo {
    logon_type: u32,
    server_token: u32,
    udp_value: u32,
    mpq_filetime: u64,
    mpq_filename: Vec<u8>,
    value_string: Vec<u8>,
    server_signature: [u8; 128]
}

pub struct AuthCheck {
    status: u32,
    info: Vec<u8>
}

pub struct AuthAccountLogon {
    status: u32,
    salt: [u8; 32],
    server_key: [u8; 32] 
}

pub struct AuthAccountLogonProof {
    status: u32,
    proof: [u8; 20],
    info: Vec<u8>
}

type E = LittleEndian;

pub trait PacketReader<R: Buf> {
    fn read_header(&self, buf: &mut R) -> (PacketID, usize) {
        // discard protocol id
        buf.get_u8();
        let id = PacketID::from_id(buf.get_u8());
        let length = buf.get_u16::<E>() as usize;

        (id, length)
    }

    fn read_cstring(buf: &mut R) -> Vec<u8> {
        let null_pos = buf.iter().position(|c| c == 0).unwrap();
        let mut slice = Vec::with_capacity(null_pos);
        buf.copy_to_slice(&mut slice);
        // skip null byte
        buf.advance(1);
        slice
    }

    fn read_null(_: &mut R) {}

    fn read_get_adv_list_ex(buf: &mut R) -> GetAdvListEx {
        let count = buf.get_u32::<E>();

        if count == 0 {
            return GetAdvListEx { 
                count : 0u32, 
                status : GetAdvListExStatus::EMPTY(buf.get_u32::<E>())
            }
        } else {
            let mut games = Vec::with_capacity(count as usize);
            for _ in 0..count {
                let game_settings = buf.get_u32::<E>();
                let language_id = buf.get_u32::<E>();
                let address_family = buf.get_u16::<E>();
                let address_port = buf.get_u16::<E>();
                let host_ip = buf.get_u32::<E>();
                let _sin_zero = buf.get_u32::<E>();
                let _sin_zero = buf.get_u32::<E>();
                let game_status = buf.get_u32::<E>();
                let elapsed_time = buf.get_u32::<E>();
                let game_name = Self::read_cstring(buf);
                let game_password = Self::read_cstring(buf);
                let game_statstring = Self::read_cstring(buf);

                games.push(GetAdvListExItem {
                    game_settings,
                    language_id,
                    address_family,
                    address_port,
                    host_ip,
                    game_status,
                    elapsed_time,
                    game_name,
                    game_password,
                    game_statstring
                });
            }

            return GetAdvListEx {
                count : count,
                status : GetAdvListExStatus::OK(games)
            }
        }
    }

    fn read_enter_chat(buf: &mut R) -> EnterChat {
        let unique_name = Self::read_cstring(buf);
        let statstring = Self::read_cstring(buf);
        let account_name = Self::read_cstring(buf);

        EnterChat {
            unique_name,
            statstring,
            account_name
        }
    }

    fn read_chat_event(buf: &mut R) -> ChatEvent {
        let event_id = ChatEventID::from_id(buf.get_u32::<LittleEndian>());
        let user_flags = buf.get_u32::<LittleEndian>();
        let ping = buf.get_u32::<LittleEndian>();
        let _ip = buf.get_u32::<LittleEndian>();
        let _acc = buf.get_u32::<LittleEndian>();
        let _auth = buf.get_u32::<LittleEndian>();
        let username = Self::read_cstring(buf);
        let text = Self::read_cstring(buf);

        ChatEvent {
            event_id,
            user_flags,
            ping,
            username,
            text
        }
    }

    fn read_start_adv_ex3(buf: &mut R) -> StartAdvEx3 {
        StartAdvEx3 {
            status : StartAdvEx3Status::from_id(buf.get_u32::<LittleEndian>())
        }
    }

    fn read_ping(buf: &mut R) -> Ping {
        Ping {
            value : buf.get_u32::<LittleEndian>()
        }
    }

    fn read_auth_info(buf: &mut R) -> AuthInfo {
        let logon_type = buf.get_u32::<LittleEndian>();
        let server_token = buf.get_u32::<LittleEndian>();
        let udp_value = buf.get_u32::<LittleEndian>();
        let mpq_filetime = buf.get_u64::<LittleEndian>();
        let mpq_filename = Self::read_cstring(buf);
        let value_string = Self::read_cstring(buf);
        let mut server_signature = [0u8; 128];
        buf.take(128).copy_to_slice(&mut server_signature);

        AuthInfo {
            logon_type,
            server_token,
            udp_value,
            mpq_filetime,
            mpq_filename,
            value_string,
            server_signature
        }
    }

    fn read_auth_check(buf: &mut R) -> AuthCheck {
        AuthCheck {
            status: buf.get_u32::<LittleEndian>(),
            info: Self::read_cstring(buf)
        }
    }

    fn read_auth_account_logon(buf: &mut R) -> AuthAccountLogon {
        let status = buf.get_u32::<LittleEndian>();
        let mut salt = [0u8; 32];
        let mut server_key = [0u8; 32];

        buf.take(32).copy_to_slice(&mut salt);
        buf.take(32).copy_to_slice(&mut server_key);

        AuthAccountLogon {
            status,
            salt,
            server_key
        }
    }

    fn read_auth_account_logon_proof(buf: &mut R) -> AuthAccountLogonProof {
        let status = buf.get_u32::<LittleEndian>();
        let mut proof = [0u8; 20];
        buf.take(20).copy_to_slice(&mut proof);
        let info = Self::read_cstring(buf);

        AuthAccountLogonProof {
            status,
            proof,
            info
        }
    }
}