use chrono::Utc;
use utils::coder;
use utils::coder::serialize;
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug,PartialEq,Eq)]
pub struct BlockHeader {
    //time stamp
    pub time: i64,
    //merkle root hash
    pub tx_hash: String,
    //pre block
    pub pre_hash: String,
}
#[derive(Debug)]
pub struct Block {
    pub header: BlockHeader,
    //hash of the block header
    pub hash: String,
    //transactions data , chain
    pub data: String,
}

impl Block {
    fn set_hash(&mut self){
        let header = serialize(&self.header);
        self.hash = coder::get_hash(&header[..]);
    }

    pub fn new_block(data:String,pre_hash:String)->Self{
        //there only one
        let transactions = coder::serialize(&data);
        let tx_hash = coder::get_hash(&transactions[..]);
        let time = Utc::now().timestamp();
        let mut block = Block{
            header:BlockHeader{
                time:time,
                tx_hash:tx_hash,
                pre_hash:pre_hash,
            },
            hash: "".to_string(),
            data:data
        };
        block.set_hash();
        block
    }
}
