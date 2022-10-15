use borsh::BorshDeserialize;
use clap::{Parser, Subcommand};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    account::ReadableAccount, pubkey::Pubkey, signature::read_keypair_file, signer::Signer,
};
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
        #[arg(
            short,
            long,
            default_value = "data/2PtfazxS7QfFoKsDryFVPeqo8zvGzY3BwiVsbssaBaU3.json"
        )]
        mint_data: String,
        #[arg(short, long)]
        new_update_authority: String,
    },
}

fn main() {
    let cli = Cli::parse();
    let rpc = &RpcClient::new(cli.rpc);

    match cli.command {
        Commands::TransferUpdateAuthority {
            signer,
            mint_data,
            new_update_authority,
        } => transfer_update_authority(rpc, signer, mint_data, new_update_authority),
    }
}

fn transfer_update_authority(
    rpc: &RpcClient,
    signer: String,
    mint_data: String,
    new_update_authority: String,
) {
    let signer = read_keypair_file(signer).expect("uanble to read signer keypair");

    let new_update_authority = new_update_authority
        .parse()
        .expect("unable to parse new update authority");

    let values = std::fs::read_to_string(mint_data).expect("unable to read mint data file");
    let values: serde_json::Value =
        serde_json::from_str(&values).expect("JSON does not have correct format");
    let values = values.as_array().expect("expected an array of data!");

    for value in values {
        let address = value.get("address").expect("expected an address");
        let address = address.as_str().expect("expected address to be a string");

        let mint = address.parse().expect("unable to parse mint address");
        let metadata = find_metadata_address(mint);
        let metadata = rpc
            .get_account(&metadata)
            .expect("unable to fetch metadata account");
        let metadata = mpl_token_metadata::state::Metadata::deserialize(&mut metadata.data())
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
