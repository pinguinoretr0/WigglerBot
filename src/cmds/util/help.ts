const { SlashCommandBuilder } = require('discord.js');

module.exports = {
  data: new SlashCommandBuilder()
    .setName('help')
    .setDescription('Provides a description of Wiggy'),
    // TODO
    async execute(interaction) {
        await interaction.reply(`I am Wiggy the Wiggler!`);
        console.log(`Audit: \'help\' ran by: ${interaction.user.username}`);
    },
};
