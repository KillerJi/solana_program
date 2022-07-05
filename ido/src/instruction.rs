use crate::error::IdoError::InvalidInstruction;
use crate::state::AuctionBuffer;
use borsh::BorshDeserialize;
use solana_program::{msg, program_error::ProgramError};

pub enum Instruction {
    ModifyAdministratorAccount { method: u8 },
    SaveAuctionData { auction_data: AuctionBuffer },
    ManualFailure,
    ProjectPartyWithdraw,
    Submit { amount: u64 },
    Claim,
    ModifyWhiteList { 
        whitelist: Vec<[u8; 32]>,
        offset: u32,
        end_flag: u8,
    },
    ProcessDevided,
}

impl Instruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        // msg!("{:?}", input);
        let (tag, mut rest) = input.split_first().ok_or(InvalidInstruction)?;
        msg!("tag {:?}", tag);
        Ok(match tag {
            0 => {
                let (method_tag, _last_data) = rest.split_first().ok_or(InvalidInstruction)?;
                Self::ModifyAdministratorAccount {
                    method: *method_tag,
                }
            }
            1 => Self::SaveAuctionData {
                auction_data: AuctionBuffer::deserialize(&mut rest).unwrap(),
            },
            2 => Self::ManualFailure,
            3 => Self::ProjectPartyWithdraw,
            4 => Self::Submit {
                amount: u64::try_from_slice(rest).unwrap(),
            },
            5 => Self::Claim,
            6 => {
                let (left, right) = rest.split_at(3);
                let offset = u32::try_from_slice(left).unwrap();
                msg!("offset {:?}", offset);
                let (left, mut right) = right.split_at(0);
                let end_flag = left[0];
                msg!("end_flag {:?}", end_flag);
                let whitelist: Vec<[u8; 32]> = BorshDeserialize::deserialize(&mut right).unwrap();
                Self::ModifyWhiteList {
                    whitelist,
                    offset,
                    end_flag,
                }
            }
            7 => Self::ProcessDevided,
            _ => return Err(InvalidInstruction.into()),
            //
            //just for test
            // _ => match rest[7] {
            //     1 => {
            //         msg!("here");
            //         let (_left, right) = rest.split_at(7);
            //         let (_tag, mut rest) = right.split_first().ok_or(InvalidInstruction)?;
            //         Self::SaveAuctionData {
            //             auction_data: AuctionBuffer::deserialize(&mut rest).unwrap(),
            //         }
            //     }
            //     4 => {
            //         let (_left, right) = rest.split_at(7);
            //         let (_tag, mut rest) = right.split_first().ok_or(InvalidInstruction)?;
            //         Self::Submit {
            //             amount: u64::try_from_slice(&mut rest).unwrap(),
            //         }
            //     }
            //     6 => {
            //         let (_left, right) = rest.split_at(7);
            //         let (_tag, mut rest) = right.split_first().ok_or(InvalidInstruction)?;
            //         let whitelist: Vec<[u8; 32]> =
            //             BorshDeserialize::deserialize(&mut rest).unwrap();
            //         Self::ModifyWhiteList {
            //             whitelist: whitelist,
            //         }
            //     }
            //     _ => {
            //         return Err(InvalidInstruction.into());
            //     }
            // },
        })
    }
}
