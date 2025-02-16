import React, { useState } from 'react';
import { useConnection, useWallet } from '@solana/wallet-adapter-react';
import * as anchor from '@project-serum/anchor';

interface TokenParams {
  name: string;
  symbol: string;
  decimals: number;
  totalSupply: number;
  taxPercentage: number;
}

const TokenLaunchForm: React.FC = () => {
  const [tokenParams, setTokenParams] = useState<TokenParams>({
    name: '',
    symbol: '',
    decimals: 9,
    totalSupply: 1_000_000,
    taxPercentage: 5
  });

  const { connection } = useConnection();
  const wallet = useWallet();

  const handleLaunchToken = async () => {
    if (!wallet.publicKey) {
      alert('Please connect wallet first');
      return;
    }

    try {
      // Initialize Anchor program
      const provider = new anchor.AnchorProvider(connection, wallet as any, {});
      const program = new anchor.Program(
        // Load your IDL here
        launchpadIdl,
        new anchor.web3.PublicKey('HWEHzrf1uts7Noq3Qz8qRxMsm9QxTmAUMdKkwQUMzQhh'),
        provider
      );

      // Generate keypairs for new accounts
      const mint = anchor.web3.Keypair.generate();
      const tokenAccount = anchor.web3.Keypair.generate();
      const launchDetails = anchor.web3.Keypair.generate();

      // Call create_token instruction
      await program.methods.createToken(
        tokenParams.name,
        tokenParams.symbol,
        tokenParams.decimals,
        new anchor.BN(tokenParams.totalSupply),
        tokenParams.taxPercentage
      ).accounts({
        creator: wallet.publicKey,
        mint: mint.publicKey,
        tokenAccount: tokenAccount.publicKey,
        launchDetails: launchDetails.publicKey,
      }).signers([mint, tokenAccount, launchDetails])
      .rpc();

      alert('Token launched successfully!');
    } catch (error) {
      console.error('Token launch failed', error);
      alert('Token launch failed');
    }
  };

  return (
    <div className="token-launch-form">
      <input 
        type="text" 
        placeholder="Token Name" 
        value={tokenParams.name}
        onChange={(e) => setTokenParams({...tokenParams, name: e.target.value})}
      />
      <input 
        type="text" 
        placeholder="Token Symbol" 
        value={tokenParams.symbol}
        onChange={(e) => setTokenParams({...tokenParams, symbol: e.target.value})}
      />
      <input 
        type="number" 
        placeholder="Total Supply" 
        value={tokenParams.totalSupply}
        onChange={(e) => setTokenParams({...tokenParams, totalSupply: Number(e.target.value)})}
      />
      <input 
        type="number" 
        placeholder="Tax Percentage" 
        value={tokenParams.taxPercentage}
        onChange={(e) => setTokenParams({...tokenParams, taxPercentage: Number(e.target.value)})}
      />
      <button onClick={handleLaunchToken}>Launch Token</button>
    </div>
  );
};

export default TokenLaunchForm;
