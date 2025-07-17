#[macro_export]
macro_rules! new_piece {
    ($piece:ty $(, $state:ty)?) => {
        impl $piece {
            #[inline]
            pub fn new(color: PieceColor, pos: Point, size: u32) -> Self
            $(
            where
                $state: Default,
            )?
            {
                let Point { x, y } = pos * size as isize;
                Self {
                    color,
                    pos,
                    rect: Rect::new(x as i32, y as i32, size, size),
                    $(
                        state: <$state as Default>::default(),
                    )?
                }
            }
        }
    };
}