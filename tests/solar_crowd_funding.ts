import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolarCrowdFunding } from "../target/types/solar_crowd_funding";

describe("solar_crowd_funding", () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.local("http://127.0.0.1:8899");

  const program = anchor.workspace
    .SolarCrowdFunding as Program<SolarCrowdFunding>;
  const adminOwner = anchor.web3.Keypair.generate();
  const adminDepositAccount = anchor.web3.Keypair.generate();
  const applicantInvestor1 = anchor.web3.Keypair.generate();
  const applicantInvestor2 = anchor.web3.Keypair.generate();
  const institutionOwner = anchor.web3.Keypair.generate();

  // admin
  let [adminPdaAuth, adminPdaBump] =
    anchor.web3.PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode("admin-auth"),
        adminDepositAccount.publicKey.toBuffer(),
      ],
      program.programId
    );
  let [adminSolVault, adminSolBump] =
    anchor.web3.PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode("admin-sol-vault"),
        adminPdaAuth.toBuffer(),
      ],
      program.programId
    );

  let [project] = anchor.web3.PublicKey.findProgramAddressSync(
    [anchor.utils.bytes.utf8.encode("project")],
    program.programId
  );

  let [investor1] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("investor"),
      applicantInvestor1.publicKey.toBuffer(),
    ],
    program.programId
  );

  let [investor2] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("investor"),
      applicantInvestor2.publicKey.toBuffer(),
    ],
    program.programId
  );

  let [institution] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("institution"),
      institutionOwner.publicKey.toBuffer(),
    ],
    program.programId
  );

  // adminOwner
  before(async () => {
    let res = await provider.connection.requestAirdrop(
      adminOwner.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );

    let latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });
  });

  // applicant Investor 1
  before(async () => {
    let res = await provider.connection.requestAirdrop(
      applicantInvestor1.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );

    let latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });
  });

  // applicant Investor 2
  before(async () => {
    let res = await provider.connection.requestAirdrop(
      applicantInvestor2.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );

    let latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });
  });

  // institutionOwner
  before(async () => {
    let res = await provider.connection.requestAirdrop(
      institutionOwner.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );

    let latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });
  });

  it("Is initialized!", async () => {
    let initParams = {
      projectReference: "ke202404120446la", // unique reference of the project
      projectName: "loresho gardens solar project", // project name
      country: "KE", // home country where project is implemented
      projectFunds: 10, // funds needed for completion of project i.e 10 Sol
      billAmount: 2, // this is the amount meant to be paid(monthly basis) by institution that acquired solar project
    };

    const tx = await program.methods
      .init(initParams)
      .accounts({
        owner: adminOwner.publicKey,
        project: project,
        adminDepositAccount: adminDepositAccount.publicKey,
        adminPdaAuth: adminPdaAuth,
        adminSolVault: adminSolVault,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([adminOwner, adminDepositAccount])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.project.fetch(project);
    console.log("project: ", result);
  });

  it("Is register first investor!", async () => {
    let initParams = {
      fullNames: "paul john",
      country: "KE",
    };

    const tx = await program.methods
      .registerInvestor(initParams)
      .accounts({
        owner: applicantInvestor1.publicKey,
        investor: investor1,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([applicantInvestor1])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.investor.fetch(investor1);
    console.log("investor: ", result);
  });

  it("Is register second investor!", async () => {
    let initParams = {
      fullNames: "moses blessing",
      country: "UG",
    };

    const tx = await program.methods
      .registerInvestor(initParams)
      .accounts({
        owner: applicantInvestor2.publicKey,
        investor: investor2,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([applicantInvestor2])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.investor.fetch(investor2);
    console.log("investor: ", result);
  });

  it("Is register institution", async () => {
    let initParams = {
      institutionName: "Prix Manufacturing Ltd",
      country: "KE",
    };

    const tx = await program.methods
      .registerInstitution(initParams)
      .accounts({
        owner: institutionOwner.publicKey,
        institution: institution,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([institutionOwner])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.institution.fetch(institution);
    console.log("institution: ", result);
  });

  it("Is invest in project - first investor!", async () => {
    let initParams = {
      amountInvested: 5, // Value is represented as Sol
    };

    const tx = await program.methods
      .investInProject(initParams)
      .accounts({
        owner: applicantInvestor1.publicKey,
        investor: investor1,
        project: project,
        adminDepositAccount: adminDepositAccount.publicKey,
        adminPdaAuth: adminPdaAuth,
        adminSolVault: adminSolVault,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([applicantInvestor1])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.project.fetch(project);
    console.log("project: ", result);
  });

  it("Is invest in project - second investor!", async () => {
    let initParams = {
      amountInvested: 5, // Value is represented as Sol
    };

    const tx = await program.methods
      .investInProject(initParams)
      .accounts({
        owner: applicantInvestor2.publicKey,
        investor: investor2,
        project: project,
        adminDepositAccount: adminDepositAccount.publicKey,
        adminPdaAuth: adminPdaAuth,
        adminSolVault: adminSolVault,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([applicantInvestor2])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.project.fetch(project);
    console.log("project: ", result);
  });

  it("Is pay solar bill!", async () => {
    let initParams = {
      amountPaid: 2, // Value is represented as Sol
    };

    const tx = await program.methods
      .paySolarBill(initParams)
      .accounts({
        owner: institutionOwner.publicKey,
        institution: institution,
        project: project,
        adminDepositAccount: adminDepositAccount.publicKey,
        adminPdaAuth: adminPdaAuth,
        adminSolVault: adminSolVault,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([institutionOwner])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.project.fetch(project);
    console.log("project: ", result);

    console.log("adminSolVault: ", adminSolVault.toBase58());
  });
});
