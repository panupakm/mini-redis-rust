use server::server::Server;

fn main() {
    let redis_serve = Server::new("127.0.0.1".to_string(), 7878);
    redis_serve.start();
}
