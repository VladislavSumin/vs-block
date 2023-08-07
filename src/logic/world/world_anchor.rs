use bevy::prelude::Component;

/// Маркерный интерфейс, для маркировки сущностей вокруг которых должен грузиться мир
/// Такие сущности обязательно должны так же включать элемент [Transform]
#[derive(Component)]
pub struct WorldAnchor {
    /// Радиус в пределах которого будет загружаться мир
    pub load_radius: u32,
}

impl Default for WorldAnchor {
    fn default() -> Self {
        WorldAnchor {
            load_radius: 3
        }
    }
}
