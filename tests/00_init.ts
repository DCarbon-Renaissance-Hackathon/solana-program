import { web3, BN, Wallet, AnchorProvider, Program } from "@coral-xyz/anchor";
import { DcarbonProgram } from "../target/types/dcarbon_program";
import idl from "../target/idl/dcarbon_program.json";
import bs58 from "bs58";
import secp256k1 from "secp256k1";
import {
  MPL_TOKEN_METADATA_PROGRAM_ID,
  findMetadataPda,
  mplTokenMetadata,
} from "@metaplex-foundation/mpl-token-metadata";
import { publicKey } from "@metaplex-foundation/umi";
import { ethers } from "ethers";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";

require("dotenv").config();

const programID = new web3.PublicKey(process.env.PROGRAM_ID);

// random programIndex with type integer
const programIndex = parseInt(process.env.PROGRAM_INDEX);
const ethWallet = new ethers.Wallet(process.env.PRIVATE_KEY_EVM);

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
  const umi = createUmi(web3.clusterApiUrl("devnet")).use(mplTokenMetadata());
  const transaction = new web3.Transaction();

  // --- INIT PROGRAM ---
  //init params of initPdas function
  const initAuthorityParams = {
    programIndex: programIndex,
    signer: getSigner(),
    adminAddress: owner.publicKey,
  };

  // init authority pda to authorize for mint tokens and mint nfts
  const [authorityPda] = web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("authorityPda"),
      new BN(programIndex).toArrayLike(Buffer, "be", 1),
    ],
    programID
  );
  console.log("authority Pda", authorityPda);

  // init wallet pda to get transfer sol from the buyer and withdraw sol for owner's account
  const [walletPda] = web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("walletPda"),
      new BN(programIndex).toArrayLike(Buffer, "be", 1),
    ],
    programID
  );
  console.log("wallet pda", walletPda);

  transaction.add(
    await program.methods
      //@ts-ignore
      .initAuthority(initAuthorityParams)
      .accounts({
        authorityPda: authorityPda,
        walletPda: walletPda,
        payer: owner.publicKey,
        systemProgram: SYSTEM_PROGRAM_ID,
      })
      .signers([owner.payer])
      .instruction()
  );

  const initializeCarbonStateParam = {
    programIndex: programIndex,
    fee: new BN(0.05 * 10 ** 9),
    decimals: 9,
    name: "CARBON",
    symbol: "CARBON",
    uri: "https://static.innovaz.io/nft/metadata/65ae2eecbc73838f5feea43c/1.json",

    devices: [
      {
        evmAddress: ethers.utils.arrayify(
          "0x29E754233F6A50ee5AE3ee6A0217aD907dc3386B"
        ),
        deviceType: 1,
        limitAmount: new BN("1000000000000"),
        owner: owner.publicKey.toString(),
      },
      {
        evmAddress: ethers.utils.arrayify(
          "0x574033830a3F134570FbDe995C91b68E8a062aaa"
        ),
        deviceType: 1,
        limitAmount: new BN("10000000000000"),
        owner: owner.publicKey.toString(),
      },
      {
        evmAddress: ethers.utils.arrayify(
          "0xC4d3770de3D0642be0e1C9d0aE12f673f83997A1"
        ),
        deviceType: 1,
        limitAmount: new BN("100000000000000"),
        owner: owner.publicKey.toString(),
      },
    ],
    ethAddress: Array.from(ethers.utils.arrayify(ethWallet.address)),
  };

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

  // get derive metadata account of nft collection
  let metadataAccount = findMetadataPda(umi, {
    mint: publicKey(mint),
  })[0];
  console.log("metadataAccount: ", metadataAccount);

  transaction.add(
    await program.methods
      //@ts-ignore
      .initializeCarbonState(initializeCarbonStateParam)
      .accounts({
        owner: owner.publicKey,
        mint: mint,
        authorityPda: authorityPda,
        projectState: projectState,
        metadataAccount: metadataAccount,
        systemProgram: SYSTEM_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        tokenMetadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
        rent: web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([owner.payer])
      .instruction()
  );

  const initTokenAccountParams = {
    programIndex: programIndex,
  };

  // init token account pda which will be received token from owner's account
  const [tokenAccount] = web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("tokenAccount"),
      new BN(programIndex).toArrayLike(Buffer, "be", 1),
    ],
    programID
  );
  console.log("tokenAccount Pda", tokenAccount);

  // init token account pda which will be received token from owner's account
  transaction.add(
    await program.methods
      .initTokenAccount(initTokenAccountParams)
      .accounts({
        mint: mint,
        tokenAccount: tokenAccount,
        owner: owner.publicKey,
        systemProgram: SYSTEM_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([owner.payer])
      .instruction()
  );

  const tx = await connection.sendTransaction(transaction, [owner.payer]);
  await connection.confirmTransaction(tx);

  console.log(`https://explorer.solana.com/tx/${tx}?cluster=devnet`);

  const projectData = await program.account.projectState.fetch(projectState);
  console.log("projectData: ", JSON.stringify(projectData, null, 2));
};

const getSigner = () => {
  const backend = web3.Keypair.fromSecretKey(
    bs58.decode(process.env.PRIVATE_KEY_BACKEND)
  );
  const secp256k1PrivateKey = backend.secretKey.slice(0, 32);

  // Derive the public key
  let secp256k1PublicKey = secp256k1
    .publicKeyCreate(secp256k1PrivateKey, false)
    .slice(1);

  console.log("##### secp256k1PublicKey", secp256k1PublicKey);

  return secp256k1PublicKey;
};
main();
