var N = null;var searchIndex = {};
searchIndex["cfg_if"]={"doc":"A macro for defining `#[cfg]` if-else statements.","items":[[14,"cfg_if","cfg_if","",N,N]],"paths":[]};
searchIndex["iata_bcbp"]={"doc":"`iata_bcbp is` a Rust library for parsing IATA BCBP Type M objects conforming to versions 2 through 6 of the standard inclusively.  This format is used by airlines to encode boarding pass information into  electronic ticket itinerary document barcodes in addition to paper and  mobile boarding passes.","items":[[3,"Bcbp","iata_bcbp","",N,N],[3,"Leg","","",N,N],[3,"SecurityData","","",N,N],[4,"Field","","",N,N],[13,"FormatCode","","Item 1: Format Code. 1 byte. Data Type 'f'.",0,N],[13,"AirlineIndividualUse","","Item 4: Airline Individual Use. n bytes. Data Type unspecified.",0,N],[13,"NumberOfLegsEncoded","","Item 5: Number of Legs Encoded. 1 byte. Data Type 'N'.",0,N],[13,"FieldSizeOfVariableSizeField","","Item 6: Field Size of Variable Size Field. 2 byte. Data Type 'f'. Hexadecimal.",0,N],[13,"OperatingCarrierPnrCode","","Item 7: Operating Carrier PNR Code. 7 bytes. Data Type 'f'.",0,N],[13,"BeginningOfVersionNumber","","Item 8: Beginning of Version Number. 1 byte. Data Type 'f'.",0,N],[13,"VersionNumber","","Item 9: Version Number. 1 byte. Data Type 'f'.",0,N],[13,"FieldSizeOfStructuredMessageUnique","","Item 10: Field Size of Structured Message. 2 byte. Data Type 'f'. Hexadecimal.",0,N],[13,"PassengerName","","Item 11: Passenger Name. 20 bytes. Data Type 'f'.",0,N],[13,"SourceOfCheckIn","","Item 12: Source of Check-In. 1 byte. Data Type 'f'.",0,N],[13,"SourceOfBoardingPassIssuance","","Item 14: Source of Boarding Pass Issuance. 1 byte. Data Type 'f'.",0,N],[13,"PassengerDescription","","Item 15: Passenger Description. 1 byte. Data Type 'f'.",0,N],[13,"DocumentType","","Item 16: Document Type. 1 byte. Data Type 'f'.",0,N],[13,"FieldSizeOfStructuredMessageRepeated","","Item 17: Field Size of Structured Message. 2 byte. Data Type 'f'. Hexadecimal.",0,N],[13,"SelecteeIndicator","","Item 18: Selectee Indicator. 1 byte. Data Type 'f'.",0,N],[13,"MarketingCarrierDesignator","","Item 19: Marketing Carrier Designator. 3 bytes. Data Type 'f'.",0,N],[13,"FrequentFlyerAirlineDesignator","","Item 20: Frequent Flyer Airline Designator. 3 bytes. Data Type 'f'.",0,N],[13,"AirlineDesignatorOfBoardingPassIssuer","","Item 21: Airline Designator of Boarding Pass Issuer. 3 bytes. Data Type 'f'.",0,N],[13,"DateOfIssueOfBoardingPass","","Item 22: Date of Issue of Boarding Pass. 4 bytes. Data Type 'N'.",0,N],[13,"BaggageTagLicensePlateNumbers","","Item 23: Baggage Tag License Plate Number(s). 13 bytes. Data Type 'f'.",0,N],[13,"BeginningOfSecurityData","","Item 25: Beginning of Security Data. 1 byte. Data Type 'f'.",0,N],[13,"FromCityAirportCode","","Item 26: From City Airport Code. 3 bytes. Data Type 'a'.",0,N],[13,"TypeOfSecurityData","","Item 28: Type of Security Data. 1 byte. Data Type 'f'.",0,N],[13,"LengthOfSecurityData","","Item 29: Length of Security Data. 2 bytes. Data Type 'f'. Hexadecimal.",0,N],[13,"SecurityData","","Item 30: Security Data. n bytes. Data Type 'f'.",0,N],[13,"FirstNonConsecutiveBaggageTagLicensePlateNumbers","","Item 31: First Non-Consecutive Baggage Tag License Plate Number. 13 bytes. Data Type 'f'.",0,N],[13,"SecondNonConsecutiveBaggageTagLicensePlateNumbers","","Item 32: Second Non-Consecutive Baggage Tag License Plate Number. 13 bytes. Data Type 'f'.",0,N],[13,"ToCityAirportCode","","Item 38: To City Airport Code. 3 bytes. Data Type 'a'.",0,N],[13,"OperatingCarrierDesignator","","Item 42: Operating Carrier Designator. 3 bytes. Data Type 'f'.",0,N],[13,"FlightNumber","","Item 43: Flight Number. 5 bytes. Data Type 'NNNN[a]'.",0,N],[13,"DateOfFlight","","Item 46: Date of Flight. 3 bytes. Data Type 'N'.",0,N],[13,"CompartmentCode","","Item 71: Compartment Code. 1 byte. Data Type 'a'.",0,N],[13,"IdAdIndicator","","Item 89: Electronic Ticket Indicator. 1 byte. Data Type 'f'.",0,N],[13,"SeatNumber","","Item 104: Seat Number. 4 bytes. Data Type is usually 'NNNa', but can be 'INF ' or similar.",0,N],[13,"CheckInSequenceNumber","","Item 107: Check-In Sequence Number. 5 bytes. Data Type is usually 'NNNN[f]', but can be 'f'.",0,N],[13,"InternationalDocumentVerification","","Item 108: International Document Verification. 1 byte. Data Type 'f'.",0,N],[13,"PassengerStatus","","Item 117: Passenger Status. 1 byte. Data Type 'f'.",0,N],[13,"FreeBaggageAllowance","","Item 118: Free Baggage Allowance. 3 bytes. Data Type 'f'.",0,N],[13,"AirlineNumericCode","","Item 142: Airline Numeric Code. 3 bytes. Data Type 'N'.",0,N],[13,"DocumentFormSerialNumber","","Item 143: Document Form / Serial Number. 10 bytes. Data Type 'f'.",0,N],[13,"FrequentFlyerNumber","","Item 236: Frequent Flyer Number. 16 bytes. Data Type 'f'.",0,N],[13,"ElectronicTicketIndicator","","Item 253: Electronic Ticket Indicator. 1 byte. Data Type 'f'.",0,N],[13,"FastTrack","","Item 254: Fast Track. 1 byte. Data Type 'f'.",0,N],[4,"Error","","",N,N],[13,"UnexpectedEndOfInput","","The end of the input was reached prematurely.",1,N],[13,"SubsectionTooLong","","The length of the subsection encoded exceeds the remaining length of the input.",1,N],[13,"ExpectedInteger","","The contents of a field parsed as a numeric was not a numeric value.",1,N],[13,"InvalidStartOfVersionNumber","","The start-of-version-number value is not valid.",1,N],[13,"InvalidStartOfSecurityData","","The start-of-security-data value is not valid.",1,N],[13,"InvalidCharacters","","The BCBP string does not contain exclusively ASCII characters.",1,N],[13,"UnsupportedFormat","","The BCBP format is not supported.",1,N],[13,"TrailingCharacters","","After parsing, additional characters remain.",1,N],[5,"from_str","","Parses a boarding pass from `input_data` representable as a string reference.",N,[[["i"]],["result",["bcbp"]]]],[11,"clone","","",2,[[["self"]],["leg"]]],[11,"eq","","",2,[[["self"],["leg"]],["bool"]]],[11,"ne","","",2,[[["self"],["leg"]],["bool"]]],[11,"hash","","",2,N],[11,"fmt","","",2,[[["self"],["formatter"]],["result"]]],[11,"default","","",2,[[],["leg"]]],[11,"operating_carrier_pnr_code","","An alphanumeric string of up to 6 characters, left-aligned, space-padded. This is the Passenger Name Record used to identify the booking in the reservation system of the operating carrier.",2,[[["self"]],["str"]]],[11,"from_city_airport_code","","Three-letter or four-letter IATA code of the origin airport. Spaces indicate the field is not set. Any other values are invalid.",2,[[["self"]],["str"]]],[11,"to_city_airport_code","","Three-letter or four-letter IATA code of the destination airport. Spaces indicate the field is not set. Any other values are invalid.",2,[[["self"]],["str"]]],[11,"marketing_carrier_designator","","Airline code of the marketing carrier, which can be the same as the operating carrier. Two-character and three-letter IATA carrier designators are permitted and the string is left-justified and space padded. Spaces indicate the field is not set. Any other values are invalid.",2,[[["self"]],["option",["str"]]]],[11,"frequent_flyer_airline_designator","","Airline code associated with the frequent flyer number. Two-character and three-letter IATA carrier designators are permitted and the string is left-justified and space padded. Spaces indicate the field is not set. Any other values are invalid.",2,[[["self"]],["option",["str"]]]],[11,"frequent_flyer_number","","2 character or 3 letter airline designator followed by up to 13 numerics or alphanumerics, or 16 numerics if the FFN is 16 digits. Spaces indicate the field is not set. Any other values are invalid.",2,[[["self"]],["option",["str"]]]],[11,"id_ad_indicator","","Values are defined in Resolution 792. Spaces indicate the field is not set. Any other values are invalid.",2,[[["self"]],["option",["char"]]]],[11,"operating_carrier_designator","","Airline code of the operating carrier, which can be the same as the marketing carrier. Two-character and three-letter IATA carrier designators are permitted and the string is left-justified and space padded. Spaces indicate the field is not set. Any other values are invalid.",2,[[["self"]],["str"]]],[11,"flight_number","","A flight number comprised of four numeric characters followed by an optional alphabetic suffix. This refers to the operating carrier. Spaces indicate the field is not set.",2,[[["self"]],["str"]]],[11,"date_of_flight","","The Julian date code for the flight. The 3-digit number reflects the day of the year beginning with '0'. The year is to be inferred. Spaces indicate the field is not set.",2,[[["self"]],["str"]]],[11,"compartment_code","","IATA compartment code indiciating the class of service. Values are defined in Resolution 792. A space indicates the field is not set. Any other values are invalid.",2,[[["self"]],["char"]]],[11,"seat_number","","Seat number of the passenger. Usually 3 numerics followed by a single alphabetic. In the case of infants, can be any 4 ASCII characters, often 'INF '. Spaces indicate the field is not set.",2,[[["self"]],["str"]]],[11,"check_in_sequence_number","","Check-in sequence number. Usually 4 numerics followed by an optional alpha or blank, however in the case of infants, the format is defined by the host system and can be any 5 ASCII characters.",2,[[["self"]],["str"]]],[11,"passenger_status","","The status of the passenger. Field values are defined in Resolution 792. A space indicates the field is not set.",2,[[["self"]],["char"]]],[11,"airline_numeric_code","","The three-digit airline numeric code. This is also the first three digits of the eTicket number. Spaces indicate the field is not set.",2,[[["self"]],["option",["str"]]]],[11,"document_form_serial_number","","The ten-digit DSN. This is also the last ten digits of the eTicket number. Spaces indicate the field is not set.",2,[[["self"]],["option",["str"]]]],[11,"selectee_indicator","","This field is used by certain agencies to demarcate individuals requiring extra screening. Although a conditional field, it is now required as of Resolotion 792 Version 6 when travel involves the United States. Values '0', '1', or '3' determine the type of screening the passenger will receive at US airports. A space indicates the field is not set.",2,[[["self"]],["option",["char"]]]],[11,"international_document_verification","","This field is used by carriers to identify passengers requiring document verification. Connected to the display of the 'DOCS OK' string on international boarding passes.",2,[[["self"]],["option",["char"]]]],[11,"fast_track","","Indicates if the passenger is eligible for fast track. If 'Y', the passenger is eligible, 'N' if not, ' ' if not set. Any other values are invalid.",2,[[["self"]],["option",["char"]]]],[11,"free_baggage_allowance","","Three characters, unstructured, left-aligned and space padded, indicating how much baggage passengers are able to take with them free of charge. Spaces indicate the field is not set.",2,[[["self"]],["option",["str"]]]],[11,"airline_individual_use","","Optional unstructured data for airline individual use. Content frequently includes frequent flyer tier, passenger preferences, etc.",2,[[["self"]],["option",["str"]]]],[11,"clone","","",3,[[["self"]],["securitydata"]]],[11,"eq","","",3,[[["self"],["securitydata"]],["bool"]]],[11,"ne","","",3,[[["self"],["securitydata"]],["bool"]]],[11,"hash","","",3,N],[11,"fmt","","",3,[[["self"],["formatter"]],["result"]]],[11,"default","","",3,[[],["securitydata"]]],[11,"type_of_security_data","","Vendor specific flag indicating the type of the security data which follows.",3,[[["self"]],["option",["char"]]]],[11,"security_data","","Security data used to verify the boarding pass was not tampered with.",3,[[["self"]],["option",["str"]]]],[11,"clone","","",4,[[["self"]],["bcbp"]]],[11,"eq","","",4,[[["self"],["bcbp"]],["bool"]]],[11,"ne","","",4,[[["self"],["bcbp"]],["bool"]]],[11,"hash","","",4,N],[11,"fmt","","",4,[[["self"],["formatter"]],["result"]]],[11,"default","","",4,[[],["bcbp"]]],[11,"legs","","All legs encoded into the boarding pass. At least one needs to be present to form a valid boarding pass.",4,N],[11,"security_data","","A reference to the optional security data used to verify a boarding pass was not tamptered with.",4,[[["self"]],["securitydata"]]],[11,"electronic_ticket_indicator","","Used to differentiate between an electronic ticket ('E') and another type of travel document. Values are defined in Resolution 792. A space indicates the field is not set.",4,[[["self"]],["char"]]],[11,"passenger_description","","This describes the passenger. Values are defined in Resolution 792. Spaces indicate the field is not set.",4,[[["self"]],["option",["char"]]]],[11,"passenger_name","","The name of the passenger. Up to 20 characters, left-aligned, space padded. The format is `LAST_NAME/FIRST_NAME[TITLE]`. There is no separator between the first name and the title, and no indication a title is present. Certain names have characters which cannot be translated and special handling may be required. Spaces indicate the field is not set.",4,[[["self"]],["str"]]],[11,"source_of_check_in","","This field reflects channel in which the customer initiated check-in. Values are defined in Resolution 792 Attachment C. Spaces indicate the field is not set.",4,[[["self"]],["option",["char"]]]],[11,"source_of_boarding_pass_issuance","","This field reflects channel which issued the boarding pass. Values are defined in Resolution 792. Spaces indicate the field is not set.",4,[[["self"]],["option",["char"]]]],[11,"date_of_issue_of_boarding_pass","","Optionally the 4-digit Julian date representing when the boarding pass was issued. The first digit is the last digit of the year and the next three represent the number of days elapsed. For example:   \"6001\" represnts January 1, 2016.   \"6366\" represaents December 31, 2016 (a leap year). Spaces indicate the field is not set.",4,[[["self"]],["option",["str"]]]],[11,"document_type","","The type of the document, 'B' indicating a boarding pass. Spaces indicate the field is not set.",4,[[["self"]],["option",["char"]]]],[11,"airline_designator_of_boarding_pass_issuer","","Airline code of the boarding pass issuer. Two-character and three-letter IATA carrier designators are permitted and the string is left-justified and space padded. Spaces indicate the field is not set.",4,[[["self"]],["option",["str"]]]],[11,"baggage_tag_license_plate_numbers","","This field allows carriers to populate baggage tag numbers and the number of consecutive bags. This 13-character fiels is divided into:         0: '0' for interline tag, '1' for fall-back tag, '2' for interline rush tag.    2... 4: carrier numeric code.    5...10: carrier initial tag number with leading zeroes.   11...13: number of consecutive bags (up to 999). Spaces indicate the field is not set.",4,[[["self"]],["option",["str"]]]],[11,"first_non_consecutive_baggage_tag_license_plate_numbers","","This field allows carriers who handle non-sequential bags to include a second set of them in the boarding pass data in in the same format as `baggage_tag_license_plate_numbers`. Spaces indicate the field is not set.",4,[[["self"]],["option",["str"]]]],[11,"second_non_consecutive_baggage_tag_license_plate_numbers","","This field allows carriers who handle non-sequential bags to include a third set of them in the boarding pass data in in the same format as `baggage_tag_license_plate_numbers`. Spaces indicate the field is not set.",4,[[["self"]],["option",["str"]]]],[11,"clone","","",0,[[["self"]],["field"]]],[11,"eq","","",0,[[["self"],["field"]],["bool"]]],[11,"cmp","","",0,[[["self"],["field"]],["ordering"]]],[11,"partial_cmp","","",0,[[["self"],["field"]],["option",["ordering"]]]],[11,"fmt","","",0,[[["self"],["formatter"]],["result"]]],[11,"hash","","",0,N],[11,"len","","The required length of the field. If zero, the field may be arbitrarily long.",0,[[["self"]],["usize"]]],[11,"name","","Name of the field as defined in the Implementation Guide.",0,[[["self"]],["str"]]],[11,"fmt","","",0,[[["self"],["formatter"]],["result"]]],[11,"from_str","","",4,[[["str"]],["result"]]],[11,"clone","","",1,[[["self"]],["error"]]],[11,"eq","","",1,[[["self"],["error"]],["bool"]]],[11,"ne","","",1,[[["self"],["error"]],["bool"]]],[11,"cmp","","",1,[[["self"],["error"]],["ordering"]]],[11,"partial_cmp","","",1,[[["self"],["error"]],["option",["ordering"]]]],[11,"lt","","",1,[[["self"],["error"]],["bool"]]],[11,"le","","",1,[[["self"],["error"]],["bool"]]],[11,"gt","","",1,[[["self"],["error"]],["bool"]]],[11,"ge","","",1,[[["self"],["error"]],["bool"]]],[11,"hash","","",1,N],[11,"fmt","","",1,[[["self"],["formatter"]],["result"]]],[11,"fmt","","",1,[[["self"],["formatter"]],["result"]]],[6,"Result","","",N,N]],"paths":[[4,"Field"],[4,"Error"],[3,"Leg"],[3,"SecurityData"],[3,"Bcbp"]]};
searchIndex["log"]={"doc":"A lightweight logging facade.","items":[[3,"Record","log","The \"payload\" of a log message.",N,N],[3,"RecordBuilder","","Builder for `Record`.",N,N],[3,"Metadata","","Metadata about a log message.",N,N],[3,"MetadataBuilder","","Builder for `Metadata`.",N,N],[3,"SetLoggerError","","The type returned by [`set_logger`] if [`set_logger`] has already been called.",N,N],[3,"ParseLevelError","","The type returned by [`from_str`] when the string doesn't match any of the log levels.",N,N],[4,"Level","","An enum representing the available verbosity levels of the logger.",N,N],[13,"Error","","The \"error\" level.",0,N],[13,"Warn","","The \"warn\" level.",0,N],[13,"Info","","The \"info\" level.",0,N],[13,"Debug","","The \"debug\" level.",0,N],[13,"Trace","","The \"trace\" level.",0,N],[4,"LevelFilter","","An enum representing the available verbosity level filters of the logger.",N,N],[13,"Off","","A level lower than all log levels.",1,N],[13,"Error","","Corresponds to the `Error` log level.",1,N],[13,"Warn","","Corresponds to the `Warn` log level.",1,N],[13,"Info","","Corresponds to the `Info` log level.",1,N],[13,"Debug","","Corresponds to the `Debug` log level.",1,N],[13,"Trace","","Corresponds to the `Trace` log level.",1,N],[5,"set_max_level","","Sets the global maximum log level.",N,[[["levelfilter"]]]],[5,"max_level","","Returns the current maximum log level.",N,[[],["levelfilter"]]],[5,"set_boxed_logger","","Sets the global logger to a `Box<Log>`.",N,[[["box",["log"]]],["result",["setloggererror"]]]],[5,"set_logger","","Sets the global logger to a `&'static Log`.",N,[[["log"]],["result",["setloggererror"]]]],[5,"logger","","Returns a reference to the logger.",N,[[],["log"]]],[17,"STATIC_MAX_LEVEL","","The statically resolved maximum log level.",N,N],[8,"Log","","A trait encapsulating the operations required of a logger.",N,N],[10,"enabled","","Determines if a log message with the specified metadata would be logged.",2,[[["self"],["metadata"]],["bool"]]],[10,"log","","Logs the `Record`.",2,[[["self"],["record"]]]],[10,"flush","","Flushes any buffered records.",2,[[["self"]]]],[11,"fmt","","",0,[[["self"],["formatter"]],["result"]]],[11,"hash","","",0,N],[11,"clone","","",0,[[["self"]],["level"]]],[11,"eq","","",0,[[["self"],["level"]],["bool"]]],[11,"eq","","",0,[[["self"],["levelfilter"]],["bool"]]],[11,"partial_cmp","","",0,[[["self"],["level"]],["option",["ordering"]]]],[11,"lt","","",0,[[["self"],["level"]],["bool"]]],[11,"le","","",0,[[["self"],["level"]],["bool"]]],[11,"gt","","",0,[[["self"],["level"]],["bool"]]],[11,"ge","","",0,[[["self"],["level"]],["bool"]]],[11,"partial_cmp","","",0,[[["self"],["levelfilter"]],["option",["ordering"]]]],[11,"lt","","",0,[[["self"],["levelfilter"]],["bool"]]],[11,"le","","",0,[[["self"],["levelfilter"]],["bool"]]],[11,"gt","","",0,[[["self"],["levelfilter"]],["bool"]]],[11,"ge","","",0,[[["self"],["levelfilter"]],["bool"]]],[11,"cmp","","",0,[[["self"],["level"]],["ordering"]]],[11,"from_str","","",0,[[["str"]],["result",["level"]]]],[11,"fmt","","",0,[[["self"],["formatter"]],["result"]]],[11,"max","","Returns the most verbose logging level.",0,[[],["level"]]],[11,"to_level_filter","","Converts the `Level` to the equivalent `LevelFilter`.",0,[[["self"]],["levelfilter"]]],[11,"fmt","","",1,[[["self"],["formatter"]],["result"]]],[11,"hash","","",1,N],[11,"clone","","",1,[[["self"]],["levelfilter"]]],[11,"eq","","",1,[[["self"],["levelfilter"]],["bool"]]],[11,"eq","","",1,[[["self"],["level"]],["bool"]]],[11,"partial_cmp","","",1,[[["self"],["levelfilter"]],["option",["ordering"]]]],[11,"lt","","",1,[[["self"],["levelfilter"]],["bool"]]],[11,"le","","",1,[[["self"],["levelfilter"]],["bool"]]],[11,"gt","","",1,[[["self"],["levelfilter"]],["bool"]]],[11,"ge","","",1,[[["self"],["levelfilter"]],["bool"]]],[11,"partial_cmp","","",1,[[["self"],["level"]],["option",["ordering"]]]],[11,"lt","","",1,[[["self"],["level"]],["bool"]]],[11,"le","","",1,[[["self"],["level"]],["bool"]]],[11,"gt","","",1,[[["self"],["level"]],["bool"]]],[11,"ge","","",1,[[["self"],["level"]],["bool"]]],[11,"cmp","","",1,[[["self"],["levelfilter"]],["ordering"]]],[11,"from_str","","",1,[[["str"]],["result",["levelfilter"]]]],[11,"fmt","","",1,[[["self"],["formatter"]],["result"]]],[11,"max","","Returns the most verbose logging level filter.",1,[[],["levelfilter"]]],[11,"to_level","","Converts `self` to the equivalent `Level`.",1,[[["self"]],["option",["level"]]]],[11,"clone","","",3,[[["self"]],["record"]]],[11,"fmt","","",3,[[["self"],["formatter"]],["result"]]],[11,"builder","","Returns a new builder.",3,[[],["recordbuilder"]]],[11,"args","","The message body.",3,[[["self"]],["arguments"]]],[11,"metadata","","Metadata about the log directive.",3,[[["self"]],["metadata"]]],[11,"level","","The verbosity level of the message.",3,[[["self"]],["level"]]],[11,"target","","The name of the target of the directive.",3,[[["self"]],["str"]]],[11,"module_path","","The module path of the message.",3,[[["self"]],["option",["str"]]]],[11,"file","","The source file containing the message.",3,[[["self"]],["option",["str"]]]],[11,"line","","The line containing the message.",3,[[["self"]],["option",["u32"]]]],[11,"fmt","","",4,[[["self"],["formatter"]],["result"]]],[11,"new","","Construct new `RecordBuilder`.",4,[[],["recordbuilder"]]],[11,"args","","Set `args`.",4,[[["self"],["arguments"]],["recordbuilder"]]],[11,"metadata","","Set `metadata`. Construct a `Metadata` object with `MetadataBuilder`.",4,[[["self"],["metadata"]],["recordbuilder"]]],[11,"level","","Set `Metadata::level`.",4,[[["self"],["level"]],["recordbuilder"]]],[11,"target","","Set `Metadata::target`",4,[[["self"],["str"]],["recordbuilder"]]],[11,"module_path","","Set `module_path`",4,[[["self"],["option",["str"]]],["recordbuilder"]]],[11,"file","","Set `file`",4,[[["self"],["option",["str"]]],["recordbuilder"]]],[11,"line","","Set `line`",4,[[["self"],["option",["u32"]]],["recordbuilder"]]],[11,"build","","Invoke the builder and return a `Record`",4,[[["self"]],["record"]]],[11,"clone","","",5,[[["self"]],["metadata"]]],[11,"eq","","",5,[[["self"],["metadata"]],["bool"]]],[11,"ne","","",5,[[["self"],["metadata"]],["bool"]]],[11,"cmp","","",5,[[["self"],["metadata"]],["ordering"]]],[11,"partial_cmp","","",5,[[["self"],["metadata"]],["option",["ordering"]]]],[11,"lt","","",5,[[["self"],["metadata"]],["bool"]]],[11,"le","","",5,[[["self"],["metadata"]],["bool"]]],[11,"gt","","",5,[[["self"],["metadata"]],["bool"]]],[11,"ge","","",5,[[["self"],["metadata"]],["bool"]]],[11,"hash","","",5,N],[11,"fmt","","",5,[[["self"],["formatter"]],["result"]]],[11,"builder","","Returns a new builder.",5,[[],["metadatabuilder"]]],[11,"level","","The verbosity level of the message.",5,[[["self"]],["level"]]],[11,"target","","The name of the target of the directive.",5,[[["self"]],["str"]]],[11,"eq","","",6,[[["self"],["metadatabuilder"]],["bool"]]],[11,"ne","","",6,[[["self"],["metadatabuilder"]],["bool"]]],[11,"cmp","","",6,[[["self"],["metadatabuilder"]],["ordering"]]],[11,"partial_cmp","","",6,[[["self"],["metadatabuilder"]],["option",["ordering"]]]],[11,"lt","","",6,[[["self"],["metadatabuilder"]],["bool"]]],[11,"le","","",6,[[["self"],["metadatabuilder"]],["bool"]]],[11,"gt","","",6,[[["self"],["metadatabuilder"]],["bool"]]],[11,"ge","","",6,[[["self"],["metadatabuilder"]],["bool"]]],[11,"hash","","",6,N],[11,"fmt","","",6,[[["self"],["formatter"]],["result"]]],[11,"new","","Construct a new `MetadataBuilder`.",6,[[],["metadatabuilder"]]],[11,"level","","Setter for `level`.",6,[[["self"],["level"]],["metadatabuilder"]]],[11,"target","","Setter for `target`.",6,[[["self"],["str"]],["metadatabuilder"]]],[11,"build","","Returns a `Metadata` object.",6,[[["self"]],["metadata"]]],[11,"fmt","","",7,[[["self"],["formatter"]],["result"]]],[11,"fmt","","",7,[[["self"],["formatter"]],["result"]]],[11,"description","","",7,[[["self"]],["str"]]],[11,"fmt","","",8,[[["self"],["formatter"]],["result"]]],[11,"eq","","",8,[[["self"],["parselevelerror"]],["bool"]]],[11,"ne","","",8,[[["self"],["parselevelerror"]],["bool"]]],[11,"fmt","","",8,[[["self"],["formatter"]],["result"]]],[11,"description","","",8,[[["self"]],["str"]]],[14,"log","","The standard logging macro.",N,N],[14,"error","","Logs a message at the error level.",N,N],[14,"warn","","Logs a message at the warn level.",N,N],[14,"info","","Logs a message at the info level.",N,N],[14,"debug","","Logs a message at the debug level.",N,N],[14,"trace","","Logs a message at the trace level.",N,N],[14,"log_enabled","","Determines if a message logged at the specified level in that module will be logged.",N,N],[11,"to_owned","","",3,[[["self"]],["t"]]],[11,"clone_into","","",3,N],[11,"from","","",3,[[["t"]],["t"]]],[11,"into","","",3,[[["self"]],["u"]]],[11,"try_from","","",3,[[["u"]],["result"]]],[11,"borrow","","",3,[[["self"]],["t"]]],[11,"try_into","","",3,[[["self"]],["result"]]],[11,"borrow_mut","","",3,[[["self"]],["t"]]],[11,"get_type_id","","",3,[[["self"]],["typeid"]]],[11,"from","","",4,[[["t"]],["t"]]],[11,"into","","",4,[[["self"]],["u"]]],[11,"try_from","","",4,[[["u"]],["result"]]],[11,"borrow","","",4,[[["self"]],["t"]]],[11,"try_into","","",4,[[["self"]],["result"]]],[11,"borrow_mut","","",4,[[["self"]],["t"]]],[11,"get_type_id","","",4,[[["self"]],["typeid"]]],[11,"to_owned","","",5,[[["self"]],["t"]]],[11,"clone_into","","",5,N],[11,"from","","",5,[[["t"]],["t"]]],[11,"into","","",5,[[["self"]],["u"]]],[11,"try_from","","",5,[[["u"]],["result"]]],[11,"borrow","","",5,[[["self"]],["t"]]],[11,"try_into","","",5,[[["self"]],["result"]]],[11,"borrow_mut","","",5,[[["self"]],["t"]]],[11,"get_type_id","","",5,[[["self"]],["typeid"]]],[11,"from","","",6,[[["t"]],["t"]]],[11,"into","","",6,[[["self"]],["u"]]],[11,"try_from","","",6,[[["u"]],["result"]]],[11,"borrow","","",6,[[["self"]],["t"]]],[11,"try_into","","",6,[[["self"]],["result"]]],[11,"borrow_mut","","",6,[[["self"]],["t"]]],[11,"get_type_id","","",6,[[["self"]],["typeid"]]],[11,"to_string","","",7,[[["self"]],["string"]]],[11,"from","","",7,[[["t"]],["t"]]],[11,"into","","",7,[[["self"]],["u"]]],[11,"try_from","","",7,[[["u"]],["result"]]],[11,"borrow","","",7,[[["self"]],["t"]]],[11,"try_into","","",7,[[["self"]],["result"]]],[11,"borrow_mut","","",7,[[["self"]],["t"]]],[11,"get_type_id","","",7,[[["self"]],["typeid"]]],[11,"to_string","","",8,[[["self"]],["string"]]],[11,"from","","",8,[[["t"]],["t"]]],[11,"into","","",8,[[["self"]],["u"]]],[11,"try_from","","",8,[[["u"]],["result"]]],[11,"borrow","","",8,[[["self"]],["t"]]],[11,"try_into","","",8,[[["self"]],["result"]]],[11,"borrow_mut","","",8,[[["self"]],["t"]]],[11,"get_type_id","","",8,[[["self"]],["typeid"]]],[11,"to_owned","","",0,[[["self"]],["t"]]],[11,"clone_into","","",0,N],[11,"to_string","","",0,[[["self"]],["string"]]],[11,"from","","",0,[[["t"]],["t"]]],[11,"into","","",0,[[["self"]],["u"]]],[11,"try_from","","",0,[[["u"]],["result"]]],[11,"borrow","","",0,[[["self"]],["t"]]],[11,"try_into","","",0,[[["self"]],["result"]]],[11,"borrow_mut","","",0,[[["self"]],["t"]]],[11,"get_type_id","","",0,[[["self"]],["typeid"]]],[11,"to_owned","","",1,[[["self"]],["t"]]],[11,"clone_into","","",1,N],[11,"to_string","","",1,[[["self"]],["string"]]],[11,"from","","",1,[[["t"]],["t"]]],[11,"into","","",1,[[["self"]],["u"]]],[11,"try_from","","",1,[[["u"]],["result"]]],[11,"borrow","","",1,[[["self"]],["t"]]],[11,"try_into","","",1,[[["self"]],["result"]]],[11,"borrow_mut","","",1,[[["self"]],["t"]]],[11,"get_type_id","","",1,[[["self"]],["typeid"]]]],"paths":[[4,"Level"],[4,"LevelFilter"],[8,"Log"],[3,"Record"],[3,"RecordBuilder"],[3,"Metadata"],[3,"MetadataBuilder"],[3,"SetLoggerError"],[3,"ParseLevelError"]]};
initSearch(searchIndex);