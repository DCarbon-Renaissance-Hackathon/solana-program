import { web3, BN, Wallet, AnchorProvider, Program } from "@coral-xyz/anchor";
import * as anchor from "@coral-xyz/anchor";
import { DcarbonProgram } from "../target/types/dcarbon_program";
import idl from "../target/idl/dcarbon_program.json";
import bs58 from "bs58";
import secp256k1 from "secp256k1";
import { keccak_256 } from "js-sha3";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

import { BigNumber } from "bignumber.js";
import { ethers } from "ethers";

import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddress,
} from "@solana/spl-token";
require("dotenv").config();

const programID = new web3.PublicKey(process.env.PROGRAM_ID);

// random programIndex with type integer
const programIndex = parseInt(process.env.PROGRAM_INDEX);

const main = async () => {
  // LIST KEYPAIR
  const SYSTEM_PROGRAM_ID = new web3.PublicKey(
    "11111111111111111111111111111111"
  );

  //   SET PROGRAM
  const connection = new web3.Connection(
    web3.clusterApiUrl("devnet"),
    "confirmed"
  );

  // start program
  // select local wallet
  const owner = new Wallet(
    web3.Keypair.fromSecretKey(bs58.decode(process.env.PRIVATE_KEY_SOLANA_1))
  );
  console.log("owner: ", owner.publicKey);

  const provider = new AnchorProvider(connection, owner, {
    preflightCommitment: "recent",
    commitment: "processed",
  });

  //@ts-ignore
  const program = new Program(idl as DcarbonProgram, programID, provider);
  console.log("programId: ", program.programId);

  // select buyer wallet
  const buyer = new Wallet(
    web3.Keypair.fromSecretKey(bs58.decode(process.env.PRIVATE_KEY_SOLANA_2))
  );
  console.log("buyer: ", buyer.publicKey);

  // random projectId with type string
  const projectId = "2";

  // random certificateId with type string
  const certificateId = "660e70d3c256d5310d03eac1";

  // carbon token amount used to mint certificate
  const amount = 0.01;

  // get signature from api
  // backend is developing
  let signature = await getSignatureApi(
    projectId,
    buyer.publicKey.toString(),
    amount.toString()
  );

  //init params of burnTokens function
  const mintNftParams = {
    programIndex: programIndex,
    amount: new BN(amount * 10 ** parseInt(process.env.CARBON_TOKEN_DECIMAL)),

    price: new BN(signature.price * web3.LAMPORTS_PER_SOL),
    signature: signature.signature,
    recoveryId: signature.recoveryId,
    projectId: projectId,
    certificateId: certificateId,
  };

  console.log(
    "amount",
    amount * 10 ** parseInt(process.env.CARBON_TOKEN_DECIMAL)
  );

  // generate address for nft
  const newMintNFT = web3.Keypair.generate();
  console.log("new Mint NFT: ", newMintNFT.publicKey.toString());

  // get token account to hold nft
  const tokenAccountNFT = await getAssociatedTokenAddress(
    newMintNFT.publicKey,
    buyer.payer.publicKey
  );
  console.log("tokenAccountNFT: ", tokenAccountNFT.toString());

  // get mint pda. It is needed to determine which token need to be burned
  const mint = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("mint"), new BN(programIndex).toArrayLike(Buffer, "le", 1)],
    programID
  )[0];
  console.log("mint: ", mint.toString());

  //get token account pda. Currently, this account is10000000 holding tokens
  const [tokenAccount] = web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("tokenAccount"),
      new BN(programIndex).toArrayLike(Buffer, "be", 1),
    ],
    programID
  );
  console.log("tokenAccount Pda", tokenAccount);

  //get authority pda. This account is author of mint. This account help burn tokens
  const [authorityPda] = web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("authorityPda"),
      new BN(programIndex).toArrayLike(Buffer, "be", 1),
    ],
    programID
  );
  console.log("authority Pda", authorityPda);

  // get wallet pda which will be received sol from buyer's account
  const [walletPda] = web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("walletPda"),
      new BN(programIndex).toArrayLike(Buffer, "be", 1),
    ],
    programID
  );
  console.log("wallet pda", walletPda);

  // burn tokens with amount from BurnTokensParams and mint new nft. Then transfer sol from buyer's account to wallet pda
  const tx = await program.methods
    // @ts-ignore
    .mintNft(mintNftParams)
    .accounts({
      mintNft: newMintNFT.publicKey,
      tokenAccountNft: tokenAccountNFT,
      mintPda: mint,
      tokenAccountPda: tokenAccount,
      walletPda: walletPda,
      authorityPda: authorityPda,
      buyer: buyer.publicKey,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      systemProgram: SYSTEM_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID,
    })
    .signers([buyer.payer, newMintNFT])
    .rpc();
  console.log(`https://explorer.solana.com/tx/${tx}?cluster=devnet`);
};

const getSignatureApi = async (
  projectId: String,
  buyerPub: String,
  amount: String
) => {
  const url = `https://api.coingecko.com/api/v3/simple/price?ids=solana&vs_currencies=usd`;
  const PRICE_CARBON = 5;

  const backend = web3.Keypair.fromSecretKey(
    bs58.decode(process.env.PRIVATE_KEY_BACKEND || "")
  );
  const secp256k1PrivateKey = backend.secretKey.slice(0, 32);
  const CONST_TOKEN = new BigNumber("1000000000");

  // (amount * 5)/priceSolanaByUsd
  // convert amount to true type
  const amountConvert = new BigNumber(amount.toString());

  const response = await fetch(url);
  const priceSolana = await response.json();
  //@ts-ignore
  const priceSolanaByUsd = new BigNumber(priceSolana.solana.usd.toString());

  const priceRs = amountConvert
    .times(PRICE_CARBON.toString())
    .dividedBy(priceSolanaByUsd)
    .toString();

  const amountMess = amountConvert.times(CONST_TOKEN);

  let priceMess = new BigNumber(
    parseFloat(parseFloat(priceRs).toFixed(9)).toString()
  ).times(CONST_TOKEN);

  // console.log("projectId", projectId);
  // console.log("buyerPub", buyerPub);
  // console.log("amountMess", amountMess.toString());
  // console.log("priceMess", priceMess.toString());

  let message = Buffer.from(
    `${projectId.toString()}${buyerPub.toString()}${amountMess.toString()}${priceMess.toString()}`
  );
  console.log("message: ", `${projectId}${buyerPub}${amountMess}${priceMess}`);

  let messageHash = Buffer.from(keccak_256.update(message).digest());
  let { signature, recid: recoveryId } = secp256k1.ecdsaSign(
    messageHash,
    secp256k1PrivateKey
  );

  return {
    signature: Array.from(signature),
    recoveryId: recoveryId,
    price: parseFloat(parseFloat(priceRs).toFixed(9)),
  };
};

main();
