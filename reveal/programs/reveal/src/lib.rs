pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("9Szz8bFZzV4CSv6WdhFFSLusYLQpTvzXCvtTZ78c19Dd");

// I don't wanr to call the module reveal so relax while I think of a

#[program]
pub mod revealer {
    use super::*;

    pub fn initialize(ctx: Context<RevealAccountConstraints>, id: u64) -> Result<()> {
        reveal::handler(ctx, id)
    }
}
