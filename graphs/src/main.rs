// we’ll create a tree whose items know about their children items and their parent items.

use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>, // We want a Node to own its children, and we want to share that ownership with variables so we can access each Node in the tree directly. To do this, we define the Vec<T> items to be values of type Rc<Node>. We also want to modify which nodes are children of another node, so we have a RefCell<T> in children around the Vec<Rc<Node>>.

    // To make the child node aware of its parent, we need to add a parent field to our Node struct definition. The trouble is in deciding what the type of parent should be. We know it can’t contain an Rc<T>, because that would create a reference cycle with leaf.parent pointing to branch and branch.children pointing to leaf, which would cause their strong_count values to never be 0...RefCell

    // Thinking about the relationships another way, a parent node should own its children: if a parent node is dropped, its child nodes should be dropped as well. However, a child should not own its parent: if we drop a child node, the parent should still exist. This is a case for weak references!

    // So instead of Rc<T>, we’ll make the type of parent use Weak<T>, specifically a RefCell<Weak<Node>>. Now our Node struct definition looks like this:

    parent: RefCell<Weak<Node>>

}

fn main() {
    let leaf = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new())
    });

    let branch = Rc::new(Node {
        value: 10,
        children: RefCell::new(vec![Rc::clone(&leaf)]), //We clone the Rc<Node> in leaf and store that in branch, meaning the Node in leaf now has two owners: leaf and branch. We can get from branch to leaf through branch.children, but there’s no way to get from leaf to branch. The reason is that leaf has no reference to branch and doesn’t know they’re related. We want leaf to know that branch is its parent. We’ll do that next.
        parent: RefCell::new(Weak::new())
    });
    *leaf.parent.borrow_mut() = Rc::downgrade(&leaf);
}
