# Particle Bridge

Subscribes to particle events and emits Pixelblaze events.  For syncing Pixelblaze nodes with a particle (arduino) controller.  Personally, to aid in the transition from a bunch of particle LED nodes to a more standard WLED-based system.

> _sung to particle man by TMBG_ <br>
> 
> 🎶 Particle bridge, particle bridge <br>
> Nobody wants, not even a smidge <br>
> Niche and obtuse <br>
> Personal use <br>
> Nobody cares, particle bridge 🎶 <br>


## Setup

I can't imagine anyone would need this except me but here's how you set it up.  This is a Rust project so you should download a release if there is one matching your platform.

For now the instructions are TBD except for the normal `cargo` commands.

You need a [particle token](https://docs.particle.io/reference/cloud-apis/access-tokens/).  Put this in `.env` or set it in your shell's profile.

```
# particle access token
# https://docs.particle.io/reference/cloud-apis/access-tokens/
PARTICLE_TOKEN=xxx
```

Create a config file as shown in the config section.  This bridge supports one or many pixelblazes.

The invocation on the command line allows for one or multiple pixelblaze hosts:
```
particle_bridge --topic my-lights
```
Topic is the event name (or topic) message from [particle](https://docs.particle.io/reference/device-os/api/cloud-functions/particle-publish/).  In other words, if you broadcast "living_room_lights" to the particle events service, `--topic living_room_lights` is what you want.  This should be URL encoded so if you use a `/` slash you need `%2F` in this.

SSL on the pixelblaze is probably not setup or possible but SSL support is built in to this service.  So you can test this app using [a websocket tester](https://www.piesocket.com/websocket-tester).

There's a systemd template if you want to run this as a service.


## Config

There is an example in `config.example.json.tera` and the config file format is [Tera](https://tera.netlify.app/) because we have to substitute variables like `id` and `brightness`.  The `.tera` template format is used but you will recognize it from other template languages.  This format is used mostly because the pixelblaze generates a random ID for a theme you upload.

The config file is assumed to be in the current working directory of the binary.  For systemd, this is handled with working directory.

### Themes

Themes are a list of themes across all boards.  Each board generates a unique ID when you upload a theme in the UI.  Upload a pixelblaze theme and then copy the parameters in themes.  You can get the theme out by [using the Web API](https://electromage.com/docs/websockets-api).

For example:
```
echo '{ "listPrograms": true }' | websocat -1 ws://<pixelblaze>:81
echo '{ "getVars": true }' | websocat -1 ws://<pixelblaze>:81
echo '{ "getControls": <some id> }' | websocat -1 ws://<pixelblaze>:81
```
Then take what is returned and put it into the config file section.  For example, if you wanted [Pixelblaze's](https://electromage.com/patterns) theme _Real World Lights_ to be your theme for "white", you'd load that theme onto your board and then get it's parameters and ID from your board.  In the case of _Real World Lights_, it has a single parameter exposed called `nCurrentLight` (this is arbitrary from the theme author).  So we need to create the following `config.json.tera` file:

```
{  
  "themes": {
    "white": {
      "brightness": {{ brightness }},
      "activeProgramId": "{{ id }}",
      "setVars": { "nCurrentLight": 3 }
    }
  },
  "pixelblazes": [
    {
      "host": "ws://<pixelblaze ip>:81",
      "themeIds": {
        "white": "<id>",
      }
    }
  ]
}
```

To get the id:
```
$ echo '{ "listPrograms": true }' | websocat -1 ws://<pixelblaze>:81 | grep "Real World Lights"

ABCDEFghijk	some theme ABCDEFghijk some other theme  THEidWEwant	Real World Lights
```
The ID is **in front** of the theme name, in this case, `THEidWEwant`.


### Hosts

Pixelblazes are a list of hosts and the list of IDs of the themes. The assumption is that the pixelblazes have the same themes and want these to be sync'd up.  This bridge is _one particle event to many pixelblaze boards_.


### ENV

If you create a directory for this to run in under `/opt/particle_bridge`, then you need to create an `.env` file with these keys:

```
# particle access token
# https://docs.particle.io/reference/cloud-apis/access-tokens/
PARTICLE_TOKEN=<particle api token>
```
These ENVs could instead be set in the `particle_bridge.service` file.  See the included example in this repo.

You can also set an environment variable `PARTICLE_LOG_LEVEL` in `.env` or otherwise. to be `info`, `warn` etc to see log messages.  Default is `warn`.


## License

The MIT License (MIT)
See `LICENSE`.  I'm not updating the year in this file, assuming that is basically meaningless.
