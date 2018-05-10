fn main() {

    let mut tree = Node{left_node: None, right_node: None, value: 100};
    tree.insert(110);
    tree.insert(150);
    tree.insert(125);
    tree.insert(50);
    tree.inorder();

    let mut x = Node { value: 100, left_node: None, right_node: None };
    x.insert(200);
    x.insert(10);
    x.insert(50);

    // test
    assert!(x == Node {
        value: 100,
        left_node: Some(Box::new(Node {
            value: 10,
            left_node: None,
            right_node: Some(Box::new(Node { value: 50, left_node: None, right_node: None })),
        })),
        right_node: Some(Box::new(Node { value: 200, left_node: None, right_node: None })),
    });
}




#[derive(PartialEq)]
struct Node {
    left_node: Option<Box<Node>>,
    right_node: Option<Box<Node>>,
    value: u64
}

impl Node {

    pub fn insert(&mut self, value: u64){

        let target_node = if value <= self.value { &mut self.left_node } else {&mut self.right_node};
        match target_node{
            &mut Some(ref mut subnode) => subnode.insert(value),
            &mut None => {
                let new_node = Node {left_node: None, right_node: None, value: value};
                let boxed_node = Some(Box::new(new_node));
                *target_node = boxed_node;
            }
        }

    }

    pub fn inorder(&mut self){

        match &mut self.right_node{
            &mut Some(ref mut subnode) => subnode.inorder(),
            &mut None => {}
        }

        println!("{}",self.value);

        match &mut self.left_node{
            &mut Some(ref mut subnode) => subnode.inorder(),
            &mut None => {}
        }
    }
}