import React from 'react';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';

const WalletConnectButton: React.FC = () => {
  return (
    <div className="wallet-connect-container">
      <WalletMultiButton className="wallet-connect-button">
        Connect Wallet
      </WalletMultiButton>
    </div>
  );
};

export default WalletConnectButton;
