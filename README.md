# DateTimeLogger ( Work in progress )
Rust websocket listener logging username and datetime in SQLite database

Proof of concept tested with chromes [Smart Websocket Client](https://chrome.google.com/webstore/detail/smart-websocket-client/omalebghpgejjiaoknljcfmglgbpocdp)

To run, write

```shell
cargo run -- -n <user> [-i <ip-address> -p <port>] [-w] 
```

On first run, add a user in local mode. The websocket will reject and panic! any user not already in the database.


The <user> argument contains the name that is to be entered in the database.

The optional [-w] -flag is a boolean and is used to set the program to listen to a websocket, or to run a local test.

If the [-w] -flag is not set, the [-i and -p] arguments are unneccesary and thrown away. 

Run 
```bash
cargo run -- -h
```
to get a flashy usage message
