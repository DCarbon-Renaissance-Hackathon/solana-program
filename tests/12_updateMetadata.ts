import { web3, BN, Wallet, AnchorProvider, Program } from "@coral-xyz/anchor";
import { DcarbonProgram } from "../target/types/dcarbon_program";
import idl from "../target/idl/dcarbon_program.json";
import bs58 from "bs58";
import {
  MPL_TOKEN_METADATA_PROGRAM_ID,
  findMetadataPda,
  mplTokenMetadata,
  findMasterEditionPda,
} from "@metaplex-foundation/mpl-token-metadata";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { publicKey } from "@metaplex-foundation/umi";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

require("dotenv").config();

const programID = new web3.PublicKey(process.env.PROGRAM_ID);

// random programIndex with type integer
const programIndex = parseInt(process.env.PROGRAM_INDEX);

// get mint NFT address which created in the mint nft step
const nftMint = new web3.PublicKey(
  "5nCoCaAc8DHtCJPjgxmH4t6ChbpyUm1Ta6euGAmEKoGu"
);

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
  const umi = createUmi(web3.clusterApiUrl("devnet")).use(mplTokenMetadata());

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

  const name = "NFT DCarbon";
  const symbol = "DC";
  const uri =
    "https://raw.githubusercontent.com/UIT19522473/asset-img-nfts/main/uri_file.json";

  //init params of updateMetadata function
  const updateMetadataParams = {
    programIndex: programIndex,
    name: name,
    symbol: symbol,
    uri: uri,
  };

  console.log("nftMint: ", nftMint);

  //get authority pda. This account is author of the nft. This account help update metadata for the nft
  const [authorityPda] = web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("authorityPda"),
      new BN(programIndex).toArrayLike(Buffer, "be", 1),
    ],
    programID
  );
  console.log("authority Pda", authorityPda);

  // get metadataAccount for the nft
  let metadataAccount = findMetadataPda(umi, {
    mint: publicKey(nftMint),
  })[0];
  console.log("metadataAccount: ", metadataAccount);

  // get masterEditionAccount for the nft
  let masterEditionAccount = findMasterEditionPda(umi, {
    mint: publicKey(nftMint),
  })[0];
  console.log("masterEditionAccount: ", masterEditionAccount);

  // update metadata including: metadataAccount and masterEditionAccount for the NFT created in the mint nft step
  const tx = await program.methods
    .updateMetadata(updateMetadataParams)
    .accounts({
      mintNft: nftMint,
      authorityPda: authorityPda,
      payer: owner.publicKey,

      metadataAccount: metadataAccount,
      masterEditionAccount: masterEditionAccount,

      rent: web3.SYSVAR_RENT_PUBKEY,
      tokenMetadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SYSTEM_PROGRAM_ID,
    })
    .signers([owner.payer])
    .rpc();
  console.log(`https://explorer.solana.com/tx/${tx}?cluster=devnet`);
};

main();
