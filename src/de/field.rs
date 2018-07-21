// Copyright (C) 2018 Martin Mroz
//
// This software may be modified and distributed under the terms
// of the MIT license.  See the LICENSE file for details.

use std::fmt;

/// Format of the field when parsing in strict mode.
#[derive(Copy,Clone,Eq,PartialEq,Ord,PartialOrd,Debug,Hash)]
pub enum DataFormat {
  /// Any printable ASCII character plus tab, line-feed and carriage return.
  Arbitrary,
  /// IATA Resolution 729 Appendix A format specifier 'f'.
  /// Any valid ASCII character including spaces.
  IataAlphaNumerical,
  /// IATA Resolution 729 Appendix A format specifier 'N'.
  /// All spaces or in the ASCII range `'0' ... '9'`.
  IataNumerical,
  /// IATA Resolution 729 Appendix A format specifier 'a'.
  /// In the ASCII range `'A' ... 'Z'`.
  IataAlphabetical,
  /// A flight number with format specifier 'NNNN[a]'.
  FlightNumber,
}

#[derive(Copy,Clone,Eq,PartialEq,Ord,PartialOrd,Debug,Hash)]
pub enum Field {
  /// Item 1: Format Code. 1 byte. Data Type 'f'.
  FormatCode,
  /// Item 4: Airline Individual Use. n bytes. Data Type unspecified.
  AirlineIndividualUse,
  /// Item 5: Number of Legs Encoded. 1 byte. Data Type 'N'.
  NumberOfLegsEncoded,
  /// Item 6: Field Size of Variable Size Field. 2 byte. Data Type 'f'. Hexadecimal.
  FieldSizeOfVariableSizeField,
  /// Item 7: Operating Carrier PNR Code. 7 bytes. Data Type 'f'.
  OperatingCarrierPnrCode,
  /// Item 8: Beginning of Version Number. 1 byte. Data Type 'f'.
  BeginningOfVersionNumber,
  /// Item 9: Version Number. 1 byte. Data Type 'f'.
  VersionNumber,
  /// Item 10: Field Size of Structured Message. 2 byte. Data Type 'f'. Hexadecimal.
  FieldSizeOfStructuredMessageUnique,
  /// Item 11: Passenger Name. 20 bytes. Data Type 'f'.
  PassengerName,
  /// Item 12: Source of Check-In. 1 byte. Data Type 'f'.
  SourceOfCheckIn,
  /// Item 14: Source of Boarding Pass Issuance. 1 byte. Data Type 'f'.
  SourceOfBoardingPassIssuance,
  /// Item 15: Passenger Description. 1 byte. Data Type 'f'.
  PassengerDescription,
  /// Item 16: Document Type. 1 byte. Data Type 'f'.
  DocumentType,
  /// Item 17: Field Size of Structured Message. 2 byte. Data Type 'f'. Hexadecimal.
  FieldSizeOfStructuredMessageRepeated,
  /// Item 18: Selectee Indicator. 1 byte. Data Type 'f'.
  SelecteeIndicator,
  /// Item 19: Marketing Carrier Designator. 3 bytes. Data Type 'f'.
  MarketingCarrierDesignator,
  /// Item 20: Frequent Flyer Airline Designator. 3 bytes. Data Type 'f'.
  FrequentFlyerAirlineDesignator,
  /// Item 21: Airline Designator of Boarding Pass Issuer. 3 bytes. Data Type 'f'.
  AirlineDesignatorOfBoardingPassIssuer,
  /// Item 22: Date of Issue of Boarding Pass. 4 bytes. Data Type 'N'.
  DateOfIssueOfBoardingPass,
  /// Item 23: Baggage Tag License Plate Number(s). 13 bytes. Data Type 'f'.
  BaggageTagLicensePlateNumbers,
  /// Item 25: Beginning of Security Data. 1 byte. Data Type 'f'.
  BeginningOfSecurityData,
  /// Item 26: From City Airport Code. 3 bytes. Data Type 'a'.
  FromCityAirportCode,
  /// Item 28: Type of Security Data. 1 byte. Data Type 'f'.
  TypeOfSecurityData,
  /// Item 29: Length of Security Data. 2 bytes. Data Type 'f'. Hexadecimal.
  LengthOfSecurityData,
  /// Item 30: Security Data. n bytes. Data Type 'f'.
  SecurityData,
  /// Item 31: First Non-Consecutive Baggage Tag License Plate Number. 13 bytes. Data Type 'f'.
  FirstNonConsecutiveBaggageTagLicensePlateNumber,
  /// Item 32: Second Non-Consecutive Baggage Tag License Plate Number. 13 bytes. Data Type 'f'.
  SecondNonConsecutiveBaggageTagLicensePlateNumber,
  /// Item 38: To City Airport Code. 3 bytes. Data Type 'a'.
  ToCityAirportCode,
  /// Item 42: Operating Carrier Designator. 3 bytes. Data Type 'f'.
  OperatingCarrierDesignator,
  /// Item 43: Flight Number. 5 bytes. Data Type 'NNNN[a]'.
  FlightNumber,
  /// Item 46: Date of Flight. 3 bytes. Data Type 'N'.
  DateOfFlight,
  /// Item 71: Compartment Code. 1 byte. Data Type 'a'.
  CompartmentCode,
  /// Item 89: Electronic Ticket Indicator. 1 byte. Data Type 'f'.
  IdAdIndicator,
  /// Item 104: Seat Number. 4 bytes. Data Type is usually 'NNNa', but can be 'INF ' or similar.
  SeatNumber,
  /// Item 107: Check-In Sequence Number. 5 bytes. Data Type is usually 'NNNN[f]', but can be 'f'.
  CheckInSequenceNumber,
  /// Item 108: International Document Verification. 1 byte. Data Type 'f'.
  InternationalDocumentVerification,
  /// Item 117: Passenger Status. 1 byte. Data Type 'f'.
  PassengerStatus,
  /// Item 118: Free Baggage Allowance. 3 bytes. Data Type 'f'.
  FreeBaggageAllowance,
  /// Item 142: Airline Numeric Code. 3 bytes. Data Type 'N'.
  AirlineNumericCode,
  /// Item 143: Document Form / Serial Number. 10 bytes. Data Type 'f'.
  DocumentFormSerialNumber,
  /// Item 236: Frequent Flyer Number. 16 bytes. Data Type 'f'.
  FrequentFlyerNumber,
  /// Item 253: Electronic Ticket Indicator. 1 byte. Data Type 'f'.
  ElectronicTicketIndicator,
  /// Item 254: Fast Track. 1 byte. Data Type 'f'.
  FastTrack,
}

