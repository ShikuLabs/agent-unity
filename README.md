# Agent of Internet Computer for Unity

## Introduction

The project brings the `IC` ecosystem to `Unity`, allowing Unity developers to call the functions of canisters on IC, create encrypted account files, and access the `Internet Identity`.

But the project is in an early stage, feature-less, documentation-sparse, and API will be changed frequently.

Using the project in the experiment is ok, but not recommended for productive projects.

The project will be stable as soon as possible.

## Install

1. Download the unity package:

    * Open the page: https://github.com/ShikuLabs/agent-unity/releases/tag/alpha-0.1
    * Click `com.shikulab.ic-agent.zip` then will start downloading
    * Unzip the file, get a folder;

2. Load Package to Unity Project:

    * Open Unity, route to Windows-> Package Manager;
    * Click the plus icon then choose `add package from disk`;
    * Select the folder that unzipped last stage, Double click the `package.json` file;
    * Load Successful!

## Use Cases

### 1. Create keyStore

```cs
using IC;

...

void Start() {
    // Create encrypted keyStore structure;
    //
    // string name: the name of keyStore;
    // string password: the password to encrypted the private key;
    //
    // return: struct `HostKeyStore`;
    var keyStore = Agent.CreateKeyStore("Allen Pocket", "123456");
}

...

```

```cs
// The HostKeyStore struct
public struct HostKeyStore
{
    [JsonProperty]
    public readonly string Encoded;
    [JsonProperty]
    public readonly string Principal;
    [JsonProperty]
    public HostKeyStoreMeta Meta { get; set; }

    public HostKeyStore(string encoded, string principal, HostKeyStoreMeta meta)
    {
        Encoded = encoded;
        Principal = principal;
        Meta = meta;
    }
    
    public override string ToString() => $"{{encoded: {Encoded}, principal: {Principal}, meta: {Meta}}}";
}
```

### 2. Login/Logout/GetLoggedInfo

```cs
using IC;

...

void Start() {
    var keyStore = Agent.CreateKeyStore("Allen Pocket", "123456");


    // Login by keyStore;
    //
    // HostKeyStore keyStore: the keyStore you want to login;
    // string password: the password you want to decrypt the keyStore;
    //
    // return: struct `LoggedReceipt`;
    var receipt = Agent.LoginByHost(keyStore, "123456");

    // Get the logged receipt by principal;
    //
    // string principal: the identifier for the account, like address;
    //
    // return: struct `LoggedReceipt`;
    receiptGetByPrincipal = Agent.GetLoggedReceipt(receipt.Principal);

    // List all the logged receipts which have been logged;
    //
    // return: array of `LoggedReceipt`;
    var receipts = Agent.ListLoggedReceipt();

    // Logout by principal;
    //
    // string principal: the identifier for the account, like address;
    Agent.Logout(receipt.Principal);
}

...

```

```cs
// The LoggedReceipt struct
    public readonly struct LoggedReceipt
    {
        [JsonProperty]
        public readonly string Principal;
        [JsonProperty]
        public readonly DateTime Deadline;
        [JsonProperty]
        public readonly LoggedType LoggedType;

        public LoggedReceipt(string principal, DateTime deadline, LoggedType loggedType)
        {
            Principal = principal;
            Deadline = deadline;
            LoggedType = loggedType;
        }
        
        public override string ToString() =>
            $"{{principal: {Principal}, deadline: {Deadline}, loggedType: {LoggedType}}}";
    }
```

### 3. Query function of canister on ic

```cs
// For example, to query the `lookup` method of ii(internet identity) canister on ic mainnet;
// 
// The ii canisterId is: rdmx6-jaaaa-aaaaa-aaadq-cai, so:
const string II_CANISTER_ID = "rdmx6-jaaaa-aaaaa-aaadq-cai";

// Then you should have the ii candid file(will be posted below);
// Read candid file from file;
string II_IDL_CONTENT = ..;

// Create keyStore then login;
var keyStore = Agent.CreateKeyStore("Allen Pocket", "123456");
var receipt = Agent.LoginByHost(keyStore, "123456");

// Load candid file:
Agent.RegisterIdl(II_CANISTER_ID, II_IDL_CONTENT);

// Call `lookup` method;
//
// The function will return a struct which is serialized, that representation is literal;
string rstRaw = Agent.QuerySync(receipt.Principal, II_CANISTER_ID, "lookup", "(1974211: nat64)");
Debug.Log(rstRaw);

Agent.Logout(receipt.Principal);
```

[The ii canister candid file](./agent//src/rdmx6-jaaaa-aaaaa-aaadq-cai.did)

## Features will be supported recently

|                       Feature                       | State |
| :-------------------------------------------------: | :---: |
|            Encrypted Account File Create            |  ✅   |
|       Login/Logout By Encrypted Account File        |  ✅   |
|       Login/Logout By hex-encoded privateKey        |  ❌   |
|        Login/Logout By pem-format privateKey        |  ❌   |
|         Login/Logout By `Internet Identity`         |  ❌   |
| Call the Query Functions of Canister on IC MainNet  |  ✅   |
| Call the Update Functions of Canister on IC MainNet |  ❌   |
|         Download Candid File from Internet          |  ❌   |

## Support Platform

| Platform | Editor | Runtime |
| :------: | :----: | :-----: |
| Windows  |   ✅   |   🚧    |
|   OSX    |   ✅   |   🚧    |
|  Linux   |   ❌   |   ❌    |

## [TODO] The Unity Package Release Page

todo

## [TODO] Contributing

todo