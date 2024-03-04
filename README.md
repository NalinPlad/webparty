# webparty
> Hypertext with superpowers


Webparty is an experimental website server and editor. It monitors changes to the client's HTML and persists their changes on the server.

Webparty shines when your browser has strong developer tooling. Chrome and Firefox both have great HTML editors built in. I would describe webparty as *"what 5th grade me wished `Inspect Page` did"* (namely the changes actually saving).

It also has support for authentication if you want to host your site publicly.

### Command line arguments
> ⓘ NOTE: `webparty --help` to show this info

```
Usage: webparty [OPTIONS]

Options:
      --force          Overwrite existing webparty save with starter page
  -a, --auth           Enable Authentication
  -t, --token <TOKEN>  Use a custom token for auth (requires --auth)
      --path <PATH>    Path to persist webparty [default: ./webparty.html]
      --disable-check  Disable checking for webparty client code when writing to persisted file (not recommended)
  -v, --verbose        Verbose output [enable logging for all requests]
  -p, --port <PORT>    Port to run webparty on [default: 8000]
  -h, --help           Print help
  -V, --version        Print version
```

### Roadmap
Heres some things I want to add in the future(feel free to PR)
* TLS Support
* Automatic asset uploading(adding an image tag with a local image)
* Improve the way client side DOM manipulation is [detected](https://github.com/NalinPlad/webparty/blob/main/src/webparty.js)

