/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-unsafe-member-access */

import {
  Account,
  Connection,
  PublicKey,
  LAMPORTS_PER_SOL,
  SystemProgram,
  TransactionInstruction,
  Transaction,
  sendAndConfirmTransaction,
  SystemInstruction,
  SYSVAR_CLOCK_PUBKEY,
  Keypair,


} from '@solana/web3.js';
import fs from 'mz/fs';
import path from 'path';
import * as borsh from 'borsh';
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import {
  getPayer,
  getRpcUrl,
  newAccountWithLamports,
  readAccountFromFile,
} from './utils';
import { Md5 } from 'ts-md5/dist/md5';
import { TextEncoder, TextDecoder } from "util"
/**
 * Connection to the network
 */
let connection: Connection;
let payerAccount: Account;
let programId: PublicKey;
let total_auction_id_array = new Map();
let public_auction_id_array = new Map();
let private_auction_id_array = new Map();
const TRANSFER_ONE = ('/root/ji/transfer_one.json');
const IDO = ('/root/ji/solana-web3-demo/ido.json');
/**
 * Path to program shared object file which should be deployed on chain.
 * This file is created when running either:
 *   - `npm run build:program-c`
 *   - `npm run build:program-rust`
 */
//const PROGRAM_SO_PATH = path.join(PROGRAM_PATH, 'helloworld.so');
const PROGRAM_SO_PATH = ('/root/ji/solana-web3-demo/ido/target/deploy/ido.so');
/**
 * Path to the keypair of the deployed program.
 * This file is created when running `solana program deploy dist/program/helloworld.so`
 */
const PROGRAM_KEYPAIR_PATH = ('/root/ji/solana-web3-demo/ido/target/deploy/ido-keypair.json');

/**
 * The state of a greeting account managed by the hello world program
 */

class GreetingAccount {
  counter = 0;
  constructor(fields: { counter: number } | undefined = undefined) {
    if (fields) {
      this.counter = fields.counter;
    }
  }
}
const GreetingSchema = new Map([
  [GreetingAccount, { kind: 'struct', fields: [['counter', 'u32']] }],
]);
const GREETING_SIZE = borsh.serialize(
  GreetingSchema,
  new GreetingAccount(),
).length;

class TransferAccount {
  amount!: number;
  constructor(fields: { amount: number } | undefined = undefined) {
    if (fields) {
      this.amount = fields.amount;
    }
  }
}
const TransferAccountSchema = new Map([
  [TransferAccount, { kind: 'struct', fields: [['amount', 'u64']] }],
]);
const transfer_account = new TransferAccount({ amount: 2 });
const transfer_account_data = borsh.serialize(
  TransferAccountSchema,
  transfer_account
);

class AccountData {
  contribution!: number; // 累计submit的spl-tokens数量
  withdrawn!: number; // 累计已提取的spl-token或者项目方代币(成功或者失败)
  constructor(fields: { contribution: number, withdrawn: number } | undefined = undefined) {
    if (fields) {
      this.contribution = fields.contribution;
      this.withdrawn = fields.withdrawn;
    }
  }
}
const AccountDataSchema = new Map([
  [AccountData, {
    kind: 'struct',
    fields: [
      ['contribution', 'u64'], ['withdrawn', 'u64']
    ]

  }]
]);
const account_data = new AccountData({ contribution: 0, withdrawn: 0 });
const account_data_serialize = borsh.serialize(AccountDataSchema, account_data);
const account_data_serialize_len = account_data_serialize.length;

