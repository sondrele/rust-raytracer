use std::cmp;

use scene::shapes::{BoundingBox, Shape};

// pub enum TreeNode<'a> {
//     Child(Node<'a>),
//     Empty
// }

#[derives(Show)]
pub struct Node<'a> {
    left: Option<Box<Node<'a>>>,
    right: Option<Box<Node<'a>>>,
    shape: Option<&'a Box<Shape+'a>>,
    bbox: Option<BoundingBox>,
    leaf: bool
}

impl<'a> Node<'a> {
    pub fn new() -> Node<'a> {
        Node {
            left: None,
            right: None,
            shape: None,
            bbox: None,
            leaf: false
        }
    }

    pub fn init(left: Option<Node<'a>>, right: Option<Node<'a>>) -> Node<'a> {
        let left_bbox = match left {
            Some(ref n) => n.bbox.unwrap_or(BoundingBox::new()),
            None => BoundingBox::new()
        };
        let right_bbox = match right {
            Some(ref n) => n.bbox.unwrap_or(BoundingBox::new()),
            None => BoundingBox::new()
        };

        let mut node = Node::new();
        node.left = Some(box left.unwrap());
        node.right = Some(box right.unwrap());
        node.bbox = Some(left_bbox + right_bbox);
        node
    }

    fn add(&mut self, shape: &'a Box<Shape+'a>) {
        self.bbox = Some(shape.get_bbox());
        self.shape = Some(shape);
    }
}

pub struct Tree<'a> {
    root: Option<Node<'a>>
}

impl<'a> Tree<'a> {
    pub fn new() -> Tree<'a> {
        Tree {
            root: None
        }
    }

    pub fn init(&mut self, shapes: &'a mut [Box<Shape+'a>]) {
        let depth = 0;
        let root = self.build(shapes, depth);
        self.root = root;
    }

    pub fn build(&mut self, shapes: &'a mut [Box<Shape+'a>], depth: uint) -> Option<Node<'a>> {
        match shapes.len() {
            0 => None,
            1 => {
                let mut leaf = Node::new();
                leaf.add(&shapes[0]);
                leaf.leaf = true;
                Some(leaf)
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
                let (mut head, mut tail) = shapes.split_at_mut(half);

                let left = self.build(head, depth + 1);
                let right = self.build(tail, depth + 1);

                Some(Node::init(left, right))
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
            Some(node) => assert!(node.leaf == true),
            None => fail!("Tree should have one node")
        }
    }
}