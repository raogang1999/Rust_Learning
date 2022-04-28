extern crate protobuf;
extern crate grpc;
extern crate tls_api;
extern crate grpc_protobuf;

pub mod foobar;
pub mod foobar_grpc;



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