impl Field {

  /// Item number as defined in the Implementation Guide.
  pub fn item_number(self) -> usize {
    match self {
      Field::FormatCode => 1,
      Field::AirlineIndividualUse => 4,
      Field::NumberOfLegsEncoded => 5,
      Field::FieldSizeOfVariableSizeField => 6,
      Field::OperatingCarrierPnrCode => 7,
      Field::BeginningOfVersionNumber => 8,
      Field::VersionNumber => 9,
      Field::FieldSizeOfStructuredMessageUnique => 10,
      Field::PassengerName => 11,
      Field::SourceOfCheckIn => 12,
      Field::SourceOfBoardingPassIssuance => 14,
      Field::PassengerDescription => 15,
      Field::DocumentType => 16,
      Field::FieldSizeOfStructuredMessageRepeated => 17,
      Field::SelecteeIndicator => 18,
      Field::MarketingCarrierDesignator => 19,
      Field::FrequentFlyerAirlineDesignator => 20,
      Field::AirlineDesignatorOfBoardingPassIssuer => 21,
      Field::DateOfIssueOfBoardingPass => 22,
      Field::BaggageTagLicensePlateNumbers => 23,
      Field::BeginningOfSecurityData => 25,
      Field::FromCityAirportCode => 26,
      Field::TypeOfSecurityData => 28,
      Field::LengthOfSecurityData => 29,
      Field::SecurityData => 30,
      Field::FirstNonConsecutiveBaggageTagLicensePlateNumber => 31,
      Field::SecondNonConsecutiveBaggageTagLicensePlateNumber => 32,
      Field::ToCityAirportCode => 38,
      Field::OperatingCarrierDesignator => 42,
      Field::FlightNumber => 43,
      Field::DateOfFlight => 46,
      Field::CompartmentCode => 71,
      Field::IdAdIndicator => 89,
      Field::SeatNumber => 104,
      Field::CheckInSequenceNumber => 107,
      Field::InternationalDocumentVerification => 108,
      Field::PassengerStatus => 113,
      Field::FreeBaggageAllowance => 118,
      Field::AirlineNumericCode => 142,
      Field::DocumentFormSerialNumber => 143,
      Field::FrequentFlyerNumber => 236,
      Field::ElectronicTicketIndicator => 253,
      Field::FastTrack => 254,
    }
  }

