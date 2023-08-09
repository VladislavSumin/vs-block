use bevy::math::{IVec3, ivec3};
use strum_macros::EnumIter;
use crate::render::MeshPart;

/// Направление стороны блока в абсолютных координатах
#[derive(EnumIter, Clone, Copy)]
pub enum AbsoluteBlockFaceDirection {
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

impl Into<IVec3> for AbsoluteBlockFaceDirection {
    fn into(self) -> IVec3 {
        match self {
            AbsoluteBlockFaceDirection::PosX => { ivec3(1, 0, 0) }
            AbsoluteBlockFaceDirection::NegX => { ivec3(-1, 0, 0) }
            AbsoluteBlockFaceDirection::PosY => { ivec3(0, 1, 0) }
            AbsoluteBlockFaceDirection::NegY => { ivec3(0, -1, 0) }
            AbsoluteBlockFaceDirection::PosZ => { ivec3(0, 0, 1) }
            AbsoluteBlockFaceDirection::NegZ => { ivec3(0, 0, -1) }
        }
    }
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
                    [0., 0., 1.],
                    [0., 1., 1.],
                    [0., 1., 0.],
                    [0., 0., 0.],
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
                    [1., 0., 1.],
                    [0., 0., 1.],
                    [0., 0., 0.],
                    [1., 0., 0.],
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
                    [0., 1., 0.],
                    [1., 1., 0.],
                    [1., 0., 0.],
                    [0., 0., 0.],
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

impl MeshPart for AbsoluteBlockFaceDirection {
    fn get_indexes(&self) -> &[u32] {
        AbsoluteBlockFaceDirection::get_indexes()
    }

    fn get_positions(&self) -> &[[f32; 3]] {
        self.get_vertex_positions()
    }

    fn get_normals(&self) -> &[[f32; 3]] {
        self.get_normals()
    }

    fn get_uvs(&self) -> &[[f32; 2]] {
        self.get_uvs()
    }
}