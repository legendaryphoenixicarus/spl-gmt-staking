import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SplGmtStaking } from "../target/types/spl_gmt_staking";

describe("spl-gmt-staking", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SplGmtStaking as Program<SplGmtStaking>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
