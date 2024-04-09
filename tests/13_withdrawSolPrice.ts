import { web3, BN, Wallet, AnchorProvider, Program } from "@coral-xyz/anchor";
import { DcarbonProgram } from "../target/types/dcarbon_program";
import idl from "../target/idl/dcarbon_program.json";
import bs58 from "bs58";

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

  const withdrawPoolParams = {
    programIndex: programIndex,
  };

  //get wallet pda which will be transferred sol to owner's account
  const [walletPda] = web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("walletPda"),
      new BN(programIndex).toArrayLike(Buffer, "be", 1),
    ],
    programID
  );
  console.log("wallet pda", walletPda);

  // get authority pda
  const [authorityPda] = web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("authorityPda"),
      new BN(programIndex).toArrayLike(Buffer, "be", 1),
    ],
    programID
  );
  console.log("authority Pda", authorityPda);

  // withdraw sol from wallet pda to owner's account
  const tx = await program.methods
    .withdrawPool(withdrawPoolParams)
    .accounts({
      owner: owner.publicKey,
      walletPda: walletPda,
      authorityPda: authorityPda,
      systemProgram: SYSTEM_PROGRAM_ID,
    })
    .signers([owner.payer])
    .rpc();
  console.log(`https://explorer.solana.com/tx/${tx}?cluster=devnet`);
};

main();
