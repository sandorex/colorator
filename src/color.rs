//! Module contains color struct

use std::{fmt, num::ParseIntError};
use minijinja::{self, value::{from_args, Object, ObjectKind, StructObject}, Value, Error, ErrorKind};

// implement RGB to RGBA and HSL to HSLA .with_alpha(255)

#[derive(Debug, PartialEq, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: Option<u8>,
}

impl Color {
    fn get_luminance(&self) -> f64 {
        fn to_srgb(x: f64) -> f64 {
            if x <= 0.03928 {
                x / 12.92
            } else {
                ((x + 0.055) / 1.055).powf(2.4)
            }
        }

        // convert 0..255 -> 0..1
        let r = Into::<f64>::into(self.r) / 255.0;
        let g = Into::<f64>::into(self.g) / 255.0;
        let b = Into::<f64>::into(self.b) / 255.0;

        let a = self.a.map_or_else(|| 1.0, |x| Into::<f64>::into(x) / 255.0);

        (
            0.2126 * to_srgb(r) +
            0.7152 * to_srgb(g) +
            0.0722 * to_srgb(b)
        ) * a
    }

    pub fn contrast(&self, color: &Self) -> f64 {
        let lum1 = self.get_luminance();
        let lum2 = color.get_luminance();

        if lum1 > lum2 {
            (lum1 + 0.05) / (lum2 + 0.05)
        } else {
            (lum2 + 0.05) / (lum1 + 0.05)
        }
    }
}

impl TryFrom<&str> for Color {
    type Error = ParseIntError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let without_prefix = value.trim_start_matches("#");
        let mut color = u64::from_str_radix(without_prefix, 16)?;

        // if there is more than 6 then get alpha
        let a = if without_prefix.len() > 6 {
            let a = Some(color & 0xFF);

            // shift the alpha
            color >>= 8;

            a
        } else {
            None
        };

        let r = (color & 0xFF0000) >> 16;
        let g = (color & 0x00FF00) >> 8;
        let b = color & 0x0000FF;

        // i am unwrapping as i took exactly 0xFF above so two bytes
        Ok(Color {
            r: r.try_into().unwrap(),
            g: g.try_into().unwrap(),
            b: b.try_into().unwrap(),
            a: a.map(|x| x.try_into().unwrap())
        })
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.a {
            Some(alpha) => write!(f, "{:02X}{:02X}{:02X}{:02X}", self.r, self.g, self.b, alpha),
            None => write!(f, "{:02X}{:02X}{:02X}", self.r, self.g, self.b),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const COLOR1_HEX: &'static str = "A0E7E1";
    const COLOR1: Color = Color {
        r: 160,
        g: 231,
        b: 225,
        a: None,
    };

    const COLOR2_HEX: &'static str = "480C7AFF";
    const COLOR2: Color = Color {
        r: 72,
        g: 12,
        b: 122,
        a: Some(255),
    };

    #[test]
    fn color_luminance() {
        // check if luminance is correct (roughly)
        assert_eq!(format!("{:.3}", COLOR1.get_luminance()), "0.701");
        assert_eq!(format!("{:.3}", COLOR2.get_luminance()), "0.030");
    }

    #[test]
    fn color_contrast() {
        // ensure the order does not matter
        assert_eq!(COLOR1.contrast(&COLOR2), COLOR2.contrast(&COLOR1));

        // contrast is working
        assert_eq!(format!("{:.3}", COLOR1.contrast(&COLOR2)), "9.329");
    }

    #[test]
    fn color_display() {
        // check if display conversion is working
        assert_eq!(format!("{}", COLOR1), COLOR1_HEX.to_lowercase());
        assert_eq!(format!("{}", COLOR2), COLOR2_HEX.to_lowercase());
    }

    #[test]
    fn color_hex_parsing() {
        // test hex parsing with and without #
        assert_eq!(TryInto::<Color>::try_into(format!("#{}", COLOR1_HEX).as_str()), Ok(COLOR1));
        assert_eq!(TryInto::<Color>::try_into(COLOR1_HEX), Ok(COLOR1));

        assert_eq!(TryInto::<Color>::try_into(format!("#{}", COLOR2_HEX).as_str()), Ok(COLOR2));
        assert_eq!(TryInto::<Color>::try_into(COLOR2_HEX), Ok(COLOR2));
    }
}

impl Object for Color {
    fn kind(&self) -> ObjectKind<'_> {
        ObjectKind::Struct(self)
    }

    fn call_method(&self, _: &minijinja::State, name: &str, args: &[Value]) -> Result<Value, Error> {
        match name {
            "contrast" => {
                let (color,): (Value,) = from_args(args)?;

                match color.downcast_object_ref::<Color>() {
                    Some(x) => Ok(Value::from(self.contrast(x))),
                    None => Err(Error::new(ErrorKind::InvalidOperation, "calling contrast on non-color type")),
                }
            },
            _ => Err(Error::new(ErrorKind::UnknownMethod, name.to_string())),
        }
    }
}

impl StructObject for Color {
    fn get_field(&self, name: &str) -> Option<Value> {
        match name {
            "r" => Some(Value::from(self.r)),
            "g" => Some(Value::from(self.g)),
            "b" => Some(Value::from(self.b)),
            "a" => match self.a {
                Some(x) => Some(Value::from(x)),
                None => None,
            },
            "luminance" => Some(Value::from(self.get_luminance())),
            _ => None,
        }
    }

    fn static_fields(&self) -> Option<&'static [&'static str]> {
        Some(&["r", "g", "b", "a", "luminance"][..])
    }
}

