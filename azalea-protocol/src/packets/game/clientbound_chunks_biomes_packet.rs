use azalea_protocol_macros::ClientboundGamePacket;
use azalea_buf::McBuf;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundChunksBiomesPacket {
pub chunk_biome_data: todo!(),
}