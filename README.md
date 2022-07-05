## IDO rpc

end_point: RPC `http://120.241.37.204:8899`

program: `Fkv97rKntqTUNF1oC13531VCThFdW3gB63cyYzoLgvLR`

program-keypair

> [131,12,69,122,149,58,200,55,20,140,196,164,164,233,121,12,54,91,251,29,162,29,65,6,179,60,78,143,150,135,25,88,147,173,234,55,233,143,13,172,195,121,213,47,176,67,142,221,31,200,151,241,208,234,162,217,11,1,10,192,47,201,248,248]

## IDO 中的 Solana 名词

- payer：用于支付所有交易费用和创建账号费用的支付账号
- programId：rust 编译生成的 bpf 文件的项目 id
- accounts：传入 rust program 的账户
- instruction_data：传入 rust program 的序列化后的数据
- Pubkey: sol 上的公钥，32 位。
- Keypair：密钥对，包含私钥，公钥
- Account: sol 上的账户，用来储存数据，一个账户对应一个公钥。
- ada： 账户关联地址账号，每个用户钱包地址对于不同币种，可生成不同关联地址（子地址）
- 项目币：项目方（如 AAA）发行的币 mint 地址
- 募集币：募集币（如 ETH）发行的币 mint 地址

## 所用机制解析

- 客户端调用 program 机制：
  在客户端创建一笔交易，交易中包含对应调用 program 的指令，每条指令包含三个参数：要使用的账户（isSigner 为是否签名，isWritable 为是否可写），要调用的 programId，要使用的 data；
  完成参数输入后，发送并确认交易，包含三个参数：连接，发送的交易，交易中用到的签名。即可调用 program。
- 数据存储机制:
  在 sol 中，所有数据都是存在 account 的 data 中，并且是通过序列化数据后输入的，序列化数据要按照对应的格式进行序列化，然后根据长度和租金创建相应的 account，把需要进行存储的序列化的数据，连同已创建好用于存储的 account 发到 program 中，program 便会执行对应的存储数据到 account 的操作。
- 用户在 IDO 中派生的地址：
  1、钱包派生地址
  在交易前，检查用户先前有无在对应代币中存在关联地址账号，如果没有，则我们帮他们创建，用于之后的 program 中的交易操作
  2、用户地址对 IDO 种子的派生地址
  该账号用于存储用户在不同拍卖池中，分别 submit，和可 claim 的数量，种子为 用户地址 + IDO 拍卖池的 id
- 传入 data 第一字节路由 program 方法：
  在传入的 data 序列化数据中，第一字节确定调用 program 的方法。
- Spl-token 交易机制：
  首先确保交易双方都有对应 spl-token 的关联地址，如果没有，则需要创建；

## 拍卖池子状态判断

```rust
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
```

1. 优先判断 success_flag 的值，默认是成功。如果值为失败，则池子状态为失败
2. 当前时间小于池子开始时间，则为 Upcoming
3. 当前时间大于等于关闭时间，则为 Ended
4. 当前时间大于等于且小于关闭时间，则为 Open

## rs 中用到的数据结构

- 拍卖数据存储结构

|           字段名           |                成员                |     类型      |
| :------------------------: | :--------------------------------: | :-----------: |
|         auction_id         |             拍卖池 id              |      u32      |
|        success_flag        | 筹集成功标志（0:fail, 1:success）  |      u8       |
|           access           | 拍卖池类型 （0:public, 1:private） |      u8       |
|      allocation_date       |          提取时间(10 位)           |      u32      |
|           ratio            |                比例                |      u32      |
|       tokens_offered       |              筹款上限              |      u64      |
|        funds_raised        |             已筹款资金             |      u64      |
|       min_allocation       |            最小参与额度            |      u64      |
|       max_allocation       |            最大参与额度            |      u64      |
|         pool_opens         |        开始参与时间(10 位)         |      u32      |
|        pool_closes         |        结束参与时间(10 位)         |      u32      |
|       min_swap_level       |            最小筹款数量            |      u64      |
|       spl_program_id       |           项目方币的地址           |   [u8; 32]    |
|   spl_program_id_derive    |  项目方钱包对他们自己币的派生地址  |   [u8; 32]    |
|    underlay_program_id     |            募集币的地址            |   [u8; 32]    |
| underlay_program_id_derive |       项目方募集币的派生地址       |   [u8; 32]    |
|          poolname          |              池子名字              |   [u8; 32]    |
|         whitelist          |               白名单               | Vec<[u8; 32]> |

