use std::thread;
use use_grpc::foobar::*;
use use_grpc::foobar_grpc::*;
struct FooBarServer {}
impl FoobarService for FooBarServer {
    fn record_cab_location(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::foobar::CabLocationResquest>,
        resp: ::grpc::ServerResponseUnarySink<super::foobar::CabLocationResponse>,
    ) -> ::grpc::Result<()> {
        let mut r = CabLocationResponse::new();
        println!(
            "record cab {} at {},{}",
            req.message.get_name(),
            req.message.get_location().latitude,
            req.message.get_location().longitude
        );
        r.set_accept(true);
        req.finish(r);
    }

    fn get_cab_location(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::foobar::GetCabRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::foobar::GetCabResponse>,
    ) -> ::grpc::Result<()> {
    }
}
