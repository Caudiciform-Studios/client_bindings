sed -i '1s/^/#![allow(warnings)]/' src/bindings.rs
sed -i "s/macro_rules! __export_auto_rogue_ai_impl {/#[macro_export]\nmacro_rules! __export_auto_rogue_ai_impl {/" src/bindings.rs
sed -i "s/pub(crate) use __export_auto_rogue_ai_impl as export;/#[allow(unused_imports)]pub use __export_auto_rogue_ai_impl as export;/" src/bindings.rs
sed -i "s/macro_rules! __export_world_auto_rogue_ai_cabi{/#[macro_export]\nmacro_rules! __export_world_auto_rogue_ai_cabi {/" src/bindings.rs
sed -i "s/pub(crate) use __export_world_auto_rogue_ai_cabi;/#[allow(unused_imports)]pub use __export_world_auto_rogue_ai_cabi;/" src/bindings.rs
