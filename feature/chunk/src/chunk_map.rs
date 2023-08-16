use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use bevy_derive::{Deref, DerefMut};
use crate::{Chunk, ChunkPos};

/// Карта с чанками
#[derive(Default, Deref, DerefMut)]
pub struct ChunkMap<BLOCK, METADATA> {
    chunks: HashMap<ChunkPos, Arc<RwLock<Chunk<BLOCK, METADATA>>>>,
}
