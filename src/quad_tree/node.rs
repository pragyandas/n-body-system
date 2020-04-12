use super::Body;
use super::Coordinate;
use super::Quadrant;
use super::Vector;

#[derive(Debug)]
pub struct Node {
  body: Option<Body>,
  children: Option<Children>,
  quadrant: Quadrant,
}

impl Node {
  pub fn new(quadrant: Quadrant) -> Self {
    Node {
      body: None,
      quadrant,
      children: None,
    }
  }

  pub fn reset(&mut self) {
    self.body = None;
    self.children = None;
  }

  pub fn get_body(&self) -> &Option<Body> {
    &self.body
  }

  pub fn get_quadrant(&self) -> &Quadrant {
    &self.quadrant
  }
  pub fn get_children(&self) -> &Option<Children> {
    &self.children
  }

  pub fn insert(&mut self, body: Body) {
    match self.body {
      Some(existing_body) => {
        if self.children.is_none() {
          self.init_children();
          self.children.as_mut().unwrap().insert(existing_body);
        }

        self.children.as_mut().unwrap().insert(body);
        self.body = Some(existing_body.add_body(&body));
      }
      None => {
        self.body = Some(body);
      }
    }
  }

  pub fn merge(&mut self, nodes: &mut Vec<Box<Self>>) {
    let bodies: Vec<Option<Body>> = nodes.iter().map(|node| node.body).collect();
    let mut nodes_drain = nodes.drain(..);
    self.children = Some(Children {
      ne: nodes_drain.next().unwrap(),
      nw: nodes_drain.next().unwrap(),
      se: nodes_drain.next().unwrap(),
      sw: nodes_drain.next().unwrap(),
    });

    self.merge_set_body(bodies);
  }

  fn merge_set_body(&mut self, bodies: Vec<Option<Body>>) {
    let default_body = Body::new(0.0, Coordinate::new(0.0, 0.0));
    let mass = bodies.iter().fold(0.0, |acc, body| {
      acc + body.unwrap_or(default_body).get_mass()
    });
    let x = bodies.iter().fold(0.0, |acc, body| {
      acc
        + body.unwrap_or(default_body).get_coordinate().get_x()
          * body.unwrap_or(default_body).get_mass()
    }) / mass;
    let y = bodies.iter().fold(0.0, |acc, body| {
      acc
        + body.unwrap_or(default_body).get_coordinate().get_y()
          * body.unwrap_or(default_body).get_mass()
    }) / mass;
    let coordinate = Coordinate::new(x, y);

    self.body = Some(Body::new(mass, coordinate));
  }

  fn init_children(&mut self) {
    let (ne_quad, nw_quad, se_quad, sw_quad) = self.quadrant.get_child_quadrants();
    self.children = Some(Children {
      ne: Box::new(Node::new(ne_quad)),
      nw: Box::new(Node::new(nw_quad)),
      se: Box::new(Node::new(se_quad)),
      sw: Box::new(Node::new(sw_quad)),
    });
  }

  pub fn calculate_net_force_on(&self, body: &Body, theta: f32) -> Vector {
    match self.body {
      Some(self_body) => {
        if self_body.get_id() == body.get_id() {
          return Vector::new(0.0, 0.0);
        }

        let distance = self_body.distance_between(body);
        let side_length = self.quadrant.get_length();
        if side_length / distance < theta {
          return self_body.calculate_force_on(body);
        } else {
          return match &self.children {
            Some(children) => {
              let Children { nw, ne, sw, se } = children;
              let net_force = nw.calculate_net_force_on(body, theta)
                + ne.calculate_net_force_on(body, theta)
                + sw.calculate_net_force_on(body, theta)
                + se.calculate_net_force_on(body, theta);

              return net_force;
            }
            None => self_body.calculate_force_on(body),
          };
        }
      }
      None => Vector::new(0.0, 0.0),
    }
  }
}

#[derive(Debug)]
pub struct Children {
  pub nw: Box<Node>,
  pub ne: Box<Node>,
  pub sw: Box<Node>,
  pub se: Box<Node>,
}

impl Children {
  pub fn insert(&mut self, body: Body) {
    let child_node = self.get_containing_child_node(&body.get_coordinate());
    match child_node {
      Some(child_node) => child_node.insert(body),
      None => panic!("body doesn't belong to any quadrant"),
    }
  }

