IATA BCBP
=========

* [Documentation](http://martinmroz.github.io/iata_bcbp)

A Rust library for parsing 
[IATA BCBP Type M](https://www.iata.org/whatwedo/stb/Documents/BCBP-Implementation-Guide-5th-Edition-June-2016.pdf) 
objects conforming to versions 2 through 6 of the standard inclusively. 
This format is used by airlines to encode boarding pass information into electronic ticket itinerary
document barcodes in addition to paper and mobile boarding passes.

[![](http://meritbadge.herokuapp.com/iata_bcbp)](https://crates.io/crates/iata_bcbp)
[![Build Status](https://travis-ci.org/martinmroz/iata_bcbp.svg?branch=master)](https://travis-ci.org/martinmroz/iata_bcbp)
[![Coverage Status](https://coveralls.io/repos/github/martinmroz/iata_bcbp/badge.svg?branch=master)](https://coveralls.io/github/martinmroz/iata_bcbp?branch=master)

### Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
iata_bcbp = "0.1"
```

In addition, and this to your crate root:

```rust
extern crate iata_bcbp;
```

Here is an example which parses a BCBP object with test data and prints it:

```rust
extern crate iata_bcbp;

use std::str::FromStr;

use iata_bcbp::Bcbp;

fn main() {
    const PASS_STR: &str = "M1DESMARAIS/LUC       EABC123 YULFRAAC 0834 326J001A0025 100";
    let pass_data = Bcbp::from_str(PASS_STR).unwrap();

    println!("Passenger: {}", pass_data.passenger_name());
    
    for leg in pass_data.legs().iter() {
        println!();
        println!("      PNR: {}"  , leg.operating_carrier_pnr_code());
        println!("     From: {}"  , leg.from_city_airport_code());
        println!("       To: {}"  , leg.to_city_airport_code());
        println!("   Flight: {}{}", leg.operating_carrier_designator(), leg.flight_number());
        println!("     Seat: {}"  , leg.seat_number());
        println!(" Sequence: {}"  , leg.check_in_sequence_number());
    }
}
```

# License

`iata_bcbp` is distributed under the terms of the MIT license.

See LICENSE for details.
