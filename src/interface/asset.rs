#[macro_export]
macro_rules! asset {
    ($piece:ty, $color:ident) => {
        (
            (TypeId::of::<$piece>(), PieceColor::$color),
            PathBuf::from(
                concat!(
                    "./assets/",
                    stringify!($color),
                    "_",
                    stringify!($piece),
                    ".png"
                )
                .to_ascii_lowercase(),
            ),
        )
    };
}