use anyhow::{anyhow, Result};
use regex::Regex;
use thiserror::Error;

mod decimal {
    pub const UNIT_CHAR: &str = "kMGTPEZYRQ";
    pub const POSTFIX: &str = "B";
    pub const BASE: u16 = 1000;
}
mod binary {
    pub const UNIT_CHAR: &str = "KMGTPEZY";
    pub const POSTFIX: &str = "iB";
    pub const BASE: u16 = 1024;
}
#[derive(Debug, Error, PartialEq)]
enum SizeParserError {
    #[error("Provided format is not supported {form}\n")]
    WrongFormat { form: String },
}

struct SizeParser {}
impl SizeParser {
    fn find_power(&self, value: &str, postfix: &str, chars: &str) -> Result<u8> {
        let s = String::from(chars);
        for (i, char) in s.chars().enumerate() {
            let a = String::from(char) + postfix;
            let re = Regex::new(&a).unwrap();
            if re.find(value).is_some() {
                return Ok((i + 1) as u8);
            }
        }
        Err(anyhow!(SizeParserError::WrongFormat {
            form: String::from(value)
        }))
    }
    pub fn parse(&self, value: &str) -> Result<u128> {
        let re = Regex::new(r"\d+").unwrap();
        let r = re.find(value);
        let number: u32 = r.map(|m| m.as_str().parse::<u32>().unwrap()).unwrap_or(0);

        match self.find_power(value, decimal::POSTFIX, decimal::UNIT_CHAR) {
            Ok(power) => {
                return Ok((decimal::BASE as u128).pow(power.into()) * number as u128);
            }
            _ => {}
        };
        match self.find_power(value, binary::POSTFIX, binary::UNIT_CHAR) {
            Ok(power) => {
                return Ok((binary::BASE as u128).pow(power.into()) * number as u128);
            }
            _ => {}
        }

        Err(anyhow!(SizeParserError::WrongFormat {
            form: String::from(value)
        }))
    }
}

/*
Decimal
Value	Metric
1000	kB	kilobyte
1000^2	MB	megabyte
1000^3	GB	gigabyte
1000^4	TB	terabyte
1000^5	PB	petabyte
1000^6	EB	exabyte
1000^7	ZB	zettabyte
1000^8	YB	yottabyte
1000^9	RB	ronnabyte
1000^10	QB	quettabyte

Binary
Value	IEC	Memory
1024	KiB	kibibyte	KB	kilobyte
1024^2	MiB	mebibyte	MB	megabyte
1024^3	GiB	gibibyte	GB	gigabyte
1024^4	TiB	tebibyte	TB	terabyte
1024^5	PiB	pebibyte	–
1024^6	EiB	exbibyte	–
1024^7	ZiB	zebibyte	–
1024^8	YiB	yobibyte	–
*/

#[cfg(test)]
mod tests {
    use crate::core::size::{binary, SizeParser};

    #[test]
    fn test_sizes_binary() {
        let parser = SizeParser {};
        assert_eq!(parser.parse("10KiB").unwrap(), 10 * (binary::BASE as u128));
        assert_eq!(
            parser.parse("12GiB").unwrap(),
            12 * (binary::BASE as u128).pow(3)
        );
        assert_eq!(
            parser.parse("83MiB").unwrap(),
            83 * (binary::BASE as u128).pow(2)
        );
        assert_eq!(
            parser.parse("01TiB").unwrap(),
            01 * (binary::BASE as u128).pow(4)
        );
        assert_eq!(
            parser.parse("31PiB").unwrap(),
            31 * (binary::BASE as u128).pow(5)
        );
        assert_eq!(
            parser.parse("16EiB").unwrap(),
            16 * (binary::BASE as u128).pow(6)
        );
    }
    #[test]
    fn test_sizes_decimal() {
        let parser = SizeParser {};
        assert_eq!(parser.parse("10kB").unwrap(), 10 * 1000);
        assert_eq!(parser.parse("12GB").unwrap(), 12 * 1000u128.pow(3));
        assert_eq!(parser.parse("83MB").unwrap(), 83 * 1000u128.pow(2));
        assert_eq!(parser.parse("01TB").unwrap(), 01 * 1000u128.pow(4));
        assert_eq!(parser.parse("31PB").unwrap(), 31 * 1000u128.pow(5));
        assert_eq!(parser.parse("16EB").unwrap(), 16 * 1000u128.pow(6));
    }
    #[test]
    fn test_sizes_binary_error() {
        let parser = SizeParser {};
        let e = parser.parse("10fiB");
        let error = e.unwrap_err();
        assert_eq!(
            format!("{}", error),
            "Provided format is not supported 10fiB\n"
        );
    }
    #[test]
    fn test_sizes_no_number() {
        let parser = SizeParser {};
        let e = parser.parse("fB");
        let error = e.unwrap_err();
        assert_eq!(
            format!("{}", error),
            "Provided format is not supported fB\n"
        );
    }
    #[test]
    fn test_sizes_decimal_error() {
        let parser = SizeParser {};
        let e = parser.parse("10fB");
        let error = e.unwrap_err();
        assert_eq!(
            format!("{}", error),
            "Provided format is not supported 10fB\n"
        );
    }
}
