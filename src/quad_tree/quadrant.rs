use super::Coordinate;

#[derive(Debug, Copy, Clone)]
pub struct Quadrant {
  x: f32,
  y: f32,
  length: f32,
}

impl Quadrant {
  pub fn new(x: f32, y: f32, length: f32) -> Self {
    Quadrant {
      x,
      y,
      length,
    }
  }

  pub fn get_x(&self) -> f32 {
    self.x
  }

  pub fn get_y(&self) -> f32 {
    self.y
  }

  pub fn get_length(&self) -> f32 {
    self.length
  }

  // Quadrant order: (ne, nw, se, sw)
  pub fn get_child_quadrants(&self) -> (Quadrant, Quadrant, Quadrant, Quadrant) {
    let child_length = self.length / 2 as f32;
    (
      Quadrant {
        x: self.x + child_length,
        y: self.y + child_length,
        length: child_length,
      },
      Quadrant {
        x: self.x,
        y: self.y + child_length,
        length: child_length,
      },
      Quadrant {
        x: self.x + child_length,
        y: self.y,
        length: child_length,
      },
      Quadrant {
        x: self.x,
        y: self.y,
        length: child_length,
      }
    )
  }

  pub fn contains(&self, coordinate: &Coordinate) -> bool {
    let x = coordinate.get_x();
    let y = coordinate.get_y();
    let x_start = self.x;
    let x_end = self.x + self.length;
    let y_start = self.y;
    let y_end = self.y + self.length;
    let x_contains = x >= x_start && x <= x_end;
    let y_contains = y >= y_start && y <= y_end;

    x_contains && y_contains
  }
}

#[cfg(test)]
mod tests {
  use super::{Coordinate, Quadrant};
  #[test]
  fn test_contains() {
    let quadrant = Quadrant::new(0.0, 500.0, 500.0);
    let coordinate = Coordinate::new(200.0, 150.0);
    assert_eq!(quadrant.contains(&coordinate), true);
  }

  #[test]
  fn test_contains_x_boundary() {
    let quadrant = Quadrant::new(0.0, 500.0, 500.0);
    let coordinate = Coordinate::new(500.0, 150.0);
    assert_eq!(quadrant.contains(&coordinate), true);
  }

  #[test]
  fn test_contains_y_boundary() {
    let quadrant = Quadrant::new(0.0, 500.0, 500.0);
    let coordinate = Coordinate::new(200.0, 500.0);
    assert_eq!(quadrant.contains(&coordinate), true);
  }

  #[test]
  fn test_not_contains() {
    let quadrant = Quadrant::new(0.0, 500.0, 500.0);
    let coordinate = Coordinate::new(600.0, 200.0);
    assert_eq!(quadrant.contains(&coordinate), false);
  }

  #[test]
  fn test_get_child_quadrants() {
    let quadrant = Quadrant::new(0.0, 500.0, 500.0);
    let (ne, nw, se, sw) = quadrant.get_child_quadrants();

    assert_eq!(ne.x, 250.0);
    assert_eq!(ne.y, 250.0);
    assert_eq!(ne.length, 250.0);

    assert_eq!(nw.x, 0.0);
    assert_eq!(nw.y, 250.0);
    assert_eq!(nw.length, 250.0);

    assert_eq!(se.x, 250.0);
    assert_eq!(se.y, 500.0);
    assert_eq!(se.length, 250.0);

    assert_eq!(sw.x, 0.0);
    assert_eq!(sw.y, 500.0);
    assert_eq!(sw.length, 250.0);
  }

  #[test]
  fn test_get_child_quadrants_smaller() {
    let quadrant = Quadrant::new(0.0, 125.0, 125.0);
    let (ne, nw, se, sw) = quadrant.get_child_quadrants();

    assert_eq!(ne.x, 62.5);
    assert_eq!(ne.y, 62.5);
    assert_eq!(ne.length, 62.5);

    assert_eq!(nw.x, 0.0);
    assert_eq!(nw.y, 62.5);
    assert_eq!(nw.length, 62.5);

    assert_eq!(se.x, 62.5);
    assert_eq!(se.y, 125.0);
    assert_eq!(se.length, 62.5);

    assert_eq!(sw.x, 0.0);
    assert_eq!(sw.y, 125.0);
    assert_eq!(sw.length, 62.5);
  }
}