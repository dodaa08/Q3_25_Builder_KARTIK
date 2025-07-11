# Client Requirements for Vault Contract Interaction

## 1. Essential Dependencies

### Core Solana/Anchor Dependencies
```bash
npm install @coral-xyz/anchor @solana/web3.js @solana/wallet-adapter-base
```

### For React Applications
```bash
npm install @solana/wallet-adapter-react @solana/wallet-adapter-react-ui @solana/wallet-adapter-wallets
```

### Additional Utilities
```bash
npm install bn.js  # For handling large numbers
```

## 2. Required Files & Setup

### A. Generated IDL File
- **File**: `target/idl/vault.json`
- **Generated by**: `anchor build`
- **Contains**: Contract interface, instruction formats, account structures

### B. TypeScript Types
- **File**: `target/types/vault.ts`
- **Generated by**: `anchor build`
- **Contains**: TypeScript definitions for type safety

### C. Program ID
- **From**: Contract deployment
- **Example**: `"Av69vJsFigQuUJ9T7UxkJ7o7mRGAawHXK9fJ7W81EELF"`

## 3. Key Components for Client Interaction

### A. Connection Setup
```typescript
import { Connection, clusterApiUrl } from "@solana/web3.js";

const connection = new Connection(
  "http://127.0.0.1:8899", // localnet
  // clusterApiUrl("devnet"),  // devnet
  // clusterApiUrl("mainnet-beta"),  // mainnet
  "processed"
);
```

### B. Wallet Integration
```typescript
import { useWallet } from "@solana/wallet-adapter-react";

// In your component
const { publicKey, signTransaction, signAllTransactions } = useWallet();
```

### C. Program Instance
```typescript
import { Program, AnchorProvider } from "@coral-xyz/anchor";
import { Vault } from "./target/types/vault";
import idl from "./target/idl/vault.json";

const programId = new PublicKey("YOUR_PROGRAM_ID");
const provider = new AnchorProvider(connection, wallet, {});
const program = new Program<Vault>(idl as Vault, programId, provider);
```

## 4. Required Functions for Client

### A. PDA Derivation
```typescript
// Get vault state PDA
const [vaultStatePda] = PublicKey.findProgramAddressSync(
  [Buffer.from("state"), userPublicKey.toBuffer()],
  programId
);

// Get vault PDA
const [vaultPda] = PublicKey.findProgramAddressSync(
  [Buffer.from("vault"), vaultStatePda.toBuffer()],
  programId
);
```

### B. Contract Interaction Methods
1. **Initialize Vault**: Create new vault for user
2. **Deposit**: Send SOL to vault
3. **Withdraw**: Retrieve SOL from vault
4. **Close**: Close vault and return all funds
5. **Get Balance**: Check vault balance
6. **Check Existence**: Verify if vault exists

## 5. Error Handling

### Contract-Specific Errors
- `InvalidAmount`: Amount must be > 0
- `InsufficientFunds`: Not enough SOL in vault

### Common Solana Errors
- Transaction failures
- Network issues
- Insufficient user balance for fees

## 6. Environment Variables

```env
# .env file
REACT_APP_PROGRAM_ID=Av69vJsFigQuUJ9T7UxkJ7o7mRGAawHXK9fJ7W81EELF
REACT_APP_RPC_ENDPOINT=http://127.0.0.1:8899
```

## 7. Deployment Steps

### Before Client Development
1. **Deploy Contract**: `anchor deploy`
2. **Note Program ID**: Copy from deployment output
3. **Generate IDL**: Should be created automatically
4. **Update Client**: Use the deployed program ID

### Testing Sequence
1. Connect wallet
2. Initialize vault (if needed)
3. Deposit test amount
4. Check balance
5. Withdraw partial amount
6. Close vault (optional)

## 8. Frontend Integration Example

### React Hook Structure
```typescript
export function useVault() {
  const { publicKey } = useWallet();
  const [balance, setBalance] = useState(0);
  const [loading, setLoading] = useState(false);

  const initializeVault = async () => { /* ... */ };
  const deposit = async (amount: number) => { /* ... */ };
  const withdraw = async (amount: number) => { /* ... */ };
  const getBalance = async () => { /* ... */ };
  const closeVault = async () => { /* ... */ };

  return {
    balance,
    loading,
    initializeVault,
    deposit,
    withdraw,
    getBalance,
    closeVault
  };
}
```

## 9. Production Considerations

### Security
- Validate all inputs
- Check transaction signatures
- Handle wallet disconnection
- Implement proper error boundaries

### UX/UI
- Loading states for all operations
- Clear error messages
- Transaction confirmation feedback
- Balance updates after operations

### Performance
- Cache PDA calculations
- Batch multiple operations when possible
- Use connection pooling for high traffic

## 10. Alternative Testing Methods

If build issues persist:

### A. Solana Playground
1. Copy contract code to [beta.solpg.io](https://beta.solpg.io)
2. Build and deploy with one click
3. Test directly in playground
4. Export working client code

### B. Manual IDL Creation
1. Create IDL file manually from contract structure
2. Generate types using `anchor-client-gen`
3. Test with minimal client setup

### C. CLI Testing
```bash
# Using Solana CLI directly
solana program deploy target/deploy/vault.so
anchor test --skip-build --skip-deploy
``` 