use core::*;

/// A player
pub struct Player {
  coord: Vec2<i64>,
  dir: Dir,
}


impl Positioned for Player {
  fn get_coord(&self) -> i64 {
    self.coord
  }
  fn set_coord(&mut self, coord: i64) {
    self.coord = coord;
  }
}


impl Movable for Player {
  fn get_dir(&self) -> Dir {
    self.dir
  }
  fn set_dir(&mut self, new_dir: Dir) {
    self.dir = new_dir;
  }
}
