use std::ops::{Deref, DerefMut};
use bevy_math::IVec3;

/// Абсолютные координаты блока
#[derive(Default, Copy, Clone)]
pub struct AbsoluteBlockPos {
    pos: IVec3,
}

impl Deref for AbsoluteBlockPos {
    type Target = IVec3;
    fn deref(&self) -> &Self::Target {
        &self.pos
    }
}

impl DerefMut for AbsoluteBlockPos {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pos
    }
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
