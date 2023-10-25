// Point Region Quadtree

use crate::components::{math::Size2i, visuals::Point};


#[derive(Debug, Clone, Copy)]
pub struct Region {
    pub top_left: Point,
    pub size: Size2i,
}

pub struct RegionSubset {
    pub ne: Region,
    pub nw: Region,
    pub se: Region,
    pub sw: Region,
}

impl RegionSubset {
    /*
    pub fn array(&self) -> [Region; 4] {
        [self.nw, self.ne, self.sw, self.se]
    }
    */
}

impl From<Region> for PRQuadtree {
    fn from(region: Region) -> Self {
        PRQuadtree::new(region.top_left, region.size, 4)
    }
}

// Helper method to generate a region subset from a region
impl From<Region> for RegionSubset {
    fn from(region: Region) -> Self {
        let size = region.size / 2;
        RegionSubset {
            nw: Region {
                top_left: Point::new(region.top_left.x, region.top_left.y),
                size
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
    pub se: Option<Box<PRQuadtree>>,
    pub sw: Option<Box<PRQuadtree>>,
}

impl PRQuadtree {
    pub fn new(top_left: Point, size: Size2i, capacity: usize) -> PRQuadtree {
        PRQuadtree {
            points: Vec::with_capacity(capacity),
            capacity,
            region: Region { top_left, size },
            ne: None,
            nw: None,
            se: None,
            sw: None,
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
        todo!()
    }

    pub fn subdivide(&mut self) {
        if self.points.len() != self.capacity {
            panic!(
                "Can't subdivide, not enough points ({}/{}).",
                self.points.len(),
                self.capacity
            );
        }

       todo!()
    }

    fn redistribute(&mut self) {
        if !self.is_leaf() {
            self.subdivide()
        }
    }


}
        // Filter out empty subregions


