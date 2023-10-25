// Point Region Quadtree

use crate::components::{math::Size2i, visuals::Point};


#[derive(Debug, Clone, Copy)]
pub struct Region {
    pub top_left: Point,
    pub size: Size2i,
}

pub struct RegionSubset {
    pub nw: Region,
    pub ne: Region,
    pub sw: Region,
    pub se: Region,
}

impl RegionSubset {
    /*
    pub fn array(&self) -> [Region; 4] {
        [self.nw, self.ne, self.sw, self.se]
    }
    */
}

// Helper method to generate a region subset from a region
impl From<Region> for RegionSubset {
    fn from(region: Region) -> Self {
        let size = region.size / 2;
        RegionSubset {
            nw: Region {
                top_left: Point::new(region.top_left.x, region.top_left.y),
                size: size
            },
            ne: Region {
                top_left: Point::new(region.top_left.x + region.size.width / 2, region.top_left.y),
                size
            },
            sw: Region {
                top_left: Point::new(region.top_left.x, region.top_left.y + region.size.height / 2),
                size
            },
            se: Region {
                top_left: Point::new(region.top_left.x + region.size.width / 2, region.top_left.y + region.size.height / 2),
                size
            },
        }
    }
}

impl Region {
    pub fn contains(&self, point: &Point) -> bool {
        point.x >= self.top_left.x
            && point.y >= self.top_left.y
            && point.x <= self.top_left.x + self.size.width
            && point.y <= self.top_left.y + self.size.height
    }
    // Returns
}

struct PRNode {
    pub point: Point,
}

struct PRQuadtree {
    pub points: Vec<PRNode>,
    pub region: Region,
    pub capacity: usize,
    pub ne: Option<Box<PRQuadtree>>,
    pub nw: Option<Box<PRQuadtree>>,
    pub sw: Option<Box<PRQuadtree>>,
    pub se: Option<Box<PRQuadtree>>,
}

impl PRQuadtree {
    pub fn new(top_left: Point, size: Size2i, capacity: usize) -> PRQuadtree {
        PRQuadtree {
            points: Vec::with_capacity(capacity),
            capacity,
            region: Region { top_left, size },
            ne: None,
            nw: None,
            sw: None,
            se: None,
        }
    }

    pub fn insert(&mut self, node: PRNode) {
        // Check if node is within boundaries

        if !self.region.contains(&node.point) {
            return;
        }

        if self.points.len() < self.capacity {
            self.points.push(node);
            return;
        }

        // Node is full, subdivide it and redistribute
        // the values in the subregions

        self.subdivide();
        // Unwrap the subregions
        self.insert(node);
    }



    pub fn is_leaf(&self) -> bool {
        self.ne.is_none() && self.nw.is_none() && self.sw.is_none() && self.se.is_none()
    }

    pub fn subdivide(&mut self) {
        if self.points.len() != self.capacity {
            panic!(
                "Can't subdivide, not enough points ({}/{}).",
                self.points.len(),
                self.capacity
            );
        }
        // Create temporary subregions, then add the non-empty ones
        let subregions = RegionSubset::from(self.region);
        let mut temp_ne = PRQuadtree::new(
            subregions.ne.top_left,
            subregions.ne.size,
            self.capacity,
        );
        let mut temp_nw = PRQuadtree::new(
            subregions.nw.top_left,
            subregions.nw.size,
            self.capacity,
        );
        let mut temp_sw = PRQuadtree::new(
            subregions.sw.top_left,
            subregions.sw.size,
            self.capacity,
        );
        let mut temp_se = PRQuadtree::new(
            subregions.se.top_left,
            subregions.se.size,
            self.capacity,
        );

        // Redistribute the points
        for node in self.points.drain(..) {
            if temp_ne.region.contains(&node.point) {
                temp_ne.insert(node);
            } else if temp_nw.region.contains(&node.point) {
                temp_nw.insert(node);
            } else if temp_sw.region.contains(&node.point) {
                temp_sw.insert(node);
            } else if temp_se.region.contains(&node.point) {
                temp_se.insert(node);
            } else {
                panic!("Point is not in any subregion.");
            }
        }

        // Add only the non-empty subregions

        self.ne = Some(Box::new(temp_ne));
        self.nw = Some(Box::new(temp_nw));
        self.sw = Some(Box::new(temp_sw));
        self.se = Some(Box::new(temp_se));

        // Clear the points
        self.points.clear();
    }
}
        // Filter out empty subregions


