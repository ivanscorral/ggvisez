use crate::components::{math::Size2i, visuals::Point};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    pub fn to_array(&self) -> [Region; 4] {
        [self.nw, self.ne, self.sw, self.se]
    }
}

impl From<Region> for Quadtree {
    fn from(region: Region) -> Self {
        Quadtree::new(region.top_left, region.size)
    }
}

impl From<Region> for RegionSubset {
    fn from(region: Region) -> Self {
        let half_size = Size2i::new(region.size.width / 2, region.size.height / 2);

        // Round up if region.size is odd
        let incremented_half_size = Size2i::new((region.size.width + 1) / 2, (region.size.height + 1) / 2);

        RegionSubset {
            nw: Region { top_left: region.top_left, size: half_size },
            ne: Region { top_left: Point::new(region.top_left.x + half_size.width as u32, region.top_left.y), size: incremented_half_size },
            sw: Region { top_left: Point::new(region.top_left.x, region.top_left.y + half_size.height as u32), size: incremented_half_size },
            se: Region { top_left: Point::new(region.top_left.x + half_size.width as u32, region.top_left.y + half_size.height as u32), size: half_size },
        }
    }
}

impl Region {
    pub fn contains(&self, point: &Point) -> bool {
        point.x >= self.top_left.x
            && point.y >= self.top_left.y
            && point.x < self.top_left.x + self.size.width as u32
            && point.y < self.top_left.y + self.size.height as u32
    }
    pub fn is_subset(&self, other: &Region) -> bool {
        if self == other {
            return true; // The regions are identical
        }

        let contains_top_left = self.contains(&other.top_left);

        let bottom_right = Point {
            x: other.top_left.x + other.size.width as u32 - 1,
            y: other.top_left.y + other.size.height as u32 - 1,
        };

        let contains_bottom_right = self.contains(&bottom_right);

        contains_top_left && contains_bottom_right
    }

    pub fn intersects(&self, other: &Region) -> bool {
        let self_bottom_right = Point {
            x: self.top_left.x + self.size.width as u32 - 1,
            y: self.top_left.y + self.size.height as u32 - 1,
        };

        let other_bottom_right = Point {
            x: other.top_left.x + other.size.width as u32 - 1,
            y: other.top_left.y + other.size.height as u32 - 1,
        };

        self.top_left.x <= other_bottom_right.x
            && self_bottom_right.x >= other.top_left.x
            && self.top_left.y <= other_bottom_right.y
            && self_bottom_right.y >= other.top_left.y
    }

}

pub struct Quadtree {
    pub points: Vec<Point>,
    pub region: Region,
    pub subregions: Vec<Quadtree>,
}

impl Quadtree {
    pub fn new(top_left: Point, size: Size2i) -> Quadtree {
        Quadtree {
            points: Vec::new(),
            region: Region { top_left, size },
            subregions: Vec::new(),
        }
    }

    pub fn query_region(&self, region: &Region) -> Vec<Point> {
        let mut found_points = Vec::new();

        // Check if the regions intersect, not if one is a subset of the other
        if !self.region.intersects(region) {
            return found_points;
        }

        for point in &self.points {
            if region.contains(point) {
                found_points.push(*point);
            }
        }

        for subregion in &self.subregions {
            found_points.extend(subregion.query_region(region));
        }

        found_points
    }

    pub fn insert_point(&mut self, point: Point) {

        if !self.region.contains(&point) {
            return;
        }

        if self.subregions.is_empty() {
            println!("Inserting point ({}, {})", point.x, point.y);
            self.points.push(point);
            if self.points.len() > 4 {
                self.split_into_subregions();
                self.move_points_to_subregions();
            }
        } else {
            for subregion in &mut self.subregions {
                if subregion.region.contains(&point){
                    subregion.insert_point(point);
                    break;
                }
            }
        }
    }

    pub fn remove_point(&mut self, point: Point) {
        if !self.region.contains(&point) {
            return;
        }

        self.points.retain(|&p| p!= point);

        for subregion in &mut self.subregions {
            subregion.remove_point(point);
        }
    }

    pub fn clear(&mut self) {
        self.points.clear();
        self.subregions.clear();
    }

    pub fn size(&self) -> usize {
        let mut size = self.points.len();
        for subregion in &self.subregions {
            size += subregion.size();
        }
        size
    }

    fn split_into_subregions(&mut self) {
        let subregions = RegionSubset::from(self.region);
        self.subregions = subregions.to_array().iter().map(|region| Quadtree::from(*region)).collect();
    }

    fn move_points_to_subregions(&mut self) {
        self.points.retain(|&point| {
            for subregion in self.subregions.iter_mut() {
                if subregion.region.contains(&point) {
                    subregion.insert_point(point);
                    return false;
                }
            }
            true
        });
    }

    // TODO: Implement balancing and small node merging (when a node's childrens' sum of points is less than 4)

    fn collect(&self) -> Vec<Point> {
        self.subregions.iter()
            .flat_map(|subregion| subregion.collect())
            .chain(self.points.clone())
            .collect()
    }

    /* balancing pseudocode
            function BALANCE(node)
            if node is leaf:
                return depth of node

            depths = list of depths of all children of node

             If the difference between the deepest and shallowest child is too large, redistribute
            if MAX(depths) - MIN(depths) > THRESHOLD:
                REDISTRIBUTE_POINTS(node)

            return depth of node (which is 1 + MAX of depths of children)

        function REDISTRIBUTE_POINTS(node)
            Gather all points in node and its descendants
            Clear all points and subregions in node
            For each point:
                INSERT_POINT_IN_TREE(node, point)

     */


    fn balance(&mut self) {
        if self.subregions.is_empty() {
            return;
        }
    }

    fn redistribute_points(&mut self) {
        // Gather all points in node and its descendants
        let points = self.collect();
        // Clear all points and subregions in node
        self.points.clear();
        self.subregions.clear();
        // Iterate over all points and reinsert them
        for point in points {
            self.insert_point(point);
        }
    }
}
