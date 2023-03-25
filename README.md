# Particle Bridge

Subscribes to particle events and emits Pixelblaze events.  For controlling one Pixelblaze node with a particle (arduino) controller.  Also, to aid in the transition from a bunch of particle LED nodes to a more standard WLED-based system.


## Setup

I can't imagine anyone would need this except me but here's how you set it up.  This is a Rust project so you should download a release if there is one matching your platform.

For now the instructions are TBD except for the normal `cargo` commands.

You need a [particle token](https://docs.particle.io/reference/cloud-apis/access-tokens/).  Put this in `.env` or set it in your shell's profile.

```
# particle access token
# https://docs.particle.io/reference/cloud-apis/access-tokens/
PARTICLE_TOKEN=xxx
```

The invocation on the command line allows for one or multiple pixelblaze hosts:
```
# forward to one pixelblaze
particle_bridge --topic my-lights --pixelblaze_host ws://10.0.0.1:81

# forward to multiple pixelblazes
particle_bridge --topic my-lights --pixelblaze_host ws://10.0.0.1:81 --pixelblaze_host ws://10.0.0.42:81
```

SSL on the pixelblaze is probably not setup, unknown but also support is not built in to this service.  So the protocol is `ws://`.

There's a systemd template if you want to run this as a service.


## License

The MIT License (MIT)
See `LICENSE`.  I'm not updating the year in this file, assuming that is basically meaningless.
