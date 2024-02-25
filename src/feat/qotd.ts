/* Question of the Day:
   Creates a thread containing a question, where users can reply */
const { guildId } = require('./../data/wiggler.json');
const schedule = require('node-schedule');

const channelId = '747132041572909066';

// schedule the question of the day
schedule.scheduleJob('0 0 * * *', () => {
    postQuestionOfTheDay(guildId, channelId);
});

async function postQuestionOfTheDay(guildId, channelId) {
    const guild = await client.guilds.fetch(guildId);
    const channel = guild.channels.cache.get(channelId);

    // qotd list 
    const questionsOfTheDay = [
        "What's your favorite childhood memory?",
        "If you could have dinner with any historical figure, who would it be?",
    ];

    // random indexing 
    const randomIndex = Math.floor(Math.random() * questionsOfTheDay.length);
    const questionForToday = questionsOfTheDay[randomIndex];

    // Post the question in a thread
    const thread = await channel.threads.create({
        name: 'Question of the Day',
        autoArchiveDuration: 1440, // archive after 24 hours
    });

    thread.send(`Today\'s Question of the Day: ${questionForToday}`);
    console.log(`Async Function created \'qotd\' thread: ${thread.name}`);
}
