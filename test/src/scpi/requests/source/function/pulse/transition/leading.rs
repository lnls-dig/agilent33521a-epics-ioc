use super::super::super::super::super::ScpiRequest;
use super::super::super::super::super::str_extensions::StrExtensions;

pub fn decode(string: &str, source: usize) -> Option<ScpiRequest> {
    let command = string.skip_expected_chars("LEADing");

    if command.starts_with("?") {
        return Some(ScpiRequest::SourcePulseFunctionLeadingEdgeTransitionGet(
            source,
        ));
    }

    None
}