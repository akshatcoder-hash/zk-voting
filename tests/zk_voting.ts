import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ZkVoting } from "../target/types/zk_voting";
import { expect } from 'chai';
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { BN } from "bn.js";

describe("zk_voting", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.ZkVoting as Program<ZkVoting>;
  
  let daoState: Keypair;
  let proposal: Keypair;
  let userStatePda: PublicKey;

  before(async () => {
    daoState = Keypair.generate();
    proposal = Keypair.generate();
    [userStatePda] = await PublicKey.findProgramAddress(
      [Buffer.from("user_state"), provider.wallet.publicKey.toBuffer()],
      program.programId
    );
  });

  it("Initializes the DAO", async () => {
    try {
      const tx = await program.methods
        .initialize()
        .accounts({
          daoState: daoState.publicKey,
          authority: provider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([daoState])
        .rpc();

      console.log("Initialize transaction signature", tx);

      const daoStateAccount = await program.account.daoState.fetch(daoState.publicKey);
      expect(daoStateAccount.authority.toString()).to.equal(provider.wallet.publicKey.toString());
      expect(daoStateAccount.proposalCount.toNumber()).to.equal(0);
      
      console.log("DAO State:", daoStateAccount);
    } catch (error) {
      console.error("Error during DAO initialization:", error);
      throw error;
    }
  });

  it("Creates a proposal", async () => {
    try {
      const description = "Test Proposal";

      const tx = await program.methods
        .createProposal(description)
        .accounts({
          daoState: daoState.publicKey,
          proposal: proposal.publicKey,
          authority: provider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([proposal])
        .rpc();

      console.log("Create proposal transaction signature", tx);

      const proposalAccount = await program.account.proposal.fetch(proposal.publicKey);
      expect(proposalAccount.description).to.equal(description);
      expect(proposalAccount.isActive).to.be.true;
      expect(proposalAccount.id.toNumber()).to.equal(0);

      const updatedDaoState = await program.account.daoState.fetch(daoState.publicKey);
      expect(updatedDaoState.proposalCount.toNumber()).to.equal(1);
    } catch (error) {
      console.error("Error during proposal creation:", error);
      throw error;
    }
  });

  it("Casts a vote", async () => {
    try {
      const dummyEncryptedVote = new Array(64).fill(0);
      await program.methods
        .castVote(new BN(0), dummyEncryptedVote)
        .accounts({
          proposal: proposal.publicKey,
          userState: userStatePda,
          daoState: daoState.publicKey,
          user: provider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      const updatedProposal = await program.account.proposal.fetch(proposal.publicKey);
      expect(updatedProposal.encryptedVotes.length).to.equal(1);

      const userStateAccount = await program.account.userState.fetch(userStatePda);
      expect(userStateAccount.votedProposals.length).to.equal(1);
      expect(userStateAccount.rewardPoints.toNumber()).to.equal(1);
    } catch (error) {
      console.error("Error during vote casting:", error);
      throw error;
    }
  });

  it("Gets results", async () => {
    try {
      const tx = await program.methods
        .getResults(new BN(0))
        .accounts({
          proposal: proposal.publicKey,
          daoState: daoState.publicKey,
        })
        .rpc();

      console.log("Get results transaction signature", tx);
      // You can add assertions here if needed, based on the expected behavior of getResults
    } catch (error) {
      console.error("Error during getting results:", error);
      throw error;
    }
  });

  it("Rewards a participant", async () => {
    try {
      const tx = await program.methods
        .rewardParticipant()
        .accounts({
          userState: userStatePda,
          user: provider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      console.log("Reward participant transaction signature", tx);

      const updatedUserState = await program.account.userState.fetch(userStatePda);
      expect(updatedUserState.rewardPoints.toNumber()).to.equal(2); // 1 from voting, 1 from rewarding
    } catch (error) {
      console.error("Error during participant rewarding:", error);
      throw error;
    }
  });

  it("Fails to vote twice on the same proposal", async () => {
    const dummyEncryptedVote = new Array(64).fill(0);

    try {
      await program.methods
        .castVote(new BN(0), dummyEncryptedVote)
        .accounts({
          proposal: proposal.publicKey,
          userState: userStatePda,
          daoState: daoState.publicKey,
          user: provider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();
      
      // If we reach this point, the test has failed
      expect.fail("Should not be able to vote twice");
    } catch (error) {
      expect(error.toString()).to.include("AlreadyVoted");
    }
  });
});