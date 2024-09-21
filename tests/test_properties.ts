import * as anchor from '@coral-xyz/anchor';
import { Keypair } from '@solana/web3.js';
import type { TestTask } from '../target/types/test_task';

describe('Account Data:', () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.TestTask as anchor.Program<TestTask>;

  const userInfoAccount = new Keypair();

  it('Create the address info account', async () => {
    console.log(`Address      : ${payer.publicKey}`);
    console.log(`Info Account Address  : ${userInfoAccount.publicKey}`);

    // Instruction Ix data
    const userInfo = {
      first_name: 'Yaroslav',
      last_name: 'Buterin'
    };

    await program.methods
      .createUserInfo(userInfo.first_name, userInfo.last_name)
      .accounts({
        userInfo: userInfoAccount.publicKey,
        payer: payer.publicKey,
      })
      .signers([userInfoAccount])
      .rpc();
  });

  it("New account's data:", async () => {
    const userInfo = await program.account.userInfo.fetch(userInfoAccount.publicKey);
    console.log(`First name: ${userInfo.firstName}`);
    console.log(`Last name: ${userInfo.lastName}`);
  });
});