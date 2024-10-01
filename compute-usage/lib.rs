// You can use the compute_fn macro to measure compute unit usage of different snippets of code.
compute_fn!("My message" => {
    // Your code here
});


// Both base58 encoding and concatenation are expensive operations:
// 11962 CU !!
// Base58 encoding is expensive, concatenation is expensive
compute_fn! { "Log a pubkey to account info" =>
    msg!("A string {0}", ctx.accounts.counter.to_account_info().key());
}

// 357 CU - string concatenation is expensive
compute_fn! { "Log a pubkey simple concat" =>
    msg!("A string {0}", "5w6z5PWvtkCd4PaAV7avxE6Fy5brhZsFdbRLMt8UefRQ");
}


// If you do want to log a pubkey, you can use .key() and .log()
// 262 cu
compute_fn! { "Log a pubkey" =>
    ctx.accounts.counter.to_account_info().key().log();
}

// Larger data types use more Compute Units overall.
// 357
compute_fn! { "Push Vector u64 " =>
    let mut a: Vec<u64> = Vec::new();
    a.push(1);
    a.push(1);
    a.push(1);
    a.push(1);
    a.push(1);
    a.push(1);
}

// 211 CU
compute_fn! { "Vector u8 " =>
    let mut a: Vec<u8> = Vec::new();
    a.push(1);
    a.push(1);
    a.push(1);
    a.push(1);
    a.push(1);
    a.push(1);
}

// Serialization and deserialization are both expensive operations depending on the account struct. If possible, use zero copy and directly interact with the account data .
// 6302 CU
pub fn initialize(_ctx: Context<InitializeCounter>) -> Result<()> {
    Ok(())
}

// 5020 CU
pub fn initialize_zero_copy(_ctx: Context<InitializeCounterZeroCopy>) -> Result<()> {
    Ok(())
}

// 108 CU - total CU including serialization 2600
let counter = &mut ctx.accounts.counter;
compute_fn! { "Borsh Serialize" =>
    counter.count = counter.count.checked_add(1).unwrap();
}

// 151 CU - total CU including serialization 1254
let counter = &mut ctx.accounts.counter_zero_copy.load_mut()?;
compute_fn! { "Zero Copy Serialize" =>
    counter.count = counter.count.checked_add(1).unwrap();
}

// You can optimize finding the PDAs after initialization by saving the bump into an account and using it in the future.
pub fn pdas(ctx: Context<PdaAccounts>) -> Result<()> {
    let program_id = Pubkey::from_str("5w6z5PWvtkCd4PaAV7avxE6Fy5brhZsFdbRLMt8UefRQ").unwrap();

    // 12,136 CUs
    compute_fn! { "Find PDA" =>
        Pubkey::find_program_address(&[b"counter"], ctx.program_id);
    }

    // 1,651 CUs
    compute_fn! { "Find PDA" =>
        Pubkey::create_program_address(&[b"counter", &[248_u8]], &program_id).unwrap();
    }

    Ok(())
}

#[derive(Accounts)]
pub struct PdaAccounts<'info> {
    #[account(mut)]
    pub counter: Account<'info, CounterData>,
    // 12,136 CUs when not defining the bump
    #[account(
        seeds = [b"counter"],
        bump
    )]
    pub counter_checked: Account<'info, CounterData>,
}

#[derive(Accounts)]
pub struct PdaAccounts<'info> {
    #[account(mut)]
    pub counter: Account<'info, CounterData>,
    // only 1600 if using the bump that is saved in the counter_checked account
    #[account(
        seeds = [b"counter"],
        bump = counter_checked.bump
    )]
    pub counter_checked: Account<'info, CounterData>,
}