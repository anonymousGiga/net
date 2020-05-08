use use_grpc::foobar::*;        //use_grpc为当前包的名字，如果名字不一样需要修改
use use_grpc::foobar_grpc::*;   //use_grpc为当前包的名字，如果名字不一样需要修改
use futures::executor;

use grpc::ClientStubExt;


fn main() {
    let client = 
        FooBarServiceClient::new_plain("127.0.0.1", 
        9001, 
        Default::default())
        .unwrap();

    let mut req = CabLocationRequest::new();
    req.set_name("foo".to_string());

    let mut location = Location::new();
    location.latitude = 40.730610;
    location.longitude = -73.935242;
    req.set_location(location);

    let resp = client
        .record_cab_location(grpc::RequestOptions::new(), req)
        .join_metadata_result(); //future
    let resp = executor::block_on(resp);
    match resp {
        Err(e) => panic!("{:?}", e),
        Ok((_, r, _)) => println!("{:?}", r),
    }

    let mut nearby_req = GetCabRequest::new();
    let mut location = Location::new();
    location.latitude = 40.730610;
    location.longitude = -73.935242;    
    nearby_req.set_location(location);

    let nearby_resp = client
        .get_cabs(grpc::RequestOptions::new(), nearby_req)
        .join_metadata_result(); //返回future
    let nearby_resp = executor::block_on(nearby_resp);
    match nearby_resp {
        Err(e) => panic!("{:?}", e),
        Ok((_, cabs, _)) => println!("{:?}", cabs),
    }
}