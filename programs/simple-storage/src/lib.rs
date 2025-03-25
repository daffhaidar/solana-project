use anchor_lang::prelude::*;

// ID Program (Ganti setelah deploy)
declare_id!("GZLGG8Kh6BmnrZ4AETvYkeGWYg3eZ5cUFTwFdSspHuEC");

#[program]
mod simple_storage {
    use super::*;

    // Inisialisasi akun dengan nilai awal
    pub fn initialize(ctx: Context<Initialize>, stored_value: u64) -> Result<()> {
        let account = &mut ctx.accounts.storage_account;
        account.stored_value = stored_value;
        account.owner = *ctx.accounts.signer.key; // Simpan pemilik akun
        msg!("Initialized with value: {}!", stored_value);
        Ok(())
    }

    // Update nilai dalam akun (hanya pemilik akun yang bisa update)
    pub fn update(ctx: Context<Update>, new_value: u64) -> Result<()> {
        let account = &mut ctx.accounts.storage_account;
        require_keys_eq!(
            account.owner, 
            *ctx.accounts.signer.key, 
            StorageError::UnauthorizedSigner
        );
        account.stored_value = new_value;
        msg!("Updated stored value to: {}!", new_value);
        Ok(())
    }

    // Baca nilai dari akun
    pub fn get_value(ctx: Context<GetValue>) -> Result<u64> {
        let account = &ctx.accounts.storage_account;
        msg!("Reading stored value: {}", account.stored_value);
        Ok(account.stored_value)
    }
}

// Struktur akun
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init, 
        payer = signer, 
        space = STORAGE_ACCOUNT_SIZE, 
        seeds = [b"storage", signer.key().as_ref()], 
        bump
    )]
    pub storage_account: Account<'info, StorageAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut, has_one = owner)]
    pub storage_account: Account<'info, StorageAccount>,
    pub owner: SystemAccount<'info>, // **Tambahin ini**
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetValue<'info> {
    pub storage_account: Account<'info, StorageAccount>,
}

// Struct akun dengan data yang lebih jelas
#[account]
#[derive(Default)]
pub struct StorageAccount {
    pub stored_value: u64,
    pub owner: Pubkey, // Simpan pemilik akun
}

// Konstanta ukuran storage
const STORAGE_ACCOUNT_SIZE: usize = 8 + 8 + 32;

// Custom error handling
#[error_code]
pub enum StorageError {
    #[msg("Unauthorized signer.")]
    UnauthorizedSigner,
    #[msg("Invalid storage data.")]
    InvalidStorage,
}