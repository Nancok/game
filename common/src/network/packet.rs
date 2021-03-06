use bincode;
use Uid;
use nalgebra::Vector3;
use network::{Error, ClientMode};

pub trait Packet {
    fn serialize(&self) -> Result<Vec<u8>, Error>;
    fn from(data: &[u8]) -> Result<Self, Error> where Self: Sized;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ServerPacket {
    Connected { entity_uid: Option<Uid>, version: String },
    Kicked { reason: String },
    Shutdown,
    Ping,
    RecvChatMsg { alias: String, msg: String },
    EntityUpdate { uid: Uid, pos: Vector3<f32> },
}

impl Packet for ServerPacket {
    fn from(data: &[u8]) -> Result<ServerPacket, Error> {
        match bincode::deserialize(data) {
            Ok(sp) => Ok(sp),
            Err(_) => Err(Error::CannotDeserialize),
        }
    }

    fn serialize(&self) -> Result<Vec<u8>, Error> {
        match bincode::serialize(&self) {
            Ok(data) => Ok(data),
            Err(_) => Err(Error::CannotSerialize),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ClientPacket {
    Connect { mode: ClientMode, alias: String, version: String },
    Disconnect,
    Ping,
    ChatMsg { msg: String },
    SendCmd { cmd: String },
    PlayerEntityUpdate { pos: Vector3<f32> }
}

impl Packet for ClientPacket {
    fn from(data: &[u8]) -> Result<ClientPacket, Error> {
        match bincode::deserialize(data) {
            Ok(sp) => Ok(sp),
            Err(_) => Err(Error::CannotDeserialize),
        }
    }

    fn serialize(&self) -> Result<Vec<u8>, Error> {
        match bincode::serialize(&self) {
            Ok(data) => Ok(data),
            Err(_) => Err(Error::CannotSerialize),
        }
    }
}
