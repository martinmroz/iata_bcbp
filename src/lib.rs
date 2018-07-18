// Copyright (C) 2018 Martin Mroz
//
// This software may be modified and distributed under the terms
// of the MIT license.  See the LICENSE file for details.

//! # Issues
//! Check that all mandatory fields can be blanked out per section 4.2.1
//! Check that all optional fields can be blanked out per section 4.2.1
//! Check that a blank sequence number is valid (i.e. for infants).
//! Check that a seat number like 'INF' for infants is supported.
//! Look for Attachment A of Resolution 792 to validate formatting on each field.
//! Look for BSM specifications RP1745 for variable length field specifications.
//! Check if the Julian Date format is 0-based or 1-based.
//! Check if the BA boarding pass issue is related to the version number of the BCBP.
//! NOTE: Extra flexibility in Item 11: Name due to name translation issues.
//! NOTE: Item 253 distinguishes between pax on etickets and ptickets.
//! NOTE: Item 71 uses the compartment code and not the booking class.
//! NOTE: Item 12 source of checkin is defined in Attachment C of Resolution 792.
//! NOTE: The Julian date is formed of the last digit of the year the boarding pass 
//!       was issued and the number of elapsed days since the beginning of that particular year.
//!       If the number of elapsed days is less than 10, add two “0” after the year. 
//!       If the number of elapsed days is less than 100, add one “0” after the year.

use std::error;
use std::fmt;
use std::str::FromStr;

mod field;
use field::Field;
mod scanner;
use scanner::{CharacterSet, Scannable, Scanner };

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum BcbpParseError {
  /// No format specifier at the start of the BCBP data.
  MissingFormatSpecifier,
  /// Only type 'M' version 3 BCBP strings are supported.
  UnsupportedFormat(char),
  /// An error occurred parsing a specified field.
  ErrorParsingRequiredField(Field),
  /// An invalid version of the conditional items section was encountered.
  InvalidBeginningOfVersionNumber,
  /// An invalid beginning of security data section was encountered.
  InvalidBeginningOfSecurityData,
  /// A mismatch was found between the variable field size and contents.
  InvalidVariableFieldSize,
}

impl error::Error for BcbpParseError {
  /// Returns a string slice with a general description of a scanner error.
  /// No specific information is contained. To obtain a printable representation,
  /// use the `fmt::Display` attribute.
  fn description(&self) -> &str {
    "bcbp parse error"
  }
}

impl fmt::Display for BcbpParseError {
  /// Formats the receiver for display purposes into formatter `f`. Names are lower-case.
  /// Returns a result representing the formatted receiver or a failure to write into `f`.
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      BcbpParseError::MissingFormatSpecifier =>
        write!(f, "no format code was specified"),
      BcbpParseError::UnsupportedFormat(c) =>
        write!(f, "unsupported BCBP format code '{}'", c),
      BcbpParseError::ErrorParsingRequiredField(field) =>
        write!(f, "error parsing required field {}", field),
      BcbpParseError::InvalidBeginningOfVersionNumber =>
        write!(f, "invalid beginning of version number for conditional items"),
      BcbpParseError::InvalidBeginningOfSecurityData =>
        write!(f, "invalid beginning of security data"),
      BcbpParseError::InvalidVariableFieldSize =>
        write!(f, "variable field size specified does not match contents"),
    }
  }
}

#[derive(Clone,PartialEq,Eq,Hash,Debug,Ord,PartialOrd)]
pub struct JulianDate {
  /// The last digit of the year, if available.
  pub year: Option<u8>,
  /// The day of the year.
  pub day: u16,
}

