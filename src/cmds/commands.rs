use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

// Math Shit:
#[command]
pub async fn product(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
		let terms: Vec<f64> = args.iter::<f64>().collect::<Result<_, _>>()?;
		if terms.is_empty() {
				msg.reply(ctx, "No numbers provided for use.").await?;
				return Ok(());
		}
    let product: f64 = terms.iter().product();
    msg.channel_id.say(&ctx.http, format!("The product of your equation is: {}", product)).await?;
    Ok(())
}

#[command]
pub async fn quotient(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut terms: Vec<f64> = args.iter::<f64>().collect::<Result<_, _>>()?;
    if terms.is_empty() {
        msg.reply(ctx, "No numbers provided for use.").await?;
        return Ok(());
    }
    let first_number = terms.remove(0);
    let quotient = terms.into_iter().fold(first_number, |acc, x| acc / x);
    msg.reply(ctx, format!("The quotient of your equation is: {}", quotient)).await?;
    Ok(())
}

#[command]
pub async fn sum(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
		let terms: Vec<f64> = args.iter::<f64>().collect::<Result<_, _>>()?;
		if terms.is_empty() {
				msg.reply(ctx, "No numbers provided for use.").await?;
				return Ok(());
		}
    let sum: f64 = terms.iter().sum();
    msg.channel_id.say(&ctx.http, format!("The sum of your equation is: {}", sum)).await?;
    Ok(())
}

// &othr
#[command]
async fn echo(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
		let msgusr: Vec<String> = args.iter::<String>().collect::<Result<_, _>>()?;
		if msgusr.is_empty() {
				msg.reply(ctx, "No string of text provided.").await?;
				return Ok(());
		}
    let response = msgusr.join(" ");
    msg.reply(ctx, format!("You said: \"{}\"", response)).await?;
    Ok(())
}
