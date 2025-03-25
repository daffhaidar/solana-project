# 🚀 My Solana Project

This is a simple Solana client project that connects to the Solana blockchain, retrieves wallet balances, and interacts with on-chain programs.

## 📌 Features
- Retrieve and display Solana wallet balance
- Connect to the Solana blockchain
- Create and manage storage accounts (WIP)

## 🛠 Installation

1. **Clone the repository**  
   Clone the repository to your local machine:
   ```bash
   git clone https://github.com/daffhaidar/solana-project.git
   cd solana-project


2. **Install dependencies**
For Node.js environment, install the required dependencies:
# Using Yarn
yarn install

# Or using npm
npm install


## 🚀 Usage

1. **Run with SolPG**
   ```bash
   solpg run client.ts

 2. **Run with Node.js**
    Alternatively, you can run the project using Node.js:
    ```bash
    node client/client.ts

 3. **Run with Solana CLI**
    If you prefer using the Solana CLI manually, here are a couple of commands:
    
# Check wallet balance
solana balance

# Confirm a transaction with a specific signature
solana confirm -v <TRANSACTION_SIGNATURE>



## 🚀 Additional Notes
The project is a work in progress (WIP), so some features like managing storage accounts are still being developed.

Make sure to have the necessary Solana CLI setup for manual interactions with the blockchain.
