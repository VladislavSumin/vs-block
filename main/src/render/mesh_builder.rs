use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};

/// Часть меша, используется в [MeshBuilder]
pub trait MeshPart {
    fn get_indexes(&self) -> &[u32];
    fn get_positions(&self) -> &[[f32; 3]];
    fn get_normals(&self) -> &[[f32; 3]];
    fn get_uvs(&self) -> &[[f32; 2]];
}

/// Mesh builder для преобразования набора [MeshPart] в [Mesh]
#[derive(Default)]
pub struct MeshBuilder {
    /// Индексы вершин вертекса
    indexes: Vec<u32>,

    /// Вершины вертекса
    positions: Vec<[f32; 3]>,

    /// Нормали к вершинам вертекса
    normals: Vec<[f32; 3]>,

    /// Координаты текстуры вершин вертекса
    uvs: Vec<[f32; 2]>,

    /// Сдвиг
    /// Этот сдвиг будет применен ко всем [MeshPart::get_positions] при добавлении в [Self]
    transition: Vec3,
}

impl MeshBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_transition(&mut self, transition: Vec3) {
        self.transition = transition;
    }

    pub fn add_mesh_data<T: MeshPart>(&mut self, mesh_data: T) {
        let positions_len = self.positions.len() as u32;
        self.indexes.extend(mesh_data.get_indexes().into_iter().map(|i| { i + positions_len }));

        let positions = mesh_data.get_positions();
        let normals = mesh_data.get_normals();
        let uvs = mesh_data.get_uvs();

        if positions.len() != normals.len() || positions.len() != uvs.len() {
            panic!(
                "Incorrect vectors length, positions={}, normals={}, uvs={}",
                positions.len(),
                normals.len(),
                uvs.len(),
            );
        }
        let translated_positions = positions
            .into_iter()
            .map(|pos| {
                [pos[0] + self.transition.x, pos[1] + self.transition.y, pos[2] + self.transition.z, ]
            });
        self.positions.extend(translated_positions);
        self.normals.extend(normals);
        self.uvs.extend(uvs);
    }

    pub fn build(self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs);
        mesh.set_indices(Some(Indices::U32(self.indexes)));
        mesh
    }
}