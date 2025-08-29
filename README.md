# Dominote Rust backend

Dominote API REST Written in Rust language.

### Getting started
#### *Linux distribution*

1- Install all dependencies required to get up the database:

```bash
make install
```

2- Run the project base:
```bash
make run
```

3- Test the endpoints using curl:
``` bash
# We make a register first, you could repeat this step as many times as you need.
curl -X POST "http://localhost:{APP_PORT}/player/register" -d '{"name": "John Doe"}' -H "Content-Type: application/json"

# Then, we call to get a list of players
curl -X GET "http://localhost:{APP_PORT}/player/list"

```

And that is all for the moment. This is a easier way to use the app. 
More features are coming soon. So, Let's start to play with Dominote.