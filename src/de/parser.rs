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
    combinator::map_res,
    error::{context, convert_error, VerboseError},
    IResult,
};

/// Tests if char c is ASCII uppercase alphabetic (A-F) or numeric (0-9).
fn is_ascii_uppercase_hexdigit(c: char) -> bool {
    c.is_ascii_hexdigit() && !c.is_ascii_lowercase()
}

/// Parses a one- or two-digit ASCII uppercase hexadecimal string literal value.
///
/// # Notes
/// - Does not provide additional context.
fn hex_byte_literal<'a>(
    input: &'a str,
    length: usize,
) -> IResult<&'a str, u8, VerboseError<&'a str>> {
    assert!(length == 1 || length == 2);
    map_res(
        take_while_m_n(length, length, is_ascii_uppercase_hexdigit),
        |s: &str| u8::from_str_radix(s, 16),
    )(input)
}

/// Parses a variable-length field whose size data is in the specified first field.
fn variable_size_field_data<'a>(
    input: &'a str, 
    field_id: field::Field
) -> IResult<&'a str, &'a str, VerboseError<&'a str>> {
    let (remainder, length) = context(field_id.name(), |input| {
        hex_byte_literal(input, 2)
    })(input)?;

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
    context(field::Field::NumberOfLegsEncoded.name(), |input| {
        hex_byte_literal(input, 1)
    })(input)
}

/// Parses the format code specifier tag for an M-type IATA BCBP pass.
fn format_code_m<'a>(input: &'a str) -> IResult<&'a str, char, VerboseError<&'a str>> {
    context(field::Field::FormatCode.name(), 
        char('M')
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
    optional_character_field(input, field::Field::VersionNumber)
}

/// Parses a fixed-length String-type field.
fn string_field<'a, T>(
    input: &'a str,
    field_id: field::Field,
) -> IResult<&'a str, ArrayString<T>, VerboseError<&'a str>>
where
    T: Array<Item = u8> + Copy,
{
    // Verify that the size of the storage array matches the field exactly.
    assert_eq!(std::mem::size_of::<T>(), field_id.len());

    // Copies bytes equal to the length of the specified field into an ArrayString.
    let parse_field = map_res(
        take(field_id.len()), 
        |s: &str| ArrayString::from(s)
    );

    // Ascribe the name of the field as context for the operation.
    context(field_id.name(), parse_field)(input)
}

/// Parses an optional fixed-length String-type field within a variable-length section.
///
/// # Notes
/// - This function will succeed and return None if the remaining length of the string is zero.
/// - This function will fail if the remaining length of the string is less than that of the requested field.
fn optional_string_field<'a, T>(
    input: &'a str,
    field_id: field::Field,
) -> IResult<&'a str, Option<ArrayString<T>>, VerboseError<&'a str>>
where
    T: Array<Item = u8> + Copy,
{
    if input.len() == 0 {
        Ok((input, None))
    } else {
        string_field(input, field_id).map(|(input, field)| (input, Some(field)))
    }
}

/// Parses a single-character field.
fn character_field<'a>(
    input: &'a str,
    field_id: field::Field,
) -> IResult<&'a str, char, VerboseError<&'a str>> {
    assert_eq!(field_id.len(), 1);
    context(field_id.name(), anychar)(input)
}

/// Parses an optional single-character field within a variable-length section.
///
/// # Notes
/// - This function will succeed and return None if the remaining length of the string is zero.
fn optional_character_field<'a>(
    input: &'a str,
    field_id: field::Field,
) -> IResult<&'a str, Option<char>, VerboseError<&'a str>> {
    if input.len() == 0 {
        Ok((input, None))
    } else {
        character_field(input, field_id).map(|(input, field)| (input, Some(field)))
    }
}

