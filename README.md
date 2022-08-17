# DateTimeLogger ( Work in progress )
Rust websocket listener logging a username with a timestamp in SQLite database

Proof of concept tested with chromes [Smart Websocket Client](https://chrome.google.com/webstore/detail/smart-websocket-client/omalebghpgejjiaoknljcfmglgbpocdp)

To run:

```shell
cargo run -- -n <user> [-i <ip-address> -p <port>] [-w] 
```

On first run, add a user in local mode. 
```shell
cargo run -- -n "Ville Vässla"
```

The program will reject any attempt to enter a username not already added into database from a local machine.

After this it will register any request by that user through a websocket.
```shell
cargo run -- -n "Ville Vässla" -i 127.0.0.1 -p 8080 -w
```


To get a flashy usage message run:

```bash
cargo run -- -h
```
