using System;
using NUnit.Framework;

namespace Tests.Runtime
{
  public class AgentTest
  {
    private static string MainNet = "https://ic0.app";
    private static string IICanisterId = "rdmx6-jaaaa-aaaaa-aaadq-cai";

    private static string IIDidContent = @"type UserNumber = nat64;
type PublicKey = blob;
type CredentialId = blob;
type DeviceKey = PublicKey;
type UserKey = PublicKey;
type SessionKey = PublicKey;
type FrontendHostname = text;
type Timestamp = nat64;

type HeaderField = record { text; text; };

type HttpRequest = record {
  method: text;
  url: text;
  headers: vec HeaderField;
  body: blob;
};

type HttpResponse = record {
  status_code: nat16;
  headers: vec HeaderField;
  body: blob;
  streaming_strategy: opt StreamingStrategy;
};

type StreamingCallbackHttpResponse = record {
  body: blob;
  token: opt Token;
};

type Token = record {};

type StreamingStrategy = variant {
  Callback: record {
    callback: func (Token) -> (StreamingCallbackHttpResponse) query;
    token: Token;
  };
};

type Purpose = variant {
    recovery;
    authentication;
};

type KeyType = variant {
    unknown;
    platform;
    cross_platform;
    seed_phrase;
};

type Challenge = record {
    png_base64: text;
    challenge_key: ChallengeKey;
};

type DeviceData = record {
  pubkey : DeviceKey;
  alias : text;
  credential_id : opt CredentialId;
  purpose: Purpose;
  key_type: KeyType;
};

type RegisterResponse = variant {
  // A new user was successfully registered.
  registered: record { user_number: UserNumber; };
  // No more registrations are possible in this instance of the II service canister.
  canister_full;
  // The challenge was not successful.
  bad_challenge;
};

type AddTentativeDeviceResponse = variant {
  // The device was tentatively added.
  added_tentatively: record { verification_code: text; device_registration_timeout: Timestamp;};
  // Device registration mode is off, either due to timeout or because it was never enabled.
  device_registration_mode_off;
  // There is another device already added tentatively
  another_device_tentatively_added;
};

type VerifyTentativeDeviceResponse = variant {
  // The device was successfully verified.
  verified;
  // Wrong verification code entered. Retry with correct code.
  wrong_code: record { retries_left: nat8};
  // Device registration mode is off, either due to timeout or because it was never enabled.
  device_registration_mode_off;
  // There is no tentative device to be verified.
  no_device_to_verify;
};

type Delegation = record {
  pubkey: PublicKey;
  expiration: Timestamp;
  targets: opt vec principal;
};

type SignedDelegation = record {
  delegation: Delegation;
  signature: blob;
};

type GetDelegationResponse = variant {
  // The signed delegation was successfully retrieved.
  signed_delegation: SignedDelegation;

  // The signature is not ready. Maybe retry by calling `prepare_delegation`
  no_such_delegation
};

type InternetIdentityStats = record {
  users_registered: nat64;
  assigned_user_number_range: record { nat64; nat64; };
};

type InternetIdentityInit = record {
  assigned_user_number_range : record { nat64; nat64; };
};

type ChallengeKey = text;

type ChallengeResult = record {
    key : ChallengeKey;
    chars : text;
};

type DeviceRegistrationInfo = record {
    tentative_device : opt DeviceData;
    expiration: Timestamp;
};

type IdentityAnchorInfo = record {
    devices : vec DeviceData;
    device_registration: opt DeviceRegistrationInfo;
};

service : (opt InternetIdentityInit) -> {
  init_salt: () -> ();
  create_challenge : () -> (Challenge);
  register : (DeviceData, ChallengeResult) -> (RegisterResponse);
  add : (UserNumber, DeviceData) -> ();
  remove : (UserNumber, DeviceKey) -> ();
  // Returns all devices of the user (authentication and recovery) but no information about device registrations.
  // Note: Will be changed in the future to be more consistent with get_anchor_info.
  lookup : (UserNumber) -> (vec DeviceData) query;
  get_anchor_info : (UserNumber) -> (IdentityAnchorInfo);
  get_principal : (UserNumber, FrontendHostname) -> (principal) query;
  stats : () -> (InternetIdentityStats) query;

  enter_device_registration_mode : (UserNumber) -> (Timestamp);
  exit_device_registration_mode : (UserNumber) -> ();
  add_tentative_device : (UserNumber, DeviceData) -> (AddTentativeDeviceResponse);
  verify_tentative_device : (UserNumber, verification_code: text) -> (VerifyTentativeDeviceResponse);

  prepare_delegation : (UserNumber, FrontendHostname, SessionKey, maxTimeToLive : opt nat64) -> (UserKey, Timestamp);
  get_delegation: (UserNumber, FrontendHostname, SessionKey, Timestamp) -> (GetDelegationResponse) query;

  http_request: (request: HttpRequest) -> (HttpResponse) query;
}
";

