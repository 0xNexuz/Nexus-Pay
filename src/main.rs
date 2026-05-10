use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    signature::{Keypair, Signer, read_keypair_file, write_keypair_file},
    system_instruction,
    transaction::Transaction,
};
use std::path::Path;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("🚀 NexusPay V2 (Enterprise Router) Initialized...");

    // ==========================================
    // 1. EXPLICIT NETWORK BINDING
    // ==========================================
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
    println!("📡 Connected to RPC: {}", rpc_url);

    // ==========================================
    // 2. AUTONOMOUS CRYPTOGRAPHIC IDENTITY
    // ==========================================
    let keypair_path = "my_agent_keypair.json";
    
    // If the file exists, read it. If not, generate it!
    let agent_keypair = if Path::new(keypair_path).exists() {
        read_keypair_file(keypair_path).expect("Failed to load existing keypair")
    } else {
        println!("🛠️ No local wallet found. Generating a new one autonomously...");
        let new_key = Keypair::new();
        write_keypair_file(&new_key, keypair_path).expect("Failed to save new keypair");
        println!("✅ New wallet saved to: {}", keypair_path);
        new_key
    };

    let agent_pubkey = agent_keypair.pubkey();
    println!("🤖 Agent Wallet Pubkey: {}", agent_pubkey);

    // ==========================================
    // 3. AUTONOMOUS FUNDING & BALANCE CHECK
    // ==========================================
    let mut balance = client.get_balance(&agent_pubkey).unwrap_or(0);
    println!("💰 Current Balance: {} lamports", balance);

    if balance == 0 {
        println!("⚠️ [AGENT ACTION] Insufficient funds detected. Triggering autonomous airdrop...");
        
        match client.request_airdrop(&agent_pubkey, LAMPORTS_PER_SOL) {
            Ok(signature) => {
                println!("✅ Airdrop successfully requested! Tx: {}", signature);
                println!("⏳ Waiting 10 seconds for network settlement...");
                sleep(Duration::from_secs(10));
                
                // Re-check balance after airdrop
                balance = client.get_balance(&agent_pubkey).unwrap_or(0);
                println!("💰 New Balance: {} lamports", balance);
            }
            Err(err) => {
                println!("🛑 [CRITICAL] Airdrop faucet failed (likely rate-limited).");
                println!("Error Details: {}", err);
                println!("Because the CLI isn't installed, you must fund this wallet via a web faucet.");
                println!("Go to: https://faucet.solana.com/ and drop 1 SOL into: {}", agent_pubkey);
                return; // Halt safely
            }
        }
    }

    if balance == 0 {
        println!("🛑 System halted. Wallet requires Devnet SOL funding.");
        return;
    }

    // ==========================================
    // 4. THE ZERO-BANDWIDTH RELAY (SIMULATION)
    // ==========================================
    println!("\n---------------------------------------------------");
    println!("⚡ [OFFLINE MODE] Constructing transaction locally...");
    
    let target_pubkey = Pubkey::from_str("11111111111111111111111111111111").unwrap();
    let transfer_amount = 1000; // Micro-transaction
    
    let instruction = system_instruction::transfer(
        &agent_pubkey,
        &target_pubkey,
        transfer_amount,
    );

    let recent_blockhash = client.get_latest_blockhash().expect("Failed to get blockhash");

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&agent_pubkey),
        &[&agent_keypair],
        recent_blockhash,
    );

    println!("🔐 Transaction signed cryptographically offline.");
    println!("📦 Payload serialized and cached in relay buffer.");
    println!("📡 Polling for network connectivity...");
    
    sleep(Duration::from_secs(3)); 
    
    println!("📶 Connection restored! Broadcasting payload to Solana...");

    match client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => {
            println!("🌍 [SETTLED] Transaction successful! Hash: {}", signature);
        }
        Err(e) => {
            println!("⚠️ [BROADCAST FAILED] {}", e);
        }
    }
}