#[derive(Clone,PartialEq,Eq,Hash,Debug)]
pub struct FlightLeg {
  /// Item 7: Operating Carrier PNR Code.
  pub operating_carrier_pnr: String,
  /// Item 26: Origin Airport IATA Code.
  pub from_airport: String,
  /// Item 38: Destination Airport IATA Code.
  pub to_airport: String,
  /// Item 42: Operating Carrier IATA Code.
  pub operating_carrier: String,
  /// Item 43: Operating Carrier Flight Number.
  pub flight_number: String,
  /// Item 46: Date of Flight.
  pub date_of_flight: JulianDate,
  /// Item 71: Compartment Code.
  pub compartment_code: char,
  /// Item 106: Seat Number.
  pub seat_number: String,
  /// Item 107: Check-in Sequence Number.
  pub check_in_sequence_number: String,
  /// Item 113: Passenger Status.
  pub passenger_status: char,
  /// Item 142: Airline Numeric Code.
  pub airline_numeric_code: Option<u16>,
  /// Item 143: Document Form / Serial Number.
  pub document_form_serial_number: Option<String>,
  /// Item 18: Selectee Indicator.
  pub selectee_indicator: Option<char>,
  /// Item 108: International Document Verification.
  pub international_document_verification: Option<char>,
  /// Item 19: Marketing Carrier IATA Code.
  pub marketing_carrier: Option<String>,
  /// Item 20: Frequent Flyer Airline IATA Code.
  pub frequent_flyer_carrier: Option<String>,
  /// Item 236: Frequent Flyer Number.
  pub frequent_flyer_number: Option<String>,
  /// Item 89: ID / AD Indicator.
  pub id_ad_indicator: Option<char>,
  /// Item 118: Frequent Flyer Number.
  pub free_baggage_allowance: Option<String>,
  /// Item 254: Fast Track.
  pub fast_track: Option<bool>,
}

#[derive(Clone,PartialEq,Eq,Hash,Debug)]
pub struct Bcbp {
  /// Item 11: The name of the passenger as `LAST_NAME/FIRST_NAME[TITLE]`.
  pub passenger_name: String,
  /// Item 253: If `true` the underlying backs an electronic ticket.
  pub is_eticket: bool,
  /// Leg-specific information.
  pub legs: Vec<FlightLeg>,
  /// Item 15: Passenger Description Code.
  pub passenger_description: Option<char>,
  /// Item 12: Source of Check-In.
  pub source_of_check_in: Option<char>,
  /// Item 14: Source of Boarding Pass Issuance.
  pub source_of_boarding_pass_issuance: Option<char>,
  /// Item 22: Date of Issue of Boarding Pass.
  pub boarding_pass_issued_on: Option<JulianDate>,
  /// Item 16: Document Type.
  pub document_type: Option<char>,
  /// Item 21: Airline Designator of Boarding Pass Issuer.
  pub boarding_pass_issuing_airline: Option<String>,
  /// Item 23: Baggage Tag License Plate Number(s).
  pub baggage_tag_license_plate_numbers: Option<String>,
  /// Item 31: First Non-Consecutive Baggage Tag License Plate Number.
  pub first_non_consecutive_baggage_tag_license_plate_number: Option<String>,
  /// Item 32: Second Non-Consecutive Baggage Tag License Plate Number.
  pub second_non_consecutive_baggage_tag_license_plate_number: Option<String>,
}

struct Parser<'a> {
  /// The scanner used to extract information from the input string.
  scanner: Scanner<'a>,
}

impl<'a> Parser<'a> {

  pub fn new(scanner: Scanner<'a>) -> Self {
    Parser {
      scanner: scanner,
    }
  }

  /// Item 1: Format Code. 1 byte. Data Type 'f'.
  fn expect_format_code(&mut self) -> Result<char, BcbpParseError> {
    self.scanner
      .scan_character_from_set(CharacterSet::IataAlphaNumerical)
      .map_err(|_| {
        BcbpParseError::MissingFormatSpecifier
      })
  }

