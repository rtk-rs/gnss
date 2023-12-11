//! GNSS signals
use thiserror::Error;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Error, Clone, Debug)]
pub enum Error {
    #[error("unknown carrier signal")]
    UnknownCarrierSignal,
    #[error("invalid carrier signal")]
    InvalidCarrierSignal,
    #[error("invalid glonass fdma channel [-7, +6]")]
    InvalidFDMAChannel,
}

/// GNSS carriers shared attributes
pub trait GNSSCarrier {
    fn frequency(&self) -> f64;
    fn wavelength(&self) -> f64;
    fn frequency_mhz(&self) -> f64;
}

// GNSS modulations shared attributes
// pub trait GNSSModulation {
//     fn bandwidth_mhz(&self) -> f64;
// }

/// GNSS carriers that transmit GNSS signals.
#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Carrier {
    /// L1: 1575.42MHz (GPS_L1, SBAS_L1, QZSS_L1, GAL_E1, BDS_B1)
    #[default]
    L1,
    /// L5: 1176.45 (GPS_L5, QZSS_L5, GAL_E5, BDS_B5, LNAV_I5)
    L5,
    /// G1(GLO_L1): 1602.0 MHz
    G1,
    /// G2(GLO_L2): 1246.0 MHz
    G2,
    /// G3(GLO_L3): 1201.0 MHz
    G3,
    /// B2(BDS): 1561.098 MHz
    B2,
    /// B6(BDS): 1268.52 MHz
    B6,
    /// L6: 1278.75 MHz (QZSS_L6, GAL_E6)
    L6,
    /// B8E8: 1191.795 MHz (BDS_B8, GAL_E8)
    B8E8,
    /// S: 2492.028 MHz (IRNSS/L-NAV)
    S,
}

impl std::fmt::Display for Carrier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::L1 => write!(f, "L1"),
            Self::L5 => write!(f, "L5"),
            Self::L6 => write!(f, "L6"),
            Self::G1 => write!(f, "G1"),
            Self::G2 => write!(f, "G2"),
            Self::G3 => write!(f, "G3"),
            Self::B2 => write!(f, "B2"),
            Self::B6 => write!(f, "B6"),
            Self::B8E8 => write!(f, "B8"),
            Self::S => write!(f, "S"),
        }
    }
}

impl std::str::FromStr for Carrier {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let content = s.to_uppercase();
        let content = content.trim();
        if content.contains("L1") || content.contains("E1") || content.contains("B1") {
            Ok(Self::L1)
        } else if content.contains("L5")
            || content.contains("E5")
            || content.contains("B5")
            || content.contains("I5")
        {
            Ok(Self::L5)
        } else if content.contains("B8") || content.contains("E8") {
            Ok(Self::B8E8)
        } else if content.contains("L6") {
            Ok(Self::L6)
        } else if content.contains("B6") {
            Ok(Self::B6)
        } else if content.contains("B2") {
            Ok(Self::B2)
        } else if content.contains("G1") {
            Ok(Self::G1)
        } else if content.contains("G2") {
            Ok(Self::G2)
        } else if content.contains("G3") {
            Ok(Self::G1)
        } else if content.contains("S") {
            Ok(Self::S)
        } else {
            Err(Error::UnknownCarrierSignal)
        }
    }
}

