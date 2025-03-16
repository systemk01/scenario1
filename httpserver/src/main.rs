mod handler;
mod router;
mod server;

use server::Server;



fn main() {
    //start a new Server
    let server = Server::new("localhost:3000");
    //run the Server
    server.run();
}