  /// Item 4: Airline Individual Use. n bytes. Data Type unspecified.
  /// The 'f' data type will be used as it is the most permissive.
  fn expect_airline_individual_use(&mut self, length: usize) -> Result<String, BcbpParseError> {
    self.scanner
      .scan_characters_from_set(length, CharacterSet::IataAlphaNumerical)
      .map(String::from)
      .map_err(|_| {
        BcbpParseError::ErrorParsingRequiredField(Field::AirlineIndividualUse)
      })
  }

  /// Item 5: Number of Legs Encoded. 1 byte. Data Type 'N'.
  fn expect_number_of_legs_encoded(&mut self) -> Result<u64, BcbpParseError> {
    self.scanner
      .scan_decimal(1)
      .map_err(|_| {
        BcbpParseError::ErrorParsingRequiredField(Field::NumberOfLegs)
      })
  }

  /// Item 6: Field Size of Variable Size Field. 2 byte. Data Type 'f'.
  /// Format specifies right-justified with leading zeroes.
  /// The field is actually hexadecimal.
  fn expect_variable_field_size(&mut self) -> Result<u64, BcbpParseError> {
    self.scanner
      .scan_hexadecimal(2)
      .map_err(|_| {
        BcbpParseError::ErrorParsingRequiredField(Field::FieldSizeOfVariableSizeField)
      })
  }

  /// Item 7: Operating Carrier PNR Code. 7 bytes. Data Type 'f'.
  /// Format specifies left-justified with trailing blanks.
  /// These are trimmed out before returning.
  fn expect_operating_carrier_pnr_code(&mut self) -> Result<String, BcbpParseError> {
    self.scanner
      .scan_characters_from_set(7, CharacterSet::IataAlphaNumerical)
      .map(|s| String::from(s.trim()))
      .map_err(|_| {
        BcbpParseError::ErrorParsingRequiredField(Field::OperatingCarrierPnrCode)
      })
  }

  /// Item 8: Beginning of Version Number. 1 byte. Literal '>'.
  fn expect_beginning_of_version_number(&mut self) -> Result<(), BcbpParseError> {
    if self.scanner.scan_character('>') {
      Ok(())
    } else {
      Err(BcbpParseError::InvalidBeginningOfVersionNumber)
    }
  }

  /// Item 9: Version Number. 1 byte. Data Type 'f'.
  fn expect_version_number(&mut self) -> Result<char, BcbpParseError> {
    self.scanner
      .scan_character_from_set(CharacterSet::IataAlphaNumerical)
      .map_err(|_| {
        BcbpParseError::ErrorParsingRequiredField(Field::VersionNumber)
      })
  }

  /// Item 10: Field Size of Structured Message. 2 byte. Data Type 'f'.
  /// Format specifies right-justified with leading zeroes.
  /// The field is actually hexadecimal.
  fn expect_field_size_of_structured_message_unique(&mut self) -> Result<u64, BcbpParseError> {
    self.scanner
      .scan_hexadecimal(2)
      .map_err(|_| {
        BcbpParseError::ErrorParsingRequiredField(Field::FieldSizeOfStructuredMessageUnique)
      })
  }

  /// Item 11: Passenger Name. 20 bytes. Data Type 'f'.
  fn expect_passenger_name(&mut self) -> Result<String, BcbpParseError> {
    self.scanner
      .scan_characters_from_set(20, CharacterSet::IataAlphaNumerical)
      .map(|s| String::from(s.trim()))
      .map_err(|e| {
        BcbpParseError::ErrorParsingRequiredField(Field::PassengerName)
      })
  }

  /// Item 12: Source of Check-In. 1 byte. Data Type 'f'.
  fn expect_source_of_check_in(&mut self) -> Result<char, BcbpParseError> {
    self.scanner
      .scan_character_from_set(CharacterSet::IataAlphaNumerical)
      .map_err(|_| {
        BcbpParseError::ErrorParsingRequiredField(Field::SourceOfCheckIn)
      })
  }

