#![cfg(feature = "test-bpf")]
use {
    borsh::{BorshDeserialize, BorshSerialize},
    ido::entrypoint::process_instruction,
    ido::state::{AccountData, AuctionBuffer},
    solana_program::{
        instruction::{AccountMeta, Instruction},
        program_pack::Pack,
        pubkey::Pubkey,
        system_instruction,
    },
    solana_program_test::*,
    solana_sdk::{
        account::Account, signature::Signer, signer::keypair::Keypair, sysvar::rent::Rent,
        transaction::Transaction,
    },
    spl_associated_token_account::get_associated_token_address,
    std::{char, convert::TryInto, str::FromStr},
};

#[tokio::test]
async fn test_ido_test() {
    let sys = Pubkey::from_str(&"11111111111111111111111111111111").unwrap();
    // let program_id = Pubkey::from_str(&"idotest111111111111111111111111111111111111").unwrap();
    let program_id = Pubkey::from_str(&"EysZryoNh1LxAaeqBF8vnHnQbWQJmqWUjXzM3pavXcym").unwrap();
    let sysclock = Pubkey::from_str(&"SysvarC1ock11111111111111111111111111111111").unwrap();
    let rent_pubkey = Pubkey::from_str(&"SysvarRent111111111111111111111111111111111").unwrap();
    let token_program_id =
        Pubkey::from_str(&"TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap();
    let max_admin = Pubkey::from_str(&"AwUk5nxvLreRgDbXTRLsZDLgwJfEmveFsqnrnbedWUhu").unwrap();
    let max_admin_keypair = Keypair::from_bytes(&[
        131, 12, 69, 122, 149, 58, 200, 55, 20, 140, 196, 164, 164, 233, 121, 12, 54, 91, 251, 29,
        162, 29, 65, 6, 179, 60, 78, 143, 150, 135, 25, 88, 147, 173, 234, 55, 233, 143, 13, 172,
        195, 121, 213, 47, 176, 67, 142, 221, 31, 200, 151, 241, 208, 234, 162, 217, 11, 1, 10,
        192, 47, 201, 248, 248,
    ])
    .unwrap();

    let ido = Keypair::new();
    let ido_pubkey = ido.pubkey();
    let project = Keypair::new();
    let project_pubkey = project.pubkey();
    let spl_token_address_project = Keypair::new();
    let spl_token_address_project_pubkey = spl_token_address_project.pubkey();
    let spl_token_address_underlay = Keypair::new();
    let spl_token_address_underlay_pubkey = spl_token_address_underlay.pubkey();
    let user_keypair = Keypair::new();
    let user_pubkey = user_keypair.pubkey();
    let user_native_ada =
        get_associated_token_address(&user_pubkey, &spl_token_address_project_pubkey);
    let user_other_ada =
        get_associated_token_address(&user_pubkey, &spl_token_address_underlay_pubkey);
    let project_native_ada =
        get_associated_token_address(&project_pubkey, &spl_token_address_project_pubkey);
    let profect_other_ada =
        get_associated_token_address(&project_pubkey, &spl_token_address_underlay_pubkey); //16 0 1 2 3
    let mut auction_seed_test = [0u8; 4];
    47u32.serialize(&mut auction_seed_test.as_mut()).unwrap();
    let mut auction_seed_one = [0u8; 4];
    0u32.serialize(&mut auction_seed_one.as_mut()).unwrap();
    let mut auction_seed_two = [0u8; 4];
    1u32.serialize(&mut auction_seed_two.as_mut()).unwrap();
    let mut auction_seed_three = [0u8; 4];
    2u32.serialize(&mut auction_seed_three.as_mut()).unwrap();
    let mut auction_seed_four = [0u8; 4];
    3u32.serialize(&mut auction_seed_four.as_mut()).unwrap();
    let pro = Pubkey::from_str(&"pqpGTYYHVZtHNsqjcwts8QAWVCH6RR8MjAQ2sXrGBoE").unwrap();
    let under = Pubkey::from_str(&"BSeQ2y1vH4dgxKAxsb7WSJT1NfCZDsvRMpcyrj5GNBB8").unwrap();
    let (address_test, _) =
        Pubkey::find_program_address(&[b"ido", &auction_seed_test], &program_id);
    println!(" address_test  {:?}", address_test);

    let (address_one, _) = Pubkey::find_program_address(&[b"ido", &auction_seed_one], &program_id);
    let (address_two, _) = Pubkey::find_program_address(&[b"ido", &auction_seed_two], &program_id);
    let (address_three, _) =
        Pubkey::find_program_address(&[b"ido", &auction_seed_three], &program_id);
    let (address_four, _) =
        Pubkey::find_program_address(&[b"ido", &auction_seed_four], &program_id);
    let ido_native_ada_test =
        get_associated_token_address(&address_test, &spl_token_address_project_pubkey);
    println!("ido_native_ada_test {:?}", ido_native_ada_test);
    let ido_other_ada_test =
        get_associated_token_address(&address_test, &spl_token_address_underlay_pubkey);
    let ido_native_ada_one =
        get_associated_token_address(&address_one, &spl_token_address_project_pubkey);

    let ido_other_ada_one =
        get_associated_token_address(&address_one, &spl_token_address_underlay_pubkey);
    let ido_native_ada_two =
        get_associated_token_address(&address_two, &spl_token_address_project_pubkey);
    let ido_other_ada_two =
        get_associated_token_address(&address_two, &spl_token_address_underlay_pubkey);
    let ido_native_ada_three =
        get_associated_token_address(&address_three, &spl_token_address_project_pubkey);
    let ido_other_ada_three =
        get_associated_token_address(&address_three, &spl_token_address_underlay_pubkey);
    let ido_native_ada_four =
        get_associated_token_address(&address_four, &spl_token_address_project_pubkey);
    let ido_other_ada_four =
        get_associated_token_address(&address_four, &spl_token_address_underlay_pubkey);
    let admin = Pubkey::new_unique();
    let new_key = Pubkey::new_unique();
    let new_key2 = Pubkey::new_unique();
    let new_key3_keypair = Keypair::new();
    let new_key3 = new_key3_keypair.pubkey();
    let new_key4 = Pubkey::new_unique();
    let new_key5 = Pubkey::new_unique();
    let new_key6 = Pubkey::new_unique();
    let auction = Pubkey::new_unique();
    let auction2 = Pubkey::new_unique();
    let auction3 = Pubkey::new_unique();
    let auction4 = Pubkey::new_unique();
    let auction_test = Pubkey::new_unique();

    let mut pool_name = String::from("bbb");
    let bytes_pool_name = unsafe { pool_name.as_bytes_mut() };
    println!("bytes_pool_name {:?}", bytes_pool_name);
    let mut pool_name_buffer = [0u8; 32];
    bytes_pool_name.swap_with_slice(&mut pool_name_buffer[0..bytes_pool_name.len()]);
    println!("pool_name_buffer {:?}", pool_name_buffer);

    let mut bytes_pool_name_test = [65, 71, 71];
    println!("bytes_pool_name {:?}", bytes_pool_name_test);
    let mut pool_name_test_buffer = [0u8; 32];
    bytes_pool_name_test.swap_with_slice(&mut pool_name_test_buffer[0..3]);
    println!("pool_name_buffer {:?}", pool_name_test_buffer);

    let a = spl_token_address_project_pubkey;
    let spl_program_id = a.to_bytes();
    let mut spl_program_id_buffer = [0u8; 32];
    spl_program_id_buffer.copy_from_slice(&spl_program_id);

    let b = project_native_ada;
    let spl_program_id_derive = b.to_bytes();
    let mut spl_program_id_derive_buffer = [0u8; 32];
    spl_program_id_derive_buffer.copy_from_slice(&spl_program_id_derive);

    let d = spl_token_address_underlay_pubkey;
    let underlay_program_id = d.to_bytes();
    let mut underlay_program_id_buffer = [0u8; 32];
    underlay_program_id_buffer.copy_from_slice(&underlay_program_id);

    let e = profect_other_ada;
    let underlay_program_id_derive = e.to_bytes();
    let mut underlay_program_id_derive_buffer = [0u8; 32];
    underlay_program_id_derive_buffer.copy_from_slice(&underlay_program_id_derive);

    let mut program_test = ProgramTest::new("ido", program_id, processor!(process_instruction));
    let test_auction: AuctionBuffer = AuctionBuffer {
        auction_id: 47,
        access: 0,                       //公有，私有池 public,private
        whitelist: [].to_vec(),          //白名单
        poolname: pool_name_test_buffer, //池子名字
        ratio: 250,                      //比例
        tokens_offered: 1000000000,      //筹款上限
        funds_raised: 0,                 //已筹款资金
        min_allocation: 1000000,         //最小参与额度
        max_allocation: 100000000,       //最大参与额度
        pool_opens: 1628039100,          //开始参与时间
        pool_closes: 1628870400,         //结束参与时间
        min_swap_level: 10000000,        //最小筹款数量
        spl_program_id: pro.to_bytes(),  // 项目方币的地址
        spl_program_id_derive: [
            156, 114, 240, 2, 112, 10, 24, 98, 189, 153, 36, 68, 83, 63, 109, 101, 39, 87, 17, 197,
            198, 128, 141, 231, 28, 20, 80, 47, 185, 182, 136, 51,
        ], // 项目方钱包对他们自己币的派z生地址 给我们发币的地址
        underlay_program_id: under.to_bytes(), // 募集币的地址
        underlay_program_id_derive: [
            116, 71, 187, 51, 149, 20, 248, 248, 58, 41, 138, 151, 204, 160, 118, 85, 6, 72, 170,
            101, 131, 237, 252, 216, 210, 61, 138, 223, 121, 77, 224, 48,
        ], // 项目方募集币的派生地址
        allocation_date: 1628956800,     // 提取时间
        success_flag: 1,
    };
    let new_auction: AuctionBuffer = AuctionBuffer {
        auction_id: 0,
        access: 1, //公有，私有池 public,private
        whitelist: [user_pubkey.to_bytes(), user_pubkey.to_bytes()].to_vec(), //白名单
        poolname: pool_name_test_buffer, //池子名字
        ratio: 10, //比例
        tokens_offered: 100, //筹款上限
        funds_raised: 0, //已筹款资金
        min_allocation: 1, //最小参与额度
        max_allocation: 5, //最大参与额度
        pool_opens: 10, //开始参与时间
        pool_closes: 2222222222, //结束参与时间
        min_swap_level: 50, //最小筹款数量
        spl_program_id: spl_program_id_buffer, // 项目方币的地址
        spl_program_id_derive: spl_program_id_derive_buffer, // 项目方钱包对他们自己币的派z生地址 给我们发币的地址
        underlay_program_id: underlay_program_id_buffer,     // 募集币的地址
        underlay_program_id_derive: underlay_program_id_derive_buffer, // 项目方募集币的派生地址
        allocation_date: 1628956800,                         // 提取时间
        success_flag: 1,
    };
    let new_auction2: AuctionBuffer = AuctionBuffer {
        auction_id: 1,
        access: 1, //公有，私有池 public,private
        whitelist: [user_pubkey.to_bytes(), user_other_ada.to_bytes()].to_vec(), //白名单
        poolname: pool_name_buffer, //池子名字
        ratio: 100000, //比例
        tokens_offered: 100000, //筹款上限
        funds_raised: 0, //已筹款资金
        min_allocation: 1000, //最小参与额度
        max_allocation: 5000, //最大参与额度
        pool_opens: 100, //开始参与时间
        pool_closes: 2222222222, //结束参与时间
        min_swap_level: 50000, //最小筹款数量
        spl_program_id: spl_program_id_buffer, // 项目方币的地址
        spl_program_id_derive: spl_program_id_derive_buffer, // 项目方钱包对他们自己币的派z生地址 给我们发币的地址
        underlay_program_id: underlay_program_id_buffer,     // 募集币的地址
        underlay_program_id_derive: underlay_program_id_derive_buffer, // 项目方募集币的派生地址
        allocation_date: 1000000000,                         // 提取时间
        success_flag: 0,
    };
    let new_auction3: AuctionBuffer = AuctionBuffer {
        auction_id: 2,
        access: 0, //公有，私有池 public,private
        whitelist: [user_pubkey.to_bytes(), user_pubkey.to_bytes()].to_vec(), //白名单
        poolname: pool_name_buffer, //池子名字
        ratio: 100000, //比例
        tokens_offered: 100000, //筹款上限
        funds_raised: 0, //已筹款资金
        min_allocation: 1000, //最小参与额度
        max_allocation: 5000, //最大参与额度
        pool_opens: 100, //开始参与时间
        pool_closes: 1000000000, //结束参与时间
        min_swap_level: 50000, //最小筹款数量
        spl_program_id: spl_program_id_buffer, // 项目方币的地址
        spl_program_id_derive: spl_program_id_derive_buffer, // 项目方钱包对他们自己币的派z生地址 给我们发币的地址
        underlay_program_id: underlay_program_id_buffer,     // 募集币的地址
        underlay_program_id_derive: underlay_program_id_derive_buffer, // 项目方募集币的派生地址
        allocation_date: 1000000000,                         // 提取时间
        success_flag: 0,
    };
    let new_auction4: AuctionBuffer = AuctionBuffer {
        auction_id: 3,
        access: 0, //公有，私有池 public,private
        whitelist: [user_pubkey.to_bytes(), user_pubkey.to_bytes()].to_vec(), //白名单
        poolname: pool_name_buffer, //池子名字
        ratio: 100000, //比例
        tokens_offered: 100000, //筹款上限
        funds_raised: 0, //已筹款资金
        min_allocation: 1, //最小参与额度
        max_allocation: 5000, //最大参与额度
        pool_opens: 100, //开始参与时间
        pool_closes: 1000000000, //结束参与时间
        min_swap_level: 50000, //最小筹款数量
        spl_program_id: spl_program_id_buffer, // 项目方币的地址
        spl_program_id_derive: spl_program_id_derive_buffer, // 项目方钱包对他们自己币的派z生地址 给我们发币的地址
        underlay_program_id: underlay_program_id_buffer,     // 募集币的地址
        underlay_program_id_derive: underlay_program_id_derive_buffer, // 项目方募集币的派生地址
        allocation_date: 1000000000,                         // 提取时间
        success_flag: 0,
    };
    let mut encode_new_auction = new_auction.try_to_vec().unwrap();
    let mut encode_new_auction2 = new_auction2.try_to_vec().unwrap();
    let mut encode_new_auction3 = new_auction3.try_to_vec().unwrap();
    let mut encode_new_auction4 = new_auction4.try_to_vec().unwrap();
    let mut encode_test_auction = test_auction.try_to_vec().unwrap();
    let auction_len3 = encode_new_auction3.len();
    let auction_len4 = encode_new_auction4.len();
    encode_test_auction.insert(0, 1);
    println!("encode_test_auction {:?}", encode_test_auction.len());
    encode_new_auction.insert(0, 1);
    println!("encode_new_auction {:?}", encode_new_auction.len());
    encode_new_auction2.insert(0, 1);
    encode_new_auction3.insert(0, 1);
    encode_new_auction4.insert(0, 1);
    let user_acc_data = AccountData {
        contribution: 0,
        withdrawn: 0,
    };
    let user_acc_data_serialize = user_acc_data.try_to_vec().unwrap();
    let user_acc_data_serialize_len = user_acc_data_serialize.len();
    let user_acc_data_claim = AccountData {
        contribution: 10,
        withdrawn: 10,
    };
    let user_acc_data_claim_serialize = user_acc_data_claim.try_to_vec().unwrap();
    let user_acc_data_claim_serialize_len = user_acc_data_claim_serialize.len();

    let submit_amount: u64 = 3;
    let mut submit_amount_serialize = submit_amount.to_le_bytes();
    let mut test_submit_buffer = [0u8; 9];
    test_submit_buffer[0] = 4;
    submit_amount_serialize.swap_with_slice(&mut test_submit_buffer[1..]);
    //
    let mut buffer = [0u8; 4];
    new_auction
        .auction_id
        .serialize(&mut buffer.as_mut())
        .unwrap();
    let mut data = user_keypair.pubkey().to_bytes().to_vec();
    data.extend_from_slice(&mut buffer);
    let digest = md5::compute(data.as_slice());
    let seed = TryInto::<[u8; 16]>::try_into(digest)
        .unwrap()
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
        .collect::<String>();
    println!("seed 1 {:?}", seed);
    let user_derived_find =
        Pubkey::create_with_seed(&user_keypair.pubkey(), &seed, &program_id).unwrap();
    //
    let mut buffer = [0u8; 4];
    new_auction4
        .auction_id
        .serialize(&mut buffer.as_mut())
        .unwrap();
    let mut data = user_keypair.pubkey().to_bytes().to_vec();
    data.extend_from_slice(&mut buffer);
    let digest = md5::compute(data.as_slice());
    let seed = TryInto::<[u8; 16]>::try_into(digest)
        .unwrap()
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
        .collect::<String>();
    let user_derived_find_claim =
        Pubkey::create_with_seed(&user_keypair.pubkey(), &seed, &program_id).unwrap();
    println!("seed {:?}", seed);
    //
    program_test.add_account(
        admin,
        Account {
            lamports: Rent::default().minimum_balance(1000),
            owner: program_id, // Can only withdraw lamports from accounts owned by the program
            data: [0; 1000].to_vec(),
            ..Account::default()
        },
    );
    program_test.add_account(
        auction_test,
        Account {
            lamports: Rent::default().minimum_balance(10485760),
            owner: program_id, // Can only withdraw lamports from accounts owned by the program
            data: vec![0; 10485760],
            ..Account::default()
        },
    );
    program_test.add_account(
        auction,
        Account {
            lamports: Rent::default().minimum_balance(10485760),
            owner: program_id, // Can only withdraw lamports from accounts owned by the program
            data: vec![0; 10485760],
            ..Account::default()
        },
    );
    program_test.add_account(
        auction2,
        Account {
            lamports: Rent::default().minimum_balance(10485760),
            owner: program_id, // Can only withdraw lamports from accounts owned by the program
            data: vec![0; 10485760],
            ..Account::default()
        },
    );
    program_test.add_account(
        auction3,
        Account {
            lamports: Rent::default().minimum_balance(auction_len3),
            owner: program_id, // Can only withdraw lamports from accounts owned by the program
            data: vec![0; auction_len3],
            ..Account::default()
        },
    );
    program_test.add_account(
        auction4,
        Account {
            lamports: Rent::default().minimum_balance(auction_len4),
            owner: program_id, // Can only withdraw lamports from accounts owned by the program
            data: vec![0; auction_len4],
            ..Account::default()
        },
    );
    program_test.add_account(
        user_derived_find,
        Account {
            lamports: Rent::default().minimum_balance(user_acc_data_serialize_len),
            owner: program_id, // Can only withdraw lamports from accounts owned by the program
            data: vec![0; user_acc_data_serialize_len],
            ..Account::default()
        },
    );
    program_test.add_account(
        user_derived_find_claim,
        Account {
            lamports: Rent::default().minimum_balance(user_acc_data_claim_serialize_len),
            owner: program_id, // Can only withdraw lamports from accounts owned by the program
            data: user_acc_data_claim_serialize,
            ..Account::default()
        },
    );
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
    //
    //modifity admin
    println!("modifity admin start");
    let mut transaction = Transaction::new_with_payer(
        &[
            Instruction::new_with_bincode(
                program_id,
                &[0u8, 0],
                vec![
                    AccountMeta::new(max_admin, true),
                    AccountMeta::new(admin, false),
                    AccountMeta::new(new_key, false),
                    AccountMeta::new(new_key3, false),
                    AccountMeta::new(new_key4, false),
                ],
            ),
            Instruction::new_with_bincode(
                program_id,
                &[0u8, 1],
                vec![
                    AccountMeta::new(max_admin, true),
                    AccountMeta::new(admin, false),
                    AccountMeta::new(new_key, false),
                    AccountMeta::new(new_key2, false),
                    AccountMeta::new(new_key4, false),
                ],
            ),
        ],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &max_admin_keypair], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    let admin_vec = banks_client.get_account(admin).await.unwrap().unwrap();
    let account_data = Vec::<[u8; 32]>::deserialize(&mut &admin_vec.data[..]).unwrap();
    assert_eq!(account_data, [new_key3.to_bytes()]);
    println!("modifity admin success");
    //
    //create_mint_account
    println!("create mint start");
    let rent = banks_client.get_rent().await.unwrap();
    let mint_rent = rent.minimum_balance(spl_token::state::Mint::LEN);

    let mut transaction_create_mint = Transaction::new_with_payer(
        &[
            system_instruction::create_account(
                &payer.pubkey(),
                &spl_token_address_project_pubkey,
                mint_rent,
                spl_token::state::Mint::LEN as u64,
                &token_program_id,
            ),
            spl_token::instruction::initialize_mint(
                &token_program_id,
                &spl_token_address_project_pubkey,
                &project_pubkey,
                None,
                2,
            )
            .unwrap(),
            system_instruction::create_account(
                &payer.pubkey(),
                &spl_token_address_underlay_pubkey,
                mint_rent,
                spl_token::state::Mint::LEN as u64,
                &token_program_id,
            ),
            spl_token::instruction::initialize_mint(
                &token_program_id,
                &spl_token_address_underlay_pubkey,
                &ido_pubkey,
                None,
                2,
            )
            .unwrap(),
        ],
        Some(&payer.pubkey()),
    );
    transaction_create_mint.sign(
        &[
            &payer,
            &spl_token_address_project,
            &spl_token_address_underlay,
        ],
        recent_blockhash,
    );
    banks_client
        .process_transaction(transaction_create_mint)
        .await
        .unwrap();
    println!("create mint success");
    //
    //save_auction_data
    println!("save_auction_data start");
    let mut transaction_save_auction_data_test = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &encode_test_auction,
            vec![
                AccountMeta::new(admin, false),
                AccountMeta::new(new_key3, true),
                AccountMeta::new(auction_test, false),
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new(address_test, false),
                AccountMeta::new(ido_other_ada_test, false),
                AccountMeta::new(ido_native_ada_test, false),
                AccountMeta::new(spl_token_address_project_pubkey, false),
                AccountMeta::new(spl_token_address_underlay_pubkey, false),
                AccountMeta::new(token_program_id, false),
                AccountMeta::new(spl_associated_token_account::id(), false),
                AccountMeta::new(sys, false),
                AccountMeta::new(rent_pubkey, false),
            ],
        )],
        Some(&payer.pubkey()),
    );

    transaction_save_auction_data_test.sign(&[&payer, &new_key3_keypair], recent_blockhash);
    banks_client
        .process_transaction(transaction_save_auction_data_test)
        .await
        .unwrap();
    println!("2");
    println!("encode_new_auction {:?}", encode_new_auction);
    let mut transaction_save_auction_data1 = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &encode_new_auction,
            vec![
                AccountMeta::new(admin, false),
                AccountMeta::new(new_key3, true),
                AccountMeta::new(auction, false),
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new(address_one, false),
                AccountMeta::new(ido_other_ada_one, false),
                AccountMeta::new(ido_native_ada_one, false),
                AccountMeta::new(spl_token_address_project_pubkey, false),
                AccountMeta::new(spl_token_address_underlay_pubkey, false),
                AccountMeta::new(token_program_id, false),
                AccountMeta::new(spl_associated_token_account::id(), false),
                AccountMeta::new(sys, false),
                AccountMeta::new(rent_pubkey, false),
            ],
        )],
        Some(&payer.pubkey()),
    );
    println!("2");
    transaction_save_auction_data1.sign(&[&payer, &new_key3_keypair], recent_blockhash);
    banks_client
        .process_transaction(transaction_save_auction_data1)
        .await
        .unwrap();
    println!("2");
    let mut transaction_save_auction_data2 = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &encode_new_auction2,
            vec![
                AccountMeta::new(admin, false),
                AccountMeta::new(new_key3, true),
                AccountMeta::new(auction2, false),
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new(address_two, false),
                AccountMeta::new(ido_other_ada_two, false),
                AccountMeta::new(ido_native_ada_two, false),
                AccountMeta::new(spl_token_address_project_pubkey, false),
                AccountMeta::new(spl_token_address_underlay_pubkey, false),
                AccountMeta::new(token_program_id, false),
                AccountMeta::new(spl_associated_token_account::id(), false),
                AccountMeta::new(sys, false),
                AccountMeta::new(rent_pubkey, false),
            ],
        )],
        Some(&payer.pubkey()),
    );
    println!("2");
    transaction_save_auction_data2.sign(&[&payer, &new_key3_keypair], recent_blockhash);
    banks_client
        .process_transaction(transaction_save_auction_data2)
        .await
        .unwrap();

    let auction_acc = banks_client.get_account(auction).await.unwrap().unwrap();
    let auction_acc2 = banks_client.get_account(auction2).await.unwrap().unwrap();

    let auction_acc_data: AuctionBuffer =
        BorshDeserialize::deserialize(&mut &auction_acc.data[..]).unwrap();
    assert_eq!(auction_acc_data, new_auction);
    let auction_acc_data2: AuctionBuffer =
        BorshDeserialize::deserialize(&mut &auction_acc2.data[..]).unwrap();
    assert_eq!(auction_acc_data2, new_auction2);

    let mut transaction_save_auction_data3 = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &encode_new_auction3,
            vec![
                AccountMeta::new(admin, false),
                AccountMeta::new(new_key3, true),
                AccountMeta::new(auction3, false),
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new(address_three, false),
                AccountMeta::new(ido_other_ada_three, false),
                AccountMeta::new(ido_native_ada_three, false),
                AccountMeta::new(spl_token_address_project_pubkey, false),
                AccountMeta::new(spl_token_address_underlay_pubkey, false),
                AccountMeta::new(token_program_id, false),
                AccountMeta::new(spl_associated_token_account::id(), false),
                AccountMeta::new(sys, false),
                AccountMeta::new(rent_pubkey, false),
            ],
        )],
        Some(&payer.pubkey()),
    );
    transaction_save_auction_data3.sign(&[&payer, &new_key3_keypair], recent_blockhash);
    banks_client
        .process_transaction(transaction_save_auction_data3)
        .await
        .unwrap();
    let mut transaction_save_auction_data4 = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &encode_new_auction4,
            vec![
                AccountMeta::new(admin, false),
                AccountMeta::new(new_key3, true),
                AccountMeta::new(auction4, false),
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new(address_four, false),
                AccountMeta::new(ido_other_ada_four, false),
                AccountMeta::new(ido_native_ada_four, false),
                AccountMeta::new(spl_token_address_project_pubkey, false),
                AccountMeta::new(spl_token_address_underlay_pubkey, false),
                AccountMeta::new(token_program_id, false),
                AccountMeta::new(spl_associated_token_account::id(), false),
                AccountMeta::new(sys, false),
                AccountMeta::new(rent_pubkey, false),
            ],
        )],
        Some(&payer.pubkey()),
    );
    transaction_save_auction_data4.sign(&[&payer, &new_key3_keypair], recent_blockhash);
    banks_client
        .process_transaction(transaction_save_auction_data4)
        .await
        .unwrap();
    let auction_acc3 = banks_client.get_account(auction3).await.unwrap().unwrap();
    let auction_acc4 = banks_client.get_account(auction4).await.unwrap().unwrap();
    let auction_acc3_data: AuctionBuffer =
        BorshDeserialize::deserialize(&mut &auction_acc3.data[..]).unwrap();
    assert_eq!(auction_acc3_data, new_auction3);
    let auction_acc_data4: AuctionBuffer =
        BorshDeserialize::deserialize(&mut &auction_acc4.data[..]).unwrap();
    assert_eq!(auction_acc_data4, new_auction4);

    println!("save_auction_data success");
    //
    //manual_failure
    println!("manual_failure start");
    assert_eq!(auction_acc_data2.success_flag, 0);
    let mut transaction_manual_failure = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &[2],
            vec![
                AccountMeta::new(admin, false),
                AccountMeta::new(new_key3, true),
                AccountMeta::new(auction2, false),
                AccountMeta::new(sysclock, false),
            ],
        )],
        Some(&payer.pubkey()),
    );
    transaction_manual_failure.sign(&[&payer, &new_key3_keypair], recent_blockhash);
    banks_client
        .process_transaction(transaction_manual_failure)
        .await
        .unwrap();
    let auction_acc2 = banks_client.get_account(auction2).await.unwrap().unwrap();
    let auction_acc_data2: AuctionBuffer =
        BorshDeserialize::deserialize(&mut &auction_acc2.data[..]).unwrap();
    assert_eq!(auction_acc_data2.success_flag, 0);
    println!("manual_failure success");
    //
    //modify_whitelist
    println!("modify_whitelist start");
    assert_eq!(
        auction_acc_data2.whitelist,
        [user_pubkey.to_bytes(), user_other_ada.to_bytes()].to_vec()
    );
    let mut new_whitelist: Vec<[u8; 32]> = Vec::new();
    let new = [
        13, 4, 127, 103, 48, 141, 204, 12, 233, 59, 92, 184, 169, 188, 133, 144, 51, 163, 239, 183,
        245, 137, 149, 230, 100, 241, 235, 254, 165, 192, 28, 38,
    ];
    new_whitelist.push(new_key5.to_bytes());
    new_whitelist.push(new_key6.to_bytes());
    // new_whitelist.push(new);
    // new_whitelist.push(new);
    // new_whitelist.push(new);
    let mut new_whitelist = new_whitelist.try_to_vec().unwrap();
    println!("whitelist_xuliehua: {:?}", new_whitelist);
    new_whitelist.insert(0, 6);
    let mut transaction_modify_whitelist = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &[new_whitelist],
            vec![
                AccountMeta::new(admin, false),
                AccountMeta::new(new_key3, true),
                AccountMeta::new(auction2, false),
                AccountMeta::new(sysclock, false),
            ],
        )],
        Some(&payer.pubkey()),
    );
    transaction_modify_whitelist.sign(&[&payer, &new_key3_keypair], recent_blockhash);
    banks_client
        .process_transaction(transaction_modify_whitelist)
        .await
        .unwrap();
    let auction_acc2 = banks_client.get_account(auction2).await.unwrap().unwrap();
    let auction_acc_data2: AuctionBuffer =
        BorshDeserialize::deserialize(&mut &auction_acc2.data[..]).unwrap();
    assert_eq!(
        auction_acc_data2.whitelist,
        [new_key5.to_bytes(), new_key6.to_bytes()].to_vec() // [new, new, new].to_vec()
    );
    println!("modify_whitelist success");
    //
    //program derived
    // println!("program derived start");
    // let mut transaction_program_derived = Transaction::new_with_payer(
    //     &[
    //         Instruction::new_with_bincode(
    //             program_id,
    //             &[7],
    //             vec![
    //                 AccountMeta::new(payer.pubkey(), true),
    //                 AccountMeta::new(address, false),
    //                 AccountMeta::new(ido_native_ada, false),
    //                 AccountMeta::new(spl_token_address_project_pubkey, false),
    //                 AccountMeta::new(token_program_id, false),
    //                 AccountMeta::new(spl_associated_token_account::id(), false),
    //                 AccountMeta::new(sys, false),
    //                 AccountMeta::new(rent_pubkey, false),
    //             ],
    //         ),
    //         Instruction::new_with_bincode(
    //             program_id,
    //             &[7],
    //             vec![
    //                 AccountMeta::new(payer.pubkey(), true),
    //                 AccountMeta::new(address, false),
    //                 AccountMeta::new(ido_other_ada, false),
    //                 AccountMeta::new(spl_token_address_underlay_pubkey, false),
    //                 AccountMeta::new(token_program_id, false),
    //                 AccountMeta::new(spl_associated_token_account::id(), false),
    //                 AccountMeta::new(sys, false),
    //                 AccountMeta::new(rent_pubkey, false),
    //             ],
    //         ),
    //     ],
    //     Some(&payer.pubkey()),
    // );
    // transaction_program_derived.sign(&[&payer], recent_blockhash);
    // banks_client
    //     .process_transaction(transaction_program_derived)
    //     .await
    //     .unwrap();
    // println!("program derived success");
    //
    // create mint account, ada, and mint_to ido and user
    println!("create ada, and mint_to ido and user start");

    let mut transaction_create_mint3 = Transaction::new_with_payer(
        &[
            spl_associated_token_account::create_associated_token_account(
                &payer.pubkey(),
                &project_pubkey,
                &spl_token_address_project_pubkey,
            ),
            spl_associated_token_account::create_associated_token_account(
                &payer.pubkey(),
                &project_pubkey,
                &spl_token_address_underlay_pubkey,
            ),
            spl_associated_token_account::create_associated_token_account(
                &payer.pubkey(),
                &ido_pubkey,
                &spl_token_address_project_pubkey,
            ),
            spl_associated_token_account::create_associated_token_account(
                &payer.pubkey(),
                &ido_pubkey,
                &spl_token_address_underlay_pubkey,
            ),
            spl_associated_token_account::create_associated_token_account(
                &payer.pubkey(),
                &user_pubkey,
                &spl_token_address_underlay_pubkey,
            ),
            spl_associated_token_account::create_associated_token_account(
                &payer.pubkey(),
                &user_pubkey,
                &spl_token_address_project_pubkey,
            ),
        ],
        Some(&payer.pubkey()),
    );
    transaction_create_mint3.sign(&[&payer], recent_blockhash);
    banks_client
        .process_transaction(transaction_create_mint3)
        .await
        .unwrap();
    let mut transaction_create_mint2 = Transaction::new_with_payer(
        &[
            spl_token::instruction::mint_to_checked(
                &token_program_id,
                &spl_token_address_underlay_pubkey,
                &ido_other_ada_one,
                &ido.pubkey(),
                &[],
                10000000,
                2,
            )
            .unwrap(),
            spl_token::instruction::mint_to_checked(
                &token_program_id,
                &spl_token_address_underlay_pubkey,
                &user_other_ada,
                &ido.pubkey(),
                &[],
                1000,
                2,
            )
            .unwrap(),
            spl_token::instruction::mint_to_checked(
                &token_program_id,
                &spl_token_address_project_pubkey,
                &ido_native_ada_one,
                &project.pubkey(),
                &[],
                10000000,
                2,
            )
            .unwrap(),
            spl_token::instruction::mint_to_checked(
                &token_program_id,
                &spl_token_address_project_pubkey,
                &ido_native_ada_three,
                &project.pubkey(),
                &[],
                10000000,
                2,
            )
            .unwrap(),
            spl_token::instruction::mint_to_checked(
                &token_program_id,
                &spl_token_address_underlay_pubkey,
                &ido_other_ada_three,
                &ido.pubkey(),
                &[],
                10000000,
                2,
            )
            .unwrap(),
            spl_token::instruction::mint_to_checked(
                &token_program_id,
                &spl_token_address_project_pubkey,
                &ido_native_ada_four,
                &project.pubkey(),
                &[],
                10000000,
                2,
            )
            .unwrap(),
            spl_token::instruction::mint_to_checked(
                &token_program_id,
                &spl_token_address_underlay_pubkey,
                &ido_other_ada_four,
                &ido.pubkey(),
                &[],
                10000000,
                2,
            )
            .unwrap(),
        ],
        Some(&payer.pubkey()),
    );
    transaction_create_mint2.sign(&[&payer, &ido, &project], recent_blockhash);
    banks_client
        .process_transaction(transaction_create_mint2)
        .await
        .unwrap();
    println!("create ada, and mint_to ido and user success");
    //
    //user submit
    println!("user submit start");

    let auction_acc = banks_client.get_account(auction).await.unwrap().unwrap();
    let auction_acc_deserialize: AuctionBuffer =
        BorshDeserialize::deserialize(&mut &auction_acc.data[..]).unwrap();
    assert_eq!(auction_acc_deserialize.funds_raised, 0);
    let user_derived_find_acc = banks_client
        .get_account(user_derived_find)
        .await
        .unwrap()
        .unwrap();
    let user_derived_find_acc_deserialize: AccountData =
        BorshDeserialize::deserialize(&mut &user_derived_find_acc.data[..]).unwrap();
    assert_eq!(user_derived_find_acc_deserialize.contribution, 0);
    assert_eq!(user_derived_find_acc_deserialize.withdrawn, 0);
    let mut transaction_submit = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &[test_submit_buffer],
            vec![
                AccountMeta::new(user_keypair.pubkey(), true), //用户
                AccountMeta::new(user_other_ada, false),       //用户关联账号
                AccountMeta::new(user_derived_find, false),    //用户派生地址
                AccountMeta::new(address_one, false),          //program派生地址
                AccountMeta::new(ido_other_ada_one, false),    //program 募集币spl ada
                AccountMeta::new(auction, false),
                AccountMeta::new(sys, false),
                AccountMeta::new(program_id, false),
                AccountMeta::new(sysclock, false),
                // AccountMeta::new(spl_associated_token_account::id(), false),
                AccountMeta::new(token_program_id, false),
                AccountMeta::new(spl_token_address_underlay_pubkey, false),
                AccountMeta::new(rent_pubkey, false),
            ],
        )],
        Some(&payer.pubkey()),
    );
    transaction_submit.sign(&[&payer, &user_keypair], recent_blockhash);
    println!("sysclock {:?}", AccountMeta::new(sysclock, false));
    banks_client
        .process_transaction(transaction_submit)
        .await
        .expect("failed to submit");
    let auction_acc = banks_client.get_account(auction).await.unwrap().unwrap();
    let auction_acc_deserialize: AuctionBuffer =
        BorshDeserialize::deserialize(&mut &auction_acc.data[..]).unwrap();
    assert_eq!(auction_acc_deserialize.funds_raised, 3);
    let user_derived_find_acc = banks_client
        .get_account(user_derived_find)
        .await
        .unwrap()
        .unwrap();
    let user_derived_find_acc_deserialize: AccountData =
        BorshDeserialize::deserialize(&mut &user_derived_find_acc.data[..]).unwrap();
    assert_eq!(user_derived_find_acc_deserialize.contribution, 3);
    assert_eq!(user_derived_find_acc_deserialize.withdrawn, 3);
    println!("user submit success");
    //
    //project-party withdraw
    println!("project-party withdraw start");
    let mut transaction_project_withdraw = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &[3],
            vec![
                AccountMeta::new(admin, false),
                AccountMeta::new(new_key3, true),
                AccountMeta::new(sysclock, false),
                AccountMeta::new(auction3, false),
                AccountMeta::new(address_three, false), //program派生地址
                AccountMeta::new(ido_native_ada_three, false), //program spl ada
                AccountMeta::new(ido_other_ada_three, false), //program spl ada
                AccountMeta::new(project_native_ada, false),
                AccountMeta::new(profect_other_ada, false),
                AccountMeta::new(token_program_id, false),
                AccountMeta::new(spl_token_address_project_pubkey, false),
                AccountMeta::new(spl_token_address_underlay_pubkey, false),
            ],
        )],
        Some(&payer.pubkey()),
    );
    transaction_project_withdraw.sign(&[&payer, &new_key3_keypair], recent_blockhash);

    banks_client
        .process_transaction(transaction_project_withdraw)
        .await
        .expect("failed to submit");
    println!("project-party withdraw success");
    //
    //claim
    println!("claim start");

    let user_derived_find_claim_acc = banks_client
        .get_account(user_derived_find_claim)
        .await
        .unwrap()
        .unwrap();

    let user_derived_find_claim_acc_deserialize: AccountData =
        BorshDeserialize::deserialize(&mut &user_derived_find_claim_acc.data[..]).unwrap();
    assert_eq!(user_derived_find_claim_acc_deserialize.contribution, 10);
    assert_eq!(user_derived_find_claim_acc_deserialize.withdrawn, 10);
    // let user_native_ada_acc = banks_client
    //     .get_account(user_native_ada)
    //     .await
    //     .unwrap()
    //     .unwrap();
    // let user_other_ada_acc = banks_client
    //     .get_account(user_other_ada)
    //     .await
    //     .unwrap()
    //     .unwrap();
    // let user_native_ada_acc_unpack =
    //     TokenAccount::unpack_from_slice(&user_native_ada_acc.data).unwrap();
    // let user_other_ada_acc_unpack =
    //     TokenAccount::unpack_from_slice(&user_other_ada_acc.data).unwrap();
    // println!(
    //     "user_native_ada_acc_unpack {:?}",
    //     user_native_ada_acc_unpack
    // );
    // println!("user_other_ada_acc_unpack {:?}", user_other_ada_acc_unpack);
    let mut transaction_user_claim = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &[5],
            vec![
                AccountMeta::new(user_keypair.pubkey(), true), //用户
                AccountMeta::new(auction4, false),
                AccountMeta::new(sysclock, false),
                AccountMeta::new(sys, false),
                AccountMeta::new(rent_pubkey, false),
                AccountMeta::new(spl_associated_token_account::id(), false),
                AccountMeta::new(user_derived_find_claim, false), //用户派生地址
                AccountMeta::new(address_four, false),            //program派生地址
                AccountMeta::new(ido_native_ada_four, false),     //program native spl ada
                AccountMeta::new(ido_other_ada_four, false),      //program other spl ada
                AccountMeta::new(user_native_ada, false),
                AccountMeta::new(user_other_ada, false),
                AccountMeta::new(token_program_id, false),
                AccountMeta::new(spl_token_address_project_pubkey, false),
                AccountMeta::new(spl_token_address_underlay_pubkey, false),
            ],
        )],
        Some(&payer.pubkey()),
    );
    transaction_user_claim.sign(&[&payer, &user_keypair], recent_blockhash);

    banks_client
        .process_transaction(transaction_user_claim)
        .await
        .expect("failed to submit");
    let user_derived_find_claim_acc = banks_client
        .get_account(user_derived_find_claim)
        .await
        .unwrap()
        .unwrap();
    let user_derived_find_claim_acc_deserialize: AccountData =
        BorshDeserialize::deserialize(&mut &user_derived_find_claim_acc.data[..]).unwrap();
    assert_eq!(user_derived_find_claim_acc_deserialize.contribution, 10);
    assert_eq!(user_derived_find_claim_acc_deserialize.withdrawn, 0);
    println!("claim success");
}
