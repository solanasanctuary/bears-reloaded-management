
```bash
$ shdw-drive create-storage-account -kp ~/keys/bears-reloaded/bears-reloaded.json -n 'Bears Reloaded' -s 35GB -r $RPC
This is beta software running on Solana's Mainnet. Use at your own discretion.
✔ By creating your first storage account on Shadow Drive, you agree to the Terms of Service as outlined here: https://shadowdrive.org. Confirm? … yes
✔ This storage account will require an estimated 8.75 SHDW to setup. Would you like to continue? … yes
✔ Successfully created your new storage account of 35GB located at the following address on Solana: B9EYPPmmMEu4nEeKyyiPur7mTAWFRJxFRTpcsv33pUo5
```

```bash
$ shdw-drive upload-multiple-files -kp ~/keys/bears-reloaded/bears-reloaded.json -r $RPC -s B9EYPPmmMEu4nEeKyyiPur7mTAWFRJxFRTpcsv33pUo5 -d public
This is beta software running on Solana's Mainnet. Use at your own discretion.
Writing upload logs to /Users/levicook/bears-reloaded-firebase/shdw-drive-upload-16669738104.json.
✔ Collecting all files
✔ Fetching all storage accounts
Upload Progress | ████████████████████████████████████████ | 100% || 20004/20004 Files
20004 files uploaded.

$ cat shdw-drive-upload-16674054379.json| jq . | grep '"status"' | sort | uniq -c
19854     "status": "Not uploaded: File already exists.",
 150     "status": "Uploaded.",
```


## Use Shadow Drive

Usage:

Set an environment variable for your preferred rpc server.
Set an environment variable for your update authority key file.

eg:
```bash
export RPC=https://api.mainnet-beta.solana.com
export KEY=~/keys/bears-reloaded/bears-reloaded.json
```

Update the first set of mints:
```bash
cargo run -- --rpc ${RPC} use-shadow-drive --signer ${KEY} --mints data/mints-round1.csv
```

Update the second set of mints:
```bash
cargo run -- --rpc ${RPC} use-shadow-drive --signer ${KEY} --mints data/mints-round2.csv
```