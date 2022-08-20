use azalea_buf::McBuf;
use packet_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundSetPassengersPacket {
    #[var]
    pub vehicle: u32,
    #[var]
    pub passengers: Vec<u32>,
}
