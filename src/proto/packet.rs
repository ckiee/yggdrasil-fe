


use num_enum::TryFromPrimitive;

#[derive(Clone, Copy, Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum PacketType {
    KeepAlive = 0,
    Tree,
    DhtBootstrap,
    DhtBootstrapAck,
    DhtSetup,
    DhtTeardown,
    PathNotify,
    PathLookup,
    PathResponse,
    DhtTraffic,
    PathTraffic,
}
