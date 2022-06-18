use serde::{
    ser::{SerializeSeq, SerializeStruct},
    Serialize,
};

use crate::{node::Node, tree::Tree};

/// Enum to help serialize a tree
enum NodeSerialize<'a, T> {
    Node {
        tree: &'a Tree<T>,
        node: &'a Node<T>,
    },
    Child {
        tree: &'a Tree<T>,
        index: Option<usize>,
    },
}

impl<T: Serialize> Serialize for Node<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.value.serialize(serializer)
    }
}

impl<T: Serialize> Serialize for Tree<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut sequence = serializer.serialize_seq(None)?;

        let mut current_index = self.root;
        while let Some(index) = current_index {
            let node = &self.nodes[index];

            sequence.serialize_element(&NodeSerialize::Node { tree: self, node })?;

            current_index = node.next_sibling;
        }

        sequence.end()
    }
}

impl<'a, T: Serialize> Serialize for NodeSerialize<'a, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            NodeSerialize::Node { tree, node } => {
                let mut node_struct = serializer.serialize_struct("Node", 2)?;

                node_struct.serialize_field("value", &node.value)?;
                node_struct.serialize_field(
                    "children",
                    &NodeSerialize::Child {
                        tree,
                        index: node.first_child,
                    },
                )?;

                node_struct.end()
            }
            NodeSerialize::Child { tree, index } => {
                let mut sequence = serializer.serialize_seq(None)?;

                let mut current_index = *index;
                while let Some(index) = current_index {
                    let node = &tree.nodes[index];

                    sequence.serialize_element(&NodeSerialize::Node { tree, node })?;

                    current_index = node.next_sibling;
                }

                sequence.end()
            }
        }
    }
}
