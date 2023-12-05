#[macro_export]
macro_rules! create_type {
    ($type_name:ident) => {
        paste::paste! {
            #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
            pub struct $type_name(u32);

            impl $type_name {
                pub fn new(value: u32) -> Self {
                    Self(value)
                }

                pub fn value(&self) -> u32 {
                    self.0
                }
            }

            impl From<u32> for $type_name {
                fn from(value: u32) -> $type_name {
                    Self::new(value)
                }
            }

            impl From<$type_name> for u32 {
                fn from(value: $type_name) -> u32 {
                    value.value()
                }
            }

            impl crate::AlmanacType for $type_name {}

            impl ::std::ops::Add<usize> for $type_name {
                type Output = $type_name;

                fn add(self, value: usize) -> Self::Output {
                    Self::new((self.0 as usize + value) as u32)
                }
            }

            impl ::std::ops::Sub<$type_name> for $type_name {
                type Output = usize;

                fn sub(self, value: $type_name) -> Self::Output {
                    (self.0 - value.0) as usize
                }
            }

            impl ::std::str::FromStr for $type_name {
                type Err = [<Parse $type_name Error>];

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    Ok(Self::new(::std::str::FromStr::from_str(s)?))
                }
            }

            impl ::std::fmt::Display for $type_name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    write!(f, "{}", self.0)
                }
            }

            #[derive(Debug, Eq, PartialEq)]
            pub struct [<Parse $type_name Error>](::std::num::ParseIntError);

            impl ::std::fmt::Display for [<Parse $type_name Error>] {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    write!(f, "Failed to parse a {}: {}", stringify!($type_name), self.0)
                }
            }

            impl ::std::error::Error for [<Parse $type_name Error>] {}

            impl From<::std::num::ParseIntError> for [<Parse $type_name Error>] {
                fn from(value: ::std::num::ParseIntError) -> Self {
                    Self(value)
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    create_type!(Test);

    #[test]
    fn test_parse_test() {
        assert_eq!(Test::from_str("59"), Ok(Test(59)));
        assert_eq!(
            Test::from_str("59a")
                .expect_err("parsing did not fail")
                .to_string(),
            "Failed to parse a Test: invalid digit found in string"
        );
    }
}
