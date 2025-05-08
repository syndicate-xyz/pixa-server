use serde_json;
use teloxide::{
    prelude::*,
    types::{InputFile, ParseMode},
    utils::markdown::link,
};
use url::Url;
use utils::{
    endpoints::vybe::{types::VybeTokenDetails, util::VYBE_TOKEN_API},
    http::HttpError,
    math::calculate_price_change,
    number::{format_decimal_price, format_long_number},
};

// Validate if a string is a valid Solana mint address
fn is_valid_solana_mint_address(address: &str) -> bool {
    // Simple validation: Solana addresses are base58 encoded and 32-44 characters long
    // This is a basic check and can be improved with more specific validation
    address.len() >= 32
        && address.len() <= 44
        && address
            .chars()
            .all(|c| (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '1' && c <= '9'))
}

// Function to display token details
async fn display_token_details(
    bot: &Bot,
    msg: &Message,
    token_details: &VybeTokenDetails,
) -> Result<(), teloxide::RequestError> {
    if let Some(logo_url) = &token_details.logo_url {
        // Parse URL, return early if it fails
        let url = match Url::parse(logo_url) {
            Ok(url) => url,
            Err(err) => {
                eprintln!("Failed to parse logo URL: {:?}", err);
                return Ok(());
            }
        };

        let vybe_token_url = |token_mint_address: &str| {
            format!(
                "https://alpha.vybenetwork.com/tokens/{}",
                token_mint_address
            )
        };

        bot.send_photo(msg.chat.id, InputFile::url(url))
            .caption(format!(
                "🟣*{}* ({}) \n\
                \n\
                *Token details* 📊\n\
                ├ Price: *${}* ({}%) \n\
                ├ MC: *{}*\n\
                ├ Supply: *{}*\n\
                ├ Vol (24h): *${}*\n\
                └ Verified: {}\n\
                \n\
                {} \n\
                └ {}
                ",
                token_details
                    .name
                    .as_ref()
                    .map_or("Unknown".to_string(), |s| s.clone()),
                token_details.symbol,
                format_decimal_price(token_details.price, None),
                format!(
                    "{:.*}",
                    2,
                    calculate_price_change(token_details.price, token_details.price_1d)
                ),
                format_long_number(token_details.market_cap),
                format_long_number(token_details.current_supply),
                format_long_number(token_details.usd_value_volume_24h.unwrap_or(0.0)),
                if token_details.verified {
                    "🟢"
                } else {
                    "🔴"
                },
                token_details.mint_address,
                link(
                    vybe_token_url(&token_details.mint_address).as_str(),
                    "Open with Vybe"
                )
            ))
            .parse_mode(ParseMode::Markdown)
            .await?;
    }

    Ok(())
}

pub async fn handle_message(bot: Bot, msg: Message) -> Result<(), teloxide::RequestError> {
    // Extract message text, returning early if none
    let text = msg.text().unwrap_or_default().to_string();

    // Check if the message is a valid Solana mint address
    if is_valid_solana_mint_address(&text) {
        // Use the global client instead of creating a new one
        let vybe_token_api = &VYBE_TOKEN_API;

        // Get token details, returning early if it fails
        let token_details = match vybe_token_api.get_token_details(text).await {
            Ok(details) => details,
            Err(err) => {
                eprintln!("Failed to get token details: {:?}", err);

                // Extract and display error message to the user
                if let HttpError::ApiError { status: _, body } = &err {
                    // Try to parse the JSON error message
                    if let Ok(error_json) = serde_json::from_str::<serde_json::Value>(body) {
                        if let Some(error_message) =
                            error_json.get("message").and_then(|m| m.as_str())
                        {
                            let error_text = format!("Error: {}", error_message);
                            if let Err(send_err) = bot.send_message(msg.chat.id, error_text).await {
                                eprintln!("Failed to send error message: {:?}", send_err);
                            }
                        }
                    }
                }

                return Ok(());
            }
        };

        println!("Token details {:?}", token_details);

        // Display token details
        display_token_details(&bot, &msg, &token_details).await?;
    }
    Ok(())
}
