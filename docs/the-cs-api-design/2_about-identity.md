## 1. Ed25519Identity

```cs
// Create Ed25519 Identity in random;
var ed25519IdentityRandom = new Ed25519Identity();

// Create Ed25519 Identity by pem file;
var pemContent = File.ReadAll(..);
var ed25519IdentityPem = new Ed25519Identity(pemContent);

// To KeyStore
var keyStore = KeyStore.From(ed25519Identity);
```


## 2. AnonymousIdentity

```cs
// Create Anonymous Identity;
var anonymousIdentity = new AnonymousIdentity();
```

## [TODO]. Secp256k1Identity

## [TODO]. InternetIdentity

## [TODO]. ProxyIdentity

## X. Interface Identity

```cs
interface Identity {
    public string Principal { get; }

    public Signature Sign(byte[] blob);
}
```