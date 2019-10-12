// Copyright (C) 2018 Martin Mroz
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

/// Parses the field encoding the number of legs embedded in the BCBP data.
fn number_of_legs<'a>(input: &'a str) -> IResult<&'a str, u8, VerboseError<&'a str>> {
    context(field::Field::NumberOfLegsEncoded.name(), |input| {
        hex_byte_literal(input, 1)
    })(input)
}

/// Parses and yields the variable-size conditional items field for a flight leg.
fn leg_conditional_items<'a>(input: &'a str) -> IResult<&'a str, &'a str, VerboseError<&'a str>> {
    let (input, length) = context(field::Field::FieldSizeOfVariableSizeField.name(), |input| {
        hex_byte_literal(input, 2)
    })(input)?;

    match length {
        0 => Ok((input, "")),
        _ => take(length as usize)(input),
    }
}

/// Parses the format code specifier tag for an M-type IATA BCBP pass.
fn format_code_m<'a>(input: &'a str) -> IResult<&'a str, char, VerboseError<&'a str>> {
    context(field::Field::FormatCode.name(), 
        char('M')
    )(input)
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
    if (input.len() == 0) {
        Ok((input, None))
    } else {
        character_field(input, field_id).map(|(input, field)| (input, Some(field)))
    }
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

    let mut legs = Vec::new();

    for leg_index in 0 .. number_of_legs_encoded {
        // Mandatory fields common to all legs.
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
        let (input, conditional_items_input) = leg_conditional_items(input)?;

        legs.push(bcbp::Leg {
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
            ..Default::default()
        });
    }

    Ok((
        input,
        bcbp::Bcbp {
            passenger_name,
            electronic_ticket_indicator,
            legs,
            ..Default::default()
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
        assert_eq!(from_str("M1MROZ/MARTIN         EXXXXXX SJCLAXAS 3317 207U001A0006 34D>218 VV8207BAS              2502771980993865 AS AS XXXXX55200000000Z29  00010"), Err(Error::TrailingCharacters));
    }
}
