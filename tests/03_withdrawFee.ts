import { web3, BN, Wallet, AnchorProvider, Program } from "@coral-xyz/anchor";
import { DcarbonProgram } from "../target/types/dcarbon_program";
import idl from "../target/idl/dcarbon_program.json";
import bs58 from "bs58";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  getAssociatedTokenAddress,
} from "@solana/spl-token";

require("dotenv").config();

const programID = new web3.PublicKey(process.env.PROGRAM_ID);
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

  const wallet = new Wallet(
    web3.Keypair.fromSecretKey(bs58.decode(process.env.PRIVATE_KEY_SOLANA_1))
  );

  const provider = new AnchorProvider(connection, wallet, {
    preflightCommitment: "recent",
    commitment: "processed",
  });
  //@ts-ignore
  const program = new Program(idl as DcarbonProgram, programID, provider);

  console.log("programId: ", program.programId);

  const admin = web3.Keypair.fromSecretKey(
    bs58.decode(process.env.PRIVATE_KEY_SOLANA_1)
  );

  const mint = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("mint"), new BN(programIndex).toArrayLike(Buffer, "le", 1)],
    programID
  )[0];
  console.log("mint: ", mint.toString());

  const tokenAccount = await getAssociatedTokenAddress(mint, admin.publicKey);
  console.log("tokenAccount: ", tokenAccount.toString());

  // get config of campaign
  const projectState = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("project"), mint.toBuffer()],
    programID
  )[0];
  console.log("projectState: ", projectState);

  const [authorityPda] = web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("authorityPda"),
      new BN(programIndex).toArrayLike(Buffer, "be", 1),
    ],
    programID
  );
  console.log("authority Pda", authorityPda);

  const tx = await program.methods
    .withdrawFee({
      programIndex: programIndex,
    })
    .accounts({
      mint: mint,
      tokenAccount: tokenAccount,
      admin: admin.publicKey,
      projectState: projectState,
      authorityPda: authorityPda,
      systemProgram: SYSTEM_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    })
    .signers([admin])
    .rpc();
  console.log(`https://explorer.solana.com/tx/${tx}?cluster=devnet`);

  await connection.confirmTransaction(tx, "confirmed");

  const stateData = await program.account.projectState.fetch(projectState);
  console.log("projetState: ", stateData);
};

main();
