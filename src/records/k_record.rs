use crate::util::datetime::Time;
use crate::util::parse_error::ParseError;
use records::extension::Extendable;

/// An extension data record.
///
/// Contains only a timestamp by default, but can be extended with a J record.
#[derive(Debug, PartialEq, Eq)]
pub struct KRecord<'a> {
    pub time: Time,
    extension_string: &'a str,
}

impl<'a> KRecord<'a> {
    pub fn parse(line: &'a str) -> Result<Self, ParseError> {
        assert_eq!(line.as_bytes()[0], b'K');

        if line.len() <= 7 {
            return Err(ParseError::SyntaxError);
        }

        let time = Time::parse(&line[1..7])?;
        let extension_string = &line[7..];

        Ok(Self { time, extension_string })
    }
}

impl<'a> Extendable for KRecord<'a> {
    const BASE_LENGTH: usize = 7;

    fn extension_string(&self) -> &str {
        self.extension_string
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use records::extension::Extension;

    #[test]
    fn krecord_parse() {
        let sample_string = "K095214FooTheBar";
        let parsed = KRecord::parse(sample_string).unwrap();
        let expected = KRecord { time: Time::from_hms(9, 52, 14), extension_string: "FooTheBar" };

        assert_eq!(parsed, expected);
    }

    #[test]
    fn krecord_extensions() {
        let record = KRecord { time: Time::from_hms(9, 52, 14), extension_string: "FooTheBar" };
        let ext1 = Extension { start_byte: 8, end_byte: 10, mnemonic: "One" };
        let ext2 = Extension { start_byte: 11, end_byte: 13, mnemonic: "Two" };
        let ext3 = Extension { start_byte: 14, end_byte: 16, mnemonic: "Th3" };

        assert_eq!(record.get_extension(&ext1).unwrap(), "Foo");
        assert_eq!(record.get_extension(&ext2).unwrap(), "The");
        assert_eq!(record.get_extension(&ext3).unwrap(), "Bar");
    }
}
