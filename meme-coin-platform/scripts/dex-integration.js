const { Connection, PublicKey } = require('@solana/web3.js');
const { Jupiter } = require('@jup-ag/core');

async function swapTokensViaJupiter(
  connection, 
  wallet, 
  inputMint, 
  outputMint, 
  amount
) {
  const jupiter = await Jupiter.load({
    connection,
    cluster: 'devnet',
    user: wallet.publicKey
  });

  const routes = await jupiter.computeRoutes({
    inputMint: new PublicKey(inputMint),
    outputMint: new PublicKey(outputMint),
    amount,
    slippage: 1 // 1% slippage tolerance
  });

  const bestRoute = routes.routesData[0];
  
  const swapTransaction = await jupiter.exchange({
    route: bestRoute
  });

  return swapTransaction;
}

module.exports = {
  swapTokensViaJupiter
};
