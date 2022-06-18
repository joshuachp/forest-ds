use forest::tree::Tree;

const SIZE: i64 = 10000000;

fn main() {
    let mut tree: Tree<i64> = Tree::with_capacity(SIZE.try_into().unwrap());

    (SIZE..2 * SIZE).for_each(|i| {
        tree.append_sibling(i);
    });
    (0..SIZE).for_each(|i| {
        tree.append_child(i);
    });

    let mut sum = 0;
    tree.iter().enumerate().for_each(|(i, v)| {
        let expected: i64 = i.try_into().unwrap();
        assert_eq!(expected, *v);

        sum += *v;
    });

    println!("{}", sum);
}
