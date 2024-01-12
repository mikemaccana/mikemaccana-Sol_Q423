// Make space for the Offer/Escrow
impl Space for Offer {
    // ANCHOR DISCRUMINATOR is always 8 bytes
    // 8 bytes for the u64 = 64 bits is 8 bytes
    // 32 for every pubkey
    // u8s are 1 bytes
    // TODO: can    I use sizeOf() or similar?
    const INIT_SPACE: usize = 
        // Required for anything anchor
        ANCHOR_DISCRIMINATOR_SIZE 
        // id
        + 1 * U64_SIZE 
        // offer_token, desired_token
        + 2 * PUBKEY_SIZE 
        // desired_amount (offered amount is whatever amount the maker puts into the vault)
        + 1 * U64_SIZE 
        // bump, vault_bump
        + 2 * U8_SIZE; 
}