  /// The required length of the field. If zero, the field may be arbitrarily long.
  pub fn len(self) -> usize {
    match self {
      Field::FormatCode => 1,
      Field::AirlineIndividualUse => 0,
      Field::NumberOfLegsEncoded => 1,
      Field::FieldSizeOfVariableSizeField => 2,
      Field::OperatingCarrierPnrCode => 7,
      Field::BeginningOfVersionNumber => 1,
      Field::VersionNumber => 1,
      Field::FieldSizeOfStructuredMessageUnique => 2,
      Field::PassengerName => 20,
      Field::SourceOfCheckIn => 1,
      Field::SourceOfBoardingPassIssuance => 1,
      Field::PassengerDescription => 1,
      Field::DocumentType => 1,
      Field::FieldSizeOfStructuredMessageRepeated => 2,
      Field::SelecteeIndicator => 1,
      Field::MarketingCarrierDesignator => 3,
      Field::FrequentFlyerAirlineDesignator => 3,
      Field::AirlineDesignatorOfBoardingPassIssuer => 3,
      Field::DateOfIssueOfBoardingPass => 4,
      Field::BaggageTagLicensePlateNumbers => 13,
      Field::BeginningOfSecurityData => 1,
      Field::FromCityAirportCode => 3,
      Field::TypeOfSecurityData => 1,
      Field::LengthOfSecurityData => 2,
      Field::SecurityData => 0,
      Field::FirstNonConsecutiveBaggageTagLicensePlateNumber => 13,
      Field::SecondNonConsecutiveBaggageTagLicensePlateNumber => 13,
      Field::ToCityAirportCode => 3,
      Field::OperatingCarrierDesignator => 3,
      Field::FlightNumber => 5,
      Field::DateOfFlight => 3,
      Field::CompartmentCode => 1,
      Field::IdAdIndicator => 1,
      Field::SeatNumber => 4,
      Field::CheckInSequenceNumber => 5,
      Field::InternationalDocumentVerification => 1,
      Field::PassengerStatus => 1,
      Field::FreeBaggageAllowance => 3,
      Field::AirlineNumericCode => 3,
      Field::DocumentFormSerialNumber => 10,
      Field::FrequentFlyerNumber => 16,
      Field::ElectronicTicketIndicator => 1,
      Field::FastTrack => 1,
    }
  }

