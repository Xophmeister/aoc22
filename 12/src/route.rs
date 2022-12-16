use std::collections::VecDeque;

use crate::error::Error;
use crate::map::{Coord, Map};

struct LocationNode<'a> {
    parent: Option<&'a LocationNode<'a>>,
    coord: Coord,
}

// FIXME
impl Iterator for LocationNode<'_> {
    type Item<'a> = LocationNode<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.parent
    }
}

pub struct Route(Vec<Coord>);

impl TryFrom<&Map> for Route {
    type Error = Error;

    fn try_from(map: &Map) -> Result<Self, Self::Error> {
        let width = map.size().x as isize;
        let height = map.size().y as isize;

        // Maintain a tree of visited locations
        let mut tree: Vec<LocationNode> = vec![LocationNode {
            parent: None,
            coord: map.start,
        }];

        // Maintain a queue of locations from which to explore, starting with the root
        let mut to_explore: VecDeque<&LocationNode> = VecDeque::from([&tree[0]]);

        // Breadth-first search that builds up the tree until it reaches the finish
        while let Some(location) = to_explore.pop_front() {
            if location.coord == map.finish {
                // Iterate up to the root (i.e., start) and reverse to obtain the route
                let route = Vec::from_iter(*location.iter().rev().map(|location| location.coord));
                return Ok(Route(route));
            }

            // Find assailable locations (besides where we just came from) and add enqueue
            let previous = location.parent;
            for delta_x in [-1, 1] {
                for delta_y in [-1, 1] {
                    // Short-circuit the loop if potential locations fall off the edges of the map
                    let new_x = location.coord.x as isize + delta_x;
                    let new_y = location.coord.y as isize + delta_y;
                    if new_x < 0 || new_x >= width || new_y < 0 || new_y >= height {
                        continue;
                    }

                    let potential = Coord {
                        x: new_x as usize,
                        y: new_y as usize,
                    };

                    // Short-circuit the loop if we've just come from a potential location
                    if previous.is_some() && previous.unwrap().coord == potential {
                        continue;
                    }

                    // Add assailable locations to the tree and enqueue to explore
                    if (map.elevation(location.coord) - map.elevation(potential)).assailable() {
                        // FIXME
                        tree.push(LocationNode {
                            parent: Some(location),
                            coord: potential,
                        });

                        to_explore.push_back(tree.last().unwrap())
                    }
                }
            }
        }

        // No route found
        Err(Error::NoRoute(map.start, map.finish))
    }
}

impl Route {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
