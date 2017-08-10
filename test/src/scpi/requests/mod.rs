mod str_extensions;

mod output;
mod source;

use std::fmt;
use std::fmt::{Display, Formatter};

use self::str_extensions::StrExtensions;
use super::errors::{ErrorKind, Result};

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum ScpiRequest {
    OutputOn(usize),
    OutputOff(usize),
    OutputStatus(usize),
    SourceFrequencyGet(usize),
    SourceVoltageGet(usize),
    SourceArbitraryFunctionSampleRateGet(usize),
    SourceNoiseFunctionBandwidthGet(usize),
}

impl ScpiRequest {
    pub fn from(string: &str) -> Result<ScpiRequest> {
        let first_four_chars = string.view_first_chars(4);

        let decoded_request = match first_four_chars {
            "OUTP" => output::decode(string),
            "SOUR" => source::decode(string),
            _ => None,
        };

        if let Some(request) = decoded_request {
            Ok(request)
        } else {
            Err(ErrorKind::UnknownScpiRequest(String::from(string)).into())
        }
    }
}

impl Display for ScpiRequest {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match *self {
            ScpiRequest::OutputOn(channel) => {
                write!(formatter, "OUTP{} ON", channel)
            }
            ScpiRequest::OutputOff(channel) => {
                write!(formatter, "OUTP{} OFF", channel)
            }
            ScpiRequest::OutputStatus(channel) => {
                write!(formatter, "OUTP{}?", channel)
            }
            ScpiRequest::SourceFrequencyGet(source) => {
                write!(formatter, "SOUR{}:FREQ?", source)
            }
            ScpiRequest::SourceVoltageGet(source) => {
                write!(formatter, "SOUR{}:VOLT?", source)
            }
            ScpiRequest::SourceArbitraryFunctionSampleRateGet(source) => {
                write!(formatter, "SOUR{}:FUNC:ARB:SRAT?", source)
            }
            ScpiRequest::SourceNoiseFunctionBandwidthGet(source) => {
                write!(formatter, "SOUR{}:FUNC:NOIS:BAND?", source)
            }
        }
    }
}
