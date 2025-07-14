# H2DNS

`HTTP/2` based DNS Server <sub>(Actually it's `HTTP/1.x` but *H2* sounds cooler)</sub>.
It achieves the following functionality:

- Register & Update your domain with a IP to the server
- Resolve the registered domain
- Proxies DDNS requests
- Has Password Authentication

## Usage

Compile the software with 

```bash
RUSTFLAGS="-C target-cpu=native" dx bundle --platform web --release
```

And run the server binary.

### Environmental Variables

- `PORT`: Port to listen to.
- `IP`: IP Address to listen to.
- `PASSWORD`: Password to use.

### API

API are exposed in `/api/` subpath, see more in `src/backend/mod.rs`.

`curl` example:

```bash
# Run the server binary first
curl -lv -X POST -d "domain=abc123" http://localhost:8080/api/resolve
```

## Development

Use `nix` and `direnv` to automatically set up dev environment:

```bash
echo 'use nix\nmkdir $TMPDIR' > .envrc && direnv allow .
```

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve --platform web
```

