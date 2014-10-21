use std::cmp;

use ray::Ray;
use scene::shapes;
use scene::shapes::{BoundingBox, Shape};

pub enum Node<'a> {
    Member(Box<TreeNode<'a>>),
    Leaf(Box<TreeNode<'a>>),
    Empty
}

pub struct TreeNode<'a> {
    left: Node<'a>,
    right: Node<'a>,
    shape: Option<&'a Box<Shape+'a>>,
    bbox: BoundingBox
}

impl<'a> TreeNode<'a> {
    pub fn new() -> TreeNode<'a> {
        TreeNode {
            left: Empty,
            right: Empty,
            shape: None,
            bbox: BoundingBox::new()
        }
    }

    fn extract_bbox(node: &Node<'a>) -> BoundingBox {
        match node {
            &Member(ref n) => n.bbox,
            &Leaf(ref n) => n.bbox,
            &Empty => BoundingBox::new()
        }
    }

    pub fn init(left: Node<'a>, right: Node<'a>) -> TreeNode<'a> {
        let left_bbox = TreeNode::extract_bbox(&left);
        let right_bbox = TreeNode::extract_bbox(&right);

        let mut node = TreeNode::new();
        node.left = left;
        node.right = right;
        node.bbox = left_bbox + right_bbox;
        node
    }

    fn add(&mut self, shape: &'a Box<Shape+'a>) {
        self.bbox = shape.get_bbox();
        self.shape = Some(shape);
    }
}

pub struct Tree<'a> {
    root: Node<'a>
}

impl<'a> Tree<'a> {
    pub fn new() -> Tree<'a> {
        Tree {
            root: Empty
        }
    }

    pub fn init(&mut self, shapes: &'a mut [Box<Shape+'a>]) {
        let depth = 0;
        let root = self.build(shapes, depth);
        self.root = root;
    }

    pub fn build(&mut self, shapes: &'a mut [Box<Shape+'a>], depth: uint) -> Node<'a> {
        match shapes.len() {
            0 => Empty,
            1 => {
                let mut node = box TreeNode::new();
                node.add(&shapes[0]);
                Leaf(node)
            },
            _ => {
                let axis = depth as u32 % 3;
                shapes.sort_by(|a, b| {
                    match a.get_bbox().centroid()[axis] < b.get_bbox().centroid()[axis] {
                        true => cmp::Less,
                        false => cmp::Greater
                    }
                });
                let half = shapes.len() / 2;
                let (head, tail) = shapes.split_at_mut(half);

                let left = self.build(head, depth + 1);
                let right = self.build(tail, depth + 1);

                Member(box TreeNode::init(left, right))
            }
        }
    }

    pub fn intersects(&self, ray: Ray) -> shapes::ShapeIntersection {
        self.intersects_node(ray, &self.root)
    }

    fn intersects_node(&self, ray: Ray, node: &Node<'a>) -> shapes::ShapeIntersection {
        match node {
            &Empty => shapes::Missed,
            &Leaf(ref n) => match n.shape {
                Some(shape) => shape.intersects(ray),
                None => shapes::Missed
            },
            _ => shapes::Missed
        }
    }
}

#[cfg(test)]
mod tests {
    use vec::Vec3;
    use ray::Ray;
    use scene::{bvh, shapes};
    use scene::shapes::Shape;

    fn create_shape<'a>() -> Box<Shape+'a> {
        let sphere = shapes::sphere::Sphere::init(Vec3::init(0.0, 0.0, -5.0), 1.0);
        box sphere
    }

    #[test]
    fn can_init_tree_of_size_1() {
        let mut shapes= vec!(create_shape());
        let mut tree = bvh::Tree::new();
        tree.init(shapes.as_mut_slice());

        match tree.root {
            bvh::Leaf(n) => assert_eq!(n.bbox.centroid(), Vec3::init(0.0, 0.0, -5.0)),
            _ => fail!("Tree should have one Leaf-node")
        }
    }

    #[test]
    fn can_intersect_tree_of_size_1() {
        let mut shapes= vec!(create_shape());
        let mut tree = bvh::Tree::new();
        tree.init(shapes.as_mut_slice());

        let intersection = tree.intersects(
            Ray::init(Vec3::init(0.0, 0.0, 0.0), Vec3::init(0.0, 0.0, -1.0))
        );

        match intersection {
            shapes::Hit(p) => assert_eq!(p, 4.0),
            _ => fail!("Should have intersected with tree")
        }
    }
}
