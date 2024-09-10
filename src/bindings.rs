#![allow(warnings)]pub type Loc = game::auto_rogue::types::Loc;
pub type Tile = game::auto_rogue::types::Tile;
pub type Creature = game::auto_rogue::types::Creature;
pub type Item = game::auto_rogue::types::Item;
pub type InventoryItem = game::auto_rogue::types::InventoryItem;
pub type EquipmentState = game::auto_rogue::types::EquipmentState;
pub type CharacterStats = game::auto_rogue::types::CharacterStats;
pub type Buff = game::auto_rogue::types::Buff;
pub type GameState = game::auto_rogue::types::GameState;
pub type Action = game::auto_rogue::types::Action;
pub type Event = game::auto_rogue::types::Event;
pub type Command = game::auto_rogue::types::Command;
#[allow(unused_unsafe, clippy::all)]
pub fn tile_at(l: Loc) -> Option<Tile> {
    unsafe {
        #[repr(align(4))]
        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 16]);
        let game::auto_rogue::types::Loc { x: x0, y: y0 } = l;
        let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
        #[cfg(target_arch = "wasm32")]
        #[link(wasm_import_module = "$root")]
        extern "C" {
            #[link_name = "tile-at"]
            fn wit_import(_: i32, _: i32, _: *mut u8);
        }
        #[cfg(not(target_arch = "wasm32"))]
        fn wit_import(_: i32, _: i32, _: *mut u8) {
            unreachable!()
        }
        wit_import(_rt::as_i32(x0), _rt::as_i32(y0), ptr1);
        let l2 = i32::from(*ptr1.add(0).cast::<u8>());
        match l2 {
            0 => None,
            1 => {
                let e = {
                    let l3 = i32::from(*ptr1.add(4).cast::<u8>());
                    let l4 = i32::from(*ptr1.add(5).cast::<u8>());
                    let l5 = *ptr1.add(8).cast::<*mut u8>();
                    let l6 = *ptr1.add(12).cast::<usize>();
                    let len7 = l6;
                    let bytes7 = _rt::Vec::from_raw_parts(l5.cast(), len7, len7);
                    game::auto_rogue::types::Tile {
                        passable: _rt::bool_lift(l3 as u8),
                        opaque: _rt::bool_lift(l4 as u8),
                        name: _rt::string_lift(bytes7),
                    }
                };
                Some(e)
            }
            _ => _rt::invalid_enum_discriminant(),
        }
    }
}
#[allow(unused_unsafe, clippy::all)]
pub fn visible_tiles() -> _rt::Vec<(Loc, Tile)> {
    unsafe {
        #[repr(align(4))]
        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
        #[cfg(target_arch = "wasm32")]
        #[link(wasm_import_module = "$root")]
        extern "C" {
            #[link_name = "visible-tiles"]
            fn wit_import(_: *mut u8);
        }
        #[cfg(not(target_arch = "wasm32"))]
        fn wit_import(_: *mut u8) {
            unreachable!()
        }
        wit_import(ptr0);
        let l1 = *ptr0.add(0).cast::<*mut u8>();
        let l2 = *ptr0.add(4).cast::<usize>();
        let base10 = l1;
        let len10 = l2;
        let mut result10 = _rt::Vec::with_capacity(len10);
        for i in 0..len10 {
            let base = base10.add(i * 20);
            let e10 = {
                let l3 = *base.add(0).cast::<i32>();
                let l4 = *base.add(4).cast::<i32>();
                let l5 = i32::from(*base.add(8).cast::<u8>());
                let l6 = i32::from(*base.add(9).cast::<u8>());
                let l7 = *base.add(12).cast::<*mut u8>();
                let l8 = *base.add(16).cast::<usize>();
                let len9 = l8;
                let bytes9 = _rt::Vec::from_raw_parts(l7.cast(), len9, len9);
                (
                    game::auto_rogue::types::Loc {
                        x: l3,
                        y: l4,
                    },
                    game::auto_rogue::types::Tile {
                        passable: _rt::bool_lift(l5 as u8),
                        opaque: _rt::bool_lift(l6 as u8),
                        name: _rt::string_lift(bytes9),
                    },
                )
            };
            result10.push(e10);
        }
        _rt::cabi_dealloc(base10, len10 * 20, 4);
        result10
    }
}
#[allow(unused_unsafe, clippy::all)]
pub fn creature_at(l: Loc) -> Option<Creature> {
    unsafe {
        #[repr(align(8))]
        struct RetArea([::core::mem::MaybeUninit<u8>; 40]);
        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 40]);
        let game::auto_rogue::types::Loc { x: x0, y: y0 } = l;
        let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
        #[cfg(target_arch = "wasm32")]
        #[link(wasm_import_module = "$root")]
        extern "C" {
            #[link_name = "creature-at"]
            fn wit_import(_: i32, _: i32, _: *mut u8);
        }
        #[cfg(not(target_arch = "wasm32"))]
        fn wit_import(_: i32, _: i32, _: *mut u8) {
            unreachable!()
        }
        wit_import(_rt::as_i32(x0), _rt::as_i32(y0), ptr1);
        let l2 = i32::from(*ptr1.add(0).cast::<u8>());
        match l2 {
            0 => None,
            1 => {
                let e = {
                    let l3 = *ptr1.add(8).cast::<*mut u8>();
                    let l4 = *ptr1.add(12).cast::<usize>();
                    let len5 = l4;
                    let bytes5 = _rt::Vec::from_raw_parts(l3.cast(), len5, len5);
                    let l6 = *ptr1.add(16).cast::<i64>();
                    let l7 = *ptr1.add(24).cast::<i32>();
                    let l8 = i32::from(*ptr1.add(28).cast::<u8>());
                    game::auto_rogue::types::Creature {
                        name: _rt::string_lift(bytes5),
                        id: l6,
                        faction: l7 as u32,
                        broadcast: match l8 {
                            0 => None,
                            1 => {
                                let e = {
                                    let l9 = *ptr1.add(32).cast::<*mut u8>();
                                    let l10 = *ptr1.add(36).cast::<usize>();
                                    let len11 = l10;
                                    _rt::Vec::from_raw_parts(l9.cast(), len11, len11)
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        },
                    }
                };
                Some(e)
            }
            _ => _rt::invalid_enum_discriminant(),
        }
    }
}
#[allow(unused_unsafe, clippy::all)]
pub fn actor() -> (Loc, Creature) {
    unsafe {
        #[repr(align(8))]
        struct RetArea([::core::mem::MaybeUninit<u8>; 40]);
        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 40]);
        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
        #[cfg(target_arch = "wasm32")]
        #[link(wasm_import_module = "$root")]
        extern "C" {
            #[link_name = "actor"]
            fn wit_import(_: *mut u8);
        }
        #[cfg(not(target_arch = "wasm32"))]
        fn wit_import(_: *mut u8) {
            unreachable!()
        }
        wit_import(ptr0);
        let l1 = *ptr0.add(0).cast::<i32>();
        let l2 = *ptr0.add(4).cast::<i32>();
        let l3 = *ptr0.add(8).cast::<*mut u8>();
        let l4 = *ptr0.add(12).cast::<usize>();
        let len5 = l4;
        let bytes5 = _rt::Vec::from_raw_parts(l3.cast(), len5, len5);
        let l6 = *ptr0.add(16).cast::<i64>();
        let l7 = *ptr0.add(24).cast::<i32>();
        let l8 = i32::from(*ptr0.add(28).cast::<u8>());
        (
            game::auto_rogue::types::Loc {
                x: l1,
                y: l2,
            },
            game::auto_rogue::types::Creature {
                name: _rt::string_lift(bytes5),
                id: l6,
                faction: l7 as u32,
                broadcast: match l8 {
                    0 => None,
                    1 => {
                        let e = {
                            let l9 = *ptr0.add(32).cast::<*mut u8>();
                            let l10 = *ptr0.add(36).cast::<usize>();
                            let len11 = l10;
                            _rt::Vec::from_raw_parts(l9.cast(), len11, len11)
                        };
                        Some(e)
                    }
                    _ => _rt::invalid_enum_discriminant(),
                },
            },
        )
    }
}
#[allow(unused_unsafe, clippy::all)]
pub fn visible_creatures() -> _rt::Vec<(Loc, Creature)> {
    unsafe {
        #[repr(align(4))]
        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
        #[cfg(target_arch = "wasm32")]
        #[link(wasm_import_module = "$root")]
        extern "C" {
            #[link_name = "visible-creatures"]
            fn wit_import(_: *mut u8);
        }
        #[cfg(not(target_arch = "wasm32"))]
        fn wit_import(_: *mut u8) {
            unreachable!()
        }
        wit_import(ptr0);
        let l1 = *ptr0.add(0).cast::<*mut u8>();
        let l2 = *ptr0.add(4).cast::<usize>();
        let base14 = l1;
        let len14 = l2;
        let mut result14 = _rt::Vec::with_capacity(len14);
        for i in 0..len14 {
            let base = base14.add(i * 40);
            let e14 = {
                let l3 = *base.add(0).cast::<i32>();
                let l4 = *base.add(4).cast::<i32>();
                let l5 = *base.add(8).cast::<*mut u8>();
                let l6 = *base.add(12).cast::<usize>();
                let len7 = l6;
                let bytes7 = _rt::Vec::from_raw_parts(l5.cast(), len7, len7);
                let l8 = *base.add(16).cast::<i64>();
                let l9 = *base.add(24).cast::<i32>();
                let l10 = i32::from(*base.add(28).cast::<u8>());
                (
                    game::auto_rogue::types::Loc {
                        x: l3,
                        y: l4,
                    },
                    game::auto_rogue::types::Creature {
                        name: _rt::string_lift(bytes7),
                        id: l8,
                        faction: l9 as u32,
                        broadcast: match l10 {
                            0 => None,
                            1 => {
                                let e = {
                                    let l11 = *base.add(32).cast::<*mut u8>();
                                    let l12 = *base.add(36).cast::<usize>();
                                    let len13 = l12;
                                    _rt::Vec::from_raw_parts(l11.cast(), len13, len13)
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        },
                    },
                )
            };
            result14.push(e14);
        }
        _rt::cabi_dealloc(base14, len14 * 40, 8);
        result14
    }
}
#[allow(unused_unsafe, clippy::all)]
pub fn item_at(l: Loc) -> Option<Item> {
    unsafe {
        #[repr(align(8))]
        struct RetArea([::core::mem::MaybeUninit<u8>; 40]);
        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 40]);
        let game::auto_rogue::types::Loc { x: x0, y: y0 } = l;
        let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
        #[cfg(target_arch = "wasm32")]
        #[link(wasm_import_module = "$root")]
        extern "C" {
            #[link_name = "item-at"]
            fn wit_import(_: i32, _: i32, _: *mut u8);
        }
        #[cfg(not(target_arch = "wasm32"))]
        fn wit_import(_: i32, _: i32, _: *mut u8) {
            unreachable!()
        }
        wit_import(_rt::as_i32(x0), _rt::as_i32(y0), ptr1);
        let l2 = i32::from(*ptr1.add(0).cast::<u8>());
        match l2 {
            0 => None,
            1 => {
                let e = {
                    let l3 = *ptr1.add(8).cast::<i64>();
                    let l4 = *ptr1.add(16).cast::<*mut u8>();
                    let l5 = *ptr1.add(20).cast::<usize>();
                    let len6 = l5;
                    let bytes6 = _rt::Vec::from_raw_parts(l4.cast(), len6, len6);
                    let l7 = i32::from(*ptr1.add(24).cast::<u8>());
                    let l8 = i32::from(*ptr1.add(25).cast::<u8>());
                    let l9 = i32::from(*ptr1.add(28).cast::<u8>());
                    game::auto_rogue::types::Item {
                        id: l3,
                        name: _rt::string_lift(bytes6),
                        is_furniture: _rt::bool_lift(l7 as u8),
                        is_passable: _rt::bool_lift(l8 as u8),
                        metadata: match l9 {
                            0 => None,
                            1 => {
                                let e = {
                                    let l10 = *ptr1.add(32).cast::<*mut u8>();
                                    let l11 = *ptr1.add(36).cast::<usize>();
                                    let len12 = l11;
                                    let bytes12 = _rt::Vec::from_raw_parts(
                                        l10.cast(),
                                        len12,
                                        len12,
                                    );
                                    _rt::string_lift(bytes12)
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        },
                    }
                };
                Some(e)
            }
            _ => _rt::invalid_enum_discriminant(),
        }
    }
}
#[allow(unused_unsafe, clippy::all)]
pub fn visible_items() -> _rt::Vec<(Loc, Item)> {
    unsafe {
        #[repr(align(4))]
        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
        #[cfg(target_arch = "wasm32")]
        #[link(wasm_import_module = "$root")]
        extern "C" {
            #[link_name = "visible-items"]
            fn wit_import(_: *mut u8);
        }
        #[cfg(not(target_arch = "wasm32"))]
        fn wit_import(_: *mut u8) {
            unreachable!()
        }
        wit_import(ptr0);
        let l1 = *ptr0.add(0).cast::<*mut u8>();
        let l2 = *ptr0.add(4).cast::<usize>();
        let base15 = l1;
        let len15 = l2;
        let mut result15 = _rt::Vec::with_capacity(len15);
        for i in 0..len15 {
            let base = base15.add(i * 40);
            let e15 = {
                let l3 = *base.add(0).cast::<i32>();
                let l4 = *base.add(4).cast::<i32>();
                let l5 = *base.add(8).cast::<i64>();
                let l6 = *base.add(16).cast::<*mut u8>();
                let l7 = *base.add(20).cast::<usize>();
                let len8 = l7;
                let bytes8 = _rt::Vec::from_raw_parts(l6.cast(), len8, len8);
                let l9 = i32::from(*base.add(24).cast::<u8>());
                let l10 = i32::from(*base.add(25).cast::<u8>());
                let l11 = i32::from(*base.add(28).cast::<u8>());
                (
                    game::auto_rogue::types::Loc {
                        x: l3,
                        y: l4,
                    },
                    game::auto_rogue::types::Item {
                        id: l5,
                        name: _rt::string_lift(bytes8),
                        is_furniture: _rt::bool_lift(l9 as u8),
                        is_passable: _rt::bool_lift(l10 as u8),
                        metadata: match l11 {
                            0 => None,
                            1 => {
                                let e = {
                                    let l12 = *base.add(32).cast::<*mut u8>();
                                    let l13 = *base.add(36).cast::<usize>();
                                    let len14 = l13;
                                    let bytes14 = _rt::Vec::from_raw_parts(
                                        l12.cast(),
                                        len14,
                                        len14,
                                    );
                                    _rt::string_lift(bytes14)
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        },
                    },
                )
            };
            result15.push(e15);
        }
        _rt::cabi_dealloc(base15, len15 * 40, 8);
        result15
    }
}
#[allow(unused_unsafe, clippy::all)]
pub fn inventory() -> _rt::Vec<InventoryItem> {
    unsafe {
        #[repr(align(4))]
        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
        #[cfg(target_arch = "wasm32")]
        #[link(wasm_import_module = "$root")]
        extern "C" {
            #[link_name = "inventory"]
            fn wit_import(_: *mut u8);
        }
        #[cfg(not(target_arch = "wasm32"))]
        fn wit_import(_: *mut u8) {
            unreachable!()
        }
        wit_import(ptr0);
        let l1 = *ptr0.add(0).cast::<*mut u8>();
        let l2 = *ptr0.add(4).cast::<usize>();
        let base58 = l1;
        let len58 = l2;
        let mut result58 = _rt::Vec::with_capacity(len58);
        for i in 0..len58 {
            let base = base58.add(i * 40);
            let e58 = {
                let l3 = *base.add(0).cast::<*mut u8>();
                let l4 = *base.add(4).cast::<usize>();
                let len5 = l4;
                let bytes5 = _rt::Vec::from_raw_parts(l3.cast(), len5, len5);
                let l6 = *base.add(8).cast::<i64>();
                let l7 = *base.add(16).cast::<i32>();
                let l8 = *base.add(20).cast::<*mut u8>();
                let l9 = *base.add(24).cast::<usize>();
                let base49 = l8;
                let len49 = l9;
                let mut result49 = _rt::Vec::with_capacity(len49);
                for i in 0..len49 {
                    let base = base49.add(i * 16);
                    let e49 = {
                        let l10 = *base.add(0).cast::<*mut u8>();
                        let l11 = *base.add(4).cast::<usize>();
                        let len12 = l11;
                        let bytes12 = _rt::Vec::from_raw_parts(l10.cast(), len12, len12);
                        let l13 = *base.add(8).cast::<*mut u8>();
                        let l14 = *base.add(12).cast::<usize>();
                        let base48 = l13;
                        let len48 = l14;
                        let mut result48 = _rt::Vec::with_capacity(len48);
                        for i in 0..len48 {
                            let base = base48.add(i * 28);
                            let e48 = {
                                let l15 = i32::from(*base.add(0).cast::<u8>());
                                use game::auto_rogue::types::MicroAction as V47;
                                let v47 = match l15 {
                                    0 => V47::Walk,
                                    1 => {
                                        let e47 = {
                                            let l16 = *base.add(4).cast::<i32>();
                                            game::auto_rogue::types::HaulParams {
                                                strength: l16 as u32,
                                            }
                                        };
                                        V47::Haul(e47)
                                    }
                                    2 => {
                                        let e47 = {
                                            let l17 = *base.add(4).cast::<i32>();
                                            let l18 = *base.add(8).cast::<i32>();
                                            game::auto_rogue::types::AttackParams {
                                                amount: l17,
                                                range: l18 as u32,
                                            }
                                        };
                                        V47::Attack(e47)
                                    }
                                    3 => {
                                        let e47 = {
                                            let l19 = *base.add(4).cast::<i32>();
                                            let l20 = *base.add(8).cast::<*mut u8>();
                                            let l21 = *base.add(12).cast::<usize>();
                                            let len22 = l21;
                                            let bytes22 = _rt::Vec::from_raw_parts(
                                                l20.cast(),
                                                len22,
                                                len22,
                                            );
                                            let l23 = *base.add(16).cast::<i32>();
                                            let l24 = i32::from(*base.add(20).cast::<u8>());
                                            use game::auto_rogue::types::BuffDurability as V26;
                                            let v26 = match l24 {
                                                0 => V26::Transient,
                                                1 => {
                                                    let e26 = {
                                                        let l25 = *base.add(24).cast::<i32>();
                                                        l25 as u32
                                                    };
                                                    V26::DecreasePerTurn(e26)
                                                }
                                                n => {
                                                    debug_assert_eq!(n, 2, "invalid enum discriminant");
                                                    V26::Permanent
                                                }
                                            };
                                            game::auto_rogue::types::ApplyBuffParams {
                                                range: l19 as u32,
                                                name: _rt::string_lift(bytes22),
                                                amount: l23,
                                                durability: v26,
                                            }
                                        };
                                        V47::ApplyBuff(e47)
                                    }
                                    4 => {
                                        let e47 = {
                                            let l27 = *base.add(4).cast::<*mut u8>();
                                            let l28 = *base.add(8).cast::<usize>();
                                            let base33 = l27;
                                            let len33 = l28;
                                            let mut result33 = _rt::Vec::with_capacity(len33);
                                            for i in 0..len33 {
                                                let base = base33.add(i * 12);
                                                let e33 = {
                                                    let l29 = *base.add(0).cast::<*mut u8>();
                                                    let l30 = *base.add(4).cast::<usize>();
                                                    let len31 = l30;
                                                    let bytes31 = _rt::Vec::from_raw_parts(
                                                        l29.cast(),
                                                        len31,
                                                        len31,
                                                    );
                                                    let l32 = *base.add(8).cast::<i32>();
                                                    (_rt::string_lift(bytes31), l32 as u32)
                                                };
                                                result33.push(e33);
                                            }
                                            _rt::cabi_dealloc(base33, len33 * 12, 4);
                                            let l34 = *base.add(12).cast::<*mut u8>();
                                            let l35 = *base.add(16).cast::<usize>();
                                            let base39 = l34;
                                            let len39 = l35;
                                            let mut result39 = _rt::Vec::with_capacity(len39);
                                            for i in 0..len39 {
                                                let base = base39.add(i * 8);
                                                let e39 = {
                                                    let l36 = *base.add(0).cast::<*mut u8>();
                                                    let l37 = *base.add(4).cast::<usize>();
                                                    let len38 = l37;
                                                    let bytes38 = _rt::Vec::from_raw_parts(
                                                        l36.cast(),
                                                        len38,
                                                        len38,
                                                    );
                                                    _rt::string_lift(bytes38)
                                                };
                                                result39.push(e39);
                                            }
                                            _rt::cabi_dealloc(base39, len39 * 8, 4);
                                            let l40 = *base.add(20).cast::<*mut u8>();
                                            let l41 = *base.add(24).cast::<usize>();
                                            let base46 = l40;
                                            let len46 = l41;
                                            let mut result46 = _rt::Vec::with_capacity(len46);
                                            for i in 0..len46 {
                                                let base = base46.add(i * 12);
                                                let e46 = {
                                                    let l42 = *base.add(0).cast::<*mut u8>();
                                                    let l43 = *base.add(4).cast::<usize>();
                                                    let len44 = l43;
                                                    let bytes44 = _rt::Vec::from_raw_parts(
                                                        l42.cast(),
                                                        len44,
                                                        len44,
                                                    );
                                                    let l45 = *base.add(8).cast::<i32>();
                                                    (_rt::string_lift(bytes44), l45 as u32)
                                                };
                                                result46.push(e46);
                                            }
                                            _rt::cabi_dealloc(base46, len46 * 12, 4);
                                            game::auto_rogue::types::ConvertParams {
                                                input: result33,
                                                output_items: result39,
                                                output_resources: result46,
                                            }
                                        };
                                        V47::Convert(e47)
                                    }
                                    5 => V47::Equip,
                                    6 => V47::Unequip,
                                    7 => V47::PickUp,
                                    n => {
                                        debug_assert_eq!(n, 8, "invalid enum discriminant");
                                        V47::AbandonLevel
                                    }
                                };
                                v47
                            };
                            result48.push(e48);
                        }
                        _rt::cabi_dealloc(base48, len48 * 28, 4);
                        game::auto_rogue::types::Action {
                            name: _rt::string_lift(bytes12),
                            micro_actions: result48,
                        }
                    };
                    result49.push(e49);
                }
                _rt::cabi_dealloc(base49, len49 * 16, 4);
                let l50 = i32::from(*base.add(28).cast::<u8>());
                game::auto_rogue::types::InventoryItem {
                    name: _rt::string_lift(bytes5),
                    id: l6,
                    level: l7 as u32,
                    actions: result49,
                    resources: match l50 {
                        0 => None,
                        1 => {
                            let e = {
                                let l51 = *base.add(32).cast::<*mut u8>();
                                let l52 = *base.add(36).cast::<usize>();
                                let base57 = l51;
                                let len57 = l52;
                                let mut result57 = _rt::Vec::with_capacity(len57);
                                for i in 0..len57 {
                                    let base = base57.add(i * 12);
                                    let e57 = {
                                        let l53 = *base.add(0).cast::<*mut u8>();
                                        let l54 = *base.add(4).cast::<usize>();
                                        let len55 = l54;
                                        let bytes55 = _rt::Vec::from_raw_parts(
                                            l53.cast(),
                                            len55,
                                            len55,
                                        );
                                        let l56 = *base.add(8).cast::<i32>();
                                        (_rt::string_lift(bytes55), l56 as u32)
                                    };
                                    result57.push(e57);
                                }
                                _rt::cabi_dealloc(base57, len57 * 12, 4);
                                result57
                            };
                            Some(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    },
                }
            };
            result58.push(e58);
        }
        _rt::cabi_dealloc(base58, len58 * 40, 8);
        result58
    }
}
#[allow(unused_unsafe, clippy::all)]
pub fn get_equipment_state() -> EquipmentState {
    unsafe {
        #[repr(align(8))]
        struct RetArea([::core::mem::MaybeUninit<u8>; 32]);
        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 32]);
        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
        #[cfg(target_arch = "wasm32")]
        #[link(wasm_import_module = "$root")]
        extern "C" {
            #[link_name = "get-equipment-state"]
            fn wit_import(_: *mut u8);
        }
        #[cfg(not(target_arch = "wasm32"))]
        fn wit_import(_: *mut u8) {
            unreachable!()
        }
        wit_import(ptr0);
        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
        let l3 = i32::from(*ptr0.add(16).cast::<u8>());
        game::auto_rogue::types::EquipmentState {
            left_hand: match l1 {
                0 => None,
                1 => {
                    let e = {
                        let l2 = *ptr0.add(8).cast::<i64>();
                        l2
                    };
                    Some(e)
                }
                _ => _rt::invalid_enum_discriminant(),
            },
            right_hand: match l3 {
                0 => None,
                1 => {
                    let e = {
                        let l4 = *ptr0.add(24).cast::<i64>();
                        l4
                    };
                    Some(e)
                }
                _ => _rt::invalid_enum_discriminant(),
            },
        }
    }
}
#[allow(unused_unsafe, clippy::all)]
pub fn get_character_stats() -> CharacterStats {
    unsafe {
        #[repr(align(4))]
        struct RetArea([::core::mem::MaybeUninit<u8>; 32]);
        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 32]);
        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
        #[cfg(target_arch = "wasm32")]
        #[link(wasm_import_module = "$root")]
        extern "C" {
            #[link_name = "get-character-stats"]
            fn wit_import(_: *mut u8);
        }
        #[cfg(not(target_arch = "wasm32"))]
        fn wit_import(_: *mut u8) {
            unreachable!()
        }
        wit_import(ptr0);
        let l1 = *ptr0.add(0).cast::<i32>();
        let l2 = *ptr0.add(4).cast::<i32>();
        let l3 = *ptr0.add(8).cast::<i32>();
        let l4 = *ptr0.add(12).cast::<i32>();
        let l5 = *ptr0.add(16).cast::<i32>();
        let l6 = *ptr0.add(20).cast::<i32>();
        let l7 = *ptr0.add(24).cast::<i32>();
        let l8 = *ptr0.add(28).cast::<i32>();
        game::auto_rogue::types::CharacterStats {
            max: game::auto_rogue::types::Stats {
                strength: l1 as u32,
                hp: l2 as u32,
                speed: l3 as u32,
                inventory_size: l4 as u32,
            },
            current: game::auto_rogue::types::Stats {
                strength: l5 as u32,
                hp: l6 as u32,
                speed: l7 as u32,
                inventory_size: l8 as u32,
            },
        }
    }
}
#[allow(unused_unsafe, clippy::all)]
pub fn character_buffs() -> _rt::Vec<Buff> {
    unsafe {
        #[repr(align(4))]
        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
        #[cfg(target_arch = "wasm32")]
        #[link(wasm_import_module = "$root")]
        extern "C" {
            #[link_name = "character-buffs"]
            fn wit_import(_: *mut u8);
        }
        #[cfg(not(target_arch = "wasm32"))]
        fn wit_import(_: *mut u8) {
            unreachable!()
        }
        wit_import(ptr0);
        let l1 = *ptr0.add(0).cast::<*mut u8>();
        let l2 = *ptr0.add(4).cast::<usize>();
        let base10 = l1;
        let len10 = l2;
        let mut result10 = _rt::Vec::with_capacity(len10);
        for i in 0..len10 {
            let base = base10.add(i * 20);
            let e10 = {
                let l3 = *base.add(0).cast::<*mut u8>();
                let l4 = *base.add(4).cast::<usize>();
                let len5 = l4;
                let bytes5 = _rt::Vec::from_raw_parts(l3.cast(), len5, len5);
                let l6 = *base.add(8).cast::<i32>();
                let l7 = i32::from(*base.add(12).cast::<u8>());
                use game::auto_rogue::types::BuffDurability as V9;
                let v9 = match l7 {
                    0 => V9::Transient,
                    1 => {
                        let e9 = {
                            let l8 = *base.add(16).cast::<i32>();
                            l8 as u32
                        };
                        V9::DecreasePerTurn(e9)
                    }
                    n => {
                        debug_assert_eq!(n, 2, "invalid enum discriminant");
                        V9::Permanent
                    }
                };
                game::auto_rogue::types::Buff {
                    name: _rt::string_lift(bytes5),
                    amount: l6 as u32,
                    durability: v9,
                }
            };
            result10.push(e10);
        }
        _rt::cabi_dealloc(base10, len10 * 20, 4);
        result10
    }
}
#[allow(unused_unsafe, clippy::all)]
pub fn get_game_state() -> GameState {
    unsafe {
        #[repr(align(8))]
        struct RetArea([::core::mem::MaybeUninit<u8>; 24]);
        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 24]);
        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
        #[cfg(target_arch = "wasm32")]
        #[link(wasm_import_module = "$root")]
        extern "C" {
            #[link_name = "get-game-state"]
            fn wit_import(_: *mut u8);
        }
        #[cfg(not(target_arch = "wasm32"))]
        fn wit_import(_: *mut u8) {
            unreachable!()
        }
        wit_import(ptr0);
        let l1 = *ptr0.add(0).cast::<i64>();
        let l2 = *ptr0.add(8).cast::<i64>();
        let l3 = i32::from(*ptr0.add(16).cast::<u8>());
        game::auto_rogue::types::GameState {
            turn: l1,
            level_id: l2,
            level_is_stable: _rt::bool_lift(l3 as u8),
        }
    }
}
#[allow(unused_unsafe, clippy::all)]
pub fn actions() -> _rt::Vec<Action> {
    unsafe {
        #[repr(align(4))]
        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
        #[cfg(target_arch = "wasm32")]
        #[link(wasm_import_module = "$root")]
        extern "C" {
            #[link_name = "actions"]
            fn wit_import(_: *mut u8);
        }
        #[cfg(not(target_arch = "wasm32"))]
        fn wit_import(_: *mut u8) {
            unreachable!()
        }
        wit_import(ptr0);
        let l1 = *ptr0.add(0).cast::<*mut u8>();
        let l2 = *ptr0.add(4).cast::<usize>();
        let base42 = l1;
        let len42 = l2;
        let mut result42 = _rt::Vec::with_capacity(len42);
        for i in 0..len42 {
            let base = base42.add(i * 16);
            let e42 = {
                let l3 = *base.add(0).cast::<*mut u8>();
                let l4 = *base.add(4).cast::<usize>();
                let len5 = l4;
                let bytes5 = _rt::Vec::from_raw_parts(l3.cast(), len5, len5);
                let l6 = *base.add(8).cast::<*mut u8>();
                let l7 = *base.add(12).cast::<usize>();
                let base41 = l6;
                let len41 = l7;
                let mut result41 = _rt::Vec::with_capacity(len41);
                for i in 0..len41 {
                    let base = base41.add(i * 28);
                    let e41 = {
                        let l8 = i32::from(*base.add(0).cast::<u8>());
                        use game::auto_rogue::types::MicroAction as V40;
                        let v40 = match l8 {
                            0 => V40::Walk,
                            1 => {
                                let e40 = {
                                    let l9 = *base.add(4).cast::<i32>();
                                    game::auto_rogue::types::HaulParams {
                                        strength: l9 as u32,
                                    }
                                };
                                V40::Haul(e40)
                            }
                            2 => {
                                let e40 = {
                                    let l10 = *base.add(4).cast::<i32>();
                                    let l11 = *base.add(8).cast::<i32>();
                                    game::auto_rogue::types::AttackParams {
                                        amount: l10,
                                        range: l11 as u32,
                                    }
                                };
                                V40::Attack(e40)
                            }
                            3 => {
                                let e40 = {
                                    let l12 = *base.add(4).cast::<i32>();
                                    let l13 = *base.add(8).cast::<*mut u8>();
                                    let l14 = *base.add(12).cast::<usize>();
                                    let len15 = l14;
                                    let bytes15 = _rt::Vec::from_raw_parts(
                                        l13.cast(),
                                        len15,
                                        len15,
                                    );
                                    let l16 = *base.add(16).cast::<i32>();
                                    let l17 = i32::from(*base.add(20).cast::<u8>());
                                    use game::auto_rogue::types::BuffDurability as V19;
                                    let v19 = match l17 {
                                        0 => V19::Transient,
                                        1 => {
                                            let e19 = {
                                                let l18 = *base.add(24).cast::<i32>();
                                                l18 as u32
                                            };
                                            V19::DecreasePerTurn(e19)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 2, "invalid enum discriminant");
                                            V19::Permanent
                                        }
                                    };
                                    game::auto_rogue::types::ApplyBuffParams {
                                        range: l12 as u32,
                                        name: _rt::string_lift(bytes15),
                                        amount: l16,
                                        durability: v19,
                                    }
                                };
                                V40::ApplyBuff(e40)
                            }
                            4 => {
                                let e40 = {
                                    let l20 = *base.add(4).cast::<*mut u8>();
                                    let l21 = *base.add(8).cast::<usize>();
                                    let base26 = l20;
                                    let len26 = l21;
                                    let mut result26 = _rt::Vec::with_capacity(len26);
                                    for i in 0..len26 {
                                        let base = base26.add(i * 12);
                                        let e26 = {
                                            let l22 = *base.add(0).cast::<*mut u8>();
                                            let l23 = *base.add(4).cast::<usize>();
                                            let len24 = l23;
                                            let bytes24 = _rt::Vec::from_raw_parts(
                                                l22.cast(),
                                                len24,
                                                len24,
                                            );
                                            let l25 = *base.add(8).cast::<i32>();
                                            (_rt::string_lift(bytes24), l25 as u32)
                                        };
                                        result26.push(e26);
                                    }
                                    _rt::cabi_dealloc(base26, len26 * 12, 4);
                                    let l27 = *base.add(12).cast::<*mut u8>();
                                    let l28 = *base.add(16).cast::<usize>();
                                    let base32 = l27;
                                    let len32 = l28;
                                    let mut result32 = _rt::Vec::with_capacity(len32);
                                    for i in 0..len32 {
                                        let base = base32.add(i * 8);
                                        let e32 = {
                                            let l29 = *base.add(0).cast::<*mut u8>();
                                            let l30 = *base.add(4).cast::<usize>();
                                            let len31 = l30;
                                            let bytes31 = _rt::Vec::from_raw_parts(
                                                l29.cast(),
                                                len31,
                                                len31,
                                            );
                                            _rt::string_lift(bytes31)
                                        };
                                        result32.push(e32);
                                    }
                                    _rt::cabi_dealloc(base32, len32 * 8, 4);
                                    let l33 = *base.add(20).cast::<*mut u8>();
                                    let l34 = *base.add(24).cast::<usize>();
                                    let base39 = l33;
                                    let len39 = l34;
                                    let mut result39 = _rt::Vec::with_capacity(len39);
                                    for i in 0..len39 {
                                        let base = base39.add(i * 12);
                                        let e39 = {
                                            let l35 = *base.add(0).cast::<*mut u8>();
                                            let l36 = *base.add(4).cast::<usize>();
                                            let len37 = l36;
                                            let bytes37 = _rt::Vec::from_raw_parts(
                                                l35.cast(),
                                                len37,
                                                len37,
                                            );
                                            let l38 = *base.add(8).cast::<i32>();
                                            (_rt::string_lift(bytes37), l38 as u32)
                                        };
                                        result39.push(e39);
                                    }
                                    _rt::cabi_dealloc(base39, len39 * 12, 4);
                                    game::auto_rogue::types::ConvertParams {
                                        input: result26,
                                        output_items: result32,
                                        output_resources: result39,
                                    }
                                };
                                V40::Convert(e40)
                            }
                            5 => V40::Equip,
                            6 => V40::Unequip,
                            7 => V40::PickUp,
                            n => {
                                debug_assert_eq!(n, 8, "invalid enum discriminant");
                                V40::AbandonLevel
                            }
                        };
                        v40
                    };
                    result41.push(e41);
                }
                _rt::cabi_dealloc(base41, len41 * 28, 4);
                game::auto_rogue::types::Action {
                    name: _rt::string_lift(bytes5),
                    micro_actions: result41,
                }
            };
            result42.push(e42);
        }
        _rt::cabi_dealloc(base42, len42 * 16, 4);
        result42
    }
}
#[allow(unused_unsafe, clippy::all)]
pub fn load_store() -> _rt::Vec<u8> {
    unsafe {
        #[repr(align(4))]
        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
        #[cfg(target_arch = "wasm32")]
        #[link(wasm_import_module = "$root")]
        extern "C" {
            #[link_name = "load-store"]
            fn wit_import(_: *mut u8);
        }
        #[cfg(not(target_arch = "wasm32"))]
        fn wit_import(_: *mut u8) {
            unreachable!()
        }
        wit_import(ptr0);
        let l1 = *ptr0.add(0).cast::<*mut u8>();
        let l2 = *ptr0.add(4).cast::<usize>();
        let len3 = l2;
        _rt::Vec::from_raw_parts(l1.cast(), len3, len3)
    }
}
#[allow(unused_unsafe, clippy::all)]
pub fn save_store(store: &[u8]) {
    unsafe {
        let vec0 = store;
        let ptr0 = vec0.as_ptr().cast::<u8>();
        let len0 = vec0.len();
        #[cfg(target_arch = "wasm32")]
        #[link(wasm_import_module = "$root")]
        extern "C" {
            #[link_name = "save-store"]
            fn wit_import(_: *mut u8, _: usize);
        }
        #[cfg(not(target_arch = "wasm32"))]
        fn wit_import(_: *mut u8, _: usize) {
            unreachable!()
        }
        wit_import(ptr0.cast_mut(), len0);
    }
}
#[allow(unused_unsafe, clippy::all)]
pub fn broadcast(data: Option<&[u8]>) {
    unsafe {
        let (result1_0, result1_1, result1_2) = match data {
            Some(e) => {
                let vec0 = e;
                let ptr0 = vec0.as_ptr().cast::<u8>();
                let len0 = vec0.len();
                (1i32, ptr0.cast_mut(), len0)
            }
            None => (0i32, ::core::ptr::null_mut(), 0usize),
        };
        #[cfg(target_arch = "wasm32")]
        #[link(wasm_import_module = "$root")]
        extern "C" {
            #[link_name = "broadcast"]
            fn wit_import(_: i32, _: *mut u8, _: usize);
        }
        #[cfg(not(target_arch = "wasm32"))]
        fn wit_import(_: i32, _: *mut u8, _: usize) {
            unreachable!()
        }
        wit_import(result1_0, result1_1, result1_2);
    }
}
#[allow(unused_unsafe, clippy::all)]
pub fn highlight_tiles(tiles: &[Loc]) {
    unsafe {
        let vec0 = tiles;
        let ptr0 = vec0.as_ptr().cast::<u8>();
        let len0 = vec0.len();
        #[cfg(target_arch = "wasm32")]
        #[link(wasm_import_module = "$root")]
        extern "C" {
            #[link_name = "highlight-tiles"]
            fn wit_import(_: *mut u8, _: usize);
        }
        #[cfg(not(target_arch = "wasm32"))]
        fn wit_import(_: *mut u8, _: usize) {
            unreachable!()
        }
        wit_import(ptr0.cast_mut(), len0);
    }
}
#[allow(unused_unsafe, clippy::all)]
pub fn events() -> _rt::Vec<Event> {
    unsafe {
        #[repr(align(4))]
        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
        #[cfg(target_arch = "wasm32")]
        #[link(wasm_import_module = "$root")]
        extern "C" {
            #[link_name = "events"]
            fn wit_import(_: *mut u8);
        }
        #[cfg(not(target_arch = "wasm32"))]
        fn wit_import(_: *mut u8) {
            unreachable!()
        }
        wit_import(ptr0);
        let l1 = *ptr0.add(0).cast::<*mut u8>();
        let l2 = *ptr0.add(4).cast::<usize>();
        let base35 = l1;
        let len35 = l2;
        let mut result35 = _rt::Vec::with_capacity(len35);
        for i in 0..len35 {
            let base = base35.add(i * 56);
            let e35 = {
                let l3 = i32::from(*base.add(0).cast::<u8>());
                use game::auto_rogue::types::Event as V34;
                let v34 = match l3 {
                    0 => {
                        let e34 = {
                            let l4 = *base.add(8).cast::<i32>();
                            let l5 = *base.add(12).cast::<i32>();
                            (l4, l5)
                        };
                        V34::Moved(e34)
                    }
                    1 => {
                        let e34 = {
                            let l6 = *base.add(8).cast::<i32>();
                            let l7 = *base.add(12).cast::<i32>();
                            (l6, l7)
                        };
                        V34::Hauled(e34)
                    }
                    2 => {
                        let e34 = {
                            let l8 = *base.add(8).cast::<*mut u8>();
                            let l9 = *base.add(12).cast::<usize>();
                            let len10 = l9;
                            let bytes10 = _rt::Vec::from_raw_parts(
                                l8.cast(),
                                len10,
                                len10,
                            );
                            let l11 = *base.add(16).cast::<i64>();
                            let l12 = *base.add(24).cast::<i32>();
                            let l13 = i32::from(*base.add(28).cast::<u8>());
                            let l17 = *base.add(40).cast::<i32>();
                            let l18 = *base.add(44).cast::<i32>();
                            let l19 = *base.add(48).cast::<i32>();
                            game::auto_rogue::types::AttackDescription {
                                initiator: game::auto_rogue::types::Creature {
                                    name: _rt::string_lift(bytes10),
                                    id: l11,
                                    faction: l12 as u32,
                                    broadcast: match l13 {
                                        0 => None,
                                        1 => {
                                            let e = {
                                                let l14 = *base.add(32).cast::<*mut u8>();
                                                let l15 = *base.add(36).cast::<usize>();
                                                let len16 = l15;
                                                _rt::Vec::from_raw_parts(l14.cast(), len16, len16)
                                            };
                                            Some(e)
                                        }
                                        _ => _rt::invalid_enum_discriminant(),
                                    },
                                },
                                initiator_location: game::auto_rogue::types::Loc {
                                    x: l17,
                                    y: l18,
                                },
                                amount: l19,
                            }
                        };
                        V34::Attacked(e34)
                    }
                    3 => {
                        let e34 = {
                            let l20 = *base.add(8).cast::<i64>();
                            l20
                        };
                        V34::AddInventoryItem(e34)
                    }
                    4 => {
                        let e34 = {
                            let l21 = *base.add(8).cast::<i64>();
                            l21
                        };
                        V34::RemoveInventoryItem(e34)
                    }
                    5 => {
                        let e34 = {
                            let l22 = i32::from(*base.add(8).cast::<u8>());
                            use game::auto_rogue::types::EquipmentSlot as V23;
                            let v23 = match l22 {
                                0 => V23::LeftHand,
                                n => {
                                    debug_assert_eq!(n, 1, "invalid enum discriminant");
                                    V23::RightHand
                                }
                            };
                            let l24 = *base.add(16).cast::<i64>();
                            (v23, l24)
                        };
                        V34::EquipItem(e34)
                    }
                    6 => {
                        let e34 = {
                            let l25 = i32::from(*base.add(8).cast::<u8>());
                            use game::auto_rogue::types::EquipmentSlot as V26;
                            let v26 = match l25 {
                                0 => V26::LeftHand,
                                n => {
                                    debug_assert_eq!(n, 1, "invalid enum discriminant");
                                    V26::RightHand
                                }
                            };
                            v26
                        };
                        V34::UnequipItem(e34)
                    }
                    7 => {
                        let e34 = {
                            let l27 = *base.add(8).cast::<*mut u8>();
                            let l28 = *base.add(12).cast::<usize>();
                            let base33 = l27;
                            let len33 = l28;
                            let mut result33 = _rt::Vec::with_capacity(len33);
                            for i in 0..len33 {
                                let base = base33.add(i * 12);
                                let e33 = {
                                    let l29 = *base.add(0).cast::<*mut u8>();
                                    let l30 = *base.add(4).cast::<usize>();
                                    let len31 = l30;
                                    let bytes31 = _rt::Vec::from_raw_parts(
                                        l29.cast(),
                                        len31,
                                        len31,
                                    );
                                    let l32 = *base.add(8).cast::<i32>();
                                    (_rt::string_lift(bytes31), l32 as u32)
                                };
                                result33.push(e33);
                            }
                            _rt::cabi_dealloc(base33, len33 * 12, 4);
                            result33
                        };
                        V34::GainResource(e34)
                    }
                    n => {
                        debug_assert_eq!(n, 8, "invalid enum discriminant");
                        V34::ChangedLevel
                    }
                };
                v34
            };
            result35.push(e35);
        }
        _rt::cabi_dealloc(base35, len35 * 56, 8);
        result35
    }
}
#[allow(unused_unsafe, clippy::all)]
pub fn config_data() -> Option<_rt::Vec<u8>> {
    unsafe {
        #[repr(align(4))]
        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 12]);
        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
        #[cfg(target_arch = "wasm32")]
        #[link(wasm_import_module = "$root")]
        extern "C" {
            #[link_name = "config-data"]
            fn wit_import(_: *mut u8);
        }
        #[cfg(not(target_arch = "wasm32"))]
        fn wit_import(_: *mut u8) {
            unreachable!()
        }
        wit_import(ptr0);
        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
        match l1 {
            0 => None,
            1 => {
                let e = {
                    let l2 = *ptr0.add(4).cast::<*mut u8>();
                    let l3 = *ptr0.add(8).cast::<usize>();
                    let len4 = l3;
                    _rt::Vec::from_raw_parts(l2.cast(), len4, len4)
                };
                Some(e)
            }
            _ => _rt::invalid_enum_discriminant(),
        }
    }
}
#[allow(unused_unsafe, clippy::all)]
pub fn editor_debug(data: &[u8]) {
    unsafe {
        let vec0 = data;
        let ptr0 = vec0.as_ptr().cast::<u8>();
        let len0 = vec0.len();
        #[cfg(target_arch = "wasm32")]
        #[link(wasm_import_module = "$root")]
        extern "C" {
            #[link_name = "editor-debug"]
            fn wit_import(_: *mut u8, _: usize);
        }
        #[cfg(not(target_arch = "wasm32"))]
        fn wit_import(_: *mut u8, _: usize) {
            unreachable!()
        }
        wit_import(ptr0.cast_mut(), len0);
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_editor_config_cabi<T: Guest>() -> *mut u8 {
    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
    let result0 = T::editor_config();
    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
    match result0 {
        Some(e) => {
            *ptr1.add(0).cast::<u8>() = (1i32) as u8;
            let vec2 = (e).into_boxed_slice();
            let ptr2 = vec2.as_ptr().cast::<u8>();
            let len2 = vec2.len();
            ::core::mem::forget(vec2);
            *ptr1.add(8).cast::<usize>() = len2;
            *ptr1.add(4).cast::<*mut u8>() = ptr2.cast_mut();
        }
        None => {
            *ptr1.add(0).cast::<u8>() = (0i32) as u8;
        }
    };
    ptr1
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn __post_return_editor_config<T: Guest>(arg0: *mut u8) {
    let l0 = i32::from(*arg0.add(0).cast::<u8>());
    match l0 {
        0 => {}
        _ => {
            let l1 = *arg0.add(4).cast::<*mut u8>();
            let l2 = *arg0.add(8).cast::<usize>();
            let base3 = l1;
            let len3 = l2;
            _rt::cabi_dealloc(base3, len3 * 1, 1);
        }
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_step_cabi<T: Guest>() -> *mut u8 {
    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
    let result0 = T::step();
    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
    use game::auto_rogue::types::Command as V10;
    match result0 {
        V10::UseAction(e) => {
            *ptr1.add(0).cast::<u8>() = (0i32) as u8;
            let (t2_0, t2_1) = e;
            *ptr1.add(8).cast::<i32>() = _rt::as_i32(t2_0);
            match t2_1 {
                Some(e) => {
                    *ptr1.add(16).cast::<u8>() = (1i32) as u8;
                    use game::auto_rogue::types::ActionTarget as V9;
                    match e {
                        V9::Creature(e) => {
                            *ptr1.add(24).cast::<u8>() = (0i32) as u8;
                            *ptr1.add(32).cast::<i64>() = _rt::as_i64(e);
                        }
                        V9::Actor => {
                            *ptr1.add(24).cast::<u8>() = (1i32) as u8;
                        }
                        V9::EquipmentSlot(e) => {
                            *ptr1.add(24).cast::<u8>() = (2i32) as u8;
                            use game::auto_rogue::types::EquipmentSlot as V3;
                            match e {
                                V3::LeftHand => {
                                    *ptr1.add(32).cast::<u8>() = (0i32) as u8;
                                }
                                V3::RightHand => {
                                    *ptr1.add(32).cast::<u8>() = (1i32) as u8;
                                }
                            }
                        }
                        V9::EquipmentSlotAndItem(e) => {
                            *ptr1.add(24).cast::<u8>() = (3i32) as u8;
                            let (t4_0, t4_1) = e;
                            use game::auto_rogue::types::EquipmentSlot as V5;
                            match t4_0 {
                                V5::LeftHand => {
                                    *ptr1.add(32).cast::<u8>() = (0i32) as u8;
                                }
                                V5::RightHand => {
                                    *ptr1.add(32).cast::<u8>() = (1i32) as u8;
                                }
                            }
                            *ptr1.add(40).cast::<i64>() = _rt::as_i64(t4_1);
                        }
                        V9::Direction(e) => {
                            *ptr1.add(24).cast::<u8>() = (4i32) as u8;
                            use game::auto_rogue::types::Direction as V6;
                            match e {
                                V6::North => {
                                    *ptr1.add(32).cast::<u8>() = (0i32) as u8;
                                }
                                V6::NorthWest => {
                                    *ptr1.add(32).cast::<u8>() = (1i32) as u8;
                                }
                                V6::NorthEast => {
                                    *ptr1.add(32).cast::<u8>() = (2i32) as u8;
                                }
                                V6::South => {
                                    *ptr1.add(32).cast::<u8>() = (3i32) as u8;
                                }
                                V6::SouthWest => {
                                    *ptr1.add(32).cast::<u8>() = (4i32) as u8;
                                }
                                V6::SouthEast => {
                                    *ptr1.add(32).cast::<u8>() = (5i32) as u8;
                                }
                                V6::East => {
                                    *ptr1.add(32).cast::<u8>() = (6i32) as u8;
                                }
                                V6::West => {
                                    *ptr1.add(32).cast::<u8>() = (7i32) as u8;
                                }
                            }
                        }
                        V9::Items(e) => {
                            *ptr1.add(24).cast::<u8>() = (5i32) as u8;
                            let vec7 = (e).into_boxed_slice();
                            let ptr7 = vec7.as_ptr().cast::<u8>();
                            let len7 = vec7.len();
                            ::core::mem::forget(vec7);
                            *ptr1.add(36).cast::<usize>() = len7;
                            *ptr1.add(32).cast::<*mut u8>() = ptr7.cast_mut();
                        }
                        V9::Location(e) => {
                            *ptr1.add(24).cast::<u8>() = (6i32) as u8;
                            let game::auto_rogue::types::Loc { x: x8, y: y8 } = e;
                            *ptr1.add(32).cast::<i32>() = _rt::as_i32(x8);
                            *ptr1.add(36).cast::<i32>() = _rt::as_i32(y8);
                        }
                    }
                }
                None => {
                    *ptr1.add(16).cast::<u8>() = (0i32) as u8;
                }
            };
        }
        V10::Nothing => {
            *ptr1.add(0).cast::<u8>() = (1i32) as u8;
        }
    }
    ptr1
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn __post_return_step<T: Guest>(arg0: *mut u8) {
    let l0 = i32::from(*arg0.add(0).cast::<u8>());
    match l0 {
        0 => {
            let l1 = i32::from(*arg0.add(16).cast::<u8>());
            match l1 {
                0 => {}
                _ => {
                    let l2 = i32::from(*arg0.add(24).cast::<u8>());
                    match l2 {
                        0 => {}
                        1 => {}
                        2 => {}
                        3 => {}
                        4 => {}
                        5 => {
                            let l3 = *arg0.add(32).cast::<*mut u8>();
                            let l4 = *arg0.add(36).cast::<usize>();
                            let base5 = l3;
                            let len5 = l4;
                            _rt::cabi_dealloc(base5, len5 * 8, 8);
                        }
                        _ => {}
                    }
                }
            }
        }
        _ => {}
    }
}
pub trait Guest {
    fn editor_config() -> Option<_rt::Vec<u8>>;
    fn step() -> Command;
}
#[doc(hidden)]
#[macro_export]
macro_rules! __export_world_auto_rogue_ai_cabi {
    ($ty:ident with_types_in $($path_to_types:tt)*) => {
        const _ : () = { #[export_name = "editor-config"] unsafe extern "C" fn
        export_editor_config() -> * mut u8 { $($path_to_types)*::
        _export_editor_config_cabi::<$ty > () } #[export_name =
        "cabi_post_editor-config"] unsafe extern "C" fn _post_return_editor_config(arg0 :
        * mut u8,) { $($path_to_types)*:: __post_return_editor_config::<$ty > (arg0) }
        #[export_name = "step"] unsafe extern "C" fn export_step() -> * mut u8 {
        $($path_to_types)*:: _export_step_cabi::<$ty > () } #[export_name =
        "cabi_post_step"] unsafe extern "C" fn _post_return_step(arg0 : * mut u8,) {
        $($path_to_types)*:: __post_return_step::<$ty > (arg0) } };
    };
}
#[doc(hidden)]
#[allow(unused_imports)]pub use __export_world_auto_rogue_ai_cabi;
#[repr(align(8))]
struct _RetArea([::core::mem::MaybeUninit<u8>; 48]);
static mut _RET_AREA: _RetArea = _RetArea([::core::mem::MaybeUninit::uninit(); 48]);
#[allow(dead_code)]
pub mod game {
    #[allow(dead_code)]
    pub mod auto_rogue {
        #[allow(dead_code, clippy::all)]
        pub mod types {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct Loc {
                pub x: i32,
                pub y: i32,
            }
            impl ::core::fmt::Debug for Loc {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("Loc")
                        .field("x", &self.x)
                        .field("y", &self.y)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct Tile {
                pub passable: bool,
                pub opaque: bool,
                pub name: _rt::String,
            }
            impl ::core::fmt::Debug for Tile {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("Tile")
                        .field("passable", &self.passable)
                        .field("opaque", &self.opaque)
                        .field("name", &self.name)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct Creature {
                pub name: _rt::String,
                pub id: i64,
                pub faction: u32,
                pub broadcast: Option<_rt::Vec<u8>>,
            }
            impl ::core::fmt::Debug for Creature {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("Creature")
                        .field("name", &self.name)
                        .field("id", &self.id)
                        .field("faction", &self.faction)
                        .field("broadcast", &self.broadcast)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct Item {
                pub id: i64,
                pub name: _rt::String,
                pub is_furniture: bool,
                pub is_passable: bool,
                pub metadata: Option<_rt::String>,
            }
            impl ::core::fmt::Debug for Item {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("Item")
                        .field("id", &self.id)
                        .field("name", &self.name)
                        .field("is-furniture", &self.is_furniture)
                        .field("is-passable", &self.is_passable)
                        .field("metadata", &self.metadata)
                        .finish()
                }
            }
            #[derive(Clone, Copy)]
            pub enum EquipmentSlot {
                LeftHand,
                RightHand,
            }
            impl ::core::fmt::Debug for EquipmentSlot {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        EquipmentSlot::LeftHand => {
                            f.debug_tuple("EquipmentSlot::LeftHand").finish()
                        }
                        EquipmentSlot::RightHand => {
                            f.debug_tuple("EquipmentSlot::RightHand").finish()
                        }
                    }
                }
            }
            #[derive(Clone, Copy)]
            pub enum BuffDurability {
                Transient,
                DecreasePerTurn(u32),
                Permanent,
            }
            impl ::core::fmt::Debug for BuffDurability {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        BuffDurability::Transient => {
                            f.debug_tuple("BuffDurability::Transient").finish()
                        }
                        BuffDurability::DecreasePerTurn(e) => {
                            f.debug_tuple("BuffDurability::DecreasePerTurn")
                                .field(e)
                                .finish()
                        }
                        BuffDurability::Permanent => {
                            f.debug_tuple("BuffDurability::Permanent").finish()
                        }
                    }
                }
            }
            #[derive(Clone, Copy)]
            pub enum Direction {
                North,
                NorthWest,
                NorthEast,
                South,
                SouthWest,
                SouthEast,
                East,
                West,
            }
            impl ::core::fmt::Debug for Direction {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        Direction::North => f.debug_tuple("Direction::North").finish(),
                        Direction::NorthWest => {
                            f.debug_tuple("Direction::NorthWest").finish()
                        }
                        Direction::NorthEast => {
                            f.debug_tuple("Direction::NorthEast").finish()
                        }
                        Direction::South => f.debug_tuple("Direction::South").finish(),
                        Direction::SouthWest => {
                            f.debug_tuple("Direction::SouthWest").finish()
                        }
                        Direction::SouthEast => {
                            f.debug_tuple("Direction::SouthEast").finish()
                        }
                        Direction::East => f.debug_tuple("Direction::East").finish(),
                        Direction::West => f.debug_tuple("Direction::West").finish(),
                    }
                }
            }
            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct AttackParams {
                pub amount: i32,
                pub range: u32,
            }
            impl ::core::fmt::Debug for AttackParams {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("AttackParams")
                        .field("amount", &self.amount)
                        .field("range", &self.range)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct ApplyBuffParams {
                pub range: u32,
                pub name: _rt::String,
                pub amount: i32,
                pub durability: BuffDurability,
            }
            impl ::core::fmt::Debug for ApplyBuffParams {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("ApplyBuffParams")
                        .field("range", &self.range)
                        .field("name", &self.name)
                        .field("amount", &self.amount)
                        .field("durability", &self.durability)
                        .finish()
                }
            }
            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct HaulParams {
                pub strength: u32,
            }
            impl ::core::fmt::Debug for HaulParams {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("HaulParams")
                        .field("strength", &self.strength)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub enum ActionTarget {
                Creature(i64),
                Actor,
                EquipmentSlot(EquipmentSlot),
                EquipmentSlotAndItem((EquipmentSlot, i64)),
                Direction(Direction),
                Items(_rt::Vec<i64>),
                Location(Loc),
            }
            impl ::core::fmt::Debug for ActionTarget {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        ActionTarget::Creature(e) => {
                            f.debug_tuple("ActionTarget::Creature").field(e).finish()
                        }
                        ActionTarget::Actor => {
                            f.debug_tuple("ActionTarget::Actor").finish()
                        }
                        ActionTarget::EquipmentSlot(e) => {
                            f.debug_tuple("ActionTarget::EquipmentSlot")
                                .field(e)
                                .finish()
                        }
                        ActionTarget::EquipmentSlotAndItem(e) => {
                            f.debug_tuple("ActionTarget::EquipmentSlotAndItem")
                                .field(e)
                                .finish()
                        }
                        ActionTarget::Direction(e) => {
                            f.debug_tuple("ActionTarget::Direction").field(e).finish()
                        }
                        ActionTarget::Items(e) => {
                            f.debug_tuple("ActionTarget::Items").field(e).finish()
                        }
                        ActionTarget::Location(e) => {
                            f.debug_tuple("ActionTarget::Location").field(e).finish()
                        }
                    }
                }
            }
            #[derive(Clone)]
            pub enum Command {
                UseAction((u32, Option<ActionTarget>)),
                Nothing,
            }
            impl ::core::fmt::Debug for Command {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        Command::UseAction(e) => {
                            f.debug_tuple("Command::UseAction").field(e).finish()
                        }
                        Command::Nothing => f.debug_tuple("Command::Nothing").finish(),
                    }
                }
            }
            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct EquipmentState {
                pub left_hand: Option<i64>,
                pub right_hand: Option<i64>,
            }
            impl ::core::fmt::Debug for EquipmentState {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("EquipmentState")
                        .field("left-hand", &self.left_hand)
                        .field("right-hand", &self.right_hand)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct Buff {
                pub name: _rt::String,
                pub amount: u32,
                pub durability: BuffDurability,
            }
            impl ::core::fmt::Debug for Buff {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("Buff")
                        .field("name", &self.name)
                        .field("amount", &self.amount)
                        .field("durability", &self.durability)
                        .finish()
                }
            }
            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct GameState {
                pub turn: i64,
                pub level_id: i64,
                pub level_is_stable: bool,
            }
            impl ::core::fmt::Debug for GameState {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("GameState")
                        .field("turn", &self.turn)
                        .field("level-id", &self.level_id)
                        .field("level-is-stable", &self.level_is_stable)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct AttackDescription {
                pub initiator: Creature,
                pub initiator_location: Loc,
                pub amount: i32,
            }
            impl ::core::fmt::Debug for AttackDescription {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("AttackDescription")
                        .field("initiator", &self.initiator)
                        .field("initiator-location", &self.initiator_location)
                        .field("amount", &self.amount)
                        .finish()
                }
            }
            pub type Resources = _rt::Vec<(_rt::String, u32)>;
            #[derive(Clone)]
            pub struct ConvertParams {
                pub input: Resources,
                pub output_items: _rt::Vec<_rt::String>,
                pub output_resources: _rt::Vec<(_rt::String, u32)>,
            }
            impl ::core::fmt::Debug for ConvertParams {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("ConvertParams")
                        .field("input", &self.input)
                        .field("output-items", &self.output_items)
                        .field("output-resources", &self.output_resources)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub enum MicroAction {
                Walk,
                Haul(HaulParams),
                Attack(AttackParams),
                ApplyBuff(ApplyBuffParams),
                Convert(ConvertParams),
                Equip,
                Unequip,
                PickUp,
                AbandonLevel,
            }
            impl ::core::fmt::Debug for MicroAction {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        MicroAction::Walk => f.debug_tuple("MicroAction::Walk").finish(),
                        MicroAction::Haul(e) => {
                            f.debug_tuple("MicroAction::Haul").field(e).finish()
                        }
                        MicroAction::Attack(e) => {
                            f.debug_tuple("MicroAction::Attack").field(e).finish()
                        }
                        MicroAction::ApplyBuff(e) => {
                            f.debug_tuple("MicroAction::ApplyBuff").field(e).finish()
                        }
                        MicroAction::Convert(e) => {
                            f.debug_tuple("MicroAction::Convert").field(e).finish()
                        }
                        MicroAction::Equip => {
                            f.debug_tuple("MicroAction::Equip").finish()
                        }
                        MicroAction::Unequip => {
                            f.debug_tuple("MicroAction::Unequip").finish()
                        }
                        MicroAction::PickUp => {
                            f.debug_tuple("MicroAction::PickUp").finish()
                        }
                        MicroAction::AbandonLevel => {
                            f.debug_tuple("MicroAction::AbandonLevel").finish()
                        }
                    }
                }
            }
            #[derive(Clone)]
            pub struct Action {
                pub name: _rt::String,
                pub micro_actions: _rt::Vec<MicroAction>,
            }
            impl ::core::fmt::Debug for Action {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("Action")
                        .field("name", &self.name)
                        .field("micro-actions", &self.micro_actions)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct InventoryItem {
                pub name: _rt::String,
                pub id: i64,
                pub level: u32,
                pub actions: _rt::Vec<Action>,
                pub resources: Option<Resources>,
            }
            impl ::core::fmt::Debug for InventoryItem {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("InventoryItem")
                        .field("name", &self.name)
                        .field("id", &self.id)
                        .field("level", &self.level)
                        .field("actions", &self.actions)
                        .field("resources", &self.resources)
                        .finish()
                }
            }
            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct Stats {
                pub strength: u32,
                pub hp: u32,
                pub speed: u32,
                pub inventory_size: u32,
            }
            impl ::core::fmt::Debug for Stats {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("Stats")
                        .field("strength", &self.strength)
                        .field("hp", &self.hp)
                        .field("speed", &self.speed)
                        .field("inventory-size", &self.inventory_size)
                        .finish()
                }
            }
            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct CharacterStats {
                pub max: Stats,
                pub current: Stats,
            }
            impl ::core::fmt::Debug for CharacterStats {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("CharacterStats")
                        .field("max", &self.max)
                        .field("current", &self.current)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub enum Event {
                Moved((i32, i32)),
                Hauled((i32, i32)),
                Attacked(AttackDescription),
                AddInventoryItem(i64),
                RemoveInventoryItem(i64),
                EquipItem((EquipmentSlot, i64)),
                UnequipItem(EquipmentSlot),
                GainResource(Resources),
                ChangedLevel,
            }
            impl ::core::fmt::Debug for Event {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        Event::Moved(e) => {
                            f.debug_tuple("Event::Moved").field(e).finish()
                        }
                        Event::Hauled(e) => {
                            f.debug_tuple("Event::Hauled").field(e).finish()
                        }
                        Event::Attacked(e) => {
                            f.debug_tuple("Event::Attacked").field(e).finish()
                        }
                        Event::AddInventoryItem(e) => {
                            f.debug_tuple("Event::AddInventoryItem").field(e).finish()
                        }
                        Event::RemoveInventoryItem(e) => {
                            f.debug_tuple("Event::RemoveInventoryItem").field(e).finish()
                        }
                        Event::EquipItem(e) => {
                            f.debug_tuple("Event::EquipItem").field(e).finish()
                        }
                        Event::UnequipItem(e) => {
                            f.debug_tuple("Event::UnequipItem").field(e).finish()
                        }
                        Event::GainResource(e) => {
                            f.debug_tuple("Event::GainResource").field(e).finish()
                        }
                        Event::ChangedLevel => {
                            f.debug_tuple("Event::ChangedLevel").finish()
                        }
                    }
                }
            }
        }
    }
}
mod _rt {
    pub use alloc_crate::string::String;
    pub use alloc_crate::vec::Vec;
    pub fn as_i32<T: AsI32>(t: T) -> i32 {
        t.as_i32()
    }
    pub trait AsI32 {
        fn as_i32(self) -> i32;
    }
    impl<'a, T: Copy + AsI32> AsI32 for &'a T {
        fn as_i32(self) -> i32 {
            (*self).as_i32()
        }
    }
    impl AsI32 for i32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for char {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for usize {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    pub unsafe fn bool_lift(val: u8) -> bool {
        if cfg!(debug_assertions) {
            match val {
                0 => false,
                1 => true,
                _ => panic!("invalid bool discriminant"),
            }
        } else {
            val != 0
        }
    }
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub unsafe fn invalid_enum_discriminant<T>() -> T {
        if cfg!(debug_assertions) {
            panic!("invalid enum discriminant")
        } else {
            core::hint::unreachable_unchecked()
        }
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub fn as_i64<T: AsI64>(t: T) -> i64 {
        t.as_i64()
    }
    pub trait AsI64 {
        fn as_i64(self) -> i64;
    }
    impl<'a, T: Copy + AsI64> AsI64 for &'a T {
        fn as_i64(self) -> i64 {
            (*self).as_i64()
        }
    }
    impl AsI64 for i64 {
        #[inline]
        fn as_i64(self) -> i64 {
            self as i64
        }
    }
    impl AsI64 for u64 {
        #[inline]
        fn as_i64(self) -> i64 {
            self as i64
        }
    }
    extern crate alloc as alloc_crate;
    pub use alloc_crate::alloc;
}
/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
#[macro_export]
macro_rules! __export_auto_rogue_ai_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*:: __export_world_auto_rogue_ai_cabi!($ty with_types_in
        $($path_to_types_root)*);
    };
}
#[doc(inline)]
#[allow(unused_imports)]pub use __export_auto_rogue_ai_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.30.0:auto-rogue-ai:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 2578] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\x8e\x13\x01A\x02\x01\
AU\x01BA\x01r\x02\x01xz\x01yz\x04\0\x03loc\x03\0\0\x01r\x03\x08passable\x7f\x06o\
paque\x7f\x04names\x04\0\x04tile\x03\0\x02\x01p}\x01k\x04\x01r\x04\x04names\x02i\
dx\x07factiony\x09broadcast\x05\x04\0\x08creature\x03\0\x06\x01ks\x01r\x05\x02id\
x\x04names\x0cis-furniture\x7f\x0bis-passable\x7f\x08metadata\x08\x04\0\x04item\x03\
\0\x09\x01q\x02\x09left-hand\0\0\x0aright-hand\0\0\x04\0\x0eequipment-slot\x03\0\
\x0b\x01q\x03\x09transient\0\0\x11decrease-per-turn\x01y\0\x09permanent\0\0\x04\0\
\x0fbuff-durability\x03\0\x0d\x01q\x08\x05north\0\0\x0anorth-west\0\0\x0anorth-e\
ast\0\0\x05south\0\0\x0asouth-west\0\0\x0asouth-east\0\0\x04east\0\0\x04west\0\0\
\x04\0\x09direction\x03\0\x0f\x01r\x02\x06amountz\x05rangey\x04\0\x0dattack-para\
ms\x03\0\x11\x01r\x04\x05rangey\x04names\x06amountz\x0adurability\x0e\x04\0\x11a\
pply-buff-params\x03\0\x13\x01r\x01\x08strengthy\x04\0\x0bhaul-params\x03\0\x15\x01\
o\x02\x0cx\x01px\x01q\x07\x08creature\x01x\0\x05actor\0\0\x0eequipment-slot\x01\x0c\
\0\x17equipment-slot-and-item\x01\x17\0\x09direction\x01\x10\0\x05items\x01\x18\0\
\x08location\x01\x01\0\x04\0\x0daction-target\x03\0\x19\x01k\x1a\x01o\x02y\x1b\x01\
q\x02\x0ause-action\x01\x1c\0\x07nothing\0\0\x04\0\x07command\x03\0\x1d\x01kx\x01\
r\x02\x09left-hand\x1f\x0aright-hand\x1f\x04\0\x0fequipment-state\x03\0\x20\x01r\
\x03\x04names\x06amounty\x0adurability\x0e\x04\0\x04buff\x03\0\"\x01r\x03\x04tur\
nx\x08level-idx\x0flevel-is-stable\x7f\x04\0\x0agame-state\x03\0$\x01q\x05\x02up\
\0\0\x04down\0\0\x04left\0\0\x05right\0\0\x05space\0\0\x04\0\x03key\x03\0&\x01r\x03\
\x09initiator\x07\x12initiator-location\x01\x06amountz\x04\0\x12attack-descripti\
on\x03\0(\x01o\x02sy\x01p*\x04\0\x09resources\x03\0+\x01ps\x01p*\x01r\x03\x05inp\
ut,\x0coutput-items-\x10output-resources.\x04\0\x0econvert-params\x03\0/\x01q\x09\
\x04walk\0\0\x04haul\x01\x16\0\x06attack\x01\x12\0\x0aapply-buff\x01\x14\0\x07co\
nvert\x010\0\x05equip\0\0\x07unequip\0\0\x07pick-up\0\0\x0dabandon-level\0\0\x04\
\0\x0cmicro-action\x03\01\x01p2\x01r\x02\x04names\x0dmicro-actions3\x04\0\x06act\
ion\x03\04\x01p5\x01k,\x01r\x05\x04names\x02idx\x05levely\x07actions6\x09resourc\
es7\x04\0\x0einventory-item\x03\08\x01r\x04\x08strengthy\x02hpy\x05speedy\x0einv\
entory-sizey\x04\0\x05stats\x03\0:\x01r\x02\x03max;\x07current;\x04\0\x0fcharact\
er-stats\x03\0<\x01o\x02zz\x01q\x09\x05moved\x01>\0\x06hauled\x01>\0\x08attacked\
\x01)\0\x12add-inventory-item\x01x\0\x15remove-inventory-item\x01x\0\x0aequip-it\
em\x01\x17\0\x0cunequip-item\x01\x0c\0\x0dgain-resource\x01,\0\x0dchanged-level\0\
\0\x04\0\x05event\x03\0?\x03\x01\x15game:auto-rogue/types\x05\0\x02\x03\0\0\x03l\
oc\x03\0\x03loc\x03\0\x01\x02\x03\0\0\x04tile\x03\0\x04tile\x03\0\x03\x02\x03\0\0\
\x08creature\x03\0\x08creature\x03\0\x05\x02\x03\0\0\x04item\x03\0\x04item\x03\0\
\x07\x02\x03\0\0\x0einventory-item\x03\0\x0einventory-item\x03\0\x09\x02\x03\0\0\
\x0fequipment-state\x03\0\x0fequipment-state\x03\0\x0b\x02\x03\0\0\x0fcharacter-\
stats\x03\0\x0fcharacter-stats\x03\0\x0d\x02\x03\0\0\x04buff\x03\0\x04buff\x03\0\
\x0f\x02\x03\0\0\x0agame-state\x03\0\x0agame-state\x03\0\x11\x02\x03\0\0\x06acti\
on\x03\0\x06action\x03\0\x13\x02\x03\0\0\x05event\x03\0\x05event\x03\0\x15\x02\x03\
\0\0\x07command\x03\0\x07command\x03\0\x17\x01k\x04\x01@\x01\x01l\x02\0\x19\x03\0\
\x07tile-at\x01\x1a\x01o\x02\x02\x04\x01p\x1b\x01@\0\0\x1c\x03\0\x0dvisible-tile\
s\x01\x1d\x01k\x06\x01@\x01\x01l\x02\0\x1e\x03\0\x0bcreature-at\x01\x1f\x01o\x02\
\x02\x06\x01@\0\0\x20\x03\0\x05actor\x01!\x01p\x20\x01@\0\0\"\x03\0\x11visible-c\
reatures\x01#\x01k\x08\x01@\x01\x01l\x02\0$\x03\0\x07item-at\x01%\x01o\x02\x02\x08\
\x01p&\x01@\0\0'\x03\0\x0dvisible-items\x01(\x01p\x0a\x01@\0\0)\x03\0\x09invento\
ry\x01*\x01@\0\0\x0c\x03\0\x13get-equipment-state\x01+\x01@\0\0\x0e\x03\0\x13get\
-character-stats\x01,\x01p\x10\x01@\0\0-\x03\0\x0fcharacter-buffs\x01.\x01@\0\0\x12\
\x03\0\x0eget-game-state\x01/\x01p\x14\x01@\0\00\x03\0\x07actions\x011\x01p}\x01\
@\0\02\x03\0\x0aload-store\x013\x01@\x01\x05store2\x01\0\x03\0\x0asave-store\x01\
4\x01k2\x01@\x01\x04data5\x01\0\x03\0\x09broadcast\x016\x01p\x02\x01@\x01\x05til\
es7\x01\0\x03\0\x0fhighlight-tiles\x018\x01p\x16\x01@\0\09\x03\0\x06events\x01:\x01\
@\0\05\x03\0\x0bconfig-data\x01;\x01@\x01\x04data2\x01\0\x03\0\x0ceditor-debug\x01\
<\x04\0\x0deditor-config\x01;\x01@\0\0\x18\x04\0\x04step\x01=\x04\x01\x1dgame:au\
to-rogue/auto-rogue-ai\x04\0\x0b\x13\x01\0\x0dauto-rogue-ai\x03\0\0\0G\x09produc\
ers\x01\x0cprocessed-by\x02\x0dwit-component\x070.215.0\x10wit-bindgen-rust\x060\
.30.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
