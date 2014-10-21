use std::cmp;

use scene::shapes::{BoundingBox, Shape};

pub enum Node<'a> {
    Member(Box<TreeNode<'a>>),
    Empty
}

#[derives(Show)]
pub struct TreeNode<'a> {
    left: Node<'a>,
    right: Node<'a>,
    shape: Option<&'a Box<Shape+'a>>,
    bbox: Option<BoundingBox>,
    leaf: bool
}

impl<'a> TreeNode<'a> {
    pub fn new() -> TreeNode<'a> {
        TreeNode {
            left: Empty,
            right: Empty,
            shape: None,
            bbox: None,
            leaf: false
        }
    }

    pub fn init(left: Node<'a>, right: Node<'a>) -> TreeNode<'a> {
        let left_bbox = match left {
            Member(ref n) => n.bbox.unwrap_or(BoundingBox::new()),
            Empty => BoundingBox::new()
        };
        let right_bbox = match right {
            Member(ref n) => n.bbox.unwrap_or(BoundingBox::new()),
            Empty => BoundingBox::new()
        };

        let mut node = TreeNode::new();
        node.left = left;
        node.right = right;
        node.bbox = Some(left_bbox + right_bbox);
        node
    }

    fn add(&mut self, shape: &'a Box<Shape+'a>) {
        self.bbox = Some(shape.get_bbox());
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
                let mut leaf = box TreeNode::new();
                leaf.add(&shapes[0]);
                leaf.leaf = true;
                Member(leaf)
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
}

#[cfg(test)]
mod tests {
    use vec::Vec3;
    use scene::bvh;
    use scene::shapes;
    use scene::shapes::Shape;

    #[test]
    fn can_init_tree_of_size_1() {
        let sphere = shapes::sphere::Sphere::init(Vec3::init(0.0, 0.0, -5.0), 1.0);
        let shape: Box<Shape> = box sphere;
        let mut shapes= vec!(shape);
        let mut tree = bvh::Tree::new();
        tree.init(shapes.as_mut_slice());

        match tree.root {
            bvh::Member(node) => assert!(node.leaf == true),
            bvh::Empty => fail!("Tree should have one node")
        }
    }
}