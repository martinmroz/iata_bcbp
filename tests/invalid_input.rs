// Copyright (C) 2019 Martin Mroz
//
// This software may be modified and distributed under the terms
// of the MIT license.  See the LICENSE file for details.

//! Synthetic test cases with invalid boarding pass data.

extern crate iata_bcbp;

use std::str::FromStr;

use iata_bcbp::*;

#[test]
fn trailing_characters() {
    // This is a complete and valid Type 'M' boarding pass from the IATA 792B examples, with a trailing '+'.
    const PASS_STR: &str = "M1DESMARAIS/LUC       EABC123 YULFRAAC 0834 326J001A0025 100^100+";
    assert_eq!(
        Bcbp::from_str(PASS_STR),
        Err(Error::TrailingCharacters)
    );
}

#[test]
fn unsupported_format() {
    // The first character indicates the format. This is a valid Type 'M' boarding pass from the IATA 792B examples, with the wrong format code.
    const PASS_STR_S: &str = "S1DESMARAIS/LUC       EABC123 YULFRAAC 0834 326J001A0025 100^100";
    assert_eq!(
        Bcbp::from_str(PASS_STR_S), 
        Err(Error::UnsupportedFormat)
    );

    // This is the same valid Type 'M' boarding pass but with a lower-case 'm' format specifier.
    const PASS_STR_LITTLE_M: &str = "m1DESMARAIS/LUC       EABC123 YULFRAAC 0834 326J001A0025 100^100";
    assert_eq!(
        Bcbp::from_str(PASS_STR_LITTLE_M), 
        Err(Error::UnsupportedFormat)
    );
}

#[test]
fn invalid_characters() {
    // This is a complete and valid Type 'M' boarding pass from the IATA 792B examples, with a non-ASCII character.
    const PASS_STR: &str = "M1DESMARAIS/LUç       EABC123 YULFRAAC 0834 326J001A0025 100^100";
    assert_eq!(
        Bcbp::from_str(PASS_STR), 
        Err(Error::InvalidCharacters)
    );

    // This is invalid data with a non-ASCII character.
    const PASS_STR_MINIMAL: &str = "ç";
    assert_eq!(
        Bcbp::from_str(PASS_STR_MINIMAL),
        Err(Error::InvalidCharacters)
    );
}

#[test]
fn invalid_start_of_security_data() {
    // This is a complete and valid Type 'M' boarding pass from the IATA 792B examples, using a '+' instead of '^' for start of security data.
    const PASS_STR: &str = "M1DESMARAIS/LUC       EABC123 YULFRAAC 0834 326J001A0025 100+100";
    assert_eq!(
        Bcbp::from_str(PASS_STR),
        Err(Error::ParseFailed(
            "0: at line 0:\n".to_owned() +
            "M1DESMARAIS/LUC       EABC123 YULFRAAC 0834 326J001A0025 100+100\n" +
            "                                                            ^\n" +
            "expected '^', found +\n" +
            "\n" +
            "1: at line 0, in Beginning of Security Data:\n" +
            "M1DESMARAIS/LUC       EABC123 YULFRAAC 0834 326J001A0025 100+100\n" +
            "                                                            ^\n" +
            "\n"
        ))
    );
}

#[test]
fn invalid_start_of_version_number() {
    // This is a complete and valid Type 'M' boarding pass from the IATA 792B examples, using a '+' instead of '>' for start of version number.
    const PASS_STR: &str = "M2DESMARAIS/LUC       EABC123 YULFRAAC 0834 226F001A0025 14D+6181WW6225BAC 00141234560032A0141234567890 1AC AC 1234567890123    20KYLX58ZDEF456 FRAGVALH 3664 227C012C0002 12E2A0140987654321 1AC AC 1234567890123    2PCNWQ^100";
    assert_eq!(
        Bcbp::from_str(PASS_STR),
        Err(Error::ParseFailed(String::from("")))
    );
}

#[test]
fn expected_integer() {
    // This is a complete and valid Type 'M' boarding pass from the IATA 792B examples, with leg count 'X'.
    const PASS_STR_1: &str = "MXDESMARAIS/LUC       EABC123 YULFRAAC 0834 326J001A0025 100^100+";
    assert_eq!(
        Bcbp::from_str(PASS_STR_1),
        Err(Error::ParseFailed(String::from("")))
    );

    // This is a complete and valid Type 'M' boarding pass from the IATA 792B examples, with security data length 'YY'.
    const PASS_STR_2: &str = "M1DESMARAIS/LUC       EABC123 YULFRAAC 0834 326J001A0025 100^1YY";
    assert_eq!(
        Bcbp::from_str(PASS_STR_2),
        Err(Error::ParseFailed(String::from("")))
    );
}

#[test]
fn subsection_too_long() {
    // This is a complete and valid Type 'M' boarding pass from the IATA 792B examples, with an 'FF' long conditional.
    const PASS_STR: &str = "M2DESMARAIS/LUC       EABC123 YULFRAAC 0834 226F001A0025 1FF>6181WW6225BAC 00141234560032A0141234567890 1AC AC 1234567890123    20KYLX58ZDEF456 FRAGVALH 3664 227C012C0002 12E2A0140987654321 1AC AC 1234567890123    2PCNWQ^100";
    assert_eq!(
        Bcbp::from_str(PASS_STR),
        Err(Error::ParseFailed(String::from("")))
    );
}

#[test]
fn unexpected_end_of_input() {
    // This is a complete and valid Type 'M' boarding pass from the IATA 792B examples, with a security data extending past end of input.
    const PASS_STR_SEC: &str = "M2DESMARAIS/LUC       EABC123 YULFRAAC 0834 226F001A0025 14D>6181WW6225BAC 00141234560032A0141234567890 1AC AC 1234567890123    20KYLX58ZDEF456 FRAGVALH 3664 227C012C0002 12E2A0140987654321 1AC AC 1234567890123    2PCNWQ^101";
    assert_eq!(
        Bcbp::from_str(PASS_STR_SEC),
        Err(Error::UnexpectedEndOfInput)
    );

    // This is an incomplete type M pass truncated half way through the name field.
    const PASS_STR_NAME: &str = "M2DESMARAIS";
    assert_eq!(
        Bcbp::from_str(PASS_STR_NAME),
        Err(Error::UnexpectedEndOfInput)
    );
}