  /// Name of the field as defined in the Implementation Guide.
  pub fn name(self) -> &'static str {
    match self {
      Field::FormatCode => 
        "Format Code",
      Field::AirlineIndividualUse => 
        "Airline Individual Use",
      Field::NumberOfLegsEncoded => 
        "Number of Legs Encoded",
      Field::FieldSizeOfVariableSizeField => 
        "Field Size of Variable Size Field",
      Field::OperatingCarrierPnrCode => 
        "Operating Carrier PNR Code",
      Field::BeginningOfVersionNumber => 
        "Beginning of Version Number",
      Field::VersionNumber => 
        "Version Number",
      Field::FieldSizeOfStructuredMessageUnique => 
        "Field Size of Strutured Message (Unique)",
      Field::PassengerName => 
        "Passenger Name",
      Field::SourceOfCheckIn => 
        "Source of Check-In",
      Field::SourceOfBoardingPassIssuance => 
        "Source of Boarding Pass Issuance",
      Field::PassengerDescription => 
        "Passenger Description",
      Field::DocumentType => 
        "Document Type",
      Field::FieldSizeOfStructuredMessageRepeated => 
        "Field Size of Strutured Message (Repeated)",
      Field::SelecteeIndicator => 
        "Selectee Indicator",
      Field::MarketingCarrierDesignator => 
        "Marketing Carrier Designator",
      Field::FrequentFlyerAirlineDesignator => 
        "Frequent Flyer Airline Designator",
      Field::AirlineDesignatorOfBoardingPassIssuer => 
        "Airline Designator of Boarding Pass Issuer",
      Field::DateOfIssueOfBoardingPass => 
        "Date of Issue of Boarding Pass",
      Field::BaggageTagLicensePlateNumbers => 
        "Baggage Tag License Plate Number(s)",
      Field::BeginningOfSecurityData => 
        "Beginning of Security Data",
      Field::FromCityAirportCode => 
        "From City Airport Code",
      Field::TypeOfSecurityData => 
        "Type of Security Data",
      Field::LengthOfSecurityData => 
        "Length of Security Data",
      Field::SecurityData => 
        "Security Data",
      Field::FirstNonConsecutiveBaggageTagLicensePlateNumber => 
        "First Non-Consecutive Baggage Tag License Plate Number",
      Field::SecondNonConsecutiveBaggageTagLicensePlateNumber => 
        "Second Non-Consecutive Baggage Tag License Plate Number",
      Field::ToCityAirportCode => 
        "To City Airport Code",
      Field::OperatingCarrierDesignator => 
        "Operating Carrier Designator",
      Field::FlightNumber => 
        "Flight Number",
      Field::DateOfFlight => 
        "Date of Flight",
      Field::CompartmentCode => 
        "Compartment Code",
      Field::IdAdIndicator => 
        "ID/AD Indicator",
      Field::SeatNumber => 
        "Seat Number",
      Field::CheckInSequenceNumber => 
        "Check-In Sequence Number",
      Field::InternationalDocumentVerification => 
        "International Document Verification",
      Field::PassengerStatus => 
        "Passenger Status",
      Field::FreeBaggageAllowance => 
        "Free Baggage Allowance",
      Field::AirlineNumericCode => 
        "Airline Numeric Code",
      Field::DocumentFormSerialNumber => 
        "Document Form / Serial Number",
      Field::FrequentFlyerNumber => 
        "Frequent Flyer Number",
      Field::ElectronicTicketIndicator => 
        "Electronic Ticket Indicator",
      Field::FastTrack => 
        "Fast Track",
    }
  }

  /// The data format to be used in validation.
  pub fn data_format(self) -> DataFormat {
    match self {
      Field::FormatCode =>
        DataFormat::IataAlphaNumerical,
      Field::AirlineIndividualUse =>
        DataFormat::Arbitrary,
      Field::NumberOfLegsEncoded =>
        DataFormat::IataNumerical,
      Field::FieldSizeOfVariableSizeField =>
        DataFormat::IataAlphaNumerical,
      Field::OperatingCarrierPnrCode =>
        DataFormat::IataAlphaNumerical,
      Field::BeginningOfVersionNumber =>
        DataFormat::IataAlphaNumerical,
      Field::VersionNumber =>
        DataFormat::IataAlphaNumerical,
      Field::FieldSizeOfStructuredMessageUnique =>
        DataFormat::IataAlphaNumerical,
      Field::PassengerName =>
        DataFormat::IataAlphaNumerical,
      Field::SourceOfCheckIn =>
        DataFormat::IataAlphaNumerical,
      Field::SourceOfBoardingPassIssuance =>
        DataFormat::IataAlphaNumerical,
      Field::PassengerDescription =>
        DataFormat::IataAlphaNumerical,
      Field::DocumentType =>
        DataFormat::IataAlphaNumerical,
      Field::FieldSizeOfStructuredMessageRepeated =>
        DataFormat::IataAlphaNumerical,
      Field::SelecteeIndicator =>
        DataFormat::IataAlphaNumerical,
      Field::MarketingCarrierDesignator =>
        DataFormat::IataAlphaNumerical,
      Field::FrequentFlyerAirlineDesignator =>
        DataFormat::IataAlphaNumerical,
      Field::AirlineDesignatorOfBoardingPassIssuer =>
        DataFormat::IataAlphaNumerical,
      Field::DateOfIssueOfBoardingPass =>
        DataFormat::IataNumerical,
      Field::BaggageTagLicensePlateNumbers =>
        DataFormat::IataAlphaNumerical,
      Field::BeginningOfSecurityData =>
        DataFormat::IataAlphaNumerical,
      Field::FromCityAirportCode =>
        DataFormat::IataAlphabetical,
      Field::TypeOfSecurityData =>
        DataFormat::IataAlphaNumerical,
      Field::LengthOfSecurityData =>
        DataFormat::IataAlphaNumerical,
      Field::SecurityData =>
        DataFormat::Arbitrary,
      Field::FirstNonConsecutiveBaggageTagLicensePlateNumber =>
        DataFormat::IataAlphaNumerical,
      Field::SecondNonConsecutiveBaggageTagLicensePlateNumber =>
        DataFormat::IataAlphaNumerical,
      Field::ToCityAirportCode =>
        DataFormat::IataAlphabetical,
      Field::OperatingCarrierDesignator =>
        DataFormat::IataAlphaNumerical,
      Field::FlightNumber =>
        DataFormat::FlightNumber,
      Field::DateOfFlight =>
        DataFormat::IataNumerical,
      Field::CompartmentCode =>
        DataFormat::IataAlphabetical,
      Field::IdAdIndicator =>
        DataFormat::IataAlphaNumerical,
      Field::SeatNumber =>
        DataFormat::IataAlphaNumerical,
      Field::CheckInSequenceNumber =>
        DataFormat::IataAlphaNumerical,
      Field::InternationalDocumentVerification =>
        DataFormat::IataAlphaNumerical,
      Field::PassengerStatus =>
        DataFormat::IataAlphaNumerical,
      Field::FreeBaggageAllowance =>
        DataFormat::IataAlphaNumerical,
      Field::AirlineNumericCode =>
        DataFormat::IataNumerical,
      Field::DocumentFormSerialNumber =>
        DataFormat::IataAlphaNumerical,
      Field::FrequentFlyerNumber =>
        DataFormat::IataAlphaNumerical,
      Field::ElectronicTicketIndicator =>
        DataFormat::IataAlphaNumerical,
      Field::FastTrack =>
        DataFormat::IataAlphaNumerical,
    }
  }

}

impl fmt::Display for Field {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({:03}) {}", self.item_number(), self.name())
  }
}
