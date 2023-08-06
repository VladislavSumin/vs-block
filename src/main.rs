mod camera;
mod key_binding;
mod render;

use bevy::prelude::*;
use crate::camera::CameraPlugin;
use crate::key_binding::KeyBindingsPlugin;
use crate::render::{MeshBuilder, MeshPart};


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(KeyBindingsPlugin)
        .add_plugins(CameraPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Дефолтный пример отсюда https://bevyengine.org/examples/3D%20Rendering/3d-scene/
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // сторона куба, это почти целый куб, только нужно еще 5 сторон
    commands.spawn(PbrBundle {
        mesh: meshes.add(create_block_mesh()),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
}

fn create_block_mesh() -> Mesh {
    let mut builder = MeshBuilder::new();
    builder.add_mesh_data(BlockFaceMeshData::from(AbsoluteBlockFaceDirection::PosY));
    builder.add_mesh_data(BlockFaceMeshData::from(AbsoluteBlockFaceDirection::PosX));
    builder.build()
}

/// Координаты блока относительно чанка
#[derive(Default)]
struct ChunkBlockCoord {
    x: u8,
    y: u8,
    z: u8,
}

/// Направление стороны блока в абсолютных координатах
enum AbsoluteBlockFaceDirection {
    /// Позитивное направление оси X, или правая сторона
    PosX,

    /// Негативное направление оси X, или левая сторона
    NegX,

    /// Позитивное направление оси Y, или верхняя сторона
    PosY,

    /// Негативное направление оси Y, или нижняя сторона
    NegY,

    /// Позитивное направление оси Z, или передняя сторона
    PosZ,

    /// Негативное направление оси Z, или задняя сторона
    NegZ,
}

impl AbsoluteBlockFaceDirection {
    /// Возвращает массив позиций для текущего [AbsoluteBlockFaceDirection]
    /// Для описания стороны блока нужно 4 вершины каждая из которых описывается 3мя координатами
    fn get_vertex_positions(&self) -> &'static [[f32; 3]; 4] {
        match self {
            AbsoluteBlockFaceDirection::PosX => {
                &[
                    [1., 0., 0.],
                    [1., 1., 0.],
                    [1., 1., 1.],
                    [1., 0., 1.],
                ]
            }
            AbsoluteBlockFaceDirection::NegX => {
                &[
                    [0., 0., 0.],
                    [0., 1., 0.],
                    [0., 1., 1.],
                    [0., 0., 1.],
                ]
            }
            AbsoluteBlockFaceDirection::PosY => {
                &[
                    [1., 1., 0.],
                    [0., 1., 0.],
                    [0., 1., 1.],
                    [1., 1., 1.],
                ]
            }
            AbsoluteBlockFaceDirection::NegY => {
                &[
                    [1., 0., 0.],
                    [0., 0., 0.],
                    [0., 0., 1.],
                    [1., 0., 1.],
                ]
            }
            AbsoluteBlockFaceDirection::PosZ => {
                &[
                    [0., 0., 1.],
                    [1., 0., 1.],
                    [1., 1., 1.],
                    [0., 1., 1.],
                ]
            }
            AbsoluteBlockFaceDirection::NegZ => {
                &[
                    [0., 0., 0.],
                    [1., 0., 0.],
                    [1., 1., 0.],
                    [0., 1., 0.],
                ]
            }
        }
    }

    /// Возвращает нормалей позиций для текущего [AbsoluteBlockFaceDirection]
    /// Количество нормалей должно соответствовать количеству вершин у вертекса,
    /// у каждой вершины своя нормаль, см [Self::get_vertex_positions].
    /// Для описания нормали нужны 3 координаты
    fn get_normals(&self) -> &'static [[f32; 3]; 4] {
        match self {
            AbsoluteBlockFaceDirection::PosX => {
                &[
                    [1., 0., 0.],
                    [1., 0., 0.],
                    [1., 0., 0.],
                    [1., 0., 0.],
                ]
            }
            AbsoluteBlockFaceDirection::NegX => {
                &[
                    [-1., 0., 0.],
                    [-1., 0., 0.],
                    [-1., 0., 0.],
                    [-1., 0., 0.],
                ]
            }
            AbsoluteBlockFaceDirection::PosY => {
                &[
                    [0., 1., 0.],
                    [0., 1., 0.],
                    [0., 1., 0.],
                    [0., 1., 0.],
                ]
            }
            AbsoluteBlockFaceDirection::NegY => {
                &[
                    [0., -1., 0.],
                    [0., -1., 0.],
                    [0., -1., 0.],
                    [0., -1., 0.],
                ]
            }
            AbsoluteBlockFaceDirection::PosZ => {
                &[
                    [0., 0., 1.],
                    [0., 0., 1.],
                    [0., 0., 1.],
                    [0., 0., 1.],
                ]
            }
            AbsoluteBlockFaceDirection::NegZ => {
                &[
                    [0., 0., -1.],
                    [0., 0., -1.],
                    [0., 0., -1.],
                    [0., 0., -1.],
                ]
            }
        }
    }

    /// Возвращает массив позиций текстуры для текущего [AbsoluteBlockFaceDirection]
    /// Количество позиций текстуры должно соответствовать количеству вершин у вертекса,
    /// у каждой вершины своя позиция текстуры, см [Self::get_vertex_positions]
    /// Для описания позиции текстуры нужны 2 координаты
    fn get_uvs(&self) -> &'static [[f32; 2]; 4] {
        match self {
            AbsoluteBlockFaceDirection::PosX => {
                &[
                    [0., 0.],
                    [1., 0.],
                    [1., 1.],
                    [0., 1.],
                ]
            }
            AbsoluteBlockFaceDirection::NegX => {
                &[
                    [1., 0.],
                    [0., 0.],
                    [0., 1.],
                    [1., 1.],
                ]
            }
            AbsoluteBlockFaceDirection::PosY => {
                &[
                    [1., 0.],
                    [0., 0.],
                    [0., 1.],
                    [1., 1.],
                ]
            }
            AbsoluteBlockFaceDirection::NegY => {
                &[
                    [0., 0.],
                    [1., 0.],
                    [1., 1.],
                    [0., 1.],
                ]
            }
            AbsoluteBlockFaceDirection::PosZ => {
                &[
                    [0., 0.],
                    [1., 0.],
                    [1., 1.],
                    [0., 1.],
                ]
            }
            AbsoluteBlockFaceDirection::NegZ => {
                &[
                    [1., 0.],
                    [0., 0.],
                    [0., 1.],
                    [1., 1.],
                ]
            }
        }
    }

    /// Индексы описывают в какой в последовательности замыкать вершины в треугольники
    /// От направления обхода зависит в какую сторону будет смотреть плоскость треугольника, с
    /// другой стороны треугольник не будет виден
    fn get_indexes() -> &'static [u32; 6] {
        &[0, 1, 2, 2, 3, 0]
    }
}

/// Информация необходимая для построения одной стороны блока
struct BlockFaceMeshData {
    indexes: &'static [u32; 6],
    positions: &'static [[f32; 3]; 4],
    normals: &'static [[f32; 3]; 4],
    uvs: &'static [[f32; 2]; 4],
}

impl From<AbsoluteBlockFaceDirection> for BlockFaceMeshData {
    fn from(value: AbsoluteBlockFaceDirection) -> Self {
        BlockFaceMeshData {
            indexes: AbsoluteBlockFaceDirection::get_indexes(),
            positions: value.get_vertex_positions(),
            normals: value.get_normals(),
            uvs: value.get_uvs(),
        }
    }
}

impl MeshPart for BlockFaceMeshData {
    fn get_indexes(&self) -> &[u32] {
        self.indexes
    }

    fn get_positions(&self) -> &[[f32; 3]] {
        self.positions
    }

    fn get_normals(&self) -> &[[f32; 3]] {
        self.normals
    }

    fn get_uvs(&self) -> &[[f32; 2]] {
        self.uvs
    }
}