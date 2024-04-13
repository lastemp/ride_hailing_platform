//! RegisterInstitution instruction handler

use {
    crate::{error::SolarCrowdFundingError, state::institution::Institution},
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
#[instruction(params: RegisterInstitutionParams)]
pub struct RegisterInstitution<'info> {
    // init means to create institution account
    // bump to use unique address for institution account
    #[account(
        init,
        payer = owner,
        space = 8 + Institution::INIT_SPACE,
        seeds = [b"institution", owner.key().as_ref()],
        bump
    )]
    pub institution: Account<'info, Institution>,
    // mut makes it changeble (mutable)
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct RegisterInstitutionParams {
    institution_name: String, // institution name
    country: String,          // home country of institution
}

// institution name length
const INSTITUTION_NAME_LENGTH: usize = 30;
// country length
const COUNTRY_LENGTH: usize = 3;
const COUNTRY_LENGTH_2: usize = 2;

pub fn register_institution(
    ctx: Context<RegisterInstitution>,
    params: &RegisterInstitutionParams,
) -> Result<()> {
    // validate inputs
    msg!("Validate inputs");
    if params.institution_name.as_bytes().len() > 0
        && params.institution_name.as_bytes().len() <= INSTITUTION_NAME_LENGTH
    {
    } else {
        return Err(SolarCrowdFundingError::InvalidFullNamesLength.into());
    }

    if params.country.as_bytes().len() != COUNTRY_LENGTH
        && params.country.as_bytes().len() != COUNTRY_LENGTH_2
    {
        return Err(SolarCrowdFundingError::InvalidInstitutionNameLength.into());
    }

    let institution = &mut ctx.accounts.institution;
    // * - means dereferencing
    institution.owner = *ctx.accounts.owner.key;
    institution.institution_name = params.institution_name.to_string();
    institution.country = params.country.to_string();
    institution.active = true;

    Ok(())
}
