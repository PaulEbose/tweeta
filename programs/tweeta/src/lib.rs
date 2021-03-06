use anchor_lang::prelude::*;

declare_id!("9GUcf7ZYVkn2xXM7ZRpTnZu35ktfvXT4Q2RgG2dDXGgs");

#[program]
pub mod tweeta {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, address: String) -> ProgramResult {
        // Get a reference to the account.
        let base_account = &mut ctx.accounts.base_account;
        // Initialize tweet_count.
        base_account.tweet_count = 0;
        // Verify user wallet address.
        base_account.verified_addresses.push(address);
        Ok(())
    }

    pub fn add_tweet(ctx: Context<AddTweet>, tweet_content: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;
        // Build the struct.
        let tweet = TweetStruct {
            tweet_content: tweet_content.to_string(),
            user_address: *user.to_account_info().key,
        };
        // Add it to the tweets vector.
        base_account.tweets.push(tweet);
        base_account.tweet_count += 1;
        Ok(())
    }
}

// Attach variables to the function contexts.
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddTweet<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

// Create a custom struct for tweet.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct TweetStruct {
    tweet_content: String,
    user_address: Pubkey,
}

// Tell Solana what we want to store on this account.
#[account]
pub struct BaseAccount {
    pub tweet_count: u64,
    pub tweets: Vec<TweetStruct>,
    pub verified_addresses: Vec<String>,
}
