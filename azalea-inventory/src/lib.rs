use azalea_core::Slot;
use azalea_inventory_macros::declare_menus;

// the player inventory part is always the last 36 slots (except in the Player
// menu), so we don't have to explicitly specify it

// Client {
//     ...
//     pub menu: Menu,
//     pub inventory: Arc<[Slot; 36]>
// }

// Generate an `enum Menu` and `impl Menu`.
// if the `inventory` field is present, then the `player` field doesn't get
// implicitly added
declare_menus! {
    Player {
        craft_result: 1,
        craft: 4,
        armor: 4,
        inventory: 36,
        offhand: 1,
    },
    Generic9x1 {
        contents: 9,
    },
    Generic9x2 {
        contents: 18,
    },
    Generic9x3 {
        contents: 27,
    },
    Chest {
        block: 27,
    }
}
