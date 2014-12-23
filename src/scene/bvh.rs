use std::cmp;

use ray::Ray;
use scene::shapes::{BoundingBox, Primitive, Shape, ShapeIntersection};
use self::NodeIntersection::{Hit, Missed};

pub enum Node<'a> {
    Member(Box<TreeNode<'a>>),
    Leaf(Box<TreeNode<'a>>),
    Empty
}

pub enum NodeIntersection<'a> {
    Hit(&'a Box<TreeNode<'a>>, f32),
    Missed
}

pub struct TreeNode<'a> {
    left: Node<'a>,
    right: Node<'a>,
    shape: Option<Primitive>,
    bbox: BoundingBox
}

impl<'a> TreeNode<'a> {
    pub fn new() -> TreeNode<'a> {
        TreeNode {
            left: Node::Empty,
            right: Node::Empty,
            shape: None,
            bbox: BoundingBox::new()
        }
    }

    fn get_bbox(node: &Node<'a>) -> BoundingBox {
        match node {
            &Node::Member(ref n) => n.bbox,
            &Node::Leaf(ref n) => n.bbox,
            &Node::Empty => BoundingBox::new()
        }
    }

    pub fn init(left: Node<'a>, right: Node<'a>) -> TreeNode<'a> {
        let left_bbox = TreeNode::get_bbox(&left);
        let right_bbox = TreeNode::get_bbox(&right);

        let mut node = TreeNode::new();
        node.left = left;
        node.right = right;
        node.bbox = left_bbox + right_bbox;
        node
    }

    fn add(&mut self, shape: Primitive) {
        self.bbox = shape.get_bbox();
        self.shape = Some(shape);
    }

    pub fn get_shape(&'a self) -> &'a Primitive {
        match self.shape {
            Some(ref shape) => shape,
            None => panic!("Node has not been assigned a shape")
        }
    }
}

pub struct Tree<'a> {
    pub root: Node<'a>
}

impl<'a> Tree<'a> {
    pub fn new() -> Tree<'a> {
        Tree {
            root: Node::Empty
        }
    }

    pub fn init(&mut self, mut shapes: Vec<Primitive>) {
        let depth = 0;
        let root = self.build(shapes.as_mut_slice(), depth);
        self.root = root;
    }

    fn build(&mut self, shapes: &'a mut [Primitive], depth: uint) -> Node<'a> {
        match shapes.len() {
            0 => Node::Empty,
            1 => {
                let mut node = box TreeNode::new();
                node.add(shapes[0].clone());
                Node::Leaf(node)
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

                Node::Member(box TreeNode::init(left, right))
            }
        }
    }

    pub fn intersects(&'a self, ray: &Ray) -> NodeIntersection<'a> {
        Tree::intersects_node(&self.root, ray)
    }

    fn intersects_node(node: &'a Node<'a>, ray: &Ray) -> NodeIntersection<'a> {
        match node {
            &Node::Empty => Missed,
            &Node::Leaf(ref n) => match n.shape {
                Some(ref shape) => match shape.intersects(ray) {
                    ShapeIntersection::Hit(p) => Hit(n, p),
                    ShapeIntersection::Missed => Missed
                },
                None => Missed
            },
            &Node::Member(ref n) => {
                let left = Tree::intersects_node(&n.left, ray);
                let right = Tree::intersects_node(&n.right, ray);

                match (left, right) {
                    (Hit(n0, p0), Hit(n1, p1)) => if p0 < p1 { Hit(n0, p0) } else { Hit(n1, p1) },
                    (Hit(n, p), _) => Hit(n, p),
                    (_, Hit(n, p)) => Hit(n, p),
                    (_, _) => Missed
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use vec::Vec3;
    use ray::Ray;
    use scene::{bvh, shapes};
    use scene::shapes::{Primitive, Shape};

    fn create_shape<'a>() -> Primitive {
        let sphere = shapes::sphere::Sphere::init(Vec3::init(0.0, 0.0, -5.0), 1.0);
        Primitive::Sphere(sphere)
    }

    #[test]
    fn can_init_tree_of_size_1() {
        let shapes = vec!(create_shape());
        let mut tree = bvh::Tree::new();
        tree.init(shapes);

        match tree.root {
            bvh::Node::Leaf(n) => assert_eq!(n.bbox.centroid(), Vec3::init(0.0, 0.0, -5.0)),
            _ => panic!("Tree should have one Leaf-node")
        }
    }

    #[test]
    fn can_intersect_tree_of_size_1() {
        let shapes = vec!(create_shape());
        let mut tree = bvh::Tree::new();
        tree.init(shapes);

        let intersection = tree.intersects(
            &Ray::init(Vec3::init(0.0, 0.0, 0.0), Vec3::init(0.0, 0.0, -1.0))
        );

        match intersection {
            bvh::NodeIntersection::Hit(_, p) => assert_eq!(p, 4.0),
            _ => panic!("Should have intersected with tree")
        }
    }
}