  /// Item 14: Source of Boarding Pass Issuance. 1 byte. Data Type 'f'.
  fn expect_source_of_boarding_pass_issuance(&mut self) -> Result<char, BcbpParseError> {
    self.scanner
      .scan_character_from_set(CharacterSet::IataAlphaNumerical)
      .map_err(|_| {
        BcbpParseError::ErrorParsingRequiredField(Field::SourceOfBoardingPassIssuance)
      })
  }

  /// Item 15: Passenger Description. 1 byte. Data Type 'f'.
  fn expect_passenger_description(&mut self) -> Result<char, BcbpParseError> {
    self.scanner
      .scan_character_from_set(CharacterSet::IataAlphaNumerical)
      .map_err(|_| {
        BcbpParseError::ErrorParsingRequiredField(Field::PassengerDescription)
      })
  }

  /// Item 16: Document Type. 1 byte. Data Type 'f'.
  fn expect_document_type(&mut self) -> Result<char, BcbpParseError> {
    self.scanner
      .scan_character_from_set(CharacterSet::IataAlphaNumerical)
      .map_err(|_| {
        BcbpParseError::ErrorParsingRequiredField(Field::DocumentType)
      })
  }

  /// Item 17: Field Size of Structured Message. 2 byte. Data Type 'f'.
  /// Format specifies right-justified with leading zeroes.
  /// The field is actually hexadecimal.
  fn expect_field_size_of_structured_message_repeated(&mut self) -> Result<u64, BcbpParseError> {
    self.scanner
      .scan_hexadecimal(2)
      .map_err(|_| {
        BcbpParseError::ErrorParsingRequiredField(Field::FieldSizeOfStructuredMessageRepeated)
      })
  }

  /// Item 18: Selectee Indicator. 1 byte. Data Type 'f'.
  fn expect_selectee_indicator(&mut self) -> Result<char, BcbpParseError> {
    self.scanner
      .scan_character_from_set(CharacterSet::IataAlphaNumerical)
      .map_err(|_| {
        BcbpParseError::ErrorParsingRequiredField(Field::SelecteeIndicator)
      })
  }

  /// Item 19: Marketing Carrier Designator. 3 bytes. Data Type 'f'.
  /// Format specifies left-justified with trailing blanks.
  /// These are trimmed out before returning.
  fn expect_marketing_carrier_designator(&mut self) -> Result<String, BcbpParseError> {
    self.scanner
      .scan_characters_from_set(3, CharacterSet::IataAlphaNumerical)
      .map(|s| String::from(s.trim()))
      .map_err(|_| {
        BcbpParseError::ErrorParsingRequiredField(Field::MarketingCarrierDesignator)
      })
  }

  /// Item 20: Frequent Flyer Airline Designator. 3 bytes. Data Type 'f'.
  /// Format specifies left-justified with trailing blanks.
  /// These are trimmed out before returning.
  fn expect_frequent_flyer_airline_designator(&mut self) -> Result<String, BcbpParseError> {
    self.scanner
      .scan_characters_from_set(3, CharacterSet::IataAlphaNumerical)
      .map(|s| String::from(s.trim()))
      .map_err(|_| {
        BcbpParseError::ErrorParsingRequiredField(Field::FrequentFlyerAirlineDesignator)
      })
  }

  /// Item 21: Airline Designator of Boarding Pass Issuer. 3 bytes. Data Type 'f'.
  /// Format specifies left-justified with trailing blanks.
  /// These are trimmed out before returning.
  fn expect_airline_designator_of_boarding_pass_issuer(&mut self) -> Result<String, BcbpParseError> {
    self.scanner
      .scan_characters_from_set(3, CharacterSet::IataAlphaNumerical)
      .map(|s| String::from(s.trim()))
      .map_err(|_| {
        BcbpParseError::ErrorParsingRequiredField(Field::AirlineDesignatorOfBoardingPassIssuer)
      })
  }

