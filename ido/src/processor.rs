use crate::state::AuctionBuffer;
use crate::{
    error::IdoError,
    error::IdoError::{
        AllocationDateHasNotArrived, AuctionAccessError, ExceedTheRaisingLimit,
        InsufficientBalance, InvalidInstruction, InvalidSeeds, NonProjectPartyPublicKey,
        NonWhitelist, NotAdministrator, NotExistUserAda, SignatureError, StatusIsNotEnded,
        StatusIsNotOpen, SuccessFlagError, TheFundraisingIsFull, TheNumberOfSubmissionsCannotBe0,
        TimestampError, WithdrawnZero,StatusHasEnded,MinSwapLevelInputError,AllocationInputError,
        WhitelistLenTooLong,
    },
    instruction::Instruction,
    state::AccountData,
};
use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use md5;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction as SysInstruction},
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey, 
    rent::Rent,
    system_instruction,
    sysvar::{clock::Clock, Sysvar},
};
use spl_associated_token_account::get_associated_token_address;
use spl_token::state::{Account as TokenAccount, Mint};
use std::{
    cmp::{Ord, Ordering},
    convert::TryInto,
    str::FromStr,
    char,
};

pub struct Processor;
impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = Instruction::unpack(instruction_data)?;

        match instruction {
            Instruction::ModifyAdministratorAccount { method } => {
                msg!("Instruction: ModifyAdministratorAccount");
                Self::process_modify_administrator_account(program_id, accounts, method)
            }
            Instruction::SaveAuctionData { auction_data } => {
                msg!("Instruction: SaveAuctionData");
                Self::process_save_auction_data(program_id, accounts, &auction_data)
            }
            Instruction::ManualFailure => {
                msg!("Instruction: ManualFailure");
                Self::process_manual_failure(program_id, accounts, instruction_data)
            }
            Instruction::ProjectPartyWithdraw => {
                msg!("Instruction: ProjectPartyWithdraw");
                Self::process_project_party_withdraw(program_id, accounts, instruction_data)
            }
            Instruction::Submit { amount } => {
                msg!("Instruction: Submit");
                Self::process_submit(program_id, accounts, amount)
            }
            Instruction::Claim => {
                msg!("Instruction: Claim");
                Self::process_claim(program_id, accounts, instruction_data)
            }
            Instruction::ModifyWhiteList { whitelist,offset,end_flag } => {
                msg!("Instruction: ModifyWhiteList");
                Self::process_modifywhitelist(program_id, accounts, whitelist,offset,end_flag)
            }
            Instruction::ProcessDevided => {
                msg!("Instruction: ProcessDevided");
                Self::process_devided(program_id, accounts)
            }
        }
    }
    fn process_modify_administrator_account(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        method: u8,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let check = next_account_info(account_info_iter)?;
        if check
            .key
            .ne(&Pubkey::from_str("AwUk5nxvLreRgDbXTRLsZDLgwJfEmveFsqnrnbedWUhu").unwrap())
            && (check.is_signer == true)
        {
            return Err(NotAdministrator.into());
        }
        let admin = next_account_info(account_info_iter)?;
        msg!("1");
        let buffer = account_info_iter
            .map(|a| a.key.to_bytes())
            .collect::<Vec<[u8; 32]>>();
            msg!("1");
        let mut account_data =
            Vec::<[u8; 32]>::deserialize(&mut &admin.data.borrow_mut()[..]).unwrap();
            msg!("1");
        let account_data_member_len = account_data.len();
        match method {
            //新增
            0 => {
                msg!("Add admin member");
                if account_data_member_len == 0 {
                    buffer.serialize(&mut &mut admin.data.borrow_mut()[..])?;
                } else {
                    account_data.extend(buffer.iter());
                    account_data.sort();
                    account_data.dedup();
                    account_data.serialize(&mut &mut admin.data.borrow_mut()[..])?;
                }
            }
            //删除
            1 => {
                msg!("delete admin member");
                if account_data_member_len == 0 {
                    msg!("None Admin");
                } else {
                    account_data.retain(|a| !buffer.iter().any(|i| i == a));
                    account_data.serialize(&mut &mut admin.data.borrow_mut()[..])?;
                }
            }
            _ => return Err(InvalidInstruction.into()),
        }
        Ok(())
    }

    fn process_save_auction_data(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        auction_data: &AuctionBuffer,
    ) -> ProgramResult {
        msg!("radio {:?}", auction_data.ratio);
        let account_info_iter = &mut accounts.iter();
        if auction_data.min_allocation >= auction_data.max_allocation {
                return Err(AllocationInputError.into());
        }
        if auction_data.min_swap_level > auction_data.tokens_offered{
                return Err(MinSwapLevelInputError.into());
        }
        if auction_data.access == 1 {
            if auction_data.whitelist.len() == 0 {
                return Err(NonWhitelist.into());
            }
        }
        let check = next_account_info(account_info_iter)?;
        let key = next_account_info(account_info_iter)?;
        Self::administrator_verification(check, key)?;
        let auction_save_account = next_account_info(account_info_iter)?;
        auction_data.serialize(&mut &mut auction_save_account.data.borrow_mut()[..])?;
        let funder_info = next_account_info(account_info_iter)?;
        let program_devided_account_info = next_account_info(account_info_iter)?;

        let program_associated_account_info_underlay = next_account_info(account_info_iter)?;
        let program_associated_account_info_project = next_account_info(account_info_iter)?;
        let spl_token_mint_info_project = next_account_info(account_info_iter)?;
        let spl_token_mint_info_underlay = next_account_info(account_info_iter)?;
        let spl_token_program_info = next_account_info(account_info_iter)?;
        let spl_associated_program_info = next_account_info(account_info_iter)?;
        let system_program_info = next_account_info(account_info_iter)?;
        let rent_sysvar_info = next_account_info(account_info_iter)?;
        Self::process_devided(
            program_id,
            &[
                funder_info.clone(),
                program_devided_account_info.clone(),
                program_associated_account_info_project.clone(),
                spl_token_mint_info_project.clone(),
                spl_token_program_info.clone(),
                spl_associated_program_info.clone(),
                system_program_info.clone(),
                rent_sysvar_info.clone(),
                auction_save_account.clone(),
            ],
        )?;
        Self::process_devided(
            program_id,
            &[
                funder_info.clone(),
                program_devided_account_info.clone(),
                program_associated_account_info_underlay.clone(),
                spl_token_mint_info_underlay.clone(),
                spl_token_program_info.clone(),
                spl_associated_program_info.clone(),
                system_program_info.clone(),
                rent_sysvar_info.clone(),
                auction_save_account.clone(),
            ],
        )?;
        msg!("done");
        Ok(())
    }

    fn process_manual_failure(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        _instruction_data: &[u8],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let check = next_account_info(account_info_iter)?;
        let key = next_account_info(account_info_iter)?;
        Self::administrator_verification(check, key)?;
        let auction_save_account = next_account_info(account_info_iter)?;
        let timer = &Clock::from_account_info(next_account_info(account_info_iter)?)?;

        let mut auction_save_account_data: AuctionBuffer =
            BorshDeserialize::deserialize(&mut &auction_save_account.data.borrow_mut()[..])
                .unwrap();

        let current_status = Self::current_status(
            auction_save_account_data.pool_opens,
            auction_save_account_data.pool_closes,
            timer.unix_timestamp as u32,
        )?;
        if current_status == "Ended".to_string() {
            return Err(StatusHasEnded.into());
        }

        auction_save_account_data.success_flag = 0;
        auction_save_account_data
            .serialize(&mut &mut auction_save_account.data.borrow_mut()[..])?;
        Ok(())
    }
    
    fn process_project_party_withdraw(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        _instruction_data: &[u8],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let check = next_account_info(account_info_iter)?;
        let key = next_account_info(account_info_iter)?;
        Self::administrator_verification(check, key)?;
        let timer = &Clock::from_account_info(next_account_info(account_info_iter)?)?;
        let auction_save_account = next_account_info(account_info_iter)?;
        let program_devided_account_info = next_account_info(account_info_iter)?;

        let program_associated_account_info_project = next_account_info(account_info_iter)?;
        let program_associated_account_info_underlay = next_account_info(account_info_iter)?;
        let project_native_ada = next_account_info(account_info_iter)?;
        let project_other_ada = next_account_info(account_info_iter)?;
        let token_program = next_account_info(account_info_iter)?;
        let spl_token_account_project = next_account_info(account_info_iter)?;
        let spl_token_account_underlay = next_account_info(account_info_iter)?;
        //
        let mut auction_save_account_data: AuctionBuffer =
            BorshDeserialize::deserialize(&mut &auction_save_account.data.borrow_mut()[..])
                .unwrap();
        let (program_devided, bump_seed) =
            Self::program_devided_address(&program_id, auction_save_account_data.auction_id);
        let mut auction_seed = [0u8; 4];
        auction_save_account_data
            .auction_id
            .serialize(&mut auction_seed.as_mut())
            .unwrap();
        let devided_signer_seeds: &[&[_]] = &[Self::program_seeds(), &auction_seed, &[bump_seed]];
        let source_account_project = Self::program_associated_adress(
            &program_id,
            &spl_token_account_project.key,
            auction_save_account_data.auction_id,
        );
        let source_account_underlay = Self::program_associated_adress(
            &program_id,
            &spl_token_account_underlay.key,
            auction_save_account_data.auction_id,
        );
        if program_devided != *program_devided_account_info.key {
            return Err(InvalidSeeds.into());
        }
        if source_account_project != *program_associated_account_info_project.key {
            return Err(InvalidSeeds.into());
        }
        if source_account_underlay != *program_associated_account_info_underlay.key {
            return Err(InvalidSeeds.into());
        }
        if auction_save_account_data.success_flag == 1 {
            let current_status = Self::current_status(
                auction_save_account_data.pool_opens,
                auction_save_account_data.pool_closes,
                timer.unix_timestamp as u32,
            )?;
            //状态检查
            if current_status != "Ended".to_string() {
                if auction_save_account_data.funds_raised == auction_save_account_data.tokens_offered{

                }
                else{
                    return Err(StatusIsNotEnded.into());
                }
            }
            //状态是否筹集成功
            if auction_save_account_data.funds_raised
                < auction_save_account_data.min_swap_level.into()
            {
                auction_save_account_data.success_flag = 0;
            }
        }
        let mint_info_project = Mint::unpack(&spl_token_account_underlay.data.borrow())?;
        let mint_info_underlay = Mint::unpack(&spl_token_account_underlay.data.borrow())?;
        match auction_save_account_data.success_flag {
            1 => {
                if project_other_ada.key.to_bytes()
                    != auction_save_account_data.underlay_program_id_derive
                {
                    return Err(NonProjectPartyPublicKey.into());
                }
                invoke_signed(
                    &spl_token::instruction::transfer_checked(
                        &token_program.key,
                        &program_associated_account_info_underlay.key,
                        &spl_token_account_underlay.key,
                        &project_other_ada.key,
                        &program_devided_account_info.key,
                        &[],
                        auction_save_account_data.funds_raised,
                        mint_info_underlay.decimals,
                    )?,
                    &[
                        spl_token_account_underlay.clone(),
                        token_program.clone(),
                        program_associated_account_info_underlay.clone(),
                        project_other_ada.clone(),
                        program_devided_account_info.clone(),
                    ],
                    &[devided_signer_seeds],
                )?;
                if auction_save_account_data.funds_raised
                    != auction_save_account_data.tokens_offered
                {
                    let var1 = auction_save_account_data
                        .tokens_offered
                        .checked_sub(auction_save_account_data.funds_raised)
                        .ok_or(IdoError::CalculationOverflow)?;
                    let mut var2 = var1
                        .checked_mul(auction_save_account_data.ratio.into())
                        .ok_or(IdoError::CalculationOverflow)?;
                    var2 = var2
                        .checked_div(10000)
                        .ok_or(IdoError::CalculationOverflow)?;
                    let mut transfer_amount = var2;
                    msg!("mint_info_underlay.decimals {:?}  mint_info_project.decimals {:?} transfer_amount {:?}",
                        mint_info_underlay.decimals,
                        mint_info_project.decimals, 
                        transfer_amount);
                    transfer_amount = Self::compare_decimals(
                        mint_info_underlay.decimals,
                        mint_info_project.decimals,
                        transfer_amount,
                    )?;
                    // let program_associated_account_info_project_unpack =
                    //     TokenAccount::unpack_from_slice(
                    //         &program_associated_account_info_project.data.borrow(),
                    //     )?;
                    // msg!(
                    //     "program_associated_account_info_project {:?}",
                    //     program_associated_account_info_project_unpack
                    // );
                    // let project_native_ada_unpack =
                    //     TokenAccount::unpack_from_slice(&project_native_ada.data.borrow())?;
                    // msg!("project_native_ada_unpack {:?}", project_native_ada_unpack);
                    if project_native_ada.key.to_bytes()
                        != auction_save_account_data.spl_program_id_derive
                    {
                        return Err(NonProjectPartyPublicKey.into());
                    }
                    invoke_signed(
                        &spl_token::instruction::transfer_checked(
                            &token_program.key,
                            &program_associated_account_info_project.key,
                            &spl_token_account_project.key,
                            &project_native_ada.key,
                            &program_devided_account_info.key,
                            &[],
                            transfer_amount,
                            mint_info_project.decimals,
                        )?,
                        &[
                            spl_token_account_project.clone(),
                            token_program.clone(),
                            program_associated_account_info_project.clone(),
                            project_native_ada.clone(),
                            program_devided_account_info.clone(),
                        ],
                        &[devided_signer_seeds],
                    )?;
                }
            }
            0 => {
                msg!("0");
                let mut transfer_amount = auction_save_account_data
                    .tokens_offered
                    .checked_mul(auction_save_account_data.ratio.into())
                    .ok_or(IdoError::CalculationOverflow)?;
                transfer_amount = transfer_amount
                    .checked_div(10000)
                    .ok_or(IdoError::CalculationOverflow)?;
                msg!("mint_info_underlay.decimals {:?}  mint_info_project.decimals {:?} transfer_amount {:?}",
                    mint_info_underlay.decimals,
                    mint_info_project.decimals, 
                    transfer_amount);
                transfer_amount = Self::compare_decimals(
                    mint_info_underlay.decimals,
                    mint_info_project.decimals,
                    transfer_amount,
                )?;

                if project_native_ada.key.to_bytes()
                    != auction_save_account_data.spl_program_id_derive
                {
                    return Err(NonProjectPartyPublicKey.into());
                }
                invoke_signed(
                    &spl_token::instruction::transfer_checked(
                        &token_program.key,
                        &program_associated_account_info_project.key,
                        &spl_token_account_project.key,
                        &project_native_ada.key,
                        &program_devided_account_info.key,
                        &[],
                        transfer_amount,
                        mint_info_project.decimals,
                    )?,
                    &[
                        spl_token_account_project.clone(),
                        token_program.clone(),
                        program_associated_account_info_project.clone(),
                        project_native_ada.clone(),
                        program_devided_account_info.clone(),
                    ],
                    &[devided_signer_seeds],
                )?;
            }
            _ => {
                return Err(SuccessFlagError.into());
            }
        };
        auction_save_account_data
            .serialize(&mut &mut auction_save_account.data.borrow_mut()[..])?;
        Ok(())
    }

    fn process_submit(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        if amount == 0 {
            return Err(TheNumberOfSubmissionsCannotBe0.into());
        }
        let user_account = next_account_info(account_info_iter)?; //用户
        if !user_account.is_signer {
            return Err(SignatureError.into());
        }
        let ada_account = next_account_info(account_info_iter)?; //用户关联账号
        let user_devided_account = next_account_info(account_info_iter)?; //用户派生地址
        let program_devided_account_info = next_account_info(account_info_iter)?; //program派生地址
        let program_associated_account_info_underlay = next_account_info(account_info_iter)?; //program spl ada

        let auction_save_account = next_account_info(account_info_iter)?; //拍卖数据
        let system_program_info = next_account_info(account_info_iter)?;
        let current_program_info = next_account_info(account_info_iter)?;
        let timer = &Clock::from_account_info(next_account_info(account_info_iter)?)?;
        let mut auction_save_account_data: AuctionBuffer =
            BorshDeserialize::deserialize(&mut &auction_save_account.data.borrow_mut()[..])
                .unwrap();
        if auction_save_account_data.success_flag == 0 {
            return Err(SuccessFlagError.into());
        }
        let current_status = Self::current_status(
            auction_save_account_data.pool_opens,
            auction_save_account_data.pool_closes,
            timer.unix_timestamp as u32,
        )?;
        if current_status != "Open".to_string() {
            return Err(StatusIsNotOpen.into());
        }
        let (check_user_devided_account, seed) = Self::get_user_derived_adress(
            auction_save_account_data.auction_id,
            user_account.key,
            program_id,
        )
        .unwrap();
        if user_devided_account.key != &check_user_devided_account {
            return Err(InvalidSeeds.into());
        }
        match auction_save_account_data.access {
            0 => {}
            1 => {
                if !auction_save_account_data
                    .whitelist
                    .iter()
                    .any(|i| i == &user_account.key.to_bytes())
                {
                    return Err(NonWhitelist.into());
                }
            }
            _ => {
                return Err(AuctionAccessError.into());
            }
        }
        let token_program = next_account_info(account_info_iter)?;
        let spl_token_account = next_account_info(account_info_iter)?;
        let rent_sysvar_info = next_account_info(account_info_iter)?;
        let (program_devided, _) =
            Self::program_devided_address(&program_id, auction_save_account_data.auction_id);
        let dest_account = Self::program_associated_adress(
            &program_id,
            &spl_token_account.key,
            auction_save_account_data.auction_id,
        );
        if program_devided != *program_devided_account_info.key {
            return Err(InvalidSeeds.into());
        }
        if dest_account != *program_associated_account_info_underlay.key {
            return Err(InvalidSeeds.into());
        }

        let space: usize = 16;
        let rent = &Rent::from_account_info(rent_sysvar_info)?;
        let required_lamports = rent
            .minimum_balance(space)
            .max(1)
            .saturating_sub(user_devided_account.lamports());

        if required_lamports > 0 {
            msg!("create user derived account");
            //转账lamports
            invoke(
                &system_instruction::transfer(
                    user_account.key,
                    user_devided_account.key,
                    required_lamports,
                ),
                &[
                    user_account.clone(),
                    user_devided_account.clone(),
                    system_program_info.clone(),
                ],
            )?;
            //分配空间
            invoke(
                &system_instruction::allocate_with_seed(
                    &user_devided_account.key, //目标
                    &user_account.key,         //用户账户
                    &seed,                     //seed
                    space as u64,              //space
                    &program_id,               //owner
                ),
                &[
                    user_devided_account.clone(),
                    user_account.clone(),
                    current_program_info.clone(), //program
                ],
            )?;
            //owner change
            invoke(
                &system_instruction::assign_with_seed(
                    &user_devided_account.key,
                    &user_account.key,
                    &seed,
                    &program_id,
                ),
                &[
                    user_devided_account.clone(),
                    user_account.clone(),
                    current_program_info.clone(),
                ],
            )?;
        }
        if ada_account.data.borrow().len() < TokenAccount::LEN {
            return Err(NotExistUserAda.into());
        }
        let ada_account_unpack = TokenAccount::unpack_from_slice(&ada_account.data.borrow())?;
        //余额是否充足
        if ada_account_unpack.amount < amount {
            return Err(InsufficientBalance.into());
        }
        //检查项目是否完成筹集
        if auction_save_account_data.funds_raised == auction_save_account_data.tokens_offered {
            return Err(TheFundraisingIsFull.into());
        }
        //submit + 项目已筹集数 是否大于最大筹集数
        if (auction_save_account_data
            .funds_raised
            .checked_add(amount)
            .ok_or(IdoError::CalculationOverflow))?
            > auction_save_account_data.tokens_offered
        {
            return Err(ExceedTheRaisingLimit.into());
        }
        let mut saving_account_data: AccountData =
            BorshDeserialize::deserialize(&mut &user_devided_account.data.borrow_mut()[..])
                .unwrap();
        //submit数量不能小于最小提交数量 大于最大提交数量
        if (auction_save_account_data.min_allocation as u64)
            > (saving_account_data
                .contribution
                .checked_add(amount)
                .ok_or(IdoError::CalculationOverflow)?)
            || (auction_save_account_data.max_allocation as u64)
                < (saving_account_data
                    .contribution
                    .checked_add(amount)
                    .ok_or(IdoError::CalculationOverflow)?)
        {
            return Err(TheNumberOfSubmissionsCannotBe0.into());
        }
        let mint_info = Mint::unpack(&spl_token_account.data.borrow())?;
        invoke(
            &spl_token::instruction::transfer_checked(
                &token_program.key,
                &ada_account.key,
                &spl_token_account.key,
                &program_associated_account_info_underlay.key,
                &user_account.key,
                &[],
                amount,
                mint_info.decimals,
            )?,
            &[
                spl_token_account.clone(),
                user_account.clone(),
                token_program.clone(),
                ada_account.clone(),
                program_associated_account_info_underlay.clone(),
                program_devided_account_info.clone(),
            ],
        )?;

        saving_account_data.contribution = saving_account_data
            .contribution
            .checked_add(amount)
            .ok_or(IdoError::CalculationOverflow)?;
        saving_account_data.withdrawn = saving_account_data
            .withdrawn
            .checked_add(amount)
            .ok_or(IdoError::CalculationOverflow)?;
        saving_account_data.serialize(&mut &mut user_devided_account.data.borrow_mut()[..])?;
        auction_save_account_data.funds_raised = auction_save_account_data
            .funds_raised
            .checked_add(amount)
            .ok_or(IdoError::CalculationOverflow)?;
        auction_save_account_data
            .serialize(&mut &mut auction_save_account.data.borrow_mut()[..])?;
        Ok(())
    }

    fn process_claim(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        _instruction_data: &[u8],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let user_account = next_account_info(account_info_iter)?;
        if user_account.is_signer != true {
            return Err(SignatureError.into());
        }
        let auction_save_account = next_account_info(account_info_iter)?;
        let timer = &Clock::from_account_info(next_account_info(account_info_iter)?)?;
        let system_account = next_account_info(account_info_iter)?;
        let rent_account = next_account_info(account_info_iter)?;
        let rent = &Rent::from_account_info(rent_account)?;
        let spl_associated_program_info = next_account_info(account_info_iter)?;

        let user_devided_account = next_account_info(account_info_iter)?; //用户派生地址
        let program_devided_account_info = next_account_info(account_info_iter)?;
        let program_associated_account_info_project = next_account_info(account_info_iter)?;
        let program_associated_account_info_underlay = next_account_info(account_info_iter)?;

        let user_native_ada = next_account_info(account_info_iter)?;
        let user_other_ada = next_account_info(account_info_iter)?;
        let token_program = next_account_info(account_info_iter)?;
        let spl_token_account_project = next_account_info(account_info_iter)?;
        let spl_token_account_underlay = next_account_info(account_info_iter)?;
        //
        let auction_save_account_data: AuctionBuffer =
            BorshDeserialize::deserialize(&mut &auction_save_account.data.borrow_mut()[..])
                .unwrap();
        let (program_devided, bump_seed) =
            Self::program_devided_address(&program_id, auction_save_account_data.auction_id);
        let mut auction_seed = [0u8; 4];
        auction_save_account_data
            .auction_id
            .serialize(&mut auction_seed.as_mut())
            .unwrap();
        let devided_signer_seeds: &[&[_]] = &[Self::program_seeds(), &auction_seed, &[bump_seed]];
        let source_account_project = Self::program_associated_adress(
            &program_id,
            &spl_token_account_project.key,
            auction_save_account_data.auction_id,
        );
        let source_account_underlay = Self::program_associated_adress(
            &program_id,
            &spl_token_account_underlay.key,
            auction_save_account_data.auction_id,
        );
        if program_devided != *program_devided_account_info.key {
            return Err(InvalidSeeds.into());
        }
        if source_account_project != *program_associated_account_info_project.key {
            return Err(InvalidSeeds.into());
        }
        if source_account_underlay != *program_associated_account_info_underlay.key {
            return Err(InvalidSeeds.into());
        }
        //
        let mut auction_save_account_data: AuctionBuffer =
            BorshDeserialize::deserialize(&mut &auction_save_account.data.borrow_mut()[..])
                .unwrap();
        //
        let (check_user_devided_account, _) = Self::get_user_derived_adress(
            auction_save_account_data.auction_id,
            user_account.key,
            program_id,
        )
        .unwrap();
        if user_devided_account.key != &check_user_devided_account {
            return Err(InvalidSeeds.into());
        }
        //
        if auction_save_account_data.success_flag == 1 {
            let current_status = Self::current_status(
                auction_save_account_data.pool_opens,
                auction_save_account_data.pool_closes,
                timer.unix_timestamp as u32,
            )?;
            if current_status != "Ended".to_string() {
                if auction_save_account_data.funds_raised == auction_save_account_data.tokens_offered{

                }
                else{
                    return Err(StatusIsNotEnded.into());
                }
            }
            if (timer.unix_timestamp as u32) < auction_save_account_data.allocation_date {
                return Err(AllocationDateHasNotArrived.into());
            }

            if auction_save_account_data.funds_raised
                < auction_save_account_data.min_swap_level.into()
            {
                auction_save_account_data.success_flag = 0;
            }

        }
        let mut user_ido_ada_data: AccountData =
            BorshDeserialize::deserialize(&mut &user_devided_account.data.borrow_mut()[..])
                .unwrap();
        if user_ido_ada_data.withdrawn == 0 {
            return Err(WithdrawnZero.into());
        }
        let mint_info_project = Mint::unpack(&spl_token_account_project.data.borrow())?;
        let mint_info_underlay = Mint::unpack(&spl_token_account_underlay.data.borrow())?;
        match auction_save_account_data.success_flag {
            1 => {
                msg!("claim success start");
                //
                let required_lamports = rent
                    .minimum_balance(spl_token::state::Account::LEN)
                    .max(1)
                    .saturating_sub(user_native_ada.lamports());
                if required_lamports > 0 {
                    msg!("create user ada");
                    invoke(
                        &spl_associated_token_account::create_associated_token_account(
                            &user_account.key,
                            &user_account.key,
                            &spl_token_account_project.key,
                        ),
                        &[
                            user_account.clone(),
                            user_native_ada.clone(),
                            spl_token_account_project.clone(),
                            system_account.clone(),
                            token_program.clone(),
                            rent_account.clone(),
                            spl_associated_program_info.clone(),
                        ],
                    )?;
                }
                let check_user_devided_account_project =
                    get_associated_token_address(user_account.key, spl_token_account_project.key);
                if user_native_ada.key != &check_user_devided_account_project {
                    return Err(NotExistUserAda.into());
                }
                //
                let mut transfer_amount = user_ido_ada_data
                    .withdrawn
                    .checked_mul(auction_save_account_data.ratio.into())
                    .ok_or(IdoError::CalculationOverflow)?;
                msg!("'user_ido_ada_data
                .withdrawn {:?} auction_save_account_data.ratio {:?}",user_ido_ada_data
                .withdrawn,auction_save_account_data.ratio);    
                transfer_amount = transfer_amount
                    .checked_div(10000)
                    .ok_or(IdoError::CalculationOverflow)?;
                msg!("mint_info_underlay.decimals {:?}  mint_info_project.decimals {:?} transfer_amount {:?}",
                    mint_info_underlay.decimals,
                    mint_info_project.decimals, 
                    transfer_amount);
                transfer_amount = Self::compare_decimals(
                    mint_info_underlay.decimals,
                    mint_info_project.decimals,
                    transfer_amount,
                )?;
                invoke_signed(
                    &spl_token::instruction::transfer_checked(
                        &token_program.key,
                        &program_associated_account_info_project.key,
                        &spl_token_account_project.key,
                        &user_native_ada.key,
                        &program_devided_account_info.key,
                        &[],
                        transfer_amount,
                        mint_info_project.decimals,
                    )?,
                    &[
                        spl_token_account_project.clone(),
                        token_program.clone(),
                        program_associated_account_info_project.clone(),
                        user_native_ada.clone(),
                        program_devided_account_info.clone(),
                        user_account.clone(),
                    ],
                    &[devided_signer_seeds],
                )?;
            }
            0 => {
                msg!("claim failed start");
                //
                let required_lamports = rent
                    .minimum_balance(spl_token::state::Account::LEN)
                    .max(1)
                    .saturating_sub(user_other_ada.lamports());
                if required_lamports > 0 {
                    msg!("create user ada");
                    invoke(
                        &spl_associated_token_account::create_associated_token_account(
                            &user_account.key,
                            &user_account.key,
                            &spl_token_account_underlay.key,
                        ),
                        &[
                            user_account.clone(),
                            user_other_ada.clone(),
                            spl_token_account_underlay.clone(),
                            system_account.clone(),
                            token_program.clone(),
                            rent_account.clone(),
                            spl_associated_program_info.clone(),
                        ],
                    )?;
                }
                let check_user_devided_account_project =
                    get_associated_token_address(user_account.key, spl_token_account_underlay.key);
                if user_other_ada.key != &check_user_devided_account_project {
                    return Err(NotExistUserAda.into());
                }
                //
                invoke_signed(
                    &spl_token::instruction::transfer_checked(
                        &token_program.key,
                        &program_associated_account_info_underlay.key,
                        &spl_token_account_underlay.key,
                        &user_other_ada.key,
                        &program_devided_account_info.key,
                        &[],
                        user_ido_ada_data.contribution,
                        mint_info_underlay.decimals,
                    )?,
                    &[
                        spl_token_account_underlay.clone(),
                        token_program.clone(),
                        program_associated_account_info_underlay.clone(),
                        user_other_ada.clone(),
                        program_devided_account_info.clone(),
                        user_account.clone(),
                    ],
                    &[devided_signer_seeds],
                )?;
            }
            _ => {
                return Err(SuccessFlagError.into());
            }
        }
        user_ido_ada_data.withdrawn = 0;
        user_ido_ada_data.serialize(&mut &mut user_devided_account.data.borrow_mut()[..])?;
        auction_save_account_data
            .serialize(&mut &mut auction_save_account.data.borrow_mut()[..])?;
        Ok(())
    }

    fn process_devided(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let funder_info = next_account_info(account_info_iter)?;
        let program_devided_account_info = next_account_info(account_info_iter)?;
        let program_associated_account_info = next_account_info(account_info_iter)?;
        let spl_token_mint_info = next_account_info(account_info_iter)?;
        let spl_token_program_info = next_account_info(account_info_iter)?;
        let spl_associated_program_info = next_account_info(account_info_iter)?;
        let system_program_info = next_account_info(account_info_iter)?;
        let rent_sysvar_info = next_account_info(account_info_iter)?;
        let auction_save_account = next_account_info(account_info_iter)?;
        let auction_save_account_data: AuctionBuffer =
            BorshDeserialize::deserialize(&mut &auction_save_account.data.borrow_mut()[..])
                .unwrap();
        //校验用户签名 program派生账号 program派生账号ada
        let (program_devided, devided_bump_seed) =
            Self::program_devided_address(&program_id, auction_save_account_data.auction_id);
        // let mut seed: Vec<u8> = Vec::new();
        let mut auction_seed = [0u8; 4];
        auction_save_account_data
            .auction_id
            .serialize(&mut auction_seed.as_mut())
            .unwrap();
        let devided_signer_seeds: &[&[_]] =
            &[Self::program_seeds(), &auction_seed, &[devided_bump_seed]];
        let program_associated = Self::program_associated_adress(
            &program_id,
            &spl_token_mint_info.key,
            auction_save_account_data.auction_id,
        );
        msg!("program_associated {:?}", program_associated);
        if !funder_info.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        if program_devided != *program_devided_account_info.key {
            return Err(ProgramError::InvalidSeeds);
        }
        if program_associated != *program_associated_account_info.key {
            return Err(ProgramError::InvalidSeeds);
        }
        // 校验spl-ada 的pubkey
        if Self::associated_token_program_address() != *spl_associated_program_info.key {
            return Err(ProgramError::InvalidInstructionData);
        }
        //调用spl-ada创建program 派生地址对spl-token的ada
        invoke_signed(
            &SysInstruction {
                program_id: *spl_associated_program_info.key,
                accounts: vec![
                    AccountMeta::new(*funder_info.key, true), //支付地址
                    AccountMeta::new(*program_associated_account_info.key, false), //program派生地址的关联账号
                    AccountMeta::new_readonly(*program_devided_account_info.key, false), //program派生地址
                    AccountMeta::new_readonly(*spl_token_mint_info.key, false),          //币地址
                    AccountMeta::new_readonly(*system_program_info.key, false),          //sys id
                    AccountMeta::new_readonly(*spl_token_program_info.key, false),       //token id
                    AccountMeta::new_readonly(*rent_sysvar_info.key, false),             //rent id
                ],
                data: vec![],
            },
            &[
                spl_associated_program_info.clone(),
                funder_info.clone(),
                program_devided_account_info.clone(),
                program_associated_account_info.clone(),
                spl_token_mint_info.clone(),
                system_program_info.clone(),
                spl_token_program_info.clone(),
                rent_sysvar_info.clone(),
            ],
            &[devided_signer_seeds],
        )
    }

    fn process_modifywhitelist(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        whitelist: Vec<[u8; 32]>,
        mut offset: u32,
        end_flag: u8,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let check = next_account_info(account_info_iter)?;
        let key = next_account_info(account_info_iter)?;
        Self::administrator_verification(check, key)?;
        let auction = next_account_info(account_info_iter)?;
        let timer = &Clock::from_account_info(next_account_info(account_info_iter)?)?;

        let mut auction_save_account_data: AuctionBuffer =
            BorshDeserialize::deserialize(&mut &auction.data.borrow_mut()[..]).unwrap();
            
        if whitelist.len() > 15{
            return Err(WhitelistLenTooLong.into());
        }

        let current_status = Self::current_status(
            auction_save_account_data.pool_opens,
            auction_save_account_data.pool_closes,
            timer.unix_timestamp as u32,
        )?;
        if current_status == "Ended".to_string() {
            return Err(StatusHasEnded.into());
        }

        if offset == 0{
            auction_save_account_data.whitelist.clear();
        }
        

        for i in whitelist {
            auction_save_account_data.whitelist[offset as usize] = i;
            offset += 1;
        }

        if end_flag == 1 {
            auction_save_account_data.whitelist.sort();
            auction_save_account_data.whitelist.dedup();
        }
        msg!("whitelist {:?} ",auction_save_account_data.whitelist);
        auction_save_account_data.serialize(&mut &mut auction.data.borrow_mut()[..])?;
        
        Ok(())
    }

    fn administrator_verification(
        admin: &AccountInfo,
        check_member: &AccountInfo,
    ) -> ProgramResult {
        let admin_data = Vec::<[u8; 32]>::deserialize(&mut &admin.data.borrow_mut()[..]).unwrap();
        if (!admin_data.iter().any(|i| i == &check_member.key.to_bytes()))
            || (check_member.is_signer == false)
        {
            return Err(NotAdministrator.into());
        }
        Ok(())
    }

    fn current_status(
        pool_opens: u32,
        pool_closes: u32,
        timestmp: u32,
    ) -> Result<String, ProgramError> {
        if pool_opens > timestmp {
            return Ok("Upcoming".to_string());
        } else if timestmp >= pool_opens && timestmp < pool_closes {
            return Ok("Open".to_string());
        } else if pool_closes <= timestmp {
            return Ok("Ended".to_string());
        } else {
            return Err(TimestampError.into());
        }
    }

    fn associated_token_program_address() -> Pubkey {
        Pubkey::from_str("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL").unwrap()
    }

    fn program_seeds() -> &'static [u8] {
        "ido".as_bytes()
    }

    fn program_devided_address(program_id: &Pubkey, auction_id: u32) -> (Pubkey, u8) {
        let mut auction_seed = [0u8; 4];
        auction_id.serialize(&mut auction_seed.as_mut()).unwrap();
        let mut buffer = Self::program_seeds().to_vec();
        buffer.extend_from_slice(&mut auction_seed);
        Pubkey::find_program_address(&[&buffer], &program_id)
    }

    fn program_associated_adress(
        program_id: &Pubkey,
        spl_token: &Pubkey,
        auction_id: u32,
    ) -> Pubkey {
        let (wallet_address, _) = Self::program_devided_address(program_id, auction_id);
        let a = get_associated_token_address(&wallet_address, spl_token);
        a
    }

    fn get_user_derived_adress(
        auction_save_account_data_id: u32,
        user_account_key: &Pubkey,
        program_id: &Pubkey,
    ) -> Result<(Pubkey, String), ProgramError> {
        let mut buffer = [0u8; 4];
        auction_save_account_data_id
            .serialize(&mut buffer.as_mut())
            .unwrap();
        let mut data = user_account_key.to_bytes().to_vec();
        data.extend_from_slice(&mut buffer);
        let digest = md5::compute(data.as_slice());
        let seed = Self::to_string(&TryInto::<[u8; 16]>::try_into(digest).unwrap());
        let check_user_devided_account =
            Pubkey::create_with_seed(user_account_key, &seed, program_id).unwrap();
        Ok((check_user_devided_account, seed))
    }

    fn compare_decimals(
        undelay_decimals: u8,
        project_decimals: u8,
        transfer_amount: u64,
    ) -> Result<u64, ProgramError> {
        let last_amount = match undelay_decimals.cmp(&project_decimals) {
            Ordering::Less => transfer_amount
                .checked_mul(
                    project_decimals
                        .checked_sub(undelay_decimals)
                        .ok_or(IdoError::CalculationOverflow)? as u64,
                )
                .ok_or(IdoError::CalculationOverflow)?,
            Ordering::Equal => transfer_amount,
            Ordering::Greater => transfer_amount
                .checked_div(
                    undelay_decimals
                        .checked_sub(project_decimals)
                        .ok_or(IdoError::CalculationOverflow)? as u64,
                )
                .ok_or(IdoError::CalculationOverflow)?,
        };
        Ok(last_amount)
    }

    fn to_string(input: &[u8; 16]) -> String {
        input
            .iter()
            .map(|&x| {
                [
                    char::from_digit((x >> 4) as u32, 16).unwrap(),
                    char::from_digit((x & 0x0F) as u32, 16).unwrap(),
                ]
                .iter()
                .map(|&h| h.to_string().to_uppercase())
                .collect::<String>()
            })
            .collect::<String>()
    }
}
