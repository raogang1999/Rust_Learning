use bincode;
use serde::{Serialize, Deserialize};
use crypto::digest::Digest;
use crypto::sha3::Sha3;

pub fn get_hash(data:&[u8]) ->String{
    let mut hasher = Sha3::sha3_256();
    hasher.input(data);
    hasher.result_str()
}



pub fn serialize<T: ?Sized>(value: &T) -> Vec<u8>
    where T: Serialize,
{
    bincode::serialize(value).unwrap()
}

pub fn deserialize<'a, T>(bytes: &'a [u8]) -> T
    where T: Deserialize<'a>,
{
    bincode::deserialize(bytes).unwrap()
}


#[derive(Serialize,PartialEq,Debug,Deserialize)]
pub struct Point{
    x:i32,
    y:i32,
}

#[cfg(test)]
mod tests{
    use super::Point;
    use super::*;
    #[test]
    fn it_works(){
        let p1 = Point{
            x:2,
            y:3,
        };
        let p = serialize(&p1);
        let o = deserialize(&p);
        assert_eq!(p1,o)
    }
}
