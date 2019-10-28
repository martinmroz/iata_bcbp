// Copyright (C) 2019 Martin Mroz
//
// This software may be modified and distributed under the terms
// of the MIT license.  See the LICENSE file for details.

use bcbp;
use de::field;
use error::{Error, Result};

use arrayvec::{Array, ArrayString};
use nom::{
    bytes::complete::{take, take_while_m_n},
    character::complete::{anychar, char},
    combinator::{map, map_res},
    error::{context, convert_error, ParseError, VerboseError},
    sequence::tuple,
    IResult,
};

/// Tests if char c is ASCII uppercase alphabetic (A-F) or numeric (0-9).
fn is_ascii_uppercase_hexdigit(c: char) -> bool {
    c.is_ascii_hexdigit() && !c.is_ascii_lowercase()
}

/// Returns a parser for a one- or two-digit ASCII uppercase hexadecimal string literal value.
///
/// # Notes
/// - Does not provide additional context.
fn hex_byte_literal<'a, Error: ParseError<&'a str>>(
    length: usize
) -> impl Fn(&'a str) -> IResult<&'a str, u8, Error> {
    assert!(length == 1 || length == 2);
    map_res(
        take_while_m_n(length, length, is_ascii_uppercase_hexdigit),
        |s: &str| u8::from_str_radix(s, 16),
    )
}

/// Parses a variable-length field whose size data is in the specified first field.
fn variable_size_field_data<'a>(
    input: &'a str, 
    field_id: field::Field
) -> IResult<&'a str, &'a str, VerboseError<&'a str>> {
    let (remainder, length) = context(field_id.name(), 
        hex_byte_literal(2)
    )(input)?;

    match length {
        0 => Ok((remainder, &input[0 .. 0])),
        _ => take(length as usize)(remainder),
    }
}

/// Parses an optional variable-length field whose size data is specified in the first field.
fn optional_variable_size_field_data<'a>(
    input: &'a str, 
    field_id: field::Field
) -> IResult<&'a str, &'a str, VerboseError<&'a str>> {
    if input.len() == 0 {
        Ok((input, input))
    } else {
        variable_size_field_data(input, field_id)
    }
}

/// Parses the field encoding the number of legs embedded in the BCBP data.
fn number_of_legs<'a>(input: &'a str) -> IResult<&'a str, u8, VerboseError<&'a str>> {
    context(field::Field::NumberOfLegsEncoded.name(),
        hex_byte_literal(1)
    )(input)
}

/// Parses and returns an (optional) version number field, beginning with the '>' indicator.
fn optional_version_number<'a>(input: &'a str) -> IResult<&'a str, Option<char>, VerboseError<&'a str>> {
    if input.len() == 0 {
        return Ok((input, None));
    }

    // If data is available, match the beginning-of-version-number chevron character.
    let (input, _) = context(field::Field::BeginningOfVersionNumber.name(),
        char('>')
    )(input)?;

    // Consume and return the version number character.
    optional_chr_field(field::Field::VersionNumber)(input)
}

