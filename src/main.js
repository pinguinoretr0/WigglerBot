const { Client, Collection, Events, GatewayIntentBits } = require('discord.js');
const { token } = require('./data/wiggler.json');

const fs = require('node:fs');
const path = require('node:path');
const client = new Client({ intents: [GatewayIntentBits.Guilds] });

client.commands = new Collection();
const foldersPath = [
		path.join(__dirname, 'cmds'),
		path.join(__dirname, 'cmds', 'entertainment'),
		path.join(__dirname, 'cmds', 'rwg'),
		path.join(__dirname, 'cmds', 'util'),
		path.join(__dirname, 'cmds', 'voice'),
		path.join(__dirname, 'feat')];
const commandFolders = foldersPath.flatMap(folder => fs.readdirSync(folder));

for (const folderPath of foldersPath) {
	if (!fs.existsSync(folderPath)) {
		console.log(`[INFO] Directory ${folderPath} does not exist.`);
		continue;
	}

	const commandFiles = fs.readdirSync(folderPath).filter(file => file.endsWith('.js'));
	for (const file of commandFiles) {
		const filePath = path.join(folderPath, file);
		const command = require(filePath);
		if ('data' in command && 'execute' in command) {
			client.commands.set(command.data.name, command);
		} else {
			console.log(`[WARNING] The command at ${filePath} is missing a required "data" or "execute" property.`);
		}
	}
}


/* This is the Wiggler Server that will print information
	 into the terminal */
client.once(Events.ClientReady, () => {
		console.log('Wiggler is ready for use!');
});

client.on(Events.InteractionCreate, async interaction => {
	if (!interaction.isChatInputCommand()) return;

	const command = client.commands.get(interaction.commandName);

	if (!command) return;

	try {
		await command.execute(interaction);
	} catch (error) {
		console.error(error);
		if (interaction.replied || interaction.deferred) {
			await interaction.followUp({ content: 'There was an error while executing this command!', ephemeral: true });
		} else {
			await interaction.reply({ content: 'There was an error while executing this command!', ephemeral: true });
		}
	}
});

client.login(token);
