use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use borsh::BorshDeserialize;
use clap::{Parser, Subcommand};
use mpl_token_metadata::state::Data;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    account::ReadableAccount, pubkey::Pubkey, signature::read_keypair_file, signer::Signer,
};
use url::Url;
#[derive(Parser)]
struct Cli {
    #[arg(short, long, default_value = "https://api.mainnet-beta.solana.com")]
    rpc: String,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    TransferUpdateAuthority {
        #[arg(short, long)]
        signer: String,
        #[arg(short, long)]
        mints: String,
        #[arg(short, long)]
        new_update_authority: String,
    },
    UseShadowDrive {
        #[arg(short, long)]
        signer: String,
        #[arg(short, long)]
        mints: String,
    },
}

fn main() {
    let cli = Cli::parse();
    let rpc = &RpcClient::new(cli.rpc);

    match cli.command {
        Commands::TransferUpdateAuthority {
            signer,
            mints,
            new_update_authority,
        } => transfer_update_authority(rpc, signer, mints, new_update_authority),
        Commands::UseShadowDrive { signer, mints } => use_shadow_drive(rpc, signer, mints),
    }
}

fn transfer_update_authority(
    rpc: &RpcClient,
    signer: String,
    mints: String,
    new_update_authority: String,
) {
    let signer = read_keypair_file(signer).expect("uanble to read signer keypair");

    let new_update_authority = new_update_authority
        .parse()
        .expect("unable to parse new update authority");

    let mints = File::open(mints).expect("unable to read mint data file");
    let reader = BufReader::new(mints);

    for value in reader.lines() {
        let address = value.expect("unable to read mint address");

        let mint = address.parse().expect("unable to parse mint address");
        let metadata = find_metadata_address(mint);

        match rpc.get_account(&metadata) {
            Ok(metadata) => {
                let metadata =
                    mpl_token_metadata::state::Metadata::deserialize(&mut metadata.data())
                        .expect("unable to deserialize metadata");

                if metadata.update_authority == new_update_authority {
                    println!("{} -", mint.to_string());
                    continue;
                }

                let tx = solana_sdk::transaction::Transaction::new_signed_with_payer(
                    &vec![mpl_token_metadata::instruction::update_metadata_accounts(
                        mpl_token_metadata::id(),
                        find_metadata_address(mint),
                        signer.pubkey(),
                        Some(new_update_authority),
                        None,
                        None,
                    )],
                    Some(&signer.pubkey()),
                    &[&signer],
                    rpc.get_latest_blockhash()
                        .expect("unable to get latest blockhash"),
                );

                let res = rpc.send_transaction(&tx);
                match res {
                    Ok(sig) => println!("{} {}", mint.to_string(), sig),
                    Err(err) => println!("{} {}", mint.to_string(), err),
                }
            }
            Err(err) => println!("{} {}", mint.to_string(), err),
        }
    }
}

fn use_shadow_drive(rpc: &RpcClient, signer: String, mints: String) {
    const SHDW_ACCOUNT: &str =
        "https://shdw-drive.genesysgo.net/B9EYPPmmMEu4nEeKyyiPur7mTAWFRJxFRTpcsv33pUo5/";

    let signer = read_keypair_file(signer).expect("uanble to read signer keypair");

    let mints = File::open(mints).expect("unable to read mint data file");
    let reader = BufReader::new(mints);

    for value in reader.lines() {
        let address = value.expect("unable to read mint address");

        let mint = address.parse().expect("unable to parse mint address");
        let metadata = find_metadata_address(mint);
        let metadata = rpc
            .get_account(&metadata)
            .expect("unable to fetch metadata account");
        let metadata = mpl_token_metadata::state::Metadata::deserialize(&mut metadata.data())
            .expect("unable to deserialize metadata");

        let current_uri = metadata.data.uri.clone();
        if current_uri.starts_with(SHDW_ACCOUNT) {
            println!("{} -", mint.to_string());
            continue;
        }

        let old_uri = Url::parse(&current_uri).expect("couldn't parse url");
        let file = old_uri
            .path()
            .split("/")
            .last()
            .expect("couldn't parse file");

        let new_uri = format!("{}{}", SHDW_ACCOUNT, file);
        let new_data = Data {
            uri: new_uri.clone(),
            ..metadata.data
        };

        let tx = solana_sdk::transaction::Transaction::new_signed_with_payer(
            &vec![mpl_token_metadata::instruction::update_metadata_accounts(
                mpl_token_metadata::id(),
                find_metadata_address(mint),
                signer.pubkey(),
                None,
                Some(new_data),
                None,
            )],
            Some(&signer.pubkey()),
            &[&signer],
            rpc.get_latest_blockhash()
                .expect("unable to get latest blockhash"),
        );

        if signer.pubkey() != metadata.update_authority {
            println!(
                "{} skipped. Signer isn't update authority.\n\t got: {}\n\twant: {}",
                mint.to_string(),
                old_uri,
                new_uri,
            );
            continue;
        }

        let sig = rpc
            .send_transaction(&tx)
            .expect("unable to send transaction");
        println!("{} {}", mint.to_string(), sig);
    }
}

fn find_metadata_address(mint: Pubkey) -> Pubkey {
    let (address, _bump) = Pubkey::find_program_address(
        &[
            mpl_token_metadata::state::PREFIX.as_bytes(),
            mpl_token_metadata::id().as_ref(),
            mint.as_ref(),
        ],
        &mpl_token_metadata::id(),
    );
    address
}
