#![doc(html_logo_url = "https://github.com/rtk-rs/.github/blob/c458a3e2ce136793d281bc2e0f3161e69a663a2e/logos/logo2.jpg")]
#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[macro_use]
mod macros;

// pub modules
pub mod constellation;
pub mod sv;

#[cfg(feature = "cospar")]
pub mod cospar;

#[cfg(feature = "domes")]
pub mod domes;

// prelude (pkg)
pub mod prelude {
    pub use crate::constellation::Constellation;
    #[cfg(feature = "cospar")]
    pub use crate::cospar::COSPAR;
    #[cfg(feature = "domes")]
    pub use crate::domes::{TrackingPoint as DOMESTrackingPoint, DOMES};
    pub use crate::sv::SV;
}

// private modules
mod sbas;

#[cfg(feature = "sbas")]
pub use sbas::sbas_selection;
