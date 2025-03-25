# 🚀 My Solana Project  

This is a simple Solana client project that connects to the Solana blockchain, retrieves wallet balances, and interacts with on-chain programs.  

## 📌 Features  
- Retrieve and display Solana wallet balance  
- Connect to the Solana blockchain  
- Create and manage storage accounts (WIP)  

## 🛠 Installation  

### **1. Clone the repository**  
```bash
git clone https://github.com/daffhaidar/my-solana-project.git
cd my-solana-project

#### **2. Install dependencies**
```bash
For Node.js environment:
yarn install
or
npm install

🚀 Usage
1. Run with SolPG
solpg run client.ts

2. Run with Node.js
node client/client.ts

3. Run with Solana CLI
If you prefer using Solana CLI manually:
solana balance
solana confirm -v <TRANSACTION_SIGNATURE>
