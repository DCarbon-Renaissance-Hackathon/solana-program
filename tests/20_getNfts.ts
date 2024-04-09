import { web3, BN, Wallet, AnchorProvider, Program } from "@coral-xyz/anchor";
import { DcarbonProgram } from "../target/types/dcarbon_program";
import idl from "../target/idl/dcarbon_program.json";
import bs58 from "bs58";
import { Metaplex, Nft } from "@metaplex-foundation/js";

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

  const metaplex = new Metaplex(connection); // Replace 'connection' with your Solana connection
  // select local wallet
  const buyer = new Wallet(
    web3.Keypair.fromSecretKey(bs58.decode(process.env.PRIVATE_KEY_SOLANA_1))
  );
  console.log("buyer: ", buyer.publicKey);

  const nfts = await metaplex.nfts().findAllByOwner({ owner: buyer.publicKey });

  // init authority pda to authorize for mint nfts
  const [authorityPda] = web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("authorityPda"),
      new BN(programIndex).toArrayLike(Buffer, "be", 1),
    ],
    programID
  );
  console.log("authority Pda", authorityPda);

  //  get Nfts
  // if nfts has updateAuthorityAddress equals authority pda address -> log info nfts
  // let i = 0;
  // nfts.map((item) => {
  //   if (item.updateAuthorityAddress.equals(authorityPda)) {
  //     console.log("nfts of project Carbon", item);
  //     i++;
  //   }
  // });
  //   log count of list nfts
  // console.log("count NFTs", i);
  const rs = nfts.filter((item) =>
    item.updateAuthorityAddress.equals(authorityPda)
  );
  console.log("list Nfts", rs);
  console.log("count Nfts", rs.length);
};

main();
