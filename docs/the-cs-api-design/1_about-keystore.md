## 1. How to create KeyStore

```cs
var keyStore = new KeyStore("Account Name", "Account Password");
```

## 2. The property of KeyStore

```cs
// Create KeyStore;
var keyStore = new KeyStore("Account Name", "Account Password");

// The encoded encrypted ed25519 private key;
string encoded = keyStore.Encoded;

// The account principal;
string principal = keyStore.Principal;

// The meta info;
KeyStoreMeta meta = keyStore.Meta;

// The time when the account has created;
DateTime whenCreated = meta.WhenCreated;

// Always `pkcs8`;
string storeSyntax = meta.storeSyntax;

// Alwasy `ed25519`;
string sigScheme = meta.SigScheme;

// Alwasy `["argon2", "chacha20-poly1305"]`
string[] encryptedScheme = meta.EncryptedScheme;
```

## 3. The functions of KeyStore

```cs
// Create KeyStore;
var keyStore = new KeyStore("Account Name", "Account Password");

// Change name;
keyStore.Name = "Account Name New";

// Change password;
keyStore.ChangePassword("Account Password", "Account Password New");

// To JsonStr;
var jsonStr = keyStore.ToJsonStr();

// From JsonStr;
var keyStoreFromJsonStr = KeyStore.FromJsonStr(jsonStr);
```

## 4. Decrypt to `Identity`;

```cs
// Create KeyStore(Default Ed25519);
var keyStore = new KeyStore("Account Name", "Account Password");

Identity identity = keyStore.ToIdentity("Account Password");
```