    [Test]
    public void CreateWithAnonymous_ShouldWork()
    {
      var identity = Identity.Anonymous();
      var canisterId = Principal.FromText(IICanisterId);

      Agent.Create(MainNet, identity, canisterId, IIDidContent);
    }

    [Test]
    public void CreateWithBasic_ShouldFail()
    {
      var identity = Identity.BasicRandom();
      var canisterId = Principal.FromText(IICanisterId);

      Assert.Throws<ErrorFromRust>(() => Agent.Create(MainNet, identity, canisterId, IIDidContent));
    }

    [Test]
    public void CreateWithSecp256K11_ShouldWork()
    {
      var identity = Identity.Secp256K1Random();
      var canisterId = Principal.FromText(IICanisterId);

      Agent.Create(MainNet, identity, canisterId, IIDidContent);
    }

    [Test]
    public void Query_ShouldWork()
    {
      const string expected = @"(
  vec {
    record {
      alias = ""macbook-2021"";
      pubkey = blob ""0^0\0c\06\0a+\06\01\04\01\83\b8C\01\01\03N\00\a5\01\02\03& \01!X Q\bf\c1O\11\feX\a1\1d\1a\1a|$\be\15>\12\dc/|v\bc)\db#\14\a0pM!\fdf\22X V\ac\d0t\02c\15\e7\fd\edS\ed?K\a7r\86\86K\f9\06\9a\c7\04I\15\a3\f4\00-\a6\93"";
      key_type = variant { unknown };
      purpose = variant { authentication };
      credential_id = opt blob ""\0c\d6\e3\cd\8a\ad\07\e6\95\e9\08j\90\c6.\0d\b0\d8\cc\db\f6\c7\18l\ba\1aM\c9\8b\a8\12\c8%\d2\af\12\bc\0a\cd\b1\08\9d\af\e6\f1\9c\a0Lq\b0\a2\e9-\12\cc\8a\c1\ad%\b1P\b6\f8@+_\a9\223\af\07\0d\1d\cfv\9b\0a\80\fd\8a\abE\c5"";
    };
  },
)";

       var identity = Identity.Secp256K1Random();
       var canisterId = Principal.FromText(IICanisterId);

       var agent = Agent.Create(MainNet, identity, canisterId, IIDidContent);

       var args = agent.Query("lookup", "(1974211: nat64)");

       Assert.AreEqual(expected, args.ToString());
     }

     [Test]
     public void Update_ShouldWork()
     {
       var identity = Identity.Secp256K1Random();
       var canisterId = Principal.FromText(IICanisterId);
     
       var agent = Agent.Create(MainNet, identity, canisterId, IIDidContent);

       var args = agent.Update("create_challenge", "()");

       Assert.True(args.ToString().Length != 0);
     }

     [Test]
     public void Status_ShouldWork()
     {
       var identity = Identity.Secp256K1Random();
       var canisterId = Principal.FromText(IICanisterId);

       var agent = Agent.Create(MainNet, identity, canisterId, IIDidContent);

       var status = agent.Status();

       Assert.True(status.Length != 0);
     }
  }
}