- 用户在不同拍卖池中记录的数据

|    字段名    |        成员        | 类型 |
| :----------: | :----------------: | :--: |
| contribution | 用户已 submit 数量 | u64  |
|  withdrawn   | 用户可 claim 数量  | u64  |

- 用户 submit 的数量
  submit 数量需要小端序列化传入

|    字段名     |    成员     | 类型 |
| :-----------: | :---------: | :--: |
| submit_amount | submit 数量 | u64  |

- rust 编译后生成两个文件，一个 so 文件，一个 json 文件，so 文件用于 ts 中检查项目是否部署，json 文件用于获取 programId

```ts
const PROGRAM_SO_PATH = "/root/ji/solana-web3-demo/ido/target/deploy/ido.so";
const PROGRAM_KEYPAIR_PATH =
  "/root/ji/solana-web3-demo/ido/target/deploy/ido-keypair.json";
```

- 错误类型

```rust
    #[error("Deserialize Error")]
    DeserializeError,
    #[error("Invalid Instruction")]
    InvalidInstruction,
    #[error("Not administrator")]
    NotAdministrator,
    #[error("Timestamp Error")]
    TimestampError,
    #[error("Status Is Not Open")]
    StatusIsNotOpen,
    #[error("Status Is Not Ended")]
    StatusIsNotEnded,
    #[error("Status Has Ended")]
    StatusHasEnded,
    #[error("The number of submissions cannot be 0")]
    TheNumberOfSubmissionsCannotBe0,
    #[error("The fundraising is full")]
    TheFundraisingIsFull,
    #[error("Exceed the raising limit")]
    ExceedTheRaisingLimit,
    #[error("User Exceed the raising limit")]
    UserExceedTheRaisingLimit,
    #[error("Insufficient balance")]
    InsufficientBalance,
    #[error("Non project party public key")]
    NonProjectPartyPublicKey,
    #[error("Not exist User ada")]
    NotExistUserAda,
    #[error("Calculation overflow")]
    CalculationOverflow,
    #[error("Success flag Error")]
    SuccessFlagError,
    #[error("Allocation date has not arrived")]
    AllocationDateHasNotArrived,
    #[error("Withdrawn Zero")]
    WithdrawnZero,
    #[error("Non whitelist")]
    NonWhitelist,
    #[error("Whitelist len too long")]
    WhitelistLenTooLong,
    #[error("Auction Access Error")]
    AuctionAccessError,
    #[error("Pubkey Derived Error")]
    PubkeyDerivedError,
    #[error("Signature Error")]
    SignatureError,
    #[error("Invalid Seeds")]
    InvalidSeeds,
    #[error("Allocation Input Error")]
    AllocationInputError,
    #[error("Min Swap Level Input Error")]
    MinSwapLevelInputError,
```

## Program 通用调用方法

- 创建交易，添加指令
- 指令中 programId 为部署调用的项目 Id，keys 为需要传入账号的集合数组，data 为传入的数据
- 发送并确认交易

## IDO 中 program 调用方法

```ts
keys：[program中使用到的账号],
programId:IDO的porgramId，
data: Buffer.from(Uint8Array.of(x,...))//x: 0:修改管理员账户 1:保存拍卖数据 2:手动失败 3:项目方提币 4:用户submit 5:用户claim 6:修改白名单 ...:后续数据 7:program派生地址对spl-token的关联账号生成
```

