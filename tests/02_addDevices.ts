import { web3, BN, Wallet, AnchorProvider, Program } from "@coral-xyz/anchor";
import { DcarbonProgram } from "../target/types/dcarbon_program";
import idl from "../target/idl/dcarbon_program.json";
import bs58 from "bs58";
import { ethers } from "ethers";

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

  const owner = web3.Keypair.fromSecretKey(
    bs58.decode(process.env.PRIVATE_KEY_SOLANA_2)
  );

  const devices = [
    {
      evmAddress: ethers.utils.arrayify(
        "0x26E754233F6A50ee5AE3ee6A0217aD907dc3386B"
      ),
      deviceType: 1,
      limitAmount: new BN(1 * 10 ** 9),
      owner: owner.publicKey.toString(),
    },
  ];

  const mint = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("mint"), new BN(programIndex).toArrayLike(Buffer, "le", 1)],
    programID
  )[0];
  console.log("mint: ", mint.toString());

  // get config of campaign
  const projectState = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("project"), mint.toBuffer()],
    programID
  )[0];
  console.log("projectState: ", projectState);

  const tx = await program.methods
    // @ts-ignore
    .addDevices({
      devices: devices,
    })
    .accounts({
      owner: admin.publicKey,
      projectState: projectState,
      systemProgram: SYSTEM_PROGRAM_ID,
    })
    .signers([admin])
    .rpc();
  console.log(`https://explorer.solana.com/tx/${tx}?cluster=devnet`);

  await connection.confirmTransaction(tx, "confirmed");

  const stateData = await program.account.projectState.fetch(projectState);
  console.log("projetState: ", stateData);
};

main();
