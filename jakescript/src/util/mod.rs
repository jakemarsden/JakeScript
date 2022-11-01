#[macro_export(crate)]
macro_rules! simple_enumeration {
    (
        $(#[$attribute:meta] )*
        $vis:vis $type_name:ident {$(
            $variant:ident => $display_name:literal,
        )*}
    ) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
        $(#[$attribute] )*
        $vis enum $type_name {
            $($variant, )*
        }

        impl $type_name {
            $vis fn all() -> &'static [Self] {
                const ALL: &'static [$type_name] = &[
                    $($type_name::$variant, )*
                ];
                ALL
            }

            $vis fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$variant => $display_name, )*
                }
            }
        }

        impl ::std::str::FromStr for $type_name {
            type Err = ();

            fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                ::std::result::Result::Ok(match s {
                    $($display_name => Self::$variant, )*
                    _ => return ::std::result::Result::Err(()),
                })
            }
        }

        impl ::std::fmt::Display for $type_name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
    };
}