/// Returns a parser for a specified field returning an `ArrayString` over its length.
fn str_field<'a, T, Error: ParseError<&'a str>>(
    field_id: field::Field
) -> impl Fn(&'a str) -> IResult<&'a str, ArrayString<T>, Error>
where
    T: Array<Item = u8> + Copy,
{
    // Verify that the size of the storage array matches the field exactly.
    assert_eq!(std::mem::size_of::<T>(), field_id.len());
    context(field_id.name(),
        map_res(
            take(field_id.len()), 
            |s: &str| ArrayString::from(s)
        )
    )
}

/// Returns a parser for an optional fixed-length String-type field within a variable-length section.
///
/// # Notes
/// - The parser will succeed and return None if the remaining length of the string is zero.
/// - The parser will fail if the remaining length of the string is less than that of the requested field.
fn optional_str_field<'a, T, Error: ParseError<&'a str>>(
    field_id: field::Field
) -> impl Fn(&'a str) -> IResult<&'a str, Option<ArrayString<T>>, Error>
where
    T: Array<Item = u8> + Copy,
{
    move |input: &'a str| {
        if input.len() == 0 {
            Ok((input, None))
        } else {
            map(
                str_field(field_id),
                |field_value| Some(field_value),
            )(input)
        }
    }
}

/// Returns a parser for a specified single-character field yielding a `char`.
fn chr_field<'a, Error: ParseError<&'a str>>(
    field_id: field::Field
) -> impl Fn(&'a str) -> IResult<&'a str, char, Error> {
    assert_eq!(field_id.len(), 1);
    context(field_id.name(), anychar)
}

/// Returns a parser for an optional single-character field within a variable-length section.
///
/// # Notes
/// - The parser will succeed and return None if the remaining length of the string is zero.
fn optional_chr_field<'a, Error: ParseError<&'a str>>(
    field_id: field::Field
) -> impl Fn(&'a str) -> IResult<&'a str, Option<char>, Error> {
    move |input: &'a str| {
        if input.len() == 0 {
            Ok((input, None))
        } else {
            map(
                chr_field(field_id),
                |c: char| Some(c)
            )(input)
        }
    }
}

/// Parses conditional metadata potentially embedded in the first leg.
fn conditional_metadata<'a>(input: &'a str) -> IResult<&'a str, bcbp::ConditionalMetadata, VerboseError<&'a str>> {
    let (input, version_number) = optional_version_number(input)?;

    // Conditional metadata is encoded in an optional variable-size field.
    let (remainder, conditional_item_data) =
        optional_variable_size_field_data(input, field::Field::FieldSizeOfStructuredMessageUnique)?;

    // Each field is optional, and encoded within the conditional item data section.
    let (_, (
        passenger_description,
        source_of_check_in,
        source_of_boarding_pass_issuance,
        date_of_issue_of_boarding_pass,
        document_type,
        airline_designator_of_boarding_pass_issuer,
        baggage_tag_license_plate_numbers,
        first_non_consecutive_baggage_tag_license_plate_numbers,
        second_non_consecutive_baggage_tag_license_plate_numbers,
    )) = tuple((
        optional_chr_field(field::Field::PassengerStatus),
        optional_chr_field(field::Field::SourceOfCheckIn),
        optional_chr_field(field::Field::SourceOfBoardingPassIssuance),
        optional_str_field(field::Field::DateOfIssueOfBoardingPass),
        optional_chr_field(field::Field::DocumentType),
        optional_str_field(field::Field::AirlineDesignatorOfBoardingPassIssuer),
        optional_str_field(field::Field::BaggageTagLicensePlateNumbers),
        optional_str_field(field::Field::FirstNonConsecutiveBaggageTagLicensePlateNumbers),
        optional_str_field(field::Field::SecondNonConsecutiveBaggageTagLicensePlateNumbers),
    ))(conditional_item_data)?;

    // The remainder not encluded in the conditional item data section is returned meaning
    // any fields added in the future not recognized by this parser are skipped over.
    Ok((
        remainder,
        bcbp::ConditionalMetadata {
            version_number,
            passenger_description,
            source_of_check_in,
            source_of_boarding_pass_issuance,
            date_of_issue_of_boarding_pass,
            document_type,
            airline_designator_of_boarding_pass_issuer,
            baggage_tag_license_plate_numbers,
            first_non_consecutive_baggage_tag_license_plate_numbers,
            second_non_consecutive_baggage_tag_license_plate_numbers
        }
    ))
}

/// Parses a leg.
/// 
/// When parsing the first leg, additional Pass-level data may be present.
/// This data is skipped in the context of the leg, but the location within the input
/// is returned if available when `is_first` is `true` so parsing may resume at the top-level.
fn leg<'a>(
    input: &'a str,
    is_first_leg: bool
) -> IResult<&'a str, (bcbp::Leg, Option<bcbp::ConditionalMetadata>), VerboseError<&'a str>> {
    // Parse mandatory fields common to all legs.
    let (input, (
        operating_carrier_pnr_code,
        from_city_airport_code,
        to_city_airport_code,
        operating_carrier_designator,
        flight_number,
        date_of_flight,
        compartment_code,
        seat_number,
        check_in_sequence_number,
        passenger_status,
    )) = tuple((
        str_field(field::Field::OperatingCarrierPnrCode),
        str_field(field::Field::FromCityAirportCode),
        str_field(field::Field::ToCityAirportCode),
        str_field(field::Field::OperatingCarrierDesignator),
        str_field(field::Field::FlightNumber),
        str_field(field::Field::DateOfFlight),
        chr_field(field::Field::CompartmentCode),
        str_field(field::Field::SeatNumber),
        str_field(field::Field::CheckInSequenceNumber),
        chr_field(field::Field::PassengerStatus),
    ))(input)?;

    // A set of conditional items may follow the required items for each leg.
    let (remainder, conditional_item_data) =
        variable_size_field_data(input, field::Field::FieldSizeOfVariableSizeField)?;

    // Top-level conditional metadata may be embedded in the first leg.
    let (conditional_item_data, optional_conditional_metadata) = if is_first_leg {
        map(conditional_metadata, |data| Some(data))(conditional_item_data)?
    } else {
        (conditional_item_data, None)
    };

    // Repeated conditional items are stored in a variable-length section.
    let (individual_use_data, conditional_item_data) =
        optional_variable_size_field_data(conditional_item_data, field::Field::FieldSizeOfStructuredMessageRepeated)?;

    // Conditional leg data is encoded in an optional variable-size field.
    let (_, (
        airline_numeric_code,
        document_form_serial_number,
        selectee_indicator,
        international_document_verification,
        marketing_carrier_designator,
        frequent_flyer_airline_designator,
        frequent_flyer_number,
        id_ad_indicator,
        free_baggage_allowance,
        fast_track,
    )) = tuple((
        optional_str_field(field::Field::AirlineNumericCode),
        optional_str_field(field::Field::DocumentFormSerialNumber),
        optional_chr_field(field::Field::SelecteeIndicator),
        optional_chr_field(field::Field::InternationalDocumentVerification),
        optional_str_field(field::Field::MarketingCarrierDesignator),
        optional_str_field(field::Field::FrequentFlyerAirlineDesignator),
        optional_str_field(field::Field::FrequentFlyerNumber),
        optional_chr_field(field::Field::IdAdIndicator),
        optional_str_field(field::Field::FreeBaggageAllowance),
        optional_chr_field(field::Field::FastTrack),
    ))(conditional_item_data)?;

    // Anything remaining in the section is ascribed to airline individual use.
    let airline_individual_use = if individual_use_data.len() > 0 {
        Some(String::from(individual_use_data))
    } else {
        None
    };

    let leg = bcbp::Leg {
        operating_carrier_pnr_code,
        from_city_airport_code,
        to_city_airport_code,
        operating_carrier_designator,
        flight_number,
        date_of_flight,
        compartment_code,
        seat_number,
        check_in_sequence_number,
        passenger_status,
        airline_numeric_code,
        document_form_serial_number,
        selectee_indicator,
        international_document_verification,
        marketing_carrier_designator,
        frequent_flyer_airline_designator,
        frequent_flyer_number,
        id_ad_indicator,
        free_baggage_allowance,
        fast_track,
        airline_individual_use,
    };

    Ok((remainder, (leg, optional_conditional_metadata)))
}

