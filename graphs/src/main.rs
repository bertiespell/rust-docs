// we’ll create a tree whose items know about their children items and their parent items.

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>> // We want a Node to own its children, and we want to share that ownership with variables so we can access each Node in the tree directly. To do this, we define the Vec<T> items to be values of type Rc<Node>. We also want to modify which nodes are children of another node, so we have a RefCell<T> in children around the Vec<Rc<Node>>.
}

fn main() {
    let leaf = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![])
    });

    let branch = Rc::new(Node {
        value: 10,
        children: RefCell::new(vec![Rc::clone(&leaf)]) //We clone the Rc<Node> in leaf and store that in branch, meaning the Node in leaf now has two owners: leaf and branch. We can get from branch to leaf through branch.children, but there’s no way to get from leaf to branch. The reason is that leaf has no reference to branch and doesn’t know they’re related. We want leaf to know that branch is its parent. We’ll do that next.
    });
}
