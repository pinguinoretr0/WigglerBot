const { SlashCommandBuilder } = require('discord.js');

module.exports = {
  data: new SlashCommandBuilder()
    .setName('whoami')
    .setDescription('Provides information about selected user.'),
  async execute(interaction) {
      if (interaction.user.username == "pinguinolibre") {
          await interaction.reply(`TLP, you are the author of Wiggler!`);
			} else {
        await interaction.reply(`Hello, you are: \`${interaction.user.username}\`, who joined on **${interaction.member.joinedAt}**.`);
			}
	  console.log(`Audit: \'whoami\' ran by: ${interaction.user.username}`);
  },
};