class AuctionData {
  auction_id!: number;
  access!: string;
  whitelist!: string[];
  poolname!: string;
  ratio!: number;
  tokens_offered!: number;
  funds_raised!: number;
  min_allocation!: number;
  max_allocation!: number;
  pool_opens!: number;
  pool_closes!: number;
  min_swap_level!: number;
  spl_program_id!: string;
  spl_program_id_derive!: string;
  spl_program_id_derive_ido!: string;
  underlay_program_id!: string;
  underlay_program_id_derive!: string;
  underlay_program_id_derive_ido!: string; //IDO的ETH	
  allocation_date!: number;
  success_flag!: number;
  constructor(fields: {
    auction_id: number, access: string, whitelist: string[],
    poolname: string, ratio: number, tokens_offered: number, funds_raised: number,
    min_allocation: number, max_allocation: number, pool_opens: number,
    pool_closes: number, min_swap_level: number, spl_program_id: string,
    spl_program_id_derive: string, spl_program_id_derive_ido: string,
    underlay_program_id: string, underlay_program_id_derive: string,
    underlay_program_id_derive_ido: string, allocation_date: number,
    success_flag: number,
  } | undefined = undefined) {
    if (fields) {
      this.auction_id = fields.auction_id;
      this.access = fields.access;
      this.whitelist = fields.whitelist;
      this.poolname = fields.poolname;
      this.ratio = fields.ratio;
      this.tokens_offered = fields.tokens_offered;
      this.funds_raised = fields.funds_raised;
      this.min_allocation = fields.min_allocation;
      this.max_allocation = fields.max_allocation;
      this.pool_opens = fields.pool_opens;
      this.pool_closes = fields.pool_closes;
      this.min_swap_level = fields.min_swap_level;
      this.spl_program_id = fields.spl_program_id;
      this.spl_program_id_derive = fields.spl_program_id_derive;
      this.spl_program_id_derive_ido = fields.spl_program_id_derive_ido;
      this.underlay_program_id = fields.underlay_program_id;
      this.underlay_program_id_derive = fields.underlay_program_id_derive;
      this.underlay_program_id_derive_ido = fields.underlay_program_id_derive_ido;
      this.allocation_date = fields.allocation_date;
      this.success_flag = fields.success_flag;
    }
  }
}
const AuctionSchema = new Map([
  [AuctionData, {
    kind: 'struct',
    fields: [
      ['auction_id', 'u32'], ['access', 'string'], ['whitelist', ['string']],
      ['poolname', 'string'], ['ratio', 'u32'], ['tokens_offered', 'u64'],
      ['funds_raised', 'u64'], ['min_allocation', 'u32'], ['max_allocation', 'u32'],
      ['pool_opens', 'u32'], ['pool_closes', 'u32'], ['min_swap_level', 'u32'],
      ['spl_program_id', 'string'], ['spl_program_id_derive', 'string'],
      ['spl_program_id_derive_ido', 'string'], ['underlay_program_id', 'string'],
      ['underlay_program_id_derive', 'string'], ['underlay_program_id_derive_ido', 'string'],
      ['allocation_date', 'u32'], ['success_flag', 'u8']
    ]

  }]
]);
let _arr: string[] = ['3wVY8wx8XTHMaQvQxb369ieDzPqkgFmXp9WaL4bk9LSD',
  '3wVY8wx8XTHMaQvQxb369ieDzPqkgFmXp9WaL4bk9LSD', '3wVY8wx8XTHMaQvQxb369ieDzPqkgFmXp9WaL4bk9LSD'];
let auction_num: number = 0;
const _data = new AuctionData({
  auction_id: auction_num, access: 'public', whitelist: _arr,
  poolname: 'sb', ratio: 100000, tokens_offered: 200, funds_raised: 0, min_allocation: 1,
  max_allocation: 100, pool_opens: 1626165260, pool_closes: 1626165268,
  min_swap_level: 90, spl_program_id: '9saMuwx7BgBE3LhR8pgD91yjnCpWxyGJb4net3Gp6162',
  spl_program_id_derive: '7WeNUsuL9PrrGdiedKY79zkDtZjSzpvMSUfykmTipamN',
  spl_program_id_derive_ido: '6oWg3WC2oLhdckxMSB7U7a5ruoRjepVfG2zZdNDJP8X9',
  underlay_program_id: 'ETYv226Wiac3Szr4QGwBAinNTiVgEjP6ZiwmtwgxSxib',
  underlay_program_id_derive: '4mR4DMMkoSEnVxaa2UJ5CDjpydWcTdbEW7FEk4fYQQMH',
  underlay_program_id_derive_ido: '8aFQkSa42521rmxRWpr1tNgB9x3DzmyNZmFFbUoQR1D8',
  allocation_date: 1000, success_flag: 0
});

const auction_serialize_data = borsh.serialize(AuctionSchema, _data);
console.log(auction_serialize_data);
// const auction_serialize_data_buffer = Buffer.from(auction_serialize_data);
// const newValue = borsh.deserialize(AuctionSchema, AuctionData, auction_serialize_data_buffer);
const newValueLen = auction_serialize_data.length;
export async function establishConnection(): Promise<void> {
  const rpcUrl = await getRpcUrl();
  connection = new Connection(rpcUrl, 'confirmed');
  const version = await connection.getVersion();
  console.log('Connection to cluster established:', rpcUrl, version);
}

