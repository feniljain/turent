# Turent

My experimentation with webrtc in rust, a pure webrtc-based file sharing system!

There are three sections of this project:

- discovery: Helps in signalling and datasource discovery
- Each file sharing features a pair, a datasource and a datasink
  - datasource: Each file to be shared starts (kind of like seeding) from a datasource
  - datasink: Each file to be shared is received in a datasink

## Install and Run

- For running the discovery server:
```bash
cargo run -p discovery
```

- For running the client ( here client = datasink and server = datasource ):
```bash
cargo run -p client -- init_server=true init_client=true
```
- Currently I am running client as two seperate processes of datasink and datasource ( and this is the easiest way to start poking around the project )

- Running datasource
```bash 
cargo run -p client -- init_server=true
```
  
- Running datasink
```bash
cargo run -p client -- init_client=true
```
