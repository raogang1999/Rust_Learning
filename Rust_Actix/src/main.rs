fn main() {
    println!("Hello, world!");
}
pub struct BlockHeader{
    pub time:i64,
    pub tx_hash:String,
    pub pre_hash:String,
}
pub struct Block{
    pub header:BlockHeader,
    pub hash: String,
    pub data: String,
}
pub struct BlockChain{
    pub blocks:Vec<block::Block>,
}