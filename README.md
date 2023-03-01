# Particle Bridge

Subscribes to particle events and emits WLED events.  For controlling WLED nodes with a particle (arduino) controller.  Also, to aid in the transition from a bunch of particle LED nodes to a more standard WLED-based system.


## Setup

I can't imagine anyone would need this but me but this is a Rust project so you should download a release if there is one matching your platform.

For now the instructions are TBD except for the normal `cargo` commands.

You need a [particle token](https://docs.particle.io/reference/cloud-apis/access-tokens/).  Put this in `.env`.

```
# particle access token
# https://docs.particle.io/reference/cloud-apis/access-tokens/
TOKEN=xxx

# the IP address of the pixelblaze as a websocket server
PIXELBLAZE_HOST=ws://<ip>:81
```

SSL on the pixelblaze is probably not setup, unknown but also support is not built in to this service.  So the protocol is `ws://`.


## License

The MIT License (MIT)
See `LICENSE`.  I'm not updating the year in this file, assuming that is basically meaningless.
