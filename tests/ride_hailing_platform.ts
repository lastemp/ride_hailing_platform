import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RideHailingPlatform } from "../target/types/ride_hailing_platform";

describe("ride_hailing_platform", () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.local("http://127.0.0.1:8899");

  const program = anchor.workspace
    .RideHailingPlatform as Program<RideHailingPlatform>;
  const adminOwner = anchor.web3.Keypair.generate();
  const adminDepositAccount = anchor.web3.Keypair.generate();
  const riderOwner = anchor.web3.Keypair.generate();
  const driverOwner = anchor.web3.Keypair.generate();

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

  let [configs] = anchor.web3.PublicKey.findProgramAddressSync(
    [anchor.utils.bytes.utf8.encode("configs")],
    program.programId
  );

  let [rider] = anchor.web3.PublicKey.findProgramAddressSync(
    [anchor.utils.bytes.utf8.encode("rider"), riderOwner.publicKey.toBuffer()],
    program.programId
  );

  let [trip] = anchor.web3.PublicKey.findProgramAddressSync(
    [anchor.utils.bytes.utf8.encode("trip"), riderOwner.publicKey.toBuffer()],
    program.programId
  );

  let [driver] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("driver"),
      driverOwner.publicKey.toBuffer(),
    ],
    program.programId
  );

  let [vehicle] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("vehicle"),
      driverOwner.publicKey.toBuffer(),
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

  // rider Owner
  before(async () => {
    let res = await provider.connection.requestAirdrop(
      riderOwner.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );

    let latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });
  });

  // driver Owner
  before(async () => {
    let res = await provider.connection.requestAirdrop(
      driverOwner.publicKey,
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
      driverShare: 65, // driver share split on fees i.e 65%
      singleTripToLoyaltyPointsMapping: 5, // used to compute loyalty points from a single trip i.e 5 points
    };

    const tx = await program.methods
      .init(initParams)
      .accounts({
        owner: adminOwner.publicKey,
        configs: configs,
        adminDepositAccount: adminDepositAccount.publicKey,
        adminPdaAuth: adminPdaAuth,
        adminSolVault: adminSolVault,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([adminOwner, adminDepositAccount])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.configs.fetch(configs);
    console.log("configs: ", result);
  });

  it("Is register rider!", async () => {
    let initParams = {
      fullNames: "paul john",
      country: "KE",
    };

    const tx = await program.methods
      .registerRider(initParams)
      .accounts({
        owner: riderOwner.publicKey,
        rider: rider,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([riderOwner])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.rider.fetch(rider);
    console.log("rider: ", result);
  });

  it("Is register driver!", async () => {
    let vehicleDetails = {
      make: "Toyota", // make of the vehicle
      model: "RAV4", // model of the vehicle
      manufactureDate: 1, // vehicle manufacture date in years i.e 2020
    };

    let initParams = {
      fullNames: "moses blessing",
      country: "KE",
      vehicle: vehicleDetails,
    };

    const tx = await program.methods
      .registerDriver(initParams)
      .accounts({
        owner: driverOwner.publicKey,
        driver: driver,
        vehicle: vehicle,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([driverOwner])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.driver.fetch(driver);
    console.log("driver: ", result);

    let result1 = await program.account.vehicle.fetch(vehicle);
    console.log("vehicle: ", result1);
  });

  it("Is request trip!", async () => {
    let origin = {
      latitude: "-1.288811",
      longitude: "36.823219",
    };
    let destination = {
      latitude: "-1.2359",
      longitude: "36.9352",
    };
    let initParams = {
      origin: origin, // origin of trip
      destination: destination, // destination of trip
      amount: 2, // trip amount i.e 2 Sol
    };

    const tx = await program.methods
      .requestTrip(initParams)
      .accounts({
        owner: riderOwner.publicKey,
        trip: trip,
        rider: rider,
        driver: driver,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([riderOwner])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.rider.fetch(rider);
    console.log("rider: ", result);
  });

  it("Is pay trip!", async () => {
    let initParams = {
      amountPaid: 2, // trip amount paid i.e 2 Sol
    };

    const tx = await program.methods
      .payTrip(initParams)
      .accounts({
        owner: riderOwner.publicKey,
        rider: rider,
        trip: trip,
        configs: configs,
        driver: driver,
        adminDepositAccount: adminDepositAccount.publicKey,
        adminPdaAuth: adminPdaAuth,
        adminSolVault: adminSolVault,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([riderOwner])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.rider.fetch(rider);
    console.log("rider: ", result);

    let result1 = await program.account.trip.fetch(trip);
    console.log("trip: ", result1);

    let result2 = await program.account.configs.fetch(configs);
    console.log("configs: ", result2);

    let result3 = await program.account.driver.fetch(driver);
    console.log("driver: ", result3);
  });

  it("Is withdraw driver's funds", async () => {
    let amount = new anchor.BN(1 * anchor.web3.LAMPORTS_PER_SOL);

    let initParams = {
      withdrawalAmount: amount, // This equates to 3 Sol
    };

    const tx = await program.methods
      .withdrawDriverFunds(initParams)
      .accounts({
        owner: driverOwner.publicKey,
        driver: driver,
        adminDepositAccount: adminDepositAccount.publicKey,
        adminPdaAuth: adminPdaAuth,
        adminSolVault: adminSolVault,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([driverOwner])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.driver.fetch(driver);
    console.log("driver: ", result);

    console.log("admin sol vault: ", adminSolVault.toBase58());
  });
});
