#![doc = include_str!("../README.md")]
#![feature(trait_upcasting)]

mod behavior;
mod generated;
mod range;

pub use generated::{blocks, properties};

use azalea_buf::{BufReadError, McBufReadable, McBufVarReadable, McBufVarWritable, McBufWritable};
pub use behavior::BlockBehavior;
use core::fmt::Debug;
pub use range::BlockStates;
use std::{
    any::Any,
    io::{Cursor, Write},
};

pub trait Block: Debug + Any {
    fn behavior(&self) -> BlockBehavior;
    /// Get the Minecraft ID for this block. For example `stone` or
    /// `grass_block`.
    fn id(&self) -> &'static str;
    /// Convert the block to a block state. This is lossless, as the block
    /// contains all the state data.
    fn as_block_state(&self) -> BlockState;
}
impl dyn Block {
    pub fn downcast_ref<T: Block>(&self) -> Option<&T> {
        (self as &dyn Any).downcast_ref::<T>()
    }
}

/// A representation of a state a block can be in.
///
/// For example, a stone block only has one state but each possible stair
/// rotation is a different state.
#[derive(Copy, Clone, PartialEq, Eq, Default, Hash)]
pub struct BlockState {
    /// The protocol ID for the block state. IDs may change every
    /// version, so you shouldn't hard-code them or store them in databases.
    pub id: u32,
}

impl BlockState {
    pub const AIR: BlockState = BlockState { id: 0 };

    /// Transmutes a u32 to a block state.
    ///
    /// # Safety
    /// The `state_id` should be a valid block state.
    #[inline]
    pub unsafe fn from_u32_unchecked(state_id: u32) -> Self {
        BlockState { id: state_id }
    }

    #[inline]
    pub fn is_valid_state(state_id: u32) -> bool {
        state_id <= Self::max_state()
    }
}

impl TryFrom<u32> for BlockState {
    type Error = ();

    /// Safely converts a state id to a block state.
    fn try_from(state_id: u32) -> Result<Self, Self::Error> {
        if Self::is_valid_state(state_id) {
            Ok(unsafe { Self::from_u32_unchecked(state_id) })
        } else {
            Err(())
        }
    }
}

impl McBufReadable for BlockState {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let state_id = u32::var_read_from(buf)?;
        Self::try_from(state_id).map_err(|_| BufReadError::UnexpectedEnumVariant {
            id: state_id as i32,
        })
    }
}
impl McBufWritable for BlockState {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        u32::var_write_into(&self.id, buf)
    }
}

impl std::fmt::Debug for BlockState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BlockState(id: {}, {:?})",
            self.id,
            Box::<dyn Block>::from(*self)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_u32() {
        assert_eq!(BlockState::try_from(0).unwrap(), BlockState::AIR);

        assert!(BlockState::try_from(BlockState::max_state()).is_ok());
        assert!(BlockState::try_from(BlockState::max_state() + 1).is_err());
    }

    #[test]
    fn test_from_blockstate() {
        let block: Box<dyn Block> = Box::<dyn Block>::from(BlockState::AIR);
        assert_eq!(block.id(), "air");

        let block: Box<dyn Block> =
            Box::<dyn Block>::from(BlockState::from(azalea_registry::Block::FloweringAzalea));
        assert_eq!(block.id(), "flowering_azalea");
    }

    #[test]
    fn test_debug_blockstate() {
        let formatted = format!(
            "{:?}",
            BlockState::from(azalea_registry::Block::FloweringAzalea)
        );
        assert!(formatted.ends_with(", FloweringAzalea)"), "{}", formatted);

        let formatted = format!(
            "{:?}",
            BlockState::from(azalea_registry::Block::BigDripleafStem)
        );
        assert!(
            formatted.ends_with(", BigDripleafStem { facing: North, waterlogged: false })"),
            "{}",
            formatted
        );
    }
}
