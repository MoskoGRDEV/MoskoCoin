const { Connection, Keypair, PublicKey } = require('@solana/web3.js');
const { Metaplex } = require('@metaplex-foundation/js');
const fs = require('fs');

async function addMetadata() {
  // Σύνδεση στο Solana mainnet
  const connection = new Connection('https://api.mainnet-beta.solana.com', 'confirmed');
  
  // Φόρτωση του wallet σου (keypair από το solana-keygen ή JSON file)
  const wallet = Keypair.fromSecretKey(
    Uint8Array.from(JSON.parse(fs.readFileSync('path/to/your/wallet.json')))
  );

  const metaplex = new Metaplex(connection);
  metaplex.use(wallet);

  // Το Mint Address του MoskoCoin σου
  const mintAddress = new PublicKey('YOUR_MINT_ADDRESS_HERE');

  // Upload του λογοτύπου σου (π.χ. PNG) σε IPFS ή Arweave
  const { uri } = await metaplex.nfts().uploadMetadata({
    name: 'MoskoCoin',
    symbol: 'MSK',
    description: 'The official MoskoCoin token',
    image: 'https://arweave.net/YOUR_UPLOADED_IMAGE_HASH', // Upload εικόνας πρώτα
  }).run();

  // Δημιουργία metadata
  await metaplex.nfts().create({
    uri: uri,
    name: 'MoskoCoin',
    symbol: 'MSK',
    sellerFeeBasisPoints: 500, // 5% royalties (προαιρετικό)
    mintAddress: mintAddress,
    updateAuthority: wallet,
  }).run();

  console.log('Metadata added successfully!');
}

addMetadata().catch(console.error);