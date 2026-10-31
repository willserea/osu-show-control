use anyhow::Result;
use rosc::{OscMessage, OscPacket, OscType, encoder};
use std::net::UdpSocket;

pub struct OscSender {
    socket: UdpSocket,
    target: String,
}

impl OscSender {
    pub fn new(target: &str) -> Result<Self> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;

        Ok(Self {
            socket,
            target: target.to_string(),
        })
    }

    pub fn send_string(&self, address: &str, text: &str) -> Result<()> {
        let msg = OscMessage {
            addr: address.to_string(),
            args: vec![OscType::String(text.to_string())],
        };

        self.send_packet(msg)
    }

    pub fn send_int(&self, address: &str, value: i32) -> Result<()> {
        let msg = OscMessage {
            addr: address.to_string(),
            args: vec![OscType::Int(value)],
        };

        self.send_packet(msg)
    }

    pub fn send_float(&self, address: &str, value: f32) -> Result<()> {
        let msg = OscMessage {
            addr: address.to_string(),
            args: vec![OscType::Float(value)],
        };

        self.send_packet(msg)
    }

    fn send_packet(&self, msg: OscMessage) -> Result<()> {
        let packet = OscPacket::Message(msg);
        let buffer = encoder::encode(&packet)?;

        self.socket.send_to(&buffer, &self.target)?;

        Ok(())
    }
}
