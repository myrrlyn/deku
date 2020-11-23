/*! Crate prelude

[What is a prelude?](std::prelude)
*/
pub use crate::{
    deku_derive, error::DekuError, DekuContainerRead, DekuContainerWrite, DekuRead, DekuUpdate,
    DekuWrite,
};
pub use bitvec::{
    order::BitOrder, order::Lsb0, order::Msb0, slice::BitSlice, vec::BitVec, view::BitView,
};
