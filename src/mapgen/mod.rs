//! The module for the automatic map generation

use noise::{Brownian2, Seed, open_simplex2};

// TODO pick:
// use core::*;
// use core::{Map, Tile, TileMap};
use core::{Block, Tile, TileMap};

// Probably really buggy code...

/// A map
///
/// Note: It's chunked into big chunks and small chunks.
///       Small chunks keeps the noise data. Big chunks
///       determines the overlay layer.
pub struct MapGenerator<'a> {
  seed: &'a Seed,
}

/// Types of big chunks
enum BChunkType {
  /// Reserved for history, empty per default, but gets opened as the game is played. Manually designed.
  History,
  /// Automatic map generated chunk
  Auto,
  /// Manually random generated chunk
  Manually,
  /// Empty chunck
  Empty,
}

// TODO: Find out how to prevent double islands (manually generated islands)

impl<'a> MapGenerator<'a> {
  /// Creates a new map
  pub fn new(seed: &'a Seed) -> MapGenerator<'a> {
    MapGenerator {
      seed: seed,
    }
  }

  // TODO: Add some sort of cache

  /// Get the noise value at a given point
  fn get_noise_value(&self, x: i64, y: i64) -> f64 {
    let noise = Brownian2::new(open_simplex2, 4).wavelength(32.0);
    noise.apply(&self.seed, &[x as f64, y as f64])
  }

  /// Get big chunk coordinates
  fn get_bchunk(x: i64, y: i64) -> (i64, i64) {
    // TODO: Chunk size 64?
    (((x as f64) / 64.0).floor() as i64, ((y as f64) / 64.0).floor() as i64)
  }

  /// Get the chunk type of the big chunk at (x, y)
  fn get_bchunk_type(&self, x: i64, y: i64) -> BChunkType {
    let noise = Brownian2::new(open_simplex2, 4).wavelength(32.0);
    let noise_val = noise.apply(&self.seed, &[x as f64, y as f64]);
    if noise_val > 0.3 {
      BChunkType::Empty // Too many too few? 
    } else if noise_val > 0.1 {
      BChunkType::Auto
    } else if noise_val > 0.05 {
      BChunkType::History
    } else {
      BChunkType::Manually
    }
  }

  /// Get overlay value (used for customizing the noise)
  fn get_overlay_value(&self, x: i64, y: i64) -> f64 {
    // TODO
    // Stuff here
    1.0
  }
}

impl<'a> TileMap for MapGenerator<'a> {

  // Add foreground/background
  /// Get the tile at a given point
  fn get_tile(&self, x: i64, y: i64) -> Block {
    let val = (self.get_noise_value(x, y) + self.get_overlay_value(x, y)) / 2.0;

    // Temporary map gen. Proof of concept.
    if val > 0.9 {
      Block::new(Tile::Water, Tile::Grass, Tile::Tree)
    } else {
      Block::new(Tile::Water, Tile::None, Tile::None)
    }
  }
}
