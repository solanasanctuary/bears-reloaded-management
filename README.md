

```bash
$ shdw-drive create-storage-account -kp ~/keys/bears-reloaded/bears-reloaded.json -n 'Bears Reloaded' -s 35GB -r $RPC
This is beta software running on Solana's Mainnet. Use at your own discretion.
✔ By creating your first storage account on Shadow Drive, you agree to the Terms of Service as outlined here: https://shadowdrive.org. Confirm? … yes
✔ This storage account will require an estimated 8.75 SHDW to setup. Would you like to continue? … yes
✔ Successfully created your new storage account of 35GB located at the following address on Solana: B9EYPPmmMEu4nEeKyyiPur7mTAWFRJxFRTpcsv33pUo5
```


```bash
$ shdw-drive upload-multiple-files -kp ~/keys/bears-reloaded/bears-reloaded.json -r $RPC -s B9EYPPmmMEu4nEeKyyiPur7mTAWFRJxFRTpcsv33pUo5 -d public/
```