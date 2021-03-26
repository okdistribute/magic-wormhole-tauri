## no identity

This is an experimental project for designing trusted collaborative
applications that are resistant to phishing attacks.

### Development

First, follow the [Getting Started guide for
Tauri](https://tauri.studio/en/docs/getting-started/intro) (available in windows, linux, or mac).

Second, install dependencies

```
npm install
```

To watch files and build incrementally while in development mode, use the
following commands in two separate terminals. 

You will notice that every time you save a rust file, Tauri will automatically
rebuild the application and restart it.

```
npm run watch
npm start 
```

To build the application with better debugging, try

```
npm run debug
```

### Deployment

To build the distributable application, first build the browserify bundle.

```
npm run build
```

And then build with tauri

```
npm run tauri build
```

I haven't tried to deploy it yet, but if you want to give it a whirl, [go to
their docs here](https://tauri.studio/en/docs/usage/development/publishing) and
let me know how it goes :)

### License

MIT
