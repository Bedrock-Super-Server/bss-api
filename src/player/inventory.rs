use crate::ffi::abi::{FfiStr, InventoryHandle, ItemStack};
use crate::ffi::call::HostFn;
use crate::ffi::host::{HostApi, VirtualSlot};
use crate::player::item::{Item, ItemMeta};
use crate::server::ffi_string;

/**
A player's inventory, lent to the handler of [`Player::request_inventory`](crate::player::Player::request_inventory).

Slots `0` to `8` are the hotbar and `9` to `35` the rest of the inventory, which is what
[`size`](Self::size) counts. Past them, [`HELMET`](Self::HELMET) to [`BOOTS`](Self::BOOTS) are the
armor and [`OFFHAND`](Self::OFFHAND) the off hand.
*/
pub struct Inventory {
    handle: InventoryHandle,
    host: &'static HostApi,
}

impl Inventory {
    /// The helmet slot.
    pub const HELMET: u32 = 36;
    /// The chestplate slot.
    pub const CHESTPLATE: u32 = 37;
    /// The leggings slot.
    pub const LEGGINGS: u32 = 38;
    /// The boots slot.
    pub const BOOTS: u32 = 39;
    /// The off hand slot.
    pub const OFFHAND: u32 = 40;

    pub(crate) fn new(handle: InventoryHandle, host: &'static HostApi) -> Self {
        Self { handle, host }
    }

    /// Returns the number of slots.
    pub fn size(&self) -> u32 {
        host_call!(self.host, inventory_size(self.handle)).unwrap_or(0)
    }

    /// Returns the stack in `slot`; an empty slot has `count` `0`.
    pub fn get(&self, slot: u32) -> ItemStack {
        host_call!(self.host, inventory_get(self.handle, slot)).unwrap_or_default()
    }

    /// Puts a plain stack in `slot`; a `count` of `0` empties it.
    pub fn set(&self, slot: u32, stack: ItemStack) {
        host_call!(self.host, inventory_set(self.handle, slot, stack));
    }

    /// Puts `item`, with its name, lore and enchantments, in `slot`.
    pub fn set_item(&self, slot: u32, item: &Item) {
        if let Some(f) = self.host.inventory_set_item {
            f.call((
                self.handle,
                slot,
                item.item_id,
                item.block_runtime_id,
                item.count,
                item.damage,
                FfiStr::new(&item.name),
                FfiStr::new(&item.lore_json()),
                FfiStr::new(&item.ench_json()),
                item.unbreakable,
            ));
        }
    }

    /// Returns the name, lore and enchantments of the stack in `slot`.
    pub fn get_meta(&self, slot: u32) -> ItemMeta {
        let json = host_call!(self.host, inventory_get_meta(self.handle, slot))
            .map(ffi_string)
            .unwrap_or_default();
        ItemMeta::from_json(&json)
    }
}

/**
An item shown in a virtual chest, opened with [`Player::open_virtual_chest`](crate::player::Player::open_virtual_chest).

```
use bss_api::prelude::*;

fn shop(player: &Player) {
    let server = player.server();
    let Some(apple) = server.items().id("minecraft:apple") else { return };
    let items = vec![Some(VirtualItem::new(apple.0, "§aApple").lore(vec!["5 coins".into()])), None, None];
    player.open_virtual_chest("Shop", &items, |slot, buyer| {
        if slot == 0 {
            buyer.message("You bought an apple.");
        }
    });
}
```
*/
#[derive(Clone, Default)]
pub struct VirtualItem {
    /// Numeric id of the item.
    pub item_id: i16,
    /// Runtime id of the block form of the item, or `0`.
    pub block_runtime_id: u32,
    /// Size of the stack shown.
    pub count: u8,
    /// Name shown.
    pub name: String,
    /// Lines of text under the name.
    pub lore: Vec<String>,
}

impl VirtualItem {
    /// One item of `item_id`, shown as `name`.
    pub fn new(item_id: i16, name: impl Into<String>) -> Self {
        Self {
            item_id,
            block_runtime_id: 0,
            count: 1,
            name: name.into(),
            lore: Vec::new(),
        }
    }

    /// One block item, shown as `name`.
    pub fn block(item_id: i16, block_runtime_id: u32, name: impl Into<String>) -> Self {
        Self {
            item_id,
            block_runtime_id,
            count: 1,
            name: name.into(),
            lore: Vec::new(),
        }
    }

    /// Sets the size of the stack shown.
    pub fn count(mut self, count: u8) -> Self {
        self.count = count;
        self
    }

    /// Sets the lines of lore.
    pub fn lore(mut self, lore: Vec<String>) -> Self {
        self.lore = lore;
        self
    }

    pub(crate) fn slot(item: Option<&Self>, lore_json: &str) -> VirtualSlot {
        match item {
            Some(item) => VirtualSlot {
                present: item.count > 0,
                item_id: item.item_id,
                block_runtime_id: item.block_runtime_id,
                count: item.count,
                name: crate::ffi::abi::FfiStr::new(&item.name),
                lore_json: crate::ffi::abi::FfiStr::new(lore_json),
            },
            None => VirtualSlot {
                present: false,
                item_id: 0,
                block_runtime_id: 0,
                count: 0,
                name: crate::ffi::abi::FfiStr {
                    ptr: std::ptr::null(),
                    len: 0,
                },
                lore_json: crate::ffi::abi::FfiStr {
                    ptr: std::ptr::null(),
                    len: 0,
                },
            },
        }
    }
}
