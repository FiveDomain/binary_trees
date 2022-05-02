struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>
}

fn build_tree(vec: &Vec<i32>) -> Option<Box<TreeNode>> { 
    if vec.len() == 0 {
        return None;
    }
    let mid = vec.len() / 2;
    let mut node = TreeNode {
        value: vec[mid],
        left: None,
        right: None
    };
    node.left = build_tree(&vec[0..mid].to_vec());
    node.right = build_tree(&vec[mid+1..].to_vec());
    Some(Box::new(node))
}

fn print_tree_in_order(node: &Option<Box<TreeNode>>) {
    if let Some(ref n) = *node {
        print_tree_in_order(&n.left);
        println!("{}", n.value);
        print_tree_in_order(&n.right);
    }
}

fn invert_tree(node: &mut Option<Box<TreeNode>>) {
    if let Some(ref mut n) = *node {
        let tmp = n.left.take();
        n.left = n.right.take();
        n.right = tmp;
        invert_tree(&mut n.left);
        invert_tree(&mut n.right);
    }
}

fn main() {
    // create vector of 10 random numbers
    let mut v = Vec::new();
    for _ in 0..10 {
        v.push(rand::random::<i32>());
    }

    v.sort();

    let mut tree = build_tree(&v);

    println!("Tree in order:");
    print_tree_in_order(&tree);

    println!("Inverted tree:");
    invert_tree(&mut tree);

    println!();
    print_tree_in_order(&tree);

}
