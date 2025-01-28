use bytes::Bytes;

const CMD: [u8; 12] = [
    0x01, 0x09, 0x70, 0x32, 0xe2, 0xc1, 0x79, 0x9d, 0xb4, 0xd1, 0xc7, 0xb1,
];

pub enum Command {
    Connect,
}

impl TryFrom<Command> for Bytes {
    type Error = &'static str;
    fn try_from(value: Command) -> Result<Self, Self::Error> {
        match value {
            Command::Connect => Ok(Bytes::from_static(&CMD)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        let cmd = Command::Connect;
        let bytes: Bytes = cmd.try_into().unwrap();
        assert_eq!(bytes.iter().as_slice(), CMD);
    }
}