/**
 * Establish an account to pay for everything
 */
export async function establishPayer(): Promise<void> {
  let fees = 0;
  if (!payerAccount) {
    const { feeCalculator } = await connection.getRecentBlockhash();
    // Calculate the cost to fund the greeter account
    fees += await connection.getMinimumBalanceForRentExemption(GREETING_SIZE);

    // Calculate the cost of sending transactions
    fees += feeCalculator.lamportsPerSignature * 100; // wag

    try {
      // Get payer from cli config
      payerAccount = await getPayer();
    } catch (err) {
      // Fund a new payer via airdrop
      payerAccount = await newAccountWithLamports(connection, fees);
    }
  }

  const lamports = await connection.getBalance(payerAccount.publicKey);
  console.log('lamports', lamports, 'fee', fees);
  if (lamports < fees) {
    // This should only happen when using cli config keypair
    const sig = await connection.requestAirdrop(
      payerAccount.publicKey,
      fees - lamports,
    );
    await connection.confirmTransaction(sig);
  }
  console.log('payerAccount', payerAccount);
  console.log(
    'Using account',
    payerAccount.publicKey.toBase58(),
    'containing',
    lamports / LAMPORTS_PER_SOL,
    'SOL to pay for fees',
  );
  let payerAccount_acc = await connection.getAccountInfo(payerAccount.publicKey);
  console.log('payerAccount_accp', payerAccount_acc);
}
/**
 * Check if the hello world BPF program has been deployed
 */

export async function checkProgram(): Promise<void> {
  // Read program id from keypair file
  try {
    const programAccount = await readAccountFromFile(PROGRAM_KEYPAIR_PATH);
    programId = programAccount.publicKey;
  } catch (err) {
    const errMsg = (err as Error).message;
    throw new Error(
      `Failed to read program keypair at '${PROGRAM_KEYPAIR_PATH}' due to error: ${errMsg}. Program may need to be deployed with \`solana program deploy dist/program/helloworld.so\``,
    );
  }
  const programInfo = await connection.getAccountInfo(programId);
  if (programInfo === null) {
    if (fs.existsSync(PROGRAM_SO_PATH)) {
      throw new Error(
        'Program needs to be deployed with `solana program deploy /root/ji/example-helloworld/src/cross-program-invocation/target/deploy/spl_example_cross_program_invocation.so`',
      );
    } else {
      throw new Error('Program needs to be built and deployed');
    }
  } else if (!programInfo.executable) {
    throw new Error(`Program is not executable`);
  }
  else {
    console.log('programInfo', programInfo.owner.toBase58());
  }
}


