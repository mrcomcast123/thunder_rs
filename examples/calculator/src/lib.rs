
struct Calculator { 
  proto: Box<dyn thunder_rs::PluginProtocol>
}

impl Calculator {
  fn dispatch_request(&self, req: json::JsonValue, ctx: thunder_rs::RequestContext) {
    let method = &req["method"];
    println!("method:{}", method);

    let s: String = String::from("{\"jsonrpc\":\"2.0\", \"id\":4, \"result\":\"hello from rust\"}");
    let res: json::JsonValue = json::parse(s.as_str())
      .unwrap();

    self.send_response(res, ctx);
  }

  fn send_response(&self, res: json::JsonValue, ctx: thunder_rs::RequestContext) {
    let s = json::stringify(res);
    self.proto.send_to(ctx.channel_id, s);
  }
}

impl thunder_rs::Plugin for Calculator {
  fn on_message(&self, json: String, ctx: thunder_rs::RequestContext) {
    let req: json::JsonValue = json::parse(json.as_str())
      .unwrap();
    self.dispatch_request(req, ctx);
  }
  fn on_client_connect(&self, _channel_id: u32) { }
  fn on_client_disconnect(&self, _channel_id: u32) { }
}

fn sample_plugin_init(proto: Box<dyn thunder_rs::PluginProtocol>) -> Box<dyn thunder_rs::Plugin> {
  Box::new(Calculator{ proto: proto})
}

thunder_rs::export_plugin!("Calculator", (1,0,0), sample_plugin_init);
