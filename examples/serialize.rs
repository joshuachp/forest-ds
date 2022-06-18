use forest::tree::Tree;

fn main() {
    let mut tree = Tree::new();

    let root = tree.append_child(1);

    let first = tree.insert_child(&root, 2).unwrap();
    let second = tree.insert_sibling(&first, 3).unwrap();

    tree.insert_child(&first, 1);
    tree.insert_child(&first, 2);
    tree.insert_child(&first, 3);

    tree.insert_child(&second, 1);
    tree.insert_child(&second, 2);
    tree.insert_child(&second, 3);

    let json = serde_json::to_string(&tree).unwrap();

    println!("{}", json);
}
