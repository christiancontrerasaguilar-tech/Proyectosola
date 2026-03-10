use anchor_lang::prelude::*;

declare_id!("9K4XckYK45x9ykK2DN2Wp3gqanDnuPKCLGYthJJxsenH");

#[program]
pub mod user_profile {
    use super::*;

    // CREATE
    // Crea un perfil de usuario usando una PDA
    pub fn initialize_profile(
        ctx: Context<InitializeProfile>,
        name: String,
        bio: String,
    ) -> Result<()> {

        let profile = &mut ctx.accounts.profile;

        profile.authority = ctx.accounts.authority.key();
        profile.name = name;
        profile.bio = bio;

        Ok(())
    }

    // UPDATE
    // Permite modificar el nombre o bio del perfil
    pub fn update_profile(
        ctx: Context<UpdateProfile>,
        name: String,
        bio: String,
    ) -> Result<()> {

        let profile = &mut ctx.accounts.profile;

        profile.name = name;
        profile.bio = bio;

        Ok(())
    }

    // DELETE
    // Elimina el perfil y devuelve los SOL al usuario
    pub fn delete_profile(ctx: Context<DeleteProfile>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeProfile<'info> {

    #[account(
        init,
        payer = authority,
        space = 8 + Profile::INIT_SPACE,
        seeds = [b"profile", authority.key().as_ref()],
        bump
    )]
    pub profile: Account<'info, Profile>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateProfile<'info> {

    #[account(
        mut,
        seeds = [b"profile", authority.key().as_ref()],
        bump,
        has_one = authority
    )]
    pub profile: Account<'info, Profile>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeleteProfile<'info> {

    #[account(
        mut,
        close = authority,
        seeds = [b"profile", authority.key().as_ref()],
        bump,
        has_one = authority
    )]
    pub profile: Account<'info, Profile>,

    #[account(mut)]
    pub authority: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct Profile {

    // Wallet dueña del perfil
    pub authority: Pubkey,

    // Nombre del usuario
    #[max_len(50)]
    pub name: String,

    // Biografía del usuario
    #[max_len(200)]
    pub bio: String,
}
