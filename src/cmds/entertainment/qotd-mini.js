/* A mini version of the 'Question of the Day feature' */
const { SlashCommandBuilder } = require('discord.js');

module.exports = {
	data: new SlashCommandBuilder()
		.setName('qotd-mini')
		.setDescription('A mini version of the \'Question of the day\' feature.'),
	async execute(interaction) {
		await interaction.reply('work in progress');
		console.log(`Audit: \'qotd-mini\' ran by: ${interaction.user.username}`);
	},
};
