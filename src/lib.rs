#![doc = include_str!("../README.md")]
#![cfg_attr(docrs, feature(doc_cfg))]

#[macro_use]
mod macros;

pub mod constellation;
mod snr;
pub mod sv;

pub mod prelude {
    pub use crate::constellation::Constellation;
    pub use crate::snr::SNR;
    pub use crate::sv::SV;
}

mod sbas;

#[cfg(feature = "sbas")]
pub use sbas::sbas_selection;