  /// Item 22: Date of Issue of Boarding Pass. 4 bytes. Data Type 'N'.
  /// Format specifies a Julian date where the first character is for the year
  /// while the remaining 3 digits are the right-justified and zero padded 
  /// day of the year.
  fn expect_date_of_issue_of_boarding_pass(&mut self) -> Result<JulianDate, BcbpParseError> {
    let issue_date = self.scanner
      .scan_decimal(4)
      .map_err(|_| {
        BcbpParseError::ErrorParsingRequiredField(Field::DateOfIssueOfBoardingPass)
      })?;
    Ok(JulianDate {
      year: Some((issue_date / 1000) as u8),
      day: (issue_date % 1000) as u16,
    })
  }

  /// Item 23: Baggage Tag License Plate Number(s). 13 bytes. Data Type 'f'.
  /// Format specifies BSM specification layout.
  fn expect_baggage_tag_license_plate_numbers(&mut self) -> Result<String, BcbpParseError> {
    self.scanner
      .scan_characters_from_set(13, CharacterSet::IataAlphabetical)
      .map(|s| String::from(s))
      .map_err(|_| {
        BcbpParseError::ErrorParsingRequiredField(Field::BaggageTagLicensePlateNumbers)
      })
  }

  /// Item 25: Beginning of Security Data. 1 byte. Literal '^'.
  fn expect_beginning_of_security_data(&mut self) -> Result<(), BcbpParseError> {
    if self.scanner.scan_character('^') {
      Ok(())
    } else {
      Err(BcbpParseError::InvalidBeginningOfSecurityData)
    }
  }

  /// Item 26: From City Airport Code. 3 bytes. Data Type 'a'.
  fn expect_from_city_airport_code(&mut self) -> Result<String, BcbpParseError> {
    self.scanner
      .scan_characters_from_set(3, CharacterSet::IataAlphabetical)
      .map(|s| String::from(s))
      .map_err(|_| {
        BcbpParseError::ErrorParsingRequiredField(Field::FromCityAirportCode)
      })
  }

  /// Item 28: Type of Security Data. 1 byte. Data Type 'f'.
  fn expect_type_of_security_data(&mut self) -> Result<char, BcbpParseError> {
    self.scanner
      .scan_character_from_set(CharacterSet::IataAlphaNumerical)
      .map_err(|_| {
        BcbpParseError::ErrorParsingRequiredField(Field::TypeOfSecurityData)
      })
  }

  /// Item 29: Length of Security Data. 2 bytes. Data Type 'f'.
  /// Format specifies right-justified with leading zeroes.
  /// The field is actually hexadecimal.
  fn expect_length_of_security_data(&mut self) -> Result<u64, BcbpParseError> {
    self.scanner
      .scan_hexadecimal(2)
      .map_err(|_| {
        BcbpParseError::ErrorParsingRequiredField(Field::LengthOfSecurityData)
      })
  }

  /// Item 30: Security Data. n bytes. Data Type 'f'.
  fn expect_security_data(&mut self, length: usize) -> Result<String, BcbpParseError> {
    self.scanner
      .scan_characters_from_set(length, CharacterSet::IataAlphaNumerical)
      .map(String::from)
      .map_err(|_| {
        BcbpParseError::ErrorParsingRequiredField(Field::SecurityData)
      })
  }

  /// Item 31: First Non-Consecutive Baggage Tag License Plate Number. 13 bytes. Data Type 'f'.
  /// Format specifies BSM specification layout.
  fn expect_first_non_consecutive_baggage_tag_license_plate_number(&mut self) -> Result<String, BcbpParseError> {
    self.scanner
      .scan_characters_from_set(13, CharacterSet::IataAlphabetical)
      .map(|s| String::from(s))
      .map_err(|_| {
        BcbpParseError::ErrorParsingRequiredField(Field::FirstNonConsecutiveBaggageTagLicensePlateNumber)
      })
  }

