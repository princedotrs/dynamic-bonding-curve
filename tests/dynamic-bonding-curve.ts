import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DynamicBondingCurve } from "../target/types/dynamic_bonding_curve";
import { getOrCreateAssociatedTokenAccount } from "@solana/spl-token";

describe("dynamic-bonding-curve", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();

  const program = anchor.workspace
    .dynamicBondingCurve as Program<DynamicBondingCurve>;

  const mint = anchor.web3.Keypair.generate();


  it("Admin: initialize!", async () => {
    const tx = await program.methods
      .initialize()
      .signers([provider.wallet.payer])
      .rpc();
    console.log(
      `Transfer tx: https://explorer.solana.com/tx/${tx}?cluster=custom`
    );
  });

  // it.skip("Admin: set_patams!", async () => {
  //   const admin = anchor.web3.Keypair.generate();
  //   const feeRecipient = anchor.web3.Keypair.generate();
  //   const feeBasisPoint = new anchor.BN(100);
  //   const newSupply = new anchor.BN(1_000_000_000_000);

  //   const tx = await program.methods
  //     .setParams(
  //       admin.publicKey,
  //       feeRecipient.publicKey,
  //       feeBasisPoint,
  //       newSupply
  //     )
  //     .signers([provider.wallet.payer])
  //     .rpc();
  //   console.log(
  //     `Set params tx: https://explorer.solana.com/tx/${tx}?cluster=custom`
  //   );
  // });

  it("User: create!", async()=>{
    const name = "DynamicBondingCurve";
    const symbol = "DBC";
    const uri = "https://gateway.pinata.cloud/ipfs/dbc";
    const totalFundRaising = new anchor.BN(85_000_000_000);
    const totalSell = new anchor.BN(800_000_000_000_000);
    const totalLockedAmount = new anchor.BN(0);
    const migrateFee = new anchor.BN(0);

    const tokenArgs = {
      name,
      symbol,
      uri,
    };

    const dbcCurveArgs = {
      totalFundRaising,
      totalSell,
      totalLockedAmount,
      migrateFee,
    };

    const metadata = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        new anchor.web3.PublicKey(
          "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
        ).toBuffer(),
        mint.publicKey.toBuffer(),
      ],
      new anchor.web3.PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")
    )[0];

    const tx = await program.methods.create(
      tokenArgs,
      dbcCurveArgs
    ).accounts({
      payer: provider.wallet.publicKey,
      mint: mint.publicKey,
      metadata,
    }).signers([provider.wallet.payer, mint]).rpc();
    console.log(
      `Create tx: https://explorer.solana.com/tx/${tx}?cluster=custom`
    );
  });

  it("User: buy!", async()=>{
    const amount = new anchor.BN(1_000_000_000); // 1 M token as decimal = 6
    const maxsolcost = new anchor.BN(2_500_000_000);

    const vault_pda = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("curve_vault"), mint.publicKey.toBuffer()],
      program.programId
    )[0];

    const bondingCurve_pda = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("bonding_curve"), mint.publicKey.toBuffer()],
      program.programId
    )[0];

    const userATA = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      provider.wallet.payer,
      mint.publicKey,
      provider.wallet.publicKey,
    )

    const tx = await program.methods.buy(
      amount,
      maxsolcost,
    ).accounts({
      mint: mint.publicKey,
      vault: vault_pda,
      bondingCurve: bondingCurve_pda,
      feeRecipient: provider.wallet.publicKey,
      userAta: userATA.address,
    } as any).signers([provider.wallet.payer]).rpc();
    console.log(
      `Buy tx: https://explorer.solana.com/tx/${tx}?cluster=custom`
    );

    const bondingCurve = await program.account.bondingCurve.fetch(bondingCurve_pda);
    console.log("Bonding curve account: ", bondingCurve.realSolReserve.toNumber());
  
  });

  it("User: sell!", async()=>{
    const amount = new anchor.BN(1_000_000); // 1 M token as decimal = 6
    const minsolout = new anchor.BN(1);
    
    const vault_pda = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("curve_vault"), mint.publicKey.toBuffer()],
      program.programId
    )[0];

    const bondingCurve_pda = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("bonding_curve"), mint.publicKey.toBuffer()],
      program.programId
    )[0];

    const userATA = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      provider.wallet.payer,
      mint.publicKey,
      provider.wallet.publicKey,
    )

    const tx = await program.methods.sell(
      amount,
      minsolout,
    ).accounts({
      payer: provider.wallet.publicKey,
      mint: mint.publicKey,
      vault: vault_pda,
      bondingCurve: bondingCurve_pda,
      feeRecipient: provider.wallet.publicKey,
      userAta: userATA.address,
    } as any).signers([provider.wallet.payer]).rpc();
    console.log(
      `Sell tx: https://explorer.solana.com/tx/${tx}?cluster=custom`
    );
  });


});