### 新增管理员

```ts
keys：[
  { pubkey: 管理员最高权限账号, 需要签名},
  { pubkey: 储存管理员信息的账号},
  { pubkey: 新增的管理员账号},
]，
programId: programId,
data: Buffer.from(Uint8Array.of(0, 0))//0:修改管理员账户 第二个0:新建

签名账户:[payer, 管理员最高权限账号密钥对]
```

### 删除管理员

```ts
keys：[
  { pubkey: 管理员最高权限账号, 需要签名},
  { pubkey: 储存管理员信息的账号},
  { pubkey: 新增的管理员账号},
]，
programId: programId,
data: Buffer.from(Uint8Array.of(0, 1))//0:修改管理员账户 1:删除

签名账户:[payer, 管理员最高权限账号密钥对]
```

### 存储拍卖数据

```ts
keys：[
  { pubkey: 储存管理员信息的账号},
  { pubkey: 管理员账号， 需要签名},
  { pubkey: 存储拍卖信息的账号},

  { pubkey: 支付账号（这里是管理员最高权限账号）, 需要签名},
  { pubkey: program派生地址},//由Pubkey::find_program_address(&[b"ido", 拍卖池ID（4个字节）], &program_id)得到，seed为ido
  { pubkey: program派生地址对募集币的关联账号},//program对募集币spl-token生成的关联账号,由get_associated_token_address(program派生地址, 募集币spl-token)获得
  { pubkey: program派生地址对项目币的关联帐号},//program派生地址对项目币spl-token生成的关联账号
  { pubkey: 项目币账号},
  { pubkey: 募集币账号},
  { pubkey: tokenProgram账户},//TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
  { pubkey: spl_associated_token_account::id()账号},//ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL
  { pubkey: 系统账户},//11111111111111111111111111111111
  { pubkey: rent账号},//SysvarRent111111111111111111111111111111111
]，
programId: programId,
data: Buffer.from(Uint8Array.of(1, ...auction_serialize_data))//1:存储拍卖信息 auction_serialize_data：序列化后的拍卖数据

签名账户:[payer，管理员账号密钥对]
```

### 用户 submit

```ts
//创建用户不同拍卖池的seed账号，存储数据
///哈希得出seed( 32 位字节数组)
seed = Md5.hashStr(用户账户公钥 + 拍卖池Id);

///创建对应seed公钥
let (user_derived_find, _) = Pubkey::find_program_address(seed, 用户公钥);//到（derived_pubkey,nonce）

///检查账户是否已存在派生账号，如果无则发起创建账号指令交易
///获取数据长度对应的租金
lamports_account_data_serialize_len = await connection.getMinimumBalanceForRentExemption(
    account_data_serialize_len,//上文提到的AccountData序列化数据
);
SystemProgram.createAccountWithSeed({
        fromPubkey: 用户账号公钥,//创建交易费用由用户出
        basePubkey: 用户账号公钥,
        seed: seed,
        newAccountPubkey: user_derived_pubkey,
        lamports: lamports_account_data_serialize_len,//租金
        space: account_data_serialize_len,//序列化数据的长度
        programId,
      }),

///发起调用programId指令
keys：[
  { pubkey: 用户账号, 需要签名},
  { pubkey: 用户对募集币的关联账号},//用户对募集币spl-token生成的关联账号
  { pubkey: 用户派生地址},//用上述过程获取
  { pubkey: program派生地址},//由Pubkey::find_program_address(&[b"ido", 拍卖池ID（4个字节）], &program_id)得到，seed为ido
  { pubkey: program派生地址收发募集币的关联账号},//program收发募集币spl-token生成的关联账号,由get_associated_token_address(program派生地址, 募集币spl-token)获得
  { pubkey: 拍卖数据账户},
  { pubkey: 系统账户},//11111111111111111111111111111111
  { pubkey: program账户},//programId
  { pubkey: 系统时间账户},//SysvarC1ock11111111111111111111111111111111
  { pubkey: tokenProgram账户},//TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
  { pubkey: 募集币账号},
  { pubkey: rent账号},//SysvarRent111111111111111111111111111111111
]，
programId: programId,
data: Buffer.from(Uint8Array.of(4, ...transfer_account_data))//4:用户submit transfer_account_data：序列化后的submit数据

签名账户:[用户密钥对]
```

