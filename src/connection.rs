// The connection to the Riak Procol Buffers API
//
// For more information: https://docs.basho.com/riak/kv/latest/developing/api/protocol-buffers/

use errors::{RiakErr, ServerError};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::net::{SocketAddr, ToSocketAddrs};
use std::time::Duration;

// `RiakConn` represents a connection to a Riak server.
// TODO - reconnection and connection termination handling
#[derive(Debug)]
pub struct RiakConn {
    pub peer_addr: SocketAddr,
    pub timeout: u32,
    tcpstream: TcpStream,
}

impl RiakConn {
    // Constructs a new `RiakConn`.
    //
    // This will either return the newly constructed `RiakConn` or any error
    // result of the underlying `TcpStream` that may have occurred.
    pub fn new<A: ToSocketAddrs>(addr: A, timeout: u32) -> Result<RiakConn, RiakErr> {
        debug!("trying to connect to Riak...");
        match TcpStream::connect(addr) {
            Ok(tcpstream) => {
                debug!("connection established!");

                let timeout_duration = Duration::new(timeout as u64, 0);
                match tcpstream.set_read_timeout(Some(timeout_duration)) {
                    Ok(()) => (),
                    Err(error) => return Err(RiakErr::IoError(error)),
                };
                match tcpstream.set_write_timeout(Some(timeout_duration)) {
                    Ok(()) => (),
                    Err(error) => return Err(RiakErr::IoError(error)),
                };

                // get the peer_addr
                let peer_addr = match tcpstream.peer_addr() {
                    Ok(peer_addr) => peer_addr,
                    Err(error) => return Err(RiakErr::IoError(error)),
                };

                // return the connection
                Ok(RiakConn {
                    peer_addr: peer_addr,
                    tcpstream: tcpstream,
                    timeout: timeout,
                })
            }
            Err(err) => {
                debug!("error while connecting to Riak: {:?}", err);
                Err(RiakErr::IoError(err))
            }
        }
    }

    // Reconnect to the `SocketAddr` originally connected to.
    pub fn reconnect(&mut self) -> Result<(), RiakErr> {
        let newconn = match RiakConn::new(self.peer_addr, self.timeout) {
            Ok(c) => c,
            Err(err) => return Err(err),
        };

        *self = newconn;

        Ok(())
    }

    // Send and Receive data via the `TcpStream` in a single action
    pub fn exchange(&mut self,
                    send_code: u8,
                    expected_recv_code: u8,
                    send_data: &[u8])
                    -> Result<Vec<u8>, RiakErr> {
        match self.send(send_code, send_data) {
            Ok(()) => self.receive(expected_recv_code),
            Err(error) => Err(error),
        }
    }

    // Send data over the `TcpStream`
    pub fn send(&mut self, send_code: u8, send_data: &[u8]) -> Result<(), RiakErr> {
        // The first thing sent to Riak's Protocol Buffers API is a "header" of 5 bytes.
        //
        // https://docs.basho.com/riak/kv/latest/developing/api/protocol-buffers/#protocol
        //
        // Bytes 1 through 4 inform Riak of the number of bytes being sent, and byte 5
        // is the protocol buffer code of the message being sent. Here we record the
        // number of bytes we intend to send to Riak to generate this "header".
        let mut send_header: [u8; 5] = [0u8; 5];
        let send_bytes: u32 = (send_data.len() as u32) + 1;
        send_header[0] = (send_bytes >> 24) as u8;
        send_header[1] = (send_bytes >> 16) as u8;
        send_header[2] = (send_bytes >> 8) as u8;
        send_header[3] = send_bytes as u8;
        send_header[4] = send_code;
        debug!("header was {:?}", send_header);

        // Send the header over the `TcpStream`, followed by the data
        let _ = self.tcpstream.write(&send_header);
        let _ = self.tcpstream.write(send_data);
        match self.tcpstream.flush() {
            Ok(())   => debug!("wrote header and data successfully!"),
            Err(err) => {
                debug!("could not write header and data error was: {:?}", err);
                return Err(RiakErr::IoError(err));
            }
        };

        Ok(())
    }

    // Receive data from the `TcpStream`
    pub fn receive(&mut self, expected_recv_code: u8) -> Result<Vec<u8>, RiakErr> {
        // Retrieve the header from the server
        let mut recv_header = [0u8; 4];
        match self.tcpstream.read_exact(&mut recv_header) {
            Ok(())   => debug!("received response header successfully!"),
            Err(err) => {
                debug!("could not receive response header error was: {:?}", err);
                return Err(RiakErr::IoError(err));
            }
        };

        // Convert the header to a u32.
        // This number tells us how many bytes the server will be sending
        let mut recv_bytes: u32 = 0;
        recv_bytes |= ((recv_header[0] as u32) << 24) | ((recv_header[1] as u32) << 16) |
                      ((recv_header[2] as u32) << 8) |
                      (recv_header[3] as u32);

        // Retrieve the code, the code is the first byte after the header
        let mut recv_code = [0u8; 1];
        match self.tcpstream.read_exact(&mut recv_code) {
            Ok(())   => debug!("received response code {}", recv_code[0]),
            Err(err) => {
                debug!("could not receive response code error was: {:?}", err);
                return Err(RiakErr::IoError(err));
            }
        };

        // Retrieve the protocol buffer encoded data
        let mut response = vec![0u8; (recv_bytes - 1) as usize];
        match self.tcpstream.read_exact(&mut response) {
            Ok(()) => debug!("received response of size {}", (recv_bytes - 1)),
            Err(err) => {
                debug!("failed to receive response from server of {} bytes error was: {:?}",
                       recv_bytes,
                       err);
                return Err(RiakErr::IoError(err));
            }
        }

        // Check for Riak errors
        if recv_code[0] != expected_recv_code {
            let err = ServerError::new(recv_code[0], &response);
            return Err(RiakErr::ServerError(err));
        }

        // If all went well, send back the response
        Ok(response)
    }
}