impl GNSSCarrier for Carrier {
    /// Returns frequency of Self [MHz]
    fn frequency_mhz(&self) -> f64 {
        match self {
            Self::L1 => 1575.42,
            Self::L5 => 1176.45,
            Self::L6 => 1278.75,
            Self::S => 2492.028,
            Self::B6 => 1268.52,
            Self::G1 => 1602.0,
            Self::G2 => 1246.0,
            Self::G3 => 1201.0,
            Self::B2 => 1561.098,
            Self::B8E8 => 1191.795,
        }
    }
    /// Returns frequency of Self [Hz]
    fn frequency(&self) -> f64 {
        self.frequency_mhz() * 1.0E6
    }
    /// Returns carrier wavelength
    fn wavelength(&self) -> f64 {
        299_792_458.0 / self.frequency()
    }
    // /// Returns signal bandwidth in MHz
    // pub fn bandwidth_mhz(&self) -> f64 {
    //     match self {
    //         Self::L1 | Self::G1 | Self::E1 => 15.345_f64,
    //         Self::L5 => 12.5_f64,
    //         Self::L2 | Self::G2(_) | Self::G2a => 11.0_f64,
    //         Self::L6 | Self::B8E8 | Self::G3 => {
    //             todo!("{} bandwith is not known to this day", *self)
    //         },
    //     }
    // }
}

impl Carrier {
    /// Creates a Glonass Channel associated to Self
    pub fn to_glonass_channel(&self, channel: i8) -> Result<GlonassFDMAChannel, Error> {
        match self {
            Self::G1 | Self::G2 | Self::G3 => GlonassFDMAChannel::new(*self, channel),
            _ => Err(Error::InvalidFDMAChannel),
        }
    }
}

/// Glonass FDMA channel description
pub struct GlonassFDMAChannel {
    /// Carrier signal associated to this channel
    pub carrier: Carrier,
    /// Channel ID, 0 is the bandwidth center [-7; 6]
    pub channel: i8,
}

impl GNSSCarrier for GlonassFDMAChannel {
    fn frequency(&self) -> f64 {
        self.frequency_mhz() * 1.0E6
    }
    fn frequency_mhz(&self) -> f64 {
        let f = self.carrier.frequency_mhz();
        let df = match self.carrier {
            Carrier::G1 => self.channel as f64 * 9.0 / 16.0,
            Carrier::G2 => self.channel as f64 * 7.0 / 16.0,
            Carrier::G3 => self.channel as f64 * 7.0 / 16.0,
            _ => 0.0_f64,
        };
        f + df
    }
    fn wavelength(&self) -> f64 {
        299_792_458.0 / self.frequency()
    }
}

impl GlonassFDMAChannel {
    pub fn new(carrier: Carrier, channel: i8) -> Result<Self, Error> {
        if channel > -8 && channel < 7 {
            Ok(Self { carrier, channel })
        } else {
            Err(Error::InvalidFDMAChannel)
        }
    }
    /// Returns true if Self is dedicated to testing.
    pub fn is_testing_channel(&self) -> bool {
        match self.channel {
            -6 | -5 => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;
    #[test]
    fn test_carrier() {
        for desc in [
            "L1", "E1", "B1", "L5", "E5", "B5", "I5", "G1", "G3", "B2", "B6", "L6", "B8", "E8", "S",
        ] {
            assert!(
                Carrier::from_str(desc).is_ok(),
                "failed to parse \"{}\"",
                desc
            );
        }

        for (desc, freq_mhz) in [("L1", 1575.42), ("L5", 1176.45)] {
            let carrier = Carrier::from_str(desc).unwrap();
            assert_eq!(
                carrier.frequency_mhz(),
                freq_mhz,
                "bad frequency for {}",
                carrier
            );
        }

        for (desc, formatted) in [("L1", "L1"), ("E1", "L1")] {
            let carrier = Carrier::from_str(desc).unwrap();
            assert_eq!(carrier.to_string(), formatted);
        }
    }
    #[test]
    fn test_glonass() {
        for desc in ["G1", "G2", "G3"] {
            let carrier = Carrier::from_str(desc);
            assert!(
                carrier.is_ok(),
                "failed to parse glonass carrier \"{}\"",
                desc
            );

            let carrier = carrier.unwrap();
            let channel = carrier.to_glonass_channel(0);
            assert!(
                channel.is_ok(),
                "failed to convert {} to FDMA channel",
                desc
            );

            let channel = channel.unwrap();
            assert_eq!(channel.frequency(), carrier.frequency());
        }
    }
}
