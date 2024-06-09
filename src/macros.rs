/// Creates a [crate::prelude::SV] from given (case insensitive) string description.
/// Example:
/// ```
/// use std::str::FromStr;
/// use gnss_rs::prelude::*;
/// use gnss_rs::sv; // macro
/// assert_eq!(sv!("G08").constellation, Constellation::GPS);
/// assert_eq!(sv!("G08").prn, 8);
/// assert_eq!(sv!("e05").constellation, Constellation::Galileo);
/// assert_eq!(sv!("e05").prn, 5);
/// ```
#[macro_export]
macro_rules! sv {
    ($desc: expr) => {
        SV::from_str($desc).unwrap()
    };
}

/// Creates a [crate::prelude::Constellation] from given (case insensitive) string description.
/// Example:
/// ```
/// use std::str::FromStr;
/// use gnss_rs::prelude::*;
/// use gnss_rs::gnss; // macro
/// assert_eq!(gnss!("gps"), Constellation::GPS);
/// assert_eq!(gnss!("Gal"), Constellation::Galileo);
/// assert_eq!(gnss!("GALILEO"), Constellation::Galileo);
/// ```
#[macro_export]
macro_rules! gnss {
    ($desc: expr) => {
        Constellation::from_str($desc).unwrap()
    };
}