  /// Item 32: Second Non-Consecutive Baggage Tag License Plate Number. 13 bytes. Data Type 'f'.
  /// Format specifies BSM specification layout.
  fn expect_second_non_consecutive_baggage_tag_license_plate_number(&mut self) -> Result<String, BcbpParseError> {
    self.scanner
      .scan_characters_from_set(13, CharacterSet::IataAlphabetical)
      .map(|s| String::from(s))
      .map_err(|_| {
        BcbpParseError::ErrorParsingRequiredField(Field::SecondNonConsecutiveBaggageTagLicensePlateNumber)
      })
  }

  /// Item 38: To City Airport Code. 3 bytes. Data Type 'a'.
  fn expect_to_city_airport_code(&mut self) -> Result<String, BcbpParseError> {
    self.scanner
      .scan_characters_from_set(3, CharacterSet::IataAlphabetical)
      .map(|s| String::from(s))
      .map_err(|_| {
        BcbpParseError::ErrorParsingRequiredField(Field::ToCityAirportCode)
      })
  }

  /// Item 42: Operating Carrier Designator. 3 bytes. Data Type 'f'.
  /// Format specifies left-justified with trailing blanks.
  /// These are trimmed out before returning.
  fn expect_operating_carrier_designator(&mut self) -> Result<String, BcbpParseError> {
    self.scanner
      .scan_characters_from_set(3, CharacterSet::IataAlphaNumerical)
      .map(|s| String::from(s.trim()))
      .map_err(|_| {
        BcbpParseError::ErrorParsingRequiredField(Field::OperatingCarrierDesignator)
      })
  }

  /// Item 43: Flight Number. 5 bytes. Data Type 'NNNN[a]'.
  /// Format specifies leading zeroes on numerics and alpha or blank before last digit.
  /// When returned, the format is modified to be variable length, no leading digits and
  /// the trailing alphabetic is appended if not blank.
  fn expect_flight_number(&mut self) -> Result<String, BcbpParseError> {
    let numeric_part = self.scanner
      .scan_decimal(4)
      .map_err(|_| {
        BcbpParseError::ErrorParsingRequiredField(Field::FlightNumber)
      })?;
    
    // A single alphabetical (type 'a') suffix may optionally follow the flight number.
    let suffix = self.scanner.scan_character_from_set(CharacterSet::IataAlphabetical).ok();
    if let Some(letter) = suffix {
      return Ok(format!("{}{}", numeric_part, letter));
    }

    // Some BCBP strings including the BA example on page 19 of the Implementation Guide
    // simply have the optional character completely missing. If a suffix character failed
    // to scan, attempt to scan a trailing space.
    let _ = self.scanner.scan_character(' ');
    Ok(numeric_part.to_string())
  }

  /// Item 46: Date of Flight. 3 bytes. Data Type 'N'.
  /// Format specifies leading zeroes.
  fn expect_date_of_flight(&mut self) -> Result<JulianDate, BcbpParseError> {
    let day = self.scanner
      .scan_decimal(3)
      .map_err(|_| {
        BcbpParseError::ErrorParsingRequiredField(Field::DateOfFlight)
      })?;
    Ok(JulianDate { 
      year: None, 
      day: day as u16, 
    })
  }

  /// Item 71: Compartment Code. 1 byte. Data Type 'a'.
  fn expect_compartment_code(&mut self) -> Result<char, BcbpParseError> {
    self.scanner
      .scan_character_from_set(CharacterSet::IataAlphabetical)
      .map_err(|_| {
        BcbpParseError::ErrorParsingRequiredField(Field::CompartmentCode)
      })
  }

  /// Item 89: Electronic Ticket Indicator. 1 byte. Data Type 'f'.
  fn expect_id_ad_indicator(&mut self) -> Result<char, BcbpParseError> {
    self.scanner
      .scan_character_from_set(CharacterSet::IataAlphaNumerical)
      .map_err(|_| {
        BcbpParseError::ErrorParsingRequiredField(Field::IdAdIndicator)
      })
  }

