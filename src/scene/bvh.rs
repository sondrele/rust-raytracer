use vec::Vec3;
use ray::Ray;
use scene::shapes;
use scene::shapes::{BoundingBox, Shape};

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

    fn add(&mut self, shape: &'a Box<Shape+'a>) {
        self.shape = Some(shape);
        self.bbox = Some(shape.get_bbox())
    }
}

pub struct Tree<'a> {
    dims: uint,
    root: Option<Node<'a>>
}

impl<'a> Tree<'a> {
    pub fn new() -> Tree<'a> {
        Tree {
            dims: 3,
            root: None
        }
    }

    pub fn init(&mut self, shapes: &'a Vec<Box<Shape+'a>>) {
        let depth = 0;
        let root = self.build(shapes, depth);
        self.root = root;
    }

    fn build(&mut self, shapes: &'a Vec<Box<Shape+'a>>, depth: uint) -> Option<Node<'a>> {
        match shapes.len() {
            0 => None,
            1 => {
                let mut leaf = Node::new();
                leaf.add(&shapes[0]);
                leaf.leaf = true;
                Some(leaf)
            },
            _ => {
                fail!("Only size of one so far")
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
        let shapes= vec!(shape);
        let mut tree = bvh::Tree::new();
        tree.init(&shapes);

        match tree.root {
            Some(node) => assert!(node.leaf == true),
            None => fail!("Tree should have one node")
        }
    }
}