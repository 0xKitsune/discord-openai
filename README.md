# Discord x OpenAI

Note: These docs are minimal at the moment but will be updated in more detail in the future.

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




## Running with Docker



### Slash Command Usage

### Current Limitations


