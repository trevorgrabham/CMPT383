pub mod block;
pub mod queue;

fn main() {
    let mut b0 = block::Block::initial(16);
    b0.mine_serial(); // or b0.set_proof(56231);
    let mut b1 = block::Block::next(&b0, String::from("message"));
    b1.mine_serial(); // or b1.set_proof(2159);
}

