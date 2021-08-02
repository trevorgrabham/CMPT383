#[cfg(test)]
mod block_tests {
    use crate::block::Block;

    #[test]
    fn example() {
        assert_eq!(1+1, 2);
    }

    #[test]
    fn range_serial_eq(){
        let mut b0 = Block::initial(7);
        let mut b1 = b0.clone();
        b0.mine(4);
        b1.mine_serial();
        assert_eq!(b0.hash(), b1.hash());
    }

    #[test]
    fn valid_proof(){
        let mut b0 = Block::initial(16);
        b0.mine(4);
        let mut b1 = Block::next(&b0, String::from("this is an interesting message"));
        b1.mine(4);
        let hash = b1.hash_string();
        println!("{}", hash);
        assert_eq!(&hash[63..64], "0");
        assert_eq!(&hash[62..63], "0");
        assert_eq!(&hash[61..62], "0");
        assert_eq!(&hash[60..61], "0");
    }
}
