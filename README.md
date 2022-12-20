# Resizer Actor

A [wasmCloud Actor](https://wasmcloud.dev/reference/host-runtime/actors/) for converting images to
webp format and optionally resizing them. This is purely for my own personal use, but could be used
by others or as a template for other kinds of image manipulation

## Using the application

Once this actor is exposed via a `wasmcloud:httpserver` implementation, it is fairly straightforward
to use. It is a `POST` request to the URL (an example is below):

```terminal
$ curl -X POST -H 'Content-Type: image/jpeg' -v 'http://127.0.0.1:8080' --data-binary @/path/to/your/picture --output test.webp
```

The actor can handle most of the common image types such as PNG, JPEG, GIF, PNG, and TIFF. You can
set your content-type accordingly (or omit it if your hosting provider lets you, it definitely works
without it locally).

If you want to resize the image, the `longest_side_pixels` query string parameter can be passed. If
the number of pixels exceeds the current size, it will be ignored. It will also be ignored if a
number is not passed.

```terminal
$ curl -X POST -H 'Content-Type: image/jpeg' -v 'http://127.0.0.1:8080?longest_side_pixels=1024' --data-binary @/path/to/your/picture --output test.webp
```

### Additional notes

For smaller pictures, all of the default settings should work great. However, if you are processing
larger pictures, you'll probably want to bump up some timeout values (as a wasmCloud maintainer, I
know we are thinking of making this a little less painful):

When you start your wasmCloud Host, set `WASMCLOUD_RPC_TIMEOUT_MS` to a higher value like `15000`.
And then, when you add your link definition to the http server, use the following config:

```terminal
$ echo -n '{"address": "0.0.0.0:8080", "timeout_ms": 15000}' | base64
eyJhZGRyZXNzIjogIjAuMC4wLjA6ODA4MCIsICJ0aW1lb3V0X21zIjogMTUwMDB9
```

Then in the linkdef, set
`config_b64=eyJhZGRyZXNzIjogIjAuMC4wLjA6ODA4MCIsICJ0aW1lb3V0X21zIjogMTUwMDB9`.

## Development

Besides a normal Rust build toolchain, you'll also need to have `libwebp` available. Instructions
can be found [here](https://developers.google.com/speed/webp/docs/precompiled) to install it.

Additionally, you need to have the wasi-sdk available in order to compile everything for webp
correctly. You can find the latest releases
[here](https://github.com/WebAssembly/wasi-sdk/releases). Once you download the proper tarball from
the release, you can run the following steps:

```terminal
# Replace with the correct tarball name for you
$ tar xzvf wasi-sdk-17.0-macos.tar.gz
# Now export some vars to make sure you can build, replacing with your directory location
export PATH="<path to wasi-sdk dir>/bin:${PATH}"
export TARGET_CFLAGS='-I <path to wasi-sdk dir>/share/wasi-sysroot/include/'
export CC='<path to wasi-sdk dir>/bin/clang'
```

Once that is in place, you can just call `wash build` to actually build it

NOTE: I couldn't get VSCode to build for me just by setting these vars. So I had to do
ye-olde-run-a-terminal-command to check for some of the compilation errors