  /// Item 104: Seat Number. 4 bytes. Data Type 'NNNa'.
  /// Format specifies leading zeroes on numerics.
  /// When returned, the format is modified to be variable length, no leading zeroes.
  fn expect_seat_number(&mut self) -> Result<String, BcbpParseError> {
    let seat_row = self.scanner
      .scan_decimal(3)
      .map_err(|_| {
        BcbpParseError::ErrorParsingRequiredField(Field::SeatNumber)
      })?;
    let seat_col = self.scanner
      .scan_character_from_set(CharacterSet::IataAlphabetical)
      .map_err(|_| {
        BcbpParseError::ErrorParsingRequiredField(Field::SeatNumber)
      })?;
    Ok(format!("{}{}", seat_row, seat_col))
  }

  /// Item 107: Check-In Sequence Number. 5 bytes. Data Type 'NNNN[f]'.
  /// Leading zeroes on numerics and alpha or blank on last digit.
  fn expect_check_in_sequence_number(&mut self) -> Result<String, BcbpParseError> {
    let mut sequence_number = String::with_capacity(5);

    // Scan in the numeric part of the sequence number.
    sequence_number.push_str(
      self.scanner
        .scan_characters_from_set(4, CharacterSet::IataNumerical)
        .map_err(|_| {
          BcbpParseError::ErrorParsingRequiredField(Field::CheckInSequenceNumber)
        })?
    );

    // Scan in the open part of the sequence number.
    let optional_suffix = self.scanner
      .scan_character_from_set(CharacterSet::IataAlphaNumerical)
      .map_err(|_| {
        BcbpParseError::ErrorParsingRequiredField(Field::CheckInSequenceNumber)
      })?;
    if optional_suffix != ' ' {
      sequence_number.push(optional_suffix);
    }
    
    Ok(sequence_number)
  }

  /// Item 108: International Document Verification. 1 byte. Data Type 'f'.
  fn expect_international_document_verification(&mut self) -> Result<char, BcbpParseError> {
    self.scanner
      .scan_character_from_set(CharacterSet::IataAlphaNumerical)
      .map_err(|_| {
        BcbpParseError::ErrorParsingRequiredField(Field::InternationalDocumentVerification)
      })
  }

  /// Item 117: Passenger Status. 1 byte. Data Type 'f'.
  fn expect_passenger_status(&mut self) -> Result<char, BcbpParseError> {
    self.scanner
      .scan_character_from_set(CharacterSet::IataAlphaNumerical)
      .map_err(|_| {
        BcbpParseError::ErrorParsingRequiredField(Field::PassengerStatus)
      })
  }

  /// Item 118: Free Baggage Allowance. 3 bytes. Data Type 'f'.
  /// Format specifies that it will vary between programs and alliances.
  /// Spaces are trimmed out before returning.
  fn expect_free_baggage_allowance(&mut self) -> Result<String, BcbpParseError> {
    self.scanner
      .scan_characters_from_set(3, CharacterSet::IataAlphaNumerical)
      .map(|s| String::from(s.trim()))
      .map_err(|_| {
        BcbpParseError::ErrorParsingRequiredField(Field::FreeBaggageAllowance)
      })
  }

  /// Item 142: Airline Numeric Code. 3 bytes. Data Type 'N'.
  /// Format specifies right-justified with leading zeroes.
  fn expect_airline_numeric_code(&mut self) -> Result<u16, BcbpParseError> {
    self.scanner
      .scan_decimal(3)
      .map(|number| number as u16)
      .map_err(|_| {
        BcbpParseError::ErrorParsingRequiredField(Field::AirlineNumericCode)
      })
  }
  
