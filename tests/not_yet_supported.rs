// Copyright (C) 2018 Martin Mroz
//
// This software may be modified and distributed under the terms
// of the MIT license.  See the LICENSE file for details.

//! Valid boarding pass data in formats not yet supported.

extern crate iata_bcbp;

use std::str::FromStr;

use iata_bcbp::*;

#[test]
fn v0_pass() {
    // This appears to be an old, M.0 or M.1 boarding pass which will require a different parse strategy.
    const PASS_STR: &str = "M1SOLLE/JOSUHUA       EQHSLJX ATLMEMDL 0254 006Y28C      10C3JIJI7O4M28C";
    assert!(Bcbp::from_str(PASS_STR).is_err());
}
