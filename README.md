# Dominote Rust backend

Dominote API REST Written in Rust language.

### Getting started
#### *Linux distribution*

1. Install all dependencies required to get up the database:

```bash
make install
```

2. Create an `.env`file to manage enviroment variables:
```
HOST_URL={Host url name} # This could be an url as example.com or localhost
SERVER_PORT={Port number} # Something like port 8080 (Always is a unsigned integer)
DB_PASSWORD={Db secret password} #Password for security in your own database in postgres
```

3. Run the project base:
```bash
make run
```

4. Test the endpoints using curl:
``` bash
# We make a register first, you could repeat this step as many times as you need.
curl -X POST "http://{HOST_URL}:{SERVER_PORT}/player/register" -d '{"name": "John Doe"}' -H "Content-Type: application/json"

# Then, we call to get a list of players
curl -X GET "http://{HOST_URL}:{SERVER_PORT}/player/list"

```

And that is all for the moment. This is a easier way to use the app. 
More features are coming soon. So, Let's start to play with Dominote.