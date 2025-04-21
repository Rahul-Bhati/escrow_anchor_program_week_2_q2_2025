
#[derive(account)]
#[derive(INIT_SPACE)]
pub struct Escrow{
    pub maker: PubKey,
    pub make_a : PubKey,
    pub make_b : PubKey,
    pub recieve : u64,
    pub seed : u64, // to make multiple escrow for one user
    pub bump : u64  // onchain bump store karlege after init the escrow so that we can re-use it and do not need to calculate it
} 