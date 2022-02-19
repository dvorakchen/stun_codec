use crate::attribute::{Attribute, AttributeType};

use crate::net::{SocketAddrDecoder, SocketAddrEncoder};





use bytecodec::{
    ByteCount, Decode, Encode, Eos, Result, SizedEncode,
    TryTaggedDecode,
};



use std::net::SocketAddr;


macro_rules! impl_decode {
    ($decoder:ty, $item:ident, $and_then:expr) => {
        impl Decode for $decoder {
            type Item = $item;

            fn decode(&mut self, buf: &[u8], eos: Eos) -> Result<usize> {
                track!(self.0.decode(buf, eos))
            }

            fn finish_decoding(&mut self) -> Result<Self::Item> {
                track!(self.0.finish_decoding()).and_then($and_then)
            }

            fn requiring_bytes(&self) -> ByteCount {
                self.0.requiring_bytes()
            }

            fn is_idle(&self) -> bool {
                self.0.is_idle()
            }
        }
        impl TryTaggedDecode for $decoder {
            type Tag = AttributeType;

            fn try_start_decoding(&mut self, attr_type: Self::Tag) -> Result<bool> {
                Ok(attr_type.as_u16() == $item::CODEPOINT)
            }
        }
    };
}

macro_rules! impl_encode {
    ($encoder:ty, $item:ty, $map_from:expr) => {
        impl Encode for $encoder {
            type Item = $item;

            fn encode(&mut self, buf: &mut [u8], eos: Eos) -> Result<usize> {
                track!(self.0.encode(buf, eos))
            }

            fn start_encoding(&mut self, item: Self::Item) -> Result<()> {
                track!(self.0.start_encoding($map_from(item)))
            }

            fn requiring_bytes(&self) -> ByteCount {
                self.0.requiring_bytes()
            }

            fn is_idle(&self) -> bool {
                self.0.is_idle()
            }
        }
        impl SizedEncode for $encoder {
            fn exact_requiring_bytes(&self) -> u64 {
                self.0.exact_requiring_bytes()
            }
        }
    };
}


/// `CHANGED-ADDRESS` attribute.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChangedAddress(SocketAddr);
impl ChangedAddress {
    /// nope
    pub const CODEPOINT: u16 = 0x0005;
    /// nope

    pub fn new(addr: SocketAddr) -> Self {
        ChangedAddress(addr)
    }
    /// nope

    pub fn address(&self) -> SocketAddr {
        self.0
    }
}
/// nope
impl Attribute for ChangedAddress {
    type Decoder = ChangedAddressDecoder;
    type Encoder = ChangedAddressEncoder;
    /// nope

    fn get_type(&self) -> AttributeType {
        AttributeType::new(Self::CODEPOINT)
    }
}

/// nope

#[derive(Debug, Default)]
pub struct ChangedAddressDecoder(SocketAddrDecoder);
impl ChangedAddressDecoder {
    /// nope

    pub fn new() -> Self {
        Self::default()
    }
}
impl_decode!(ChangedAddressDecoder, ChangedAddress, |item| Ok(
    ChangedAddress(item)
));

/// nope

#[derive(Debug, Default)]
pub struct ChangedAddressEncoder(SocketAddrEncoder);
impl ChangedAddressEncoder {
    /// nope

    pub fn new() -> Self {
        Self::default()
    }
}
impl_encode!(ChangedAddressEncoder, ChangedAddress, |item: Self::Item| item
    .0);