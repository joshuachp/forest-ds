use forest_ds::tree::Tree;

fn main() {
    let mut tree = Tree::new();

    let root = tree.append_child(1);

    let first = tree.append_child_to(&root, 2).unwrap();
    let second = tree.insert_sibling_after(&first, 3).unwrap();

    tree.append_child_to(&first, 1).unwrap();
    tree.append_child_to(&first, 2).unwrap();
    tree.append_child_to(&first, 3).unwrap();

    tree.append_child_to(&second, 1).unwrap();
    tree.append_child_to(&second, 2).unwrap();
    tree.append_child_to(&second, 3).unwrap();

    let json = serde_json::to_string(&tree).unwrap();

    println!("{}", json);
}
