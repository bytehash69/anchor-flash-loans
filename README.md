# 🏦 Anchor Flash Loans

A high-performance Solana program and TypeScript client for executing secure, atomic flash loans—built in 🦀 Rust and ⚡️ TypeScript using the [Anchor framework](https://book.anchor-lang.com/).

---

## ✨ Overview

**anchor-flash-loans** enables trustless, instant flash loan functionality on the Solana blockchain. Users can borrow tokens without collateral, provided they repay within the same transaction. This is ideal for arbitrage, liquidations, and advanced DeFi strategies—without risk to the lender.

---

## 🚀 Features

- ⚡️ Atomic, instant flash loans with zero collateral
- 🔒 Trustless and secure on-chain logic
- ⛓ Built with Anchor for reliability and extensibility
- 🟦 TypeScript client for seamless dApp integration
- 💡 Suitable for arbitrage, liquidations, and advanced DeFi use cases

---

## 🛠 Getting Started

### 📋 Prerequisites

- 🦀 [Rust](https://www.rust-lang.org/tools/install)
- ☀️ [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- ⚓️ [Anchor](https://book.anchor-lang.com/getting_started/installation.html)
- 🟩 [Node.js](https://nodejs.org/) (for TypeScript client)

### ⬇️ Installation

```bash
git clone https://github.com/bytehash69/anchor-flash-loans.git
cd anchor-flash-loans
```

### 🏗 Build the Program

```bash
anchor build
```

### 🛳 Deploy to Localnet

```bash
anchor deploy
```

---

## 📦 Usage

### 📝 Rust (On-Chain Program)

- The main program logic is in `programs/anchor-flash-loans`.
- Implements instructions for initiating, executing, and repaying flash loans.

### 🟦 TypeScript (Client SDK)

- The client (see `tests/` or `client/`) provides utilities to interact with the on-chain program and automate flash loan flows.
- Example scripts show how to request a loan, perform actions, and repay in a single transaction.

---

## 🤝 Contributing

Contributions are welcome! Please open issues or pull requests. For major changes, start a discussion first.

---

## 📄 License

Licensed under the MIT License.

---

## ✉️ Contact

Maintained by [bytehash69](https://github.com/bytehash69).

---

> ⚠️ **This project is not audited. Use at your own risk!**
