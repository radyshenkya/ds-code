# DS-CODE
This is a bot for discord servers, which can compile and run small code blocks and return you a result.

It is using [docker](https://www.docker.com/) for running user code.
Bot is written on **Rust** using **[serenity](https://github.com/serenity-rs/serenity)** crate.

## Usage
Bot has only one command - `~run`
After `~run` you need to pass the multiline code block like this:

![image](https://user-images.githubusercontent.com/52829258/215334580-a9958608-f48f-44e2-8055-5ed8e22da7ff.png)

It is important to write the name of language that you are using.

After that the bot will reply on your message with output of this code like this:

![image](https://user-images.githubusercontent.com/52829258/215334727-d97bf057-c61e-443d-a4b8-339c0c344e3d.png)


## Installation
Firstly, you need to install **[docker](https://www.docker.com/)** and **[rust](https://www.rust-lang.org/)**.
After that, clone this repo on your machine.

Now you need to compile the docker image.
Go to `docker-image` folder inside your cloned repo, and run this command:  
```bash
docker build -t ds-code-user-code .
```

Now you need to create `.env` file inside the root folder of repository and fill it with this lines:
```env
DOCKER_HOST=<URL_TO_DOCKER_HOST>
DISCORD_TOKEN=<YOUR_BOT_TOKEN>
```

Where `DOCKER_HOST` - url for your docker host (as for me it is unix:///user/1000/docker.sock)
And `DISCORD_TOKEN` - token of your bot.

After that steps, finally you can run project with command `cargo run`
