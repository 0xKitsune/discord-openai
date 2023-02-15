# Discord x OpenAI

Note: These docs are minimal at the moment but will be updated with more detail in the future.

`discord-openai` brings a variety of OpenAI models to your Discord channel. Currently, the Chat-GPT API is not available, however once the endpoint becomes public it will be integrated into this bot. Once the bot is added to your discord channel, you can use the `/davinci` slash command. 


## Getting Started
Getting `discord-openai` added to your Discord channel is simple and straightforward. If you have added a Discord bot to your channel in the past, this will take 5 minutes. If you are new to Discord bots channel, the install and setup process will take closer to 15 minutes. 

To set up `Discord OpenAI`, you will need to [set up a new Discord Bot](https://discordpy.readthedocs.io/en/stable/discord.html) and get an [OpenAI API key](https://platform.openai.com/account/api-keys). 

Once you have created a new bot, enable the following scopes and permissions.

Scopes:

```
- bot
- applications.commands
```

Bot Permissions:

```
- Read Messages/View Channels
- Send Messages
```

Once these permissions are enabled, you can [invite the bot to your server](https://discordpy.readthedocs.io/en/stable/discord.html#inviting-your-bot). Once the bot has successfully joined the server, all that is left is to start up `discord-openai`!


## Installing `discord-openai`

First, make sure that you have [Rust installed](https://www.rust-lang.org/tools/install) which will allow you to install / compile the program. Once Rust is installed you can install `discord-openai` with `cargo` (the  Rust package manager) or directly from the source code. To install via cargo, simply enter the following command in your terminal.

```
cargo install discord-openai
```

To install from the source code, you can run the following commands.

```
git clone https://github.com/0xKitsune/discord-openai.git
cd discord-openai
cargo install --path .
```

Congratulations, now everything is installed.


## Running the program

To run `discord-openai`, you will need a [Discord Bot Token](https://docs.discordbotstudio.org/setting-up-dbs/finding-your-bot-token) and an [OpenAI API key](https://platform.openai.com/account/api-keys). When `discord-openai` is run, it looks for these values either via command line arguments or environment variables.

If you would like to use command line arguments, you can start the program by running the following command in your terminal, replacing the `<placeholders>` with your API key and bot token.

```
discord-openai --openai_api_key <api_key> --bot_token <token>
```

If you would like to run the program by using environment variables, simply set an environment variable called `DISCORD_OPENAI_BOT_TOKEN` for the bot token and `OPENAI_API_KEY` for the api key.

Once the environment variables are set, you can run the following command in your terminal to start the program.

```
discord-openai
```

After starting the program, you should see `your_bots_name_here is connected!` in your terminal. To make sure everything is working properly, you can use the `/ping` slash command in any channel in your server and your bot should reply with a `Pong` message.



## Running with Docker

If you are using environment variables, set environment variables within the `Dockerfile` called `DISCORD_OPENAI_BOT_TOKEN` for the bot token and `OPENAI_API_KEY` for the api key.

After adding environment variables to the Dockerfile, to run the program with docker, simply build the image and run the container with the following command.

 ```
 sudo docker -t <your-image-name> . 
 sudo docker run -t <your-image-name>
```

If you choose not to set environment variables in the `Dockerfile`, you can pass the bot token and API key as command line arguments when starting the container, as seen below.

```
sudo docker -t <your-image-name> .
sudo docker run -t <your-image-name> --openai_api_key <api_key> --bot_token <token>
```

You can use this Dockerfile to deploy on any machine or a cloud provider that supports Docker.


## Slash Command Usage
Once `discord-openai` is up and running, you can use the `/davinci` slash command to ask questions, generate code snippets and anything else you can think of. Right now, `discord-openai` uses OpenAI's `text-davinci-003` model, however other models will be added as additional slash commands, including ChatGPT once it is available.

You can also use the `/ping` slash command to check if `discord-ai` is up and running at anytime.

## Current Limitations

- As mentioned previously, `discord-openai` uses the `text-davinci-003` model. In the future, more models will be integrated into the program as they are released from OpenAI

- Through the OpenAI API, there is no way to track multiple conversations at the same time with one model. Due to this, you can only ask one question at a time rather than having many conversations at once. As the OpenAI API is updated, `discord-openai` will be updated as well. 
