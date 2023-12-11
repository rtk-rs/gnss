#![doc = include_str!("../README.md")]
#![cfg_attr(docrs, feature(doc_cfg))]

#[macro_use]
mod macros;

// pub modules
pub mod constellation;
pub mod signal;
pub mod sv;

// prelude (pkg)
pub mod prelude {
    pub use crate::constellation::Constellation;
    pub use crate::signal::{Carrier, GlonassFDMAChannel};
    pub use crate::sv::SV;
}

// private modules
mod sbas;

#[cfg(feature = "sbas")]
pub use sbas::sbas_selection;
