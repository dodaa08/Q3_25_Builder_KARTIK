## Turbin3 Q3 Builder Program

Proof‑of‑work for the Solana Turbin3 Q3 Builder Program.

![Solana](https://img.shields.io/badge/Solana-%20Localnet-3ECF8E?logo=solana&logoColor=white)
![Anchor](https://img.shields.io/badge/Anchor-Framework-blueviolet)
![Rust](https://img.shields.io/badge/Rust-Program-orange)
![TypeScript](https://img.shields.io/badge/TypeScript-Clients-3178C6)

### Table of Contents
- [Project Structure](#project-structure)
- [Projects](#projects)
- [Tech Stack](#tech-stack)
- [Getting Started](#getting-started)

## Project Structure

```
turbine3/
├── Turbin3-prereqs/
├── escrow1/
├── nft-marketplace/
├── nft-staking/
├── RotoFi/
├── solana-starter/
├── vault/
└── README.md
```

## Projects

- **Turbin3-prereqs**: Solana basics and utilities (airdrop, transfer, keygen, enrollment) in TS and Rust.
- **escrow1**: Minimal Anchor escrow demonstrating token custody and conditional release.
- **nft-marketplace**: Simple marketplace: list, buy, and settle NFT trades.
- **nft-staking**: Stake NFTs to accrue rewards; lifecycle and account orchestration.
- **RotoFi**: Money circles on Solana — trustless rotating payouts powered by Anchor.
- **solana-starter**: Lightweight RS/TS starters for quick Solana experiments.
- **vault**: Time‑locked vault pattern for safe program‑controlled funds.

## Tech Stack

- **Rust**: On‑chain program development
- **TypeScript**: Tests and clients
- **Anchor**: Solana framework for DX and safety
- **Solana Web3.js**: Program interactions

## Getting Started

Run any project in its folder:

```bash
# Example: build & test an Anchor program
cd escrow1
npm install
anchor build
anchor test
```

```bash
# Example: run a TS-only script (from a scripts/tools project)
cd Turbin3-prereqs/airdrop
npm install
npm run start
```

Each project contains its own README or scripts to guide build, test, and deploy flows. 