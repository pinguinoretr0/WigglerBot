const { SlashCommandBuilder } = require('discord.js');

module.exports = {
  data: new SlashCommandBuilder()
    .setName('hello')
        .setDescription('Hello World from Wiggler!'),
    async execute(interaction) {
        await interaction.reply('Hello world!');
        console.log(`Audit: \'hello\' ran by: ${interaction.user.username}`);
    },
};
