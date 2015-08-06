//! Traits, structs, and other objects for the game

mod angle;
mod config;
mod game_view;
pub mod view;
mod map;
pub mod util;
mod vec2;
pub mod cam;

pub use self::angle::Angle;
pub use self::config::Config;
pub use self::game_view::GameView;
pub use self::view::View;
pub use self::map::{Block, Map, Tile, TileMap};
pub use self::vec2::Vec2;
pub use opengl_graphics::*;

// TODO: Rename coord to pos

/// A trait for positioned objects
pub trait Positioned {
  /// Get the x coordinate
  fn get_pos(&self) -> Vec2<i64>;
  /// Set the Vec coordinate
  fn set_pos(&mut self, new_pos: Vec2<i64>);
}

/// The direction of a given object
#[derive(Clone, Copy)]
pub enum Dir {
  Left,
  Right,
  Up,
  Down,
}
impl Dir {
  fn to_vec(&self) -> Vec2<i64> {
    Vec2(
      match *self {
        Dir::Left => 1,
        Dir::Right => -1,
        _ => 0,
      },

      match *self {
        Dir::Up => 1,
        Dir::Down => -1,
      _ => 0,
      }
    )
  }
}

// TODO: Implement two directions at once.
// TODO: Make drawable trait

/// A movable object
pub trait Movable: Positioned {
  /// Get the direction
  fn get_dir(&self) -> Dir;
  /// Set the direction
  fn set_dir(&mut self, new_dir: Dir);
  /// Is the object moving?
  fn is_moving(&self) -> bool;
  /// Move the object
  fn move_obj(&mut self, mov: Vec2<i64>) {
    let coord = self.get_pos();
    self.set_pos(coord + mov);
  }
  /// Get new coordinate
  fn get_new_pos(&self) -> Vec2<i64> {
    let dir = self.get_dir();
    self.get_pos() + dir.to_vec()
  }
  /// Move object in direction.
  fn move_obj_dir(&mut self) {
    let new_coord = self.get_new_pos();
    self.set_pos(new_coord)
  }
}

/// Trait for animated objects
pub trait Animated: Movable {
  /// Get transitition point, which is in the interval [0,1]
  fn get_trans_state(&self) -> f64;
  /// Get animation frame
  fn get_animation_frame(&self) -> i16;
}

/// Trait for sprited objects
pub trait Sprited: Animated {
  /// Get current sprite
  fn get_sprite(&self) -> &Texture;
  /// Get width, not neccesarily the width of the sprite, but rather the space
  /// the given object occupies. Given in fields.
  fn get_width(&self) -> i16;
  /// Get height, see note above
  fn get_height(&self) -> i16;
  /// Get the opacity of the object
  fn get_opacity(&self) -> f64;

  // TODO: Add draw method.
}

// TODO: Add event trait, for objects you can click on etc.

/// Entity ID type
pub struct Id(pub i64);

/// An entity
pub trait Entity: Sprited {
  /// Get the ID of the given entity
  fn id(&self) -> Id;
  /// Is the entity solid at point (x, y) relative to the position?
  fn is_solid(&self, x: i16, y: i16) -> bool;
  /// Update the entity
  fn update(&mut self, dt: f64);
  // Probably need more methods.
}
