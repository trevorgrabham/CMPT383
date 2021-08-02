pub mod block;
pub mod queue;

fn main() {
    let mut b0 = block::Block::initial(20);
    b0.mine(10);
    println!("{}", b0.hash_string());
    println!("{:02x}", b0.hash());
    let mut b1 = block::Block::next(&b0, String::from("this is an interesting message"));
    b1.mine(10);
    println!("{}", b1.hash_string());
    println!("{:02x}", b1.hash());
    let mut b2 = block::Block::next(&b1, String::from("this is not interesting"));
    b2.mine(10);
    println!("{}", b2.hash_string());
    println!("{:02x}", b2.hash());
}

