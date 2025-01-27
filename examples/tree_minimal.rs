#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

/******************************************************************************
 * The `examples/file_explorer` example shows a more complex example 'tree'
 * display. Its complexities include use of async functionality and use of the
 * VirtualScrollView. This example serves as a simpler version for tree views.
 *
 * Questions
 *
 *****************************************************************************/

use freya::prelude::*;

#[derive(Clone, Debug, PartialEq)]
struct MyNodeId(String);

impl ItemPath for MyNodeId {
    fn item_starts_with(&self, other: &Self) -> bool {
        self.0.starts_with(&other.0)
    }
}

#[derive(Clone, Debug, PartialEq)]
enum MyTree {
    Leaf(String),
    Branches(String, Vec<Box<MyTree>>),
}

impl MyTree {
    fn new(value: &str) -> Self {
        Self::Branches(value.into(), Vec::new())
    }

    fn with_leaf(mut self, value: &str) -> Self {
        match &mut self {
            Self::Leaf(_) => panic!("cannot add to leaf"),
            Self::Branches(_, branches) => branches.push(Box::new(Self::Leaf(value.into()))),
        };
        self
    }

    fn with_tree(mut self, branch: MyTree) -> Self {
        match &mut self {
            Self::Leaf(_) => panic!("cannot add to leaf"),
            Self::Branches(_, branches) => branches.push(Box::new(branch)),
        };
        self
    }
}

fn main() {
    launch(app);
}

fn app() -> Element {
    let my_tree = MyTree::new("root")
        .with_leaf("0")
        .with_tree(
            MyTree::new("1")
                .with_leaf("1.0")
                .with_leaf("1.1")
                .with_leaf("1.2"),
        )
        .with_tree(
            MyTree::new("2")
                .with_leaf("2.0")
                .with_leaf("2.1")
                .with_tree(
                    MyTree::new("2.2")
                        .with_leaf("2.2.0")
                        .with_leaf("2.2.1")
                        .with_leaf("2.2.2"),
                ),
        )
        .with_leaf("3");

    let freya_tree = to_freya_tree_item(my_tree);
    let flat_items = freya_tree.flat(0, freya_tree.id());

    rsx! {
        ScrollView {
            width: "fill",
            height: "fill",
            for item in flat_items {
                label {
                    key: "{item.id.0}",
                    margin: "0 0 0 {item.depth * 20}",
                    max_lines: "1",
                    text_overflow: "ellipsis",
                    // TODO: {item.value} wanted here, but no value present
                    {item.id.0}
                }
            }
        }
    }
}

fn to_freya_tree_item(tree: MyTree) -> TreeItem<MyNodeId, String> {
    match &tree {
        MyTree::Leaf(value) => TreeItem::Standalone {
            // TODO: How to allocate an id ??
            // The example's values here act as the id; ids must be unique, and values may not be
            id: { MyNodeId(value.to_string()) },
            value: { value.to_string() },
        },
        MyTree::Branches(value, branches) => TreeItem::Expandable {
            // TODO: How to allocate an id ??
            // The example's values here act as the id; ids must be unique, and values may not be
            id: { MyNodeId(value.to_string()) },
            value: { value.to_string() },
            state: ExpandableItemState::Open(
                branches
                    .into_iter()
                    .map(|b| to_freya_tree_item(*b.clone()))
                    .collect::<Vec<_>>(),
            ),
        },
    }
}