/// Parses conditional metadata potentially embedded in the first leg.
fn conditional_metadata<'a>(input: &'a str) -> IResult<&'a str, bcbp::ConditionalMetadata, VerboseError<&'a str>> {
    let (input, version_number) = optional_version_number(input)?;

    // Conditional metadata is encoded in an optional variable-size field.
    let (remainder, conditional_item_data) = 
        optional_variable_size_field_data(input, field::Field::FieldSizeOfStructuredMessageUnique)?;

    // Each field is optional, and encoded within the conditional item data section.
    let (conditional_item_data, passenger_description) =
        optional_character_field(conditional_item_data, field::Field::PassengerStatus)?;
    let (conditional_item_data, source_of_check_in) =
        optional_character_field(conditional_item_data, field::Field::SourceOfCheckIn)?;
    let (conditional_item_data, source_of_boarding_pass_issuance) =
        optional_character_field(conditional_item_data, field::Field::SourceOfBoardingPassIssuance)?;
    let (conditional_item_data, date_of_issue_of_boarding_pass) =
        optional_string_field(conditional_item_data, field::Field::DateOfIssueOfBoardingPass)?;
    let (conditional_item_data, document_type) =
        optional_character_field(conditional_item_data, field::Field::DocumentType)?;
    let (conditional_item_data, airline_designator_of_boarding_pass_issuer) =
        optional_string_field(conditional_item_data, field::Field::AirlineDesignatorOfBoardingPassIssuer)?;
    let (conditional_item_data, baggage_tag_license_plate_numbers) =
        optional_string_field(conditional_item_data, field::Field::BaggageTagLicensePlateNumbers)?;
    let (conditional_item_data, first_non_consecutive_baggage_tag_license_plate_numbers) =
        optional_string_field(conditional_item_data, field::Field::FirstNonConsecutiveBaggageTagLicensePlateNumbers)?;
    let (_, second_non_consecutive_baggage_tag_license_plate_numbers) =
        optional_string_field(conditional_item_data, field::Field::SecondNonConsecutiveBaggageTagLicensePlateNumbers)?;

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
    // Mandatory items common to all legs.
    let (input, operating_carrier_pnr_code) =
        string_field(input, field::Field::OperatingCarrierPnrCode)?;
    let (input, from_city_airport_code) =
        string_field(input, field::Field::FromCityAirportCode)?;
    let (input, to_city_airport_code) =
        string_field(input, field::Field::ToCityAirportCode)?;
    let (input, operating_carrier_designator) =
        string_field(input, field::Field::OperatingCarrierDesignator)?;
    let (input, flight_number) =
        string_field(input, field::Field::FlightNumber)?;
    let (input, date_of_flight) =
        string_field(input, field::Field::DateOfFlight)?;
    let (input, compartment_code) =
        character_field(input, field::Field::CompartmentCode)?;
    let (input, seat_number) =
        string_field(input, field::Field::SeatNumber)?;
    let (input, check_in_sequence_number) =
        string_field(input, field::Field::CheckInSequenceNumber)?;
    let (input, passenger_status) =
        character_field(input, field::Field::PassengerStatus)?;

    // A set of conditional items may follow the required items for each leg.
    let (remainder, conditional_item_data) =
        variable_size_field_data(input, field::Field::FieldSizeOfVariableSizeField)?;

    // Top-level conditional metadata may be embedded in the first leg.
    let (conditional_item_data, optional_conditional_metadata) = if is_first_leg {
        conditional_metadata(conditional_item_data).map(|(input, data)| (input, Some(data)))?
    } else {
        (conditional_item_data, None)
    };

    // Repeated conditional items are stored in a variable-length section.
    let (individual_use_data, conditional_item_data) =
        optional_variable_size_field_data(conditional_item_data, field::Field::FieldSizeOfStructuredMessageRepeated)?;

    // Conditional leg data is encoded in an optional variable-size field.
    let (conditional_item_data, airline_numeric_code) =
        optional_string_field(conditional_item_data, field::Field::AirlineNumericCode)?;
    let (conditional_item_data, document_form_serial_number) =
        optional_string_field(conditional_item_data, field::Field::DocumentFormSerialNumber)?;
    let (conditional_item_data, selectee_indicator) =
        optional_character_field(conditional_item_data, field::Field::SelecteeIndicator)?;
    let (conditional_item_data, international_document_verification) =
        optional_character_field(conditional_item_data, field::Field::InternationalDocumentVerification)?;
    let (conditional_item_data, marketing_carrier_designator) =
        optional_string_field(conditional_item_data, field::Field::MarketingCarrierDesignator)?;
    let (conditional_item_data, frequent_flyer_airline_designator) =
        optional_string_field(conditional_item_data, field::Field::FrequentFlyerAirlineDesignator)?;
    let (conditional_item_data, frequent_flyer_number) =
        optional_string_field(conditional_item_data, field::Field::FrequentFlyerNumber)?;
    let (conditional_item_data, id_ad_indicator) =
        optional_character_field(conditional_item_data, field::Field::IdAdIndicator)?;
    let (conditional_item_data, free_baggage_allowance) =
        optional_string_field(conditional_item_data, field::Field::FreeBaggageAllowance)?;
    let (_, fast_track) =
        optional_character_field(conditional_item_data, field::Field::FastTrack)?;

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
        character_field(input, field::Field::TypeOfSecurityData)?;
    let (input, security_data_field_data) =
        variable_size_field_data(input, field::Field::LengthOfSecurityData)?;

    // Variable-length security data is stored as a String.
    let security_data = if security_data_field_data.len() > 0 {
        Some(String::from(security_data_field_data))
    } else {
        None
    };

    Ok((
        input,
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
    // Check that the input string is likely an M-type BCBP string.
    let (input, _) = format_code_m(input)?;

    // The number of legs informs the breakdown of the various field iterators.
    let (input, number_of_legs_encoded) = number_of_legs(input)?;

    // Scan mandatory unique fields.
    let (input, passenger_name) = 
        string_field(input, field::Field::PassengerName)?;
    let (input, electronic_ticket_indicator) =
        character_field(input, field::Field::ElectronicTicketIndicator)?;

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
    let (input, security_data) = security_data(input)?;

    Ok((
        input,
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
    if !format_code_m(input).is_ok() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
    }
}
