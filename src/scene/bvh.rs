use std::cmp::Ordering;

use ray::Ray;
use scene::shapes::{BoundingBox, Primitive, Shape, ShapeIntersection};
use self::NodeIntersection::{Hit, Missed};


#[derive(PartialEq, Show)]
pub enum Node<'a> {
    Member(Box<TreeNode<'a>>),
    Leaf(Box<TreeNode<'a>>),
    Empty
}

#[derive(PartialEq, Show)]
pub enum NodeIntersection<'a> {
    Hit(&'a Box<TreeNode<'a>>, f32),
    Missed
}

#[derive(PartialEq, Show)]
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
                        true => Ordering::Less,
                        false => Ordering::Greater
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
            &Node::Leaf(ref node) => match node.shape {
                Some(ref shape) => match shape.intersects(ray) {
                    ShapeIntersection::Hit(p) => Hit(node, p),
                    ShapeIntersection::Missed => Missed
                },
                None => Missed
            },
            &Node::Member(ref node) => if node.bbox.intersects(ray) {
                let left = Tree::intersects_node(&node.left, ray);
                let right = Tree::intersects_node(&node.right, ray);

                match (left, right) {
                    (Hit(n0, p0), Hit(n1, p1)) => if p0 < p1 { Hit(n0, p0) } else { Hit(n1, p1) },
                    (Hit(node, p), _) => Hit(node, p),
                    (_, Hit(node, p)) => Hit(node, p),
                    (_, _) => Missed
                }
            } else {
                Missed
            }
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use self::test::Bencher;

    use vec::Vec3;
    use ray::Ray;
    use scene::{bvh, shapes};
    use scene::shapes::{Primitive, Shape};

    fn create_shape<'a>(pos: Vec3) -> Primitive {
        let sphere = shapes::sphere::Sphere::init(pos, 1.0);
        Primitive::Sphere(sphere)
    }

    #[test]
    fn can_init_tree_of_size_1() {
        let shapes = vec!(create_shape(Vec3::init(0.0, 0.0, -5.0)));
        let mut tree = bvh::Tree::new();
        tree.init(shapes);

        match tree.root {
            bvh::Node::Leaf(n) => assert_eq!(n.bbox.centroid(), Vec3::init(0.0, 0.0, -5.0)),
            _ => panic!("Tree should have one Leaf-node")
        }
    }

    #[test]
    fn can_intersect_tree_of_size_1() {
        let shapes = vec!(create_shape(Vec3::init(0.0, 0.0, -5.0)));
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

    #[test]
    fn can_build_tree_of_size_4() {
        let shapes = vec!(
            create_shape(Vec3::init(0.0, 0.0, 0.0)),
            create_shape(Vec3::init(-1.0, 2.0, 1.0)),
            create_shape(Vec3::init(-2.0, -2.0, 2.0)),
            create_shape(Vec3::init(2.0, 2.0, -1.0))
        );

        let mut tree = bvh::Tree::new();
        tree.init(shapes);

        let get_members = |root| match root {
            &bvh::Node::Member(ref node) => (node.bbox, &node.left, &node.right),
            _ => panic!("Node shuold be a member")
        };

        let (bbox, left, right) = get_members(&tree.root);
        assert_eq!(shapes::BoundingBox::init(
            Vec3::init(-3.0, -3.0, -2.0), Vec3::init(3.0, 3.0, 3.0)), bbox);

        let (bbox, ll, lr) = get_members(left);
        assert_eq!(shapes::BoundingBox::init(
            Vec3::init(-3.0, -3.0, 0.0), Vec3::init(0.0, 3.0, 3.0)), bbox);

        let (bbox, rl, rr) = get_members(right);
        assert_eq!(shapes::BoundingBox::init(
            Vec3::init(-1.0, -1.0, -2.0), Vec3::init(3.0, 3.0, 1.0)), bbox);

        let assert_leafnode = |sphere_node, primitive: Primitive| match sphere_node {
            &bvh::Node::Leaf(ref node) => {
                match node.shape {
                    Some(ref prim) => assert_eq!(&primitive, prim),
                    _ => panic!("Primitive is sphere")
                }
            },
            _ => panic!("Node should be a Leaf")

        };

        assert_leafnode(ll, create_shape(Vec3::init(-2.0, -2.0, 2.0)));
        assert_leafnode(lr, create_shape(Vec3::init(-1.0, 2.0, 1.0)));
        assert_leafnode(rl, create_shape(Vec3::init(0.0, 0.0, 0.0)));
        assert_leafnode(rr, create_shape(Vec3::init(2.0, 2.0, -1.0)));
    }

    #[test]
    fn can_intersect_tree_of_size_4() {
        let shapes = vec!(
            create_shape(Vec3::init(0.0, 0.0, 0.0)),
            create_shape(Vec3::init(-1.0, 2.0, 1.0)),
            create_shape(Vec3::init(-2.0, -2.0, 2.0)),
            create_shape(Vec3::init(2.0, 2.0, -1.0))
        );

        let mut tree = bvh::Tree::new();
        tree.init(shapes);

        let intersect_tree = |ray, primitive: Primitive| match tree.intersects(&ray) {
            bvh::NodeIntersection::Hit(node, _) => {
                match node.shape {
                    Some(ref prim) => assert_eq!(&primitive, prim),
                    _ => panic!("Node should have primitive")
                }
            },
            _ => panic!("Ray should have intersected tree")
        };

        intersect_tree(
            Ray::init(Vec3::init(2.0, 2.0, 2.0), Vec3::init(0.0, 0.0, -1.0)),
            create_shape(Vec3::init(2.0, 2.0, -1.0))
        );
        intersect_tree(
            Ray::init(Vec3::init(-1.0, -1.0, 1.0), Vec3::init(-1.0, -1.0, 1.0)),
            create_shape(Vec3::init(-2.0, -2.0, 2.0))
        );
        let intersection = tree.intersects(
            &Ray::init(Vec3::init(-1.0, -1.0, 1.0), Vec3::init(0.0, 0.0, 1.0))
        );
        assert_eq!(intersection, bvh::NodeIntersection::Missed);
    }

    #[bench]
    fn name(b: &mut Bencher) {
        let shapes = vec!(
            create_shape(Vec3::init(0.0, 0.0, 0.0)),
            create_shape(Vec3::init(-1.0, 2.0, 1.0)),
            create_shape(Vec3::init(-2.0, -2.0, 2.0)),
            create_shape(Vec3::init(2.0, 2.0, -1.0))
        );

        let mut tree = bvh::Tree::new();
        tree.init(shapes);

        let ray = Ray::init(Vec3::init(2.0, 2.0, 2.0), Vec3::init(0.0, 0.0, -1.0));
        b.iter(|| tree.intersects(&ray))
    }
}
