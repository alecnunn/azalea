use packet_macros::{GamePacket, McBufReadable, McBufWritable};

#[derive(Clone, Debug, GamePacket)]
pub struct ClientboundGameEventPacket {
    pub event: EventType,
    pub param: f32,
}

#[derive(Clone, Debug, Copy, McBufReadable, McBufWritable)]
pub enum EventType {
    NoRespawnBlockAvailable = 0,
    StartRaining = 1,
    StopRaining = 2,
    ChangeGameMode = 3,
    WinGame = 4,
    DemoEvent = 5,
    ArrowHitPlayer = 6,
    RainLevelChange = 7,
    ThunderLevelChange = 8,
    PufferFishSting = 9,
    GuardianElderEffect = 10,
    ImmediateRespawn = 11,
}
