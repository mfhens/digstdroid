# ADR-002: iOS Distribution Strategy

**Status:** Accepted  
**Date:** 2026-02-05  
**Deciders:** DIGST Technical Team  
**Context:** Strategy for distributing DK-AppStore applications to iOS users without duplicating implementation

---

## Context

DK-AppStore is designed as a sovereign Danish app distribution platform for critical national applications (MitID, Sundhed.dk, Borger.dk). The initial architecture focuses on Android distribution via an F-Droid-based approach.

However, iOS holds approximately 50-55% market share in Denmark. Excluding iOS users from sovereign app distribution would:

1. Leave half of Danish citizens dependent on Apple App Store for critical national infrastructure
2. Undermine the sovereignty goals of the project
3. Create an inconsistent user experience across platforms

The challenge: Apple historically prohibited alternative app distribution on iOS. This changed with the EU Digital Markets Act (DMA), which requires Apple to allow alternative distribution mechanisms in the EU as of March 2024.

### Key Constraints

1. **No double implementation**: Security-critical code must be written and audited once
2. **Citizen distribution**: Must serve all Danish citizens, not just government employees
3. **Sovereignty**: Cannot depend on Apple App Store for critical app availability
4. **Timeline**: iOS support should not delay Android launch but should follow reasonably quickly

---

## Decision

**Adopt a shared Rust core architecture with platform-specific UI layers, distributed via DMA Web Distribution for iOS.**

### Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    Shared Rust Core                         │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────────────┐   │
│  │  Signature  │ │   Index     │ │  Update/Download    │   │
│  │ Verification│ │  Parsing    │ │     Logic           │   │
│  └─────────────┘ └─────────────┘ └─────────────────────┘   │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────────────┐   │
│  │ Certificate │ │   Crypto    │ │  Repository         │   │
│  │  Pinning    │ │  Utilities  │ │  Management         │   │
│  └─────────────┘ └─────────────┘ └─────────────────────┘   │
└──────────────────────────┬──────────────────────────────────┘
                           │
              ┌────────────┴────────────┐
              │                         │
      ┌───────▼───────┐         ┌───────▼───────┐
      │    UniFFI     │         │     JNI       │
      │   Bindings    │         │   Bindings    │
      └───────┬───────┘         └───────┬───────┘
              │                         │
      ┌───────▼───────┐         ┌───────▼───────┐
      │   Swift UI    │         │  Kotlin UI    │
      │  (SwiftUI)    │         │  (Compose)    │
      └───────┬───────┘         └───────┬───────┘
              │                         │
      ┌───────▼───────┐         ┌───────▼───────┐
      │  DMA Web      │         │  Direct APK   │
      │ Distribution  │         │  Distribution │
      └───────────────┘         └───────────────┘
           iOS                      Android
