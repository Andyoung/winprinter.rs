use super::NS_PSK;
use std::str::FromStr;
use strum::EnumString;
use xml::name::OwnedName;

#[derive(EnumString, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum PredefinedMediaName {
    BusinessCard,
    CreditCard,
    ISOA0,
    ISOA1,
    ISOA10,
    ISOA2,
    ISOA3,
    ISOA3Extra,
    ISOA3Rotated,
    ISOA4,
    ISOA4Extra,
    ISOA4Rotated,
    ISOA5,
    ISOA5Extra,
    ISOA5Rotated,
    ISOA6,
    ISOA6Rotated,
    ISOA7,
    ISOA8,
    ISOA9,
    ISOB0,
    ISOB1,
    ISOB10,
    ISOB2,
    ISOB3,
    ISOB4,
    ISOB4Envelope,
    ISOB5Envelope,
    ISOB5Extra,
    ISOB7,
    ISOB8,
    ISOB9,
    ISOC0,
    ISOC1,
    ISOC10,
    ISOC2,
    ISOC3,
    ISOC3Envelope,
    ISOC4,
    ISOC4Envelope,
    ISOC5,
    ISOC5Envelope,
    ISOC6,
    ISOC6C5Envelope,
    ISOC6Envelope,
    ISOC7,
    ISOC8,
    ISOC9,
    ISODLEnvelope,
    ISODLEnvelopeRotated,
    ISOSRA3,
    JISB0,
    JISB1,
    JISB10,
    JISB2,
    JISB3,
    JISB4,
    JISB4Rotated,
    JISB5,
    JISB5Rotated,
    JISB6,
    JISB6Rotated,
    JISB7,
    JISB8,
    JISB9,
    Japan2LPhoto,
    JapanChou3Envelope,
    JapanChou3EnvelopeRotated,
    JapanChou4Envelope,
    JapanChou4EnvelopeRotated,
    JapanDoubleHagakiPostcard,
    JapanDoubleHagakiPostcardRotated,
    JapanHagakiPostcard,
    JapanHagakiPostcardRotated,
    JapanKaku2Envelope,
    JapanKaku2EnvelopeRotated,
    JapanKaku3Envelope,
    JapanKaku3EnvelopeRotated,
    JapanLPhoto,
    JapanQuadrupleHagakiPostcard,
    JapanYou1Envelope,
    JapanYou2Envelope,
    JapanYou3Envelope,
    JapanYou4Envelope,
    JapanYou4EnvelopeRotated,
    JapanYou6Envelope,
    JapanYou6EnvelopeRotated,
    NorthAmerica10x11,
    NorthAmerica10x12,
    NorthAmerica10x14,
    NorthAmerica11x17,
    NorthAmerica14x17,
    NorthAmerica4x6,
    NorthAmerica4x8,
    NorthAmerica5x7,
    NorthAmerica8x10,
    NorthAmerica9x11,
    NorthAmericaArchitectureASheet,
    NorthAmericaArchitectureBSheet,
    NorthAmericaArchitectureCSheet,
    NorthAmericaArchitectureDSheet,
    NorthAmericaArchitectureESheet,
    NorthAmericaCSheet,
    NorthAmericaDSheet,
    NorthAmericaESheet,
    NorthAmericaExecutive,
    NorthAmericaGermanLegalFanfold,
    NorthAmericaGermanStandardFanfold,
    NorthAmericaLegal,
    NorthAmericaLegalExtra,
    NorthAmericaLetter,
    NorthAmericaLetterExtra,
    NorthAmericaLetterPlus,
    NorthAmericaLetterRotated,
    NorthAmericaMonarchEnvelope,
    NorthAmericaNote,
    NorthAmericaNumber10Envelope,
    NorthAmericaNumber10EnvelopeRotated,
    NorthAmericaNumber11Envelope,
    NorthAmericaNumber12Envelope,
    NorthAmericaNumber14Envelope,
    NorthAmericaNumber9Envelope,
    NorthAmericaPersonalEnvelope,
    NorthAmericaQuarto,
    NorthAmericaStatement,
    NorthAmericaSuperA,
    NorthAmericaSuperB,
    NorthAmericaTabloid,
    NorthAmericaTabloidExtra,
    OtherMetricA3Plus,
    OtherMetricA4Plus,
    OtherMetricFolio,
    OtherMetricInviteEnvelope,
    OtherMetricItalianEnvelope,
    PRC10Envelope,
    PRC10EnvelopeRotated,
    PRC16K,
    PRC16KRotated,
    PRC1Envelope,
    PRC1EnvelopeRotated,
    PRC2Envelope,
    PRC2EnvelopeRotated,
    PRC32K,
    PRC32KBig,
    PRC32KRotated,
    PRC3Envelope,
    PRC3EnvelopeRotated,
    PRC4Envelope,
    PRC4EnvelopeRotated,
    PRC5Envelope,
    PRC5EnvelopeRotated,
    PRC6Envelope,
    PRC6EnvelopeRotated,
    PRC7Envelope,
    PRC7EnvelopeRotated,
    PRC8Envelope,
    PRC8EnvelopeRotated,
    PRC9Envelope,
    PRC9EnvelopeRotated,
    Roll04Inch,
    Roll06Inch,
    Roll08Inch,
    Roll12Inch,
    Roll15Inch,
    Roll18Inch,
    Roll22Inch,
    Roll24Inch,
    Roll30Inch,
    Roll36Inch,
    Roll54Inch,
}

impl PredefinedMediaName {
    pub fn from_name(name: &OwnedName) -> Option<Self> {
        if name.namespace_ref() == Some(NS_PSK) {
            Self::from_str(name.local_name.as_str()).ok()
        } else {
            None
        }
    }
}
