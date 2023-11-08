# Remote Central
Need access to your primary desktop for a long time but run into issues (random freezes, IP changes, etc)?

Remote Central will help you with this endeavour by syncing your desktop/s with a central server. This will let you keep track of online machines and issue commands as needed.

## Setting up

You will need a server to sync your desktop/s with. Any server Rust can compile on will do. Ideally, it will have a static IP or a domain name so your computers will never have trouble finding the server.

Then setup the remote-server and remote-cli on your server machine. The remote-server handles the ping requests from your desktop clients, and the remote-cli lets you manage your machines from the server (I didn't need a fancy desktop GUI at the time but this may come in the future).

And finally on your desktop/s, setup the remote-client. On my Windows machine I have this setup as a scheduled task to run every 5 minutes to ping the server.
