# DateTimeLogger ( Work in progress )
Rust websocket listener logging username and datetime in SQLite database

Proof of concept tested with chromes [Smart Websocket Client](https://chrome.google.com/webstore/detail/smart-websocket-client/omalebghpgejjiaoknljcfmglgbpocdp)

To run:

```shell
cargo run -- -n <user> [-i <ip-address> -p <port>] [-w] 
```

On first run, add a user in local mode. The program will reject any attempt to enter a username not already added into database from a local machine.


The <user> argument contains the name that is to be entered in the database.

The optional [-w] -flag is a boolean and is used to set the program to listen to a websocket, or to run a local test.

If the [-w] -flag is not set, the [-i and -p] arguments are unneccesary and thrown away. 

To get a flashy usage message run:

```bash
cargo run -- -h
```
