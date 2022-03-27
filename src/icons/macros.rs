#[macro_export]
macro_rules! icons {
    { $($name:ident => $path:literal),* $(,)* } => {
        pub struct Icons {
            $(
                $name: Texture2D,
            )*
        }

        impl Icons {
            pub fn load() -> Self {
                Self {
                    $(
                        $name: Texture2D::from_file_with_format(
                            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/", $path)),
                            None
                        ),
                    )*
                }
            }
        }
    };
}
