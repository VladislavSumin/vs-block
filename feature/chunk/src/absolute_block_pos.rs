use bevy_derive::{Deref, DerefMut};
use bevy_math::IVec3;

/// Абсолютные координаты блока
#[derive(Default, Copy, Clone, Deref, DerefMut)]
pub struct AbsoluteBlockPos {
    pos: IVec3,
}

impl From<IVec3> for AbsoluteBlockPos {
    fn from(value: IVec3) -> Self {
        AbsoluteBlockPos { pos: value }
    }
}

impl Into<IVec3> for AbsoluteBlockPos {
    fn into(self) -> IVec3 {
        self.pos
    }
}
