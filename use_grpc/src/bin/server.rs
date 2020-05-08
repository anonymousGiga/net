use std::thread;
use use_grpc::foobar_grpc::*;
use use_grpc::foobar::*;

struct FooBarServer;

impl FooBarService for FooBarServer {
    fn record_cab_location(&self, 
        _o: grpc::ServerHandlerContext, 
        req: grpc::ServerRequestSingle<CabLocationRequest>, 
        resp: grpc::ServerResponseUnarySink<CabLocationResponse>) 
        -> grpc::Result<()> 
    {
        let mut r = CabLocationResponse::new();
        println!("Record cab {} at {}, {}", 
            req.message.get_name(), 
            req.message.get_location().latitude, 
            req.message.get_location().longitude);
        
        r.set_accepted(true);
        resp.finish(r)
    }

    fn get_cabs(&self, 
        _o: grpc::ServerHandlerContext, 
        _req: grpc::ServerRequestSingle<GetCabRequest>, 
        resp: grpc::ServerResponseUnarySink<GetCabResponse>) 
        -> grpc::Result<()> 
    {
        let mut r = GetCabResponse::new();

        let mut location = Location::new();
        location.latitude = 40.7128;
        location.longitude = -74.0060;

        let mut one = Cab::new();
        one.set_name("Limo".to_owned());
        one.set_location(location.clone());

        let mut two = Cab::new();
        two.set_name("Merc".to_owned());
        two.set_location(location.clone());

        let vec = vec![one, two];
        let cabs = ::protobuf::RepeatedField::from_vec(vec);

        r.set_cabs(cabs);

        resp.finish(r)
    }
}

fn main() {
    let mut server = grpc::ServerBuilder::new_plain();
    server.http.set_port(9001);
    server.add_service(FooBarServiceServer::new_service_def(FooBarServer));
    let _server = server.build().expect("Could not start server");
    loop {
        thread::park();
    }
}
