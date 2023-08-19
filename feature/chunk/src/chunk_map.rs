use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use bevy_derive::{Deref, DerefMut};
use crate::{Chunk, ChunkPos};

/// Карта с чанками
pub type ChunkMap<BLOCK, METADATA> = Arc<RwLock<ChunkMapInternal<BLOCK, METADATA>>>;

#[derive(Default, Deref, DerefMut)]
pub struct ChunkMapInternal<BLOCK, METADATA> {
    chunks: HashMap<ChunkPos, Arc<RwLock<Chunk<BLOCK, METADATA>>>>,
}