export async function ido_process(): Promise<void> {
  //前置条件
  console.log('SYSVAR_CLOCK_PUBKEY ', SYSVAR_CLOCK_PUBKEY.toBase58());
  console.log('TOKEN_PROGRAM_ID ', TOKEN_PROGRAM_ID.toBase58());
  const transfer_one_private = await readAccountFromFile(TRANSFER_ONE);
  const ido_private = await readAccountFromFile(IDO);
  console.log('1111 11ido_private', ido_private);
  let auction = Keypair.generate();
  console.log('auction ', auction.publicKey.toBase58());
  let admin = Keypair.generate();
  let new_key = Keypair.generate();
  let new_key2 = Keypair.generate();
  let new_key3 = Keypair.generate();
  let new_key4 = Keypair.generate();
  const lamports = await connection.getMinimumBalanceForRentExemption(
    newValueLen,
  );
  const transaction = new Transaction().add(
    SystemProgram.createAccount({
      fromPubkey: payerAccount.publicKey,
      lamports,
      newAccountPubkey: auction.publicKey,
      programId,
      space: newValueLen,
    })
  );
  await sendAndConfirmTransaction(connection, transaction, [payerAccount, auction]);

  const lamports_admin = await connection.getMinimumBalanceForRentExemption(
    23283064365
  );
  const transaction_create_admin_acc = new Transaction().add(
    SystemProgram.createAccount({
      fromPubkey: payerAccount.publicKey,
      lamports: lamports_admin,
      newAccountPubkey: admin.publicKey,
      programId,
      space: 23283064365,
    })
  );
  await sendAndConfirmTransaction(connection, transaction_create_admin_acc, [payerAccount, admin]);
  let user_native_ada = new PublicKey('2JyiLuBDj8425Vu2Ew693uNmCFoFbwta6Vmv46QXD5Sc');//用户对项目方币的ada
  let user_other_ada = new PublicKey('CkyB6j1J4QSNthZG5JrFuQzqL5VxPEnGVY3odUrLpQZb');//用户对募集币的ada
  let project_native_ada = new PublicKey('8bMf9s4qR6WUsxLDzzHNvpFfLbiaffR9h823kjH77HBF');// 项目方对项目方币的ada;       
  let profect_other_ada = new PublicKey('6bHZtfakaFzbisgXYY7n2kN2jnMxpBrd78Ym8XpC2GLq');// 项目方对募集币的ada
  let ido_native_ada = new PublicKey('2itCtD2i6CMK2amFGLXd429eqUW6iL6rvtdyqoBoVeEe');//IDO对项目方币的ada
  let ido_other_ada = new PublicKey('H5mvHwhV4PaSiLv4NJ3xPpc1X4QQKukrE9muL1H6HtHP');//IDO对募集币的ada
  let spl_token_address_project = new PublicKey('37euiKjLaSV4nS7KAuErkH3h5nYJ3ow7rMsgTRYYhL2H');
  let spl_token_address_underlay = new PublicKey('424JgwGipQnt2rPYXjPhzKzTyBBJ9kFrwaCKru8a8PVi');
  let ido_native_ada_authority = ido_private.publicKey;
  let ido_other_ada_authority = ido_private.publicKey;
  //新增管理员账号
  console.log('instruction_modify_admin_acc_add start');
  const instruction_modify_admin_acc_add = new TransactionInstruction({
    keys: [
      { pubkey: payerAccount.publicKey, isSigner: false, isWritable: true },//运营识别账号
      { pubkey: admin.publicKey, isSigner: false, isWritable: true },
      { pubkey: new_key.publicKey, isSigner: false, isWritable: true },
      { pubkey: new_key3.publicKey, isSigner: false, isWritable: true },
      { pubkey: new_key4.publicKey, isSigner: false, isWritable: true },
    ],
    programId: programId,
    data: Buffer.from(Uint8Array.of(0, 0))
  });
  await sendAndConfirmTransaction(
    connection,
    new Transaction().add(instruction_modify_admin_acc_add),
    [payerAccount],
  );
  console.log('instruction_modify_admin_acc_add success');
  //删除管理员
  console.log('instruction_modify_admin_acc_del start');
  const instruction_modify_admin_acc_del = new TransactionInstruction({
    keys: [//不能传入过多账号 最多10个左右合适
      { pubkey: payerAccount.publicKey, isSigner: false, isWritable: true },
      { pubkey: admin.publicKey, isSigner: false, isWritable: true },
      { pubkey: new_key.publicKey, isSigner: false, isWritable: true },
      { pubkey: new_key2.publicKey, isSigner: false, isWritable: true },
      { pubkey: new_key4.publicKey, isSigner: false, isWritable: true },
    ],
    programId: programId,
    data: Buffer.from(Uint8Array.of(0, 1))
  });
  await sendAndConfirmTransaction(
    connection,
    new Transaction().add(instruction_modify_admin_acc_del),
    [payerAccount],
  );
  console.log('instruction_modify_admin_acc_del success');

  ///存储拍卖数据
  console.log('instruction_save_auction_data start');
  //存储拍卖信息  id =》 拍卖账号公钥
  //总拍卖信息存储
  total_auction_id_array.set(auction_num, auction.publicKey);
  //公私拍卖池分别存储
  // if (_data.access == 'public') {
  //   public_auction_id_array.set(auction_num, auction.publicKey);
  // }
  // else if (_data.access == 'private') {
  //   private_auction_id_array.set(auction_num, auction.publicKey);
  // }
  // else {
  //   throw new Error('Auction access Error');
  // }
  //获取拍卖池中所有id
  // for (let key of total_auction_id_array.keys()) {
  //   console.log(key);
  // }

  const instruction_save_auction_data = new TransactionInstruction({
    keys: [
      { pubkey: admin.publicKey, isSigner: false, isWritable: true },
      { pubkey: new_key3.publicKey, isSigner: false, isWritable: true },
      { pubkey: auction.publicKey, isSigner: false, isWritable: true },
    ],
    programId: programId,
    data: Buffer.from(Uint8Array.of(1, ...auction_serialize_data))
  });
  await sendAndConfirmTransaction(
    connection,
    new Transaction().add(instruction_save_auction_data),
    [payerAccount],
  );

  // let key = new PublicKey('AZnzC72bxhRWXtqAYC8h85mnDmgPYVoNyG5zS3sL5Bti');
  // console.log("key", key.toBase58());
  // let auction_account = await connection.getAccountInfo(key);
  let auction_account = await connection.getAccountInfo(auction.publicKey);

  console.log("auction_account", auction_account);
  console.log("auction.publicKey", auction.publicKey.toBase58());
  if (auction_account != null) {
    let auction_account_data = borsh.deserialize(AuctionSchema, AuctionData, auction_account.data);
    console.log("auction_account_data ", auction_account_data);
  }
  let key = new PublicKey(auction.publicKey);
  let key_account = await connection.getAccountInfo(key);
  console.log("key", key.toBase58());
  if (key_account != null) {
    let auction_account_data = borsh.deserialize(AuctionSchema, AuctionData, key_account.data);
    console.log("auction_account_data ", auction_account_data);
  }

  auction_num++;
  //查询指定id对应的拍卖信息
  // let key_num = 0;
  // let auction_account = await connection.getAccountInfo(total_auction_id_array.get(key_num));
  // if (auction_account != null) {
  //   let auction_account_data = borsh.deserialize(AuctionSchema, AuctionData, auction_account.data);
  //   console.log(auction_account_data);
  // }
  // else {
  //   console.log('none data');
  // }
  // console.log('instruction_save_auction_data success');
  //查询公有池id对应的拍卖信息 私有同理
  // let public_auction_account = await connection.getAccountInfo(public_auction_id_array.get(key_num));
  // if (public_auction_account != null) {
  //   let auction_account_data = borsh.deserialize(AuctionSchema, AuctionData, public_auction_account.data);
  //   console.log(auction_account_data);
  // }
  // else {
  //   console.log('none data');
  // }
  console.log('instruction_save_auction_data success');

  //用户submit
  // 创建seed(max_length=32)账号 存储数据
  console.log('instruction_user_submit start');
  let user_wallet = transfer_one_private.publicKey;
  // console.log((await connection.getTokenAccountBalance( user_other_ada)));
  const __SEED = Md5.hashStr(transfer_one_private.publicKey.toString() + _data.auction_id);
  console.log('__SEED', __SEED);
  let user_derived_pubkey = await PublicKey.createWithSeed(
    payerAccount.publicKey,
    __SEED,
    programId,
  );
  const lamports_account_data_serialize_len = await connection.getMinimumBalanceForRentExemption(
    account_data_serialize_len,
  );
  const user_derived_acc = await connection.getAccountInfo(user_derived_pubkey);
  if (user_derived_acc == null) {
    const transaction_submit_create_account = new Transaction().add(
      SystemProgram.createAccountWithSeed({
        fromPubkey: payerAccount.publicKey,
        basePubkey: payerAccount.publicKey,
        seed: __SEED,
        newAccountPubkey: user_derived_pubkey,
        lamports: lamports_account_data_serialize_len,
        space: account_data_serialize_len,
        programId,
      }),
    );
    await sendAndConfirmTransaction(connection, transaction_submit_create_account, [payerAccount]);
  }

  const instruction_user_submit = new TransactionInstruction({
    keys: [
      { pubkey: user_other_ada, isSigner: false, isWritable: true },//用户募集币ada 
      { pubkey: user_wallet, isSigner: true, isWritable: false },//用户签名账号
      { pubkey: ido_other_ada, isSigner: false, isWritable: true },//ido募集币ada
      { pubkey: user_derived_pubkey, isSigner: false, isWritable: true },//用户对该拍卖池的储存账户
      { pubkey: auction.publicKey, isSigner: false, isWritable: true },//拍卖账户数据
      { pubkey: SYSVAR_CLOCK_PUBKEY, isSigner: false, isWritable: true },//时间
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
      { pubkey: spl_token_address_underlay, isSigner: false, isWritable: false }
    ],
    programId: programId,
    data: Buffer.from(Uint8Array.of(4, ...transfer_account_data))
  });
  await sendAndConfirmTransaction(
    connection,
    new Transaction().add(instruction_user_submit),
    [payerAccount, transfer_one_private],
  );
  console.log('instruction_user_submit success');

  //项目方提币
  console.log('instruction_project_party_withdraw start');

  let auction_account2 = await connection.getAccountInfo(auction.publicKey);
  console.log("auction.publicKey", auction.publicKey.toBase58());

  if (auction_account2 != null) {
    console.log("data   ", auction_account2.data);
    let auction_account_data = borsh.deserialize(AuctionSchema, AuctionData, auction_account2.data);
    console.log("auction_account_data ", auction_account_data);
  }

  const instruction_project_party_withdraw = new TransactionInstruction({
    keys: [
      { pubkey: admin.publicKey, isSigner: false, isWritable: true },//管理员地址 check
      { pubkey: new_key3.publicKey, isSigner: false, isWritable: true },//key
      { pubkey: SYSVAR_CLOCK_PUBKEY, isSigner: false, isWritable: true },//时间
      { pubkey: auction.publicKey, isSigner: false, isWritable: true },//拍卖数据

      { pubkey: ido_native_ada, isSigner: false, isWritable: true },//ido项目币ada
      { pubkey: ido_native_ada_authority, isSigner: true, isWritable: false },//ido签名账户
      { pubkey: ido_other_ada, isSigner: false, isWritable: true },//ido募集币ada
      { pubkey: ido_other_ada_authority, isSigner: true, isWritable: false },//ido签名账户
      { pubkey: project_native_ada, isSigner: false, isWritable: true },//项目方项目币ada
      { pubkey: profect_other_ada, isSigner: false, isWritable: true },//项目方募集币ada
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
      { pubkey: spl_token_address_project, isSigner: false, isWritable: false },
      { pubkey: spl_token_address_underlay, isSigner: false, isWritable: false },
    ],
    programId: programId,
    data: Buffer.from(Uint8Array.of(3))
  });
  await sendAndConfirmTransaction(
    connection,
    new Transaction().add(instruction_project_party_withdraw),
    [payerAccount, ido_private],
  );
  console.log('instruction_project_party_withdraw success');

  let auction_account3 = await connection.getAccountInfo(auction.publicKey);
  console.log("auction.publicKey", auction.publicKey.toBase58());

  if (auction_account3 != null) {
    console.log("data   ", auction_account3.data);
    let auction_account_data = borsh.deserialize(AuctionSchema, AuctionData, auction_account3.data);
    console.log("auction_account_data ", auction_account_data);
  }

  //用户claim
  console.log('instruction_claim start');
  const instruction_claim = new TransactionInstruction({
    keys: [
      { pubkey: auction.publicKey, isSigner: false, isWritable: true },//拍卖数据
      { pubkey: SYSVAR_CLOCK_PUBKEY, isSigner: false, isWritable: true },//时间
      { pubkey: user_derived_pubkey, isSigner: false, isWritable: true },//用户对拍卖池的储存账户
      { pubkey: ido_native_ada, isSigner: false, isWritable: true },//ido项目币ada
      { pubkey: ido_native_ada_authority, isSigner: true, isWritable: false },//ido签名账户
      { pubkey: ido_other_ada, isSigner: false, isWritable: true },//ido募集币ada
      { pubkey: ido_other_ada_authority, isSigner: true, isWritable: false },//ido签名账户
      { pubkey: user_native_ada, isSigner: false, isWritable: true },//用户项目币ada
      { pubkey: user_other_ada, isSigner: false, isWritable: true },//用户募集币ada
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
      { pubkey: spl_token_address_project, isSigner: false, isWritable: false },
      { pubkey: spl_token_address_underlay, isSigner: false, isWritable: false },
    ],
    programId: programId,
    data: Buffer.from(Uint8Array.of(5))
  });
  await sendAndConfirmTransaction(
    connection,
    new Transaction().add(instruction_claim),
    [payerAccount, ido_private],
  );
  console.log('instruction_claim success');

  console.log('user_native_ada ', (await connection.getTokenAccountBalance(user_native_ada)));
  console.log('user_other_ada ', (await connection.getTokenAccountBalance(user_other_ada)));

  // 手动失败
  // const instruction3 = new TransactionInstruction({
  //   keys: [
  //     {pubkey: admin.publicKey ,isSigner: false, isWritable: true},
  //     {pubkey: new_key3.publicKey ,isSigner: false, isWritable: true},
  //     {pubkey: auction.publicKey ,isSigner: false, isWritable: true},
  //   ],
  //   programId: programId,
  //   data: Buffer.from(Uint8Array.of(2))
  // });
  // console.log('success');
  // await sendAndConfirmTransaction(
  //   connection,
  //   new Transaction().add(instruction3), 
  //   [payerAccount],
  // );






}