```

### Distribution Mechanisms

| Platform | Distribution Method | Requirements |
|----------|---------------------|--------------|
| **Android** | Direct APK from DK-AppStore | None (standard Android sideloading) |
| **iOS** | DMA Web Distribution | Apple notarization, EU App Store terms, user enables setting |

### Implementation Components

1. **dk-client-core** (Rust crate): All security-critical client logic
2. **dk-client-ios** (Swift): Thin UI layer using SwiftUI, calls Rust via UniFFI
3. **dk-client-android** (Kotlin): Thin UI layer, calls Rust via JNI (enhances current F-Droid fork)

---

## Rationale

### Why Shared Rust Core

| Benefit | Detail |
|---------|--------|
| **Single audit surface** | Security-critical code (signature verification, crypto) written and audited once |
| **Memory safety on both platforms** | Rust guarantees apply regardless of target platform |
| **Consistency** | Identical verification logic prevents platform-specific bugs |
| **Reduced maintenance** | Bug fixes apply to both platforms simultaneously |
| **~70% code sharing** | Only UI and platform integration remain platform-specific |

### Why DMA Web Distribution for iOS

| Factor | Assessment |
|--------|------------|
| **Lower barrier than Marketplace** | No 1M EUR letter of credit required |
| **Sufficient for government apps** | MitID unlikely to exceed 1M first-annual-installs threshold for Core Technology Fee |
| **Direct control** | Apps hosted on Danish infrastructure, not Apple servers |
| **Notarization != Review** | Apple checks for malware, not content policy; faster than App Store review |
| **User friction acceptable** | One-time setting toggle; citizens will enable for critical national apps |

### Why Not Alternative Marketplace

Apple's Alternative Marketplace program requires:
- 1,000,000 EUR standby letter of credit
- Apple approval as Marketplace Developer
- Core Technology Fee: 0.50 EUR per first annual install above 1M threshold
- Compliance with Marketplace guidelines

For a government-operated store distributing a small number of critical apps, the overhead and financial commitment are disproportionate. Web Distribution provides sovereignty without marketplace complexity.

### Why Not Progressive Web App (PWA) Only

PWAs cannot install native iOS apps. While a PWA could serve as the catalog/browsing interface, actual app installation requires native distribution. PWA may complement but cannot replace native distribution.

### Why Not Kotlin Multiplatform (KMP)

| Concern | Detail |
|---------|--------|
| **Less mature iOS support** | KMP/Native still evolving; Rust iOS toolchain more proven |
| **No memory safety** | KMP uses JVM on Android, native on iOS; no uniform safety guarantees |
| **Rust already chosen** | Server components use Rust (ADR-001); extending to client core maintains consistency |
| **Security audit confidence** | Auditors already reviewing Rust; adding KMP increases audit scope |

---

## Consequences

### Positive

1. **True code sharing**: ~70% shared Rust core across platforms
2. **Single security audit**: Crypto and verification logic audited once
3. **Sovereignty on iOS**: No App Store dependency for critical national apps
4. **Consistent security posture**: Memory-safe core on both platforms
5. **Future-proof**: Rust iOS ecosystem growing; Mozilla, 1Password, others use this pattern

### Negative

1. **iOS development complexity**: UniFFI adds build complexity vs. pure Swift
2. **DMA uncertainty**: Apple's DMA compliance still evolving; terms may change
3. **User friction**: iOS users must enable "Install apps from web" setting
4. **Notarization dependency**: Still requires Apple notarization (malware check)
5. **Two UI codebases**: SwiftUI and Kotlin Compose are separate implementations

### Risks and Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Apple changes DMA terms | Medium | High | Monitor regulatory developments; maintain App Store presence as fallback |
| UniFFI limitations | Low | Medium | Mozilla actively maintains; large community; fallback to cbindgen if needed |
| Notarization rejection | Low | High | Notarization is malware check, not policy review; government apps low risk |
| User adoption friction | Medium | Medium | Clear instructions; in-app guidance; media campaign for MitID launch |
| Rust iOS build issues | Low | Medium | Well-documented toolchain; cargo-lipo, cargo-xcode mature |

---

## Implementation Plan

### Phase 0 (Current - Android Focus)

- Extract client core logic into `dk-client-core` Rust crate
- Design FFI-friendly API boundaries
- Continue Android client development using Rust core via JNI

### Phase 1 (Post-Android Launch)

1. **Month 1-2**: UniFFI bindings for `dk-client-core`
2. **Month 2-3**: Swift UI shell (SwiftUI)
3. **Month 3**: Apple Developer Program enrollment, notarization setup
4. **Month 4**: DMA Web Distribution integration
5. **Month 5**: Beta testing with Danish users
6. **Month 6**: Public iOS launch

### Crate Structure

```
dk-appstore/
├── client-core/                    # Shared Rust core
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── repository.rs          # Repository management
│       ├── index.rs               # Index parsing and caching
│       ├── verification.rs        # Signature verification
│       ├── crypto.rs              # Cryptographic utilities
│       ├── download.rs            # Download management
│       ├── update.rs              # Update checking logic
│       └── uniffi.udl             # UniFFI interface definition
│
├── client-android/                 # Android client (Kotlin)
│   └── ...                        # F-Droid fork + JNI bindings
│
└── client-ios/                     # iOS client (Swift)
    ├── DKAppStore/
    │   ├── App/                   # SwiftUI app structure
    │   ├── Views/                 # SwiftUI views
    │   ├── ViewModels/            # MVVM view models
    │   └── RustBridge/            # UniFFI generated bindings
    ├── DKAppStore.xcodeproj
    └── Package.swift
```

### UniFFI Interface Example

```udl
// client-core/src/uniffi.udl
namespace dk_client_core {
    // Repository operations
    [Throws=ClientError]
    Repository load_repository(string url);
    