/// Parses a Security Data section.
fn security_data<'a>(input: &'a str) -> IResult<&'a str, bcbp::SecurityData, VerboseError<&'a str>> {
    if input.len() == 0 {
        return Ok((input, Default::default()));
    }

    // If data is available, match the beginning-of-security-data caret character.
    let (input, _) = context(field::Field::BeginningOfSecurityData.name(),
        char('^')
    )(input)?;

    // The type field is mandatory, as is at least the length of the security data.
    let (input, type_of_security_data) =
        chr_field(field::Field::TypeOfSecurityData)(input)?;
    let (remainder, security_data_field_data) =
        variable_size_field_data(input, field::Field::LengthOfSecurityData)?;

    // Variable-length security data is stored as a String.
    let security_data = if security_data_field_data.len() > 0 {
        Some(String::from(security_data_field_data))
    } else {
        None
    };

    Ok((
        remainder,
        bcbp::SecurityData {
            type_of_security_data: Some(type_of_security_data),
            security_data: security_data
        }
    ))
}

/// Parses a boarding pass from `input`.
///
/// The input must contain only valid ASCII characters.
fn bcbp<'a>(input: &'a str) -> IResult<&'a str, bcbp::Bcbp, VerboseError<&'a str>> {
    // Scan mandatory unique fields including the format code and the number of legs encoded.
    let (input, (
        _,
        number_of_legs_encoded,
        passenger_name,
        electronic_ticket_indicator,
    )) = tuple((
        char('M'),
        number_of_legs,
        str_field(field::Field::PassengerName),
        chr_field(field::Field::ElectronicTicketIndicator),
    ))(input)?;

    // Collect the legs and metadata fields.
    let mut legs = Vec::new();
    let mut metadata = Default::default();

    // Track the input as each leg is consumed.
    let mut input = input;

    // Consume each leg specified in the number of legs encoded.
    for leg_index in 0 .. number_of_legs_encoded {
        let is_first_leg = leg_index == 0;

        // Consume the leg and, if available, the metadata embedded in the first leg.
        let (next_input, (current_leg, first_leg_metadata)) = leg(input, is_first_leg)?;
        if let Some(value) = first_leg_metadata {
            metadata = value;
        }

        // Store the leg and advance the input.
        legs.push(current_leg);
        input = next_input;
    }

    // Consume security data that follows the last leg, if any.
    let (remainder, security_data) = security_data(input)?;

    Ok((
        remainder,
        bcbp::Bcbp {
            passenger_name,
            electronic_ticket_indicator,
            metadata,
            legs,
            security_data
        },
    ))
}

/// Parses a boarding pass from `input_data` representable as a string reference.
pub fn from_str<I>(input_data: I) -> Result<bcbp::Bcbp>
where
    I: AsRef<str>,
{
    let input = input_data.as_ref();
    if !input.is_ascii() {
        return Err(Error::InvalidCharacters);
    }

    // Sanity-check that the input is likely an IATA Type M BCBP Boarding Pass.
    if !input.starts_with("M") {
        return Err(Error::UnsupportedFormat);
    }

    // Pass the provided input data with the nom combinator and map the error.
    let (remainder, boarding_pass) = bcbp(input).map_err(|e| match e {
        nom::Err::Incomplete(_) =>
            Error::UnexpectedEndOfInput,
        nom::Err::Error(verbose_error) | nom::Err::Failure(verbose_error) =>
            Error::ParseFailed(convert_error(input, verbose_error)),
    })?;

    if remainder.len() > 0 {
        Err(Error::TrailingCharacters)
    } else {
        Ok(boarding_pass)
    }
}
