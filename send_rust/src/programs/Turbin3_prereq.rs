use solana_idlgen::idlgen;

//idlgen!("./programs/idl_turbine.json");

idlgen!({
  "version": "0.1.0",
  "name": "turbine_prereq",
  "instructions": [{
      "name": "complete",
      "accounts": [{
          "name": "signer",
          "isMut": true,
          "isSigner": true
      },
      {
          "name": "prereq",
          "isMut": true,
          "isSigner": false
      },
      {
          "name": "system_program",
          "isMut": false,
          "isSigner": false
        }],
      "args": [
      {
        "name": "github",
        "type": "bytes"
      }
    ]
  },
  {
      "name": "update",
      "accounts": [{
          "name": "signer",
          "isMut": true,
          "isSigner": true
      },
      {
          "name": "prereq",
          "isMut": true,
          "isSigner": false
      },
      {
          "name": "system_program",
          "isMut": false,
          "isSigner": false
        }],
      "args": [
      {
        "name": "github",
        "type": "bytes"
      }
    ]
  }],
  "accounts": [{
      "name": "PrereqAccount",
      "type": {
          "kind": "struct",
          "fields": [{
              "name": "github",
              "type": "bytes"
          },
          {
              "name": "key",
              "type": "pubkey"
            }]
      }
  }],
  "errors": [
    {
      "code": 6021,
      "name": "InVALIDGithubAccount",
      "msg": "Invalid Github ACCOUNT"
    }
  ],
  "metadata": {
      "address": "ADcaide4vBtKuyZQqdU689YqEGZMCmS4tL35bdTv9wJa"
  }
});

