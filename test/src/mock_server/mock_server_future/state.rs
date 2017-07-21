use std::fmt::Display;
use std::mem;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use futures::{Future, Poll};
use tokio_core::net::TcpStream;
use tokio_core::reactor::Handle;
use tokio_proto::pipeline::ServerProto;

use super::wait_for_parameters::WaitForParameters;
use super::wait_to_start::WaitToStart;
use super::super::active_mock_server::ActiveMockServer;
use super::super::errors::Error;
use super::super::super::mock_service::MockService;
use super::super::super::mock_service::MockServiceFactory;

pub enum State<P>
where
    P: ServerProto<TcpStream>,
    P::Request: Clone + Display + PartialEq,
    P::Response: Clone,
{
    WaitingToStart(WaitToStart<P>),
    WaitingForParameters(WaitForParameters<P>),
    ServerReady(ServerReady<P>),
    Processing,
}

impl<P> State<P>
where
    P: ServerProto<TcpStream>,
    P::Request: Clone + Display + PartialEq,
    P::Response: Clone,
{
    pub fn start_with(
        address: SocketAddr,
        service_factory: MockServiceFactory<P::Request, P::Response>,
        protocol: Arc<Mutex<P>>,
        handle: Handle,
    ) -> Self {
        let wait_to_start =
            WaitToStart::new(address, service_factory, protocol, handle);

        State::WaitingToStart(wait_to_start)
    }

    pub fn advance(&mut self) -> Poll<(), Error> {
        let state = mem::replace(self, State::Processing);

        let (poll_result, new_state) = state.advance_to_new_state();

        mem::replace(self, new_state);

        poll_result
    }

    fn advance_to_new_state(self) -> (Poll<(), Error>, Self) {
        match self {
            State::WaitingToStart(handler) => handler.advance(),
            State::WaitingForParameters(handler) => handler.advance(),
            State::ServerReady(handler) => handler.advance(),
            State::Processing => panic!("State has more than one owner"),
        }
    }
}

pub struct ServerReady<P>
where
    P: ServerProto<TcpStream>,
    P::Request: Clone + Display + PartialEq,
    P::Response: Clone,
{
    server: ActiveMockServer<P::Transport>,
}

impl<P> ServerReady<P>
where
    P: ServerProto<TcpStream>,
    P::Request: Clone + Display + PartialEq,
    P::Response: Clone,
{
    pub fn advance_with(
        parameters_tuple: (P::Transport, MockService<P::Request, P::Response>),
    ) -> (Poll<(), Error>, State<P>) {
        let server_ready = Self {
            server: ActiveMockServer::from_tuple(parameters_tuple),
        };

        server_ready.advance()
    }

    fn advance(mut self) -> (Poll<(), Error>, State<P>) {
        (self.server.poll(), State::ServerReady(self))
    }
}