### 项目方提币

```ts
keys：[
  { pubkey: 储存管理员信息的账号},
  { pubkey: 管理员账号，需要签名},
  { pubkey: 系统时间账户},//参考submit标注
  { pubkey: 拍卖数据账户},
  { pubkey: program派生地址},
  { pubkey: program派生地址收发项目币的关联帐号},//program派生地址收发项目币spl-token生成的关联账号
  { pubkey: program派生地址收发募集币的关联帐号},
  { pubkey: 项目方对项目币的关联账号},//项目方对项目币sol-token生成的关联账号
  { pubkey: 项目方对募集币的关联账号},//项目方对募集币sol-token生成的关联账号
  { pubkey: tokenProgram账户},
  { pubkey: 项目币账号},
  { pubkey: 募集币账号},
]，
programId: programId,
data: Buffer.from(Uint8Array.of(3))//3:项目方提币

签名账户:[payer， 管理员账户密钥对]
```

### 用户 claim

```ts
keys：[
  { pubkey: 用户账号, 需要签名},
  { pubkey: 拍卖数据账户},
  { pubkey: 系统时间账户},
  { pubkey: 系统账户},
  { pubkey: rent账户},
  { pubkey: spl_associated_token_account::id()账号},//ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL
  { pubkey: 用户派生地址},
  { pubkey: program派生地址},
  { pubkey: program派生地址收发项目币的关联帐号},//program收发项目币spl-token生成的关联账号
  { pubkey: program派生地址收发募集币的关联帐号},
  { pubkey: 用户对项目币的关联账号},//用户对项目币spl-token生成的关联账号
  { pubkey: 用户对募集币的关联账号},
  { pubkey: tokenProgram账户},
  { pubkey: 项目币账号},
  { pubkey: 募集币账号},
]，
programId: programId,
data: Buffer.from(Uint8Array.of(5))//5:用户claim

签名账户:[用户密钥对]
```

### 手动失败

```ts
keys：[
  { pubkey: 储存管理员信息的账号},
  { pubkey: 管理员账户, 需要签名},
  { pubkey: 拍卖信息账户},
  { pubkey: 系统时间账户},
]，
programId: programId,
data: Buffer.from(Uint8Array.of(2))//2:手动失败

签名账户:[payer， 管理员账户密钥对]
```

### 修改白名单

```ts
keys：[
  { pubkey: 储存管理员信息的账号},
  { pubkey: 管理员账户, 需要签名},
  { pubkey: 拍卖信息账户},
  { pubkey: 系统时间账户},
]，
programId: programId,
data: Buffer.from(Uint8Array.of(6,u32:偏移量,u8:结束标志, ...whitelist_data))//6:修改白名单 whitelist_data:白名单 Vec<[u8;32]> 序列化后的数据
//偏移量为0时清空数组，从0开始传；    结束标志为1时为结束，0时为不结束
签名账户:[payer， 管理员账户密钥对]
```

### program 派生地址对 spl-token 的关联账号生成

```ts
keys：[
  { pubkey: payer, 需要签名},
  { pubkey: program派生地址},
  { pubkey: program派生地址收发项目币的关联账号},
  { pubkey: 项目币账号},
  { pubkey: tokenProgram账户},
  { pubkey: spl_associated_token_account::id()账号},//ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL
  { pubkey: 系统账户},
  { pubkey: rent账号},
]，
programId: programId,
data: Buffer.from(Uint8Array.of(7))//7:program派生地址对spl-token的关联账号生成

签名账户:[payer]
```
