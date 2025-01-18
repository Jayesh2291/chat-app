use actix::{Actor, StreamHandler};
use actix_web_actors::ws;
use std::time::{Duration, Instant};

pub struct MyWebSocket {
    last_heartbeat: Instant,
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.last_heartbeat = Instant::now();
        ctx.run_interval(Duration::from_secs(5), |act, ctx| {
            if Instant::now().duration_since(act.last_heartbeat) > Duration::from_secs(10) {
                ctx.stop();  // Disconnect the client if no heartbeat is received
            }
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                // Broadcast message to all connected clients
                ctx.text(text);
            }
            Ok(ws::Message::Pong(_)) => {
                self.last_heartbeat = Instant::now();  // Reset the heartbeat on Pong
            }
            _ => (),
        }
    }
}
