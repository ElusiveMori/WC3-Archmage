// reference: https://bnetdocs.org/packet/index
#![allow(dead_code)]

pub mod c2s;
pub mod s2c;

use bytes::Bytes;

#[derive(Clone, Copy)]
pub enum PacketID {
    UNKNOWN                = -1,
    NULL                   = 0,   // 0x0
    STOPADV                = 2,   // 0x2
    GETADVLISTEX           = 9,   // 0x9
    ENTERCHAT              = 10,  // 0xA
    JOINCHANNEL            = 12,  // 0xC
    CHATCOMMAND            = 14,  // 0xE
    CHATEVENT              = 15,  // 0xF
    STARTADVEX3            = 28,  // 0x1C
    PING                   = 37,  // 0x25
    NETGAMEPORT            = 69,  // 0x45
    AUTHINFO               = 80,  // 0x50
    AUTHCHECK              = 81,  // 0x51
    AUTHACCOUNTLOGON       = 83,  // 0x53
    AUTHACCOUNTLOGONPROOF  = 84,  // 0x54
}

impl PacketID {
    pub fn from_id(id: u8) -> PacketID {
        match id {
            0 => PacketID::NULL,
            2 => PacketID::STOPADV,
            9 => PacketID::GETADVLISTEX,
            12 => PacketID::ENTERCHAT,
            14 => PacketID::CHATCOMMAND,
            15 => PacketID::CHATEVENT,
            28 => PacketID::STARTADVEX3,
            37 => PacketID::PING,
            69 => PacketID::NETGAMEPORT,
            80 => PacketID::AUTHINFO,
            81 => PacketID::AUTHCHECK,
            83 => PacketID::AUTHACCOUNTLOGON,
            84 => PacketID::AUTHACCOUNTLOGONPROOF,
            _ => PacketID::UNKNOWN
        }
    }
}

pub struct BNetIncomingPacket {
    pub lol: u8
}

pub struct BNetOutgoingPacket {
    pub data: Bytes
}