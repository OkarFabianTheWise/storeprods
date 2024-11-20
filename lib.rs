use anchor_lang::prelude::*;

// Unique identifier for the program
declare_id!("865m9ePhc85sKxN5LgTzYkxG3hQWiwgfxfuzGQUjjiCM");

#[program] // Main entry point of the program
pub mod stock_product {
    use super::*;

    // Initializes the account to store products
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let store_account = &mut ctx.accounts.store_account; // Access the account
        store_account.products = Vec::new(); // Start with an empty list of products
        Ok(())
    }

    // Records a new product in the store
    pub fn record_incoming(
        ctx: Context<RecordIncoming>,
        item: String,   // Product name
        price: String,  // Product price
        quantity: i64,  // Quantity of the product
        entrydate: i64, // Timestamp of when it was added
    ) -> Result<()> {
        let store_account = &mut ctx.accounts.store_account; // Access the account
        let store = Store { item, price, quantity, entrydate }; // Create a new product record
        store_account.products.push(store); // Add the record to the list
        Ok(())
    }

    // Retrieves all products stored in the account
    pub fn check_store(ctx: Context<CheckStore>) -> Result<Vec<Store>> {
        let store_account = &ctx.accounts.store_account; // Access the account
        if store_account.products.is_empty() {
            // If the list is empty
            return Err(ErrorCode::NoProducts.into()); // Return an error
        }
        Ok(store_account.products.clone()) // Return a copy of the product list
    }
}

// Context for initializing the account
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 1000)] // Creates the account with space for data
    pub store_account: Account<'info, StoreAccount>, // The account to store products
    #[account(mut)]
    pub user: Signer<'info>, // The user creating the account
    pub system_program: Program<'info, System>, // System program to handle account creation
}

// Context for recording a new product
#[derive(Accounts)]
pub struct RecordIncoming<'info> {
    #[account(mut)]
    pub store_account: Account<'info, StoreAccount>, // The account storing products
}

// Context for checking stored products
#[derive(Accounts)]
pub struct CheckStore<'info> {
    #[account(mut)]
    pub store_account: Account<'info, StoreAccount>, // The account storing products
}

// Structure for a product record
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Store {
    pub item: String,  // Product name
    pub price: String, // Product price
    pub quantity: i64, // Quantity of the product
    pub entrydate: i64, // Timestamp of when it was added
}

// Account structure to hold multiple products
#[account]
pub struct StoreAccount {
    pub products: Vec<Store>, // List of stored products
}

// Error codes for the program
#[error_code]
pub enum ErrorCode {
    #[msg("No products in store.")] // Error when no products are found
    NoProducts,
}