    [Throws=ClientError]
    Index fetch_index(Repository repo);
    
    // Verification
    [Throws=ClientError]
    boolean verify_signature(bytes data, bytes signature, bytes public_key);
    
    [Throws=ClientError]
    boolean verify_apk_signature(string path);
    
    [Throws=ClientError]
    boolean verify_ipa_signature(string path);
};

interface Repository {
    string get_url();
    [Throws=ClientError]
    sequence<App> list_apps();
};

interface Index {
    i64 get_timestamp();
    sequence<App> get_apps();
    [Throws=ClientError]
    App? get_app(string package_id);
};

[Error]
enum ClientError {
    "NetworkError",
    "VerificationFailed", 
    "ParseError",
    "NotFound",
};
```

---

## DMA Web Distribution Technical Details

### Requirements

1. **Apple Developer Program membership** (99 EUR/year)
2. **Accept EU Alternative Distribution Terms**
3. **Notarization**: Submit IPA to Apple for malware scanning
4. **Web hosting**: Serve IPA from Danish infrastructure with proper headers

### Distribution Flow

```
┌─────────┐     ┌─────────────┐     ┌─────────────┐     ┌──────────┐
│  User   │────▶│ DK-AppStore │────▶│   Apple     │────▶│  Device  │
│ (Safari)│     │   Website   │     │ Notarization│     │ Install  │
└─────────┘     └─────────────┘     └─────────────┘     └──────────┘
     │                │                    │                  │
     │  1. Browse     │                    │                  │
     │  catalog       │                    │                  │
     │◀───────────────│                    │                  │
     │                │                    │                  │
     │  2. Click      │                    │                  │
     │  "Install"     │                    │                  │
     │───────────────▶│                    │                  │
     │                │  3. Serve IPA      │                  │
     │                │  (pre-notarized)   │                  │
     │◀───────────────│                    │                  │
     │                │                    │                  │
     │  4. iOS verifies notarization       │                  │
     │─────────────────────────────────────▶                  │
     │                                     │                  │
     │  5. Install    │                    │                  │
     │  prompt        │                    │                  │
     │◀───────────────────────────────────────────────────────│
```

### Server Requirements

```nginx
# Nginx configuration for IPA distribution
location ~ \.ipa$ {
    add_header Content-Type application/octet-stream;
    add_header Content-Disposition attachment;
    # Apple requires specific headers for web distribution
    add_header X-Apple-Marketplace-Id "dk.digst.appstore";
}
```

---

## Alternatives Considered

### 1. iOS App Store Only (Rejected)

**Why rejected:**
- Contradicts sovereignty goals
- Apple retains ability to remove apps
- Subject to App Store review policies
- No control over distribution

**When acceptable:** Maintain App Store presence as supplementary channel for user convenience, but not as primary distribution.

### 2. Full Alternative Marketplace (Rejected)

**Why rejected:**
- 1M EUR financial barrier disproportionate for government operation
- Complexity of running full marketplace infrastructure
- Core Technology Fee creates cost uncertainty for high-adoption apps

**When to reconsider:** If DK-AppStore expands to host many third-party apps, marketplace status may become worthwhile.

### 3. Wait for Further DMA Evolution (Rejected)

**Why rejected:**
- Current DMA provisions sufficient for web distribution
- Delaying iOS indefinitely unacceptable given 50%+ market share
- Can adapt if terms improve

### 4. Flutter/React Native Cross-Platform (Rejected)

**Why rejected:**
- No memory safety guarantees
- Large runtime overhead
- Security audit complexity
- Diverges from Rust-first strategy (ADR-001)

---

## References

- [Apple: Distributing apps in the EU](https://developer.apple.com/support/alternative-app-distribution-in-the-european-union/)
- [Apple: Web Distribution for apps in the EU](https://developer.apple.com/support/web-distribution-eu/)
- [EU Digital Markets Act - Official Text](https://eur-lex.europa.eu/eli/reg/2022/1925/oj)
- [Mozilla UniFFI](https://mozilla.github.io/uniffi-rs/)
- [Rust on iOS Guide](https://rust-lang.github.io/rustup/cross-compilation.html)

---

## Decision Record

| Date | Action | Author |
|------|--------|--------|
| 2026-02-05 | Initial decision documented | DIGST Technical Team |
