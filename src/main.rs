fn main() {
    trait Run {
        fn run(&self);
    }
    impl Run for A {
        fn run(&self) {
            println!("A");
        }
    }
    impl Run for B {
        fn run(&self) {
            println!("B");
        }
    }
    struct A;
    struct B;
    enum Node {
        ANode(A),
        BNode(B),
    }
    impl Run for Node {
        fn run(&self) {
            match self {
                Node::ANode(a) => a.run(),
                Node::BNode(b) => b.run(),
            }
        }
    }
    let node = Node::ANode(A);
    node.run();
    let node = Node::BNode(B);
    node.run();

    println!("Hello, world!");
}