  /// Item 143: Document Form / Serial Number. 10 bytes. Data Type 'f'.
  /// Format specifies right-justified with leading zeroes.
  fn expect_document_form_serial_number(&mut self) -> Result<String, BcbpParseError> {
    self.scanner
      .scan_characters_from_set(10, CharacterSet::IataAlphaNumerical)
      .map(String::from)
      .map_err(|_| {
        BcbpParseError::ErrorParsingRequiredField(Field::DocumentFormSerialNumber)
      })
  }

  /// Item 236: Frequent Flyer Number. 16 bytes. Data Type 'f'.
  /// Format specifies that it will vary between programs and alliances.
  /// Spaces are trimmed out before returning.
  fn expect_frequent_flyer_number(&mut self) -> Result<String, BcbpParseError> {
    self.scanner
      .scan_characters_from_set(16, CharacterSet::IataAlphaNumerical)
      .map(|s| String::from(s.trim()))
      .map_err(|_| {
        BcbpParseError::ErrorParsingRequiredField(Field::FrequentFlyerNumber)
      })
  }

  /// Item 253: Electronic Ticket Indicator. 1 byte. Data Type 'f'.
  fn expect_electronic_ticket_indicator(&mut self) -> Result<char, BcbpParseError> {
    self.scanner
      .scan_character_from_set(CharacterSet::IataAlphaNumerical)
      .map_err(|_| {
        BcbpParseError::ErrorParsingRequiredField(Field::ElectronicTicketIndicator)
      })
  }

  /// Item 254: Fast Track. 1 byte. Data Type 'f'.
  fn expect_fast_track(&mut self) -> Result<char, BcbpParseError> {
    self.scanner
      .scan_character_from_set(CharacterSet::IataAlphaNumerical)
      .map_err(|_| {
        BcbpParseError::ErrorParsingRequiredField(Field::FastTrack)
      })
  }

  /// Parse an encoded flight leg.
  /// Multiple flight legs can be encoded on the same PNR.
  /// Flight legs contain both a mandatory and a conditional section.
  pub fn parse_flight_leg(&mut self) -> Result<FlightLeg, BcbpParseError> {
    Ok(FlightLeg {
      operating_carrier_pnr: self.expect_operating_carrier_pnr_code()?,
      from_airport: self.expect_from_city_airport_code()?,
      to_airport: self.expect_to_city_airport_code()?,
      operating_carrier: self.expect_operating_carrier_designator()?,
      flight_number: self.expect_flight_number()?,
      date_of_flight: self.expect_date_of_flight()?,
      compartment_code: self.expect_compartment_code()?,
      seat_number: self.expect_seat_number()?,
      check_in_sequence_number: self.expect_check_in_sequence_number()?,
      passenger_status: self.expect_passenger_status()?,
    })
  }

  /// Parse the top-level Barcode Boarding Pass.
  pub fn parse_bcbp_type_m(&mut self) -> Result<Bcbp, BcbpParseError> {
    let format = self.expect_format_code()?;
    if format != 'M' {
      return Err(BcbpParseError::UnsupportedFormat(format));
    }

    // The number of legs encoded in the Bcbp string to be collected later.
    let number_of_legs_encoded = self.expect_number_of_legs_encoded()?;

    // Parse out the unqiue mandatory fields in the header.
    let passenger_name = self.expect_passenger_name()?;
    let is_eticket = {
      let indicator = self.expect_electronic_ticket_indicator()?;
      (indicator == 'E')
    };

    // Collect the legs that follow.
    let mut legs = vec![];
    for _ in 0 .. number_of_legs_encoded {
      let leg = self.parse_flight_leg()?;
      legs.push(leg);
    }

    Ok(Bcbp {
      passenger_name: passenger_name,
      is_eticket: is_eticket,
      legs: legs,
    })
  }

}

impl FromStr for Bcbp {
  type Err = BcbpParseError;
  fn from_str(input: &str) -> Result<Self, Self::Err> {
    Parser::new(input.scanner()).parse_bcbp_type_m()
  }
}
