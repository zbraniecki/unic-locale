use crate::parser::errors::ParserError;

use std::str::FromStr;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Ord, PartialOrd)]
pub enum UnicodeExtensionKey {
    Calendar,       // ca
    CurrencyFormat, // cf
    Collation,      // co
    Currency,       // cu
    Emoji,          // em
    FirstDay,       // fw
    HourCycle,      // hc
    LineBreak,      // lb
    LineBreakWork,  // lw
    Measurement,    // ms
    Numbering,      // nu
    Region,         // rg
    RegionSubdiv,   // sd
    SentenceBreak,  // ss
    TimeZone,       // tz
    CommonVariant,  // va
}

impl FromStr for UnicodeExtensionKey {
    type Err = ParserError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        match source {
            "ca" => Ok(UnicodeExtensionKey::Calendar),
            "co" => Ok(UnicodeExtensionKey::Collation),
            "hc" => Ok(UnicodeExtensionKey::HourCycle),
            _ => Err(ParserError::InvalidExtension),
        }
    }
}

impl std::fmt::Display for UnicodeExtensionKey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            UnicodeExtensionKey::Calendar => "ca",
            UnicodeExtensionKey::Collation => "co",
            UnicodeExtensionKey::HourCycle => "hc",
            _ => unimplemented!(),
        };
        f.write_str(s)
    }
}