  pub fn get_containing_child_node(&mut self, coordinate: &Coordinate) -> Option<&mut Box<Node>> {
    let Children { nw, ne, sw, se } = self;
    let containing_child_node = vec![nw, ne, sw, se]
      .into_iter()
      .find(|child| child.quadrant.contains(coordinate));

    return containing_child_node;
  }
}

#[cfg(test)]
mod tests {
  use super::{Body, Coordinate, Node, Quadrant};

  #[test]
  fn insert_on_node_with_no_body() {
    let mut node = Node::new(Quadrant::new(0.0, 1000.0, 1000.0));
    let body = Body::new(10.0, Coordinate::new(400.0, 600.0));
    node.insert(body);

    assert_eq!(node.body.unwrap().get_mass(), 10.0);
    assert_eq!(node.body.unwrap().get_coordinate().get_x(), 400.0);
    assert_eq!(node.body.unwrap().get_coordinate().get_y(), 600.0);
  }

  #[test]
  fn insert_on_node_single_level() {
    let mut node = Node::new(Quadrant::new(0.0, 1000.0, 1000.0));
    let body_1 = Body::new(10.0, Coordinate::new(400.0, 600.0));
    let body_2 = Body::new(10.0, Coordinate::new(900.0, 600.0));
    node.insert(body_1);
    node.insert(body_2);

    assert_eq!(node.body.unwrap().get_mass(), 20.0);
    assert_eq!(node.body.unwrap().get_coordinate().get_x(), 650.0);
    assert_eq!(node.body.unwrap().get_coordinate().get_y(), 600.0);

    let children = node.children.unwrap();
    let sw_body = children.sw.body;
    let se_body = children.se.body;
    let nw_body = children.nw.body;
    let ne_body = children.ne.body;

    assert_eq!(sw_body.unwrap().get_mass(), 10.0);
    assert_eq!(sw_body.unwrap().get_coordinate().get_x(), 400.0);
    assert_eq!(sw_body.unwrap().get_coordinate().get_y(), 600.0);

    assert_eq!(se_body.unwrap().get_mass(), 10.0);
    assert_eq!(se_body.unwrap().get_coordinate().get_x(), 900.0);
    assert_eq!(se_body.unwrap().get_coordinate().get_y(), 600.0);

    assert!(nw_body.is_none());
    assert!(ne_body.is_none());
  }

  #[test]
  fn insert_on_node_multiple_level() {
    let mut node = Node::new(Quadrant::new(0.0, 1000.0, 1000.0));
    let body_1 = Body::new(10.0, Coordinate::new(200.0, 200.0));
    let body_2 = Body::new(10.0, Coordinate::new(270.0, 200.0));
    node.insert(body_1);
    node.insert(body_2);

    assert_eq!(node.body.unwrap().get_mass(), 20.0);
    assert_eq!(node.body.unwrap().get_coordinate().get_x(), 235.0);
    assert_eq!(node.body.unwrap().get_coordinate().get_y(), 200.0);

    let children = node.children.unwrap();

    assert!(children.ne.body.is_none());
    assert!(children.se.body.is_none());
    assert!(children.sw.body.is_none());

    let nw_body = children.nw.body;
    assert_eq!(nw_body.unwrap().get_mass(), 20.0);
    assert_eq!(nw_body.unwrap().get_coordinate().get_x(), 235.0);
    assert_eq!(nw_body.unwrap().get_coordinate().get_y(), 200.0);

    let nw_children = children.nw.children.unwrap();
    assert!(nw_children.nw.body.is_some());
    assert!(nw_children.ne.body.is_some());

    assert_eq!(nw_children.nw.body.unwrap().get_mass(), 10.0);
    assert_eq!(nw_children.nw.body.unwrap().get_coordinate().get_x(), 200.0);
    assert_eq!(nw_children.nw.body.unwrap().get_coordinate().get_y(), 200.0);

    assert_eq!(nw_children.ne.body.unwrap().get_mass(), 10.0);
    assert_eq!(nw_children.ne.body.unwrap().get_coordinate().get_x(), 270.0);
    assert_eq!(nw_children.ne.body.unwrap().get_coordinate().get_y(), 200.0);
  }
}
