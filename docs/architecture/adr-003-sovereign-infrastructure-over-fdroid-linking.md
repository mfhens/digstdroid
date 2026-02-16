# ADR-003: Sovereign Infrastructure Over F-Droid Repository Linking

**Status:** Accepted  
**Date:** 2026-02-16  
**Deciders:** DIGST Technical Team  
**Context:** Platform architecture decision for DK-AppStore

---

## Context

During the architecture phase, a question arose: could DK-AppStore simply function as a redirect or curated link to the existing F-Droid repository (f-droid.org), rather than operating independent infrastructure?

This would involve:
- Hosting Danish government apps on F-Droid's servers
- Pointing Danish citizens to F-Droid for downloads
- Potentially maintaining a "Danish apps" category within F-Droid

This approach would significantly reduce development and operational costs.

---

## Decision

**DK-AppStore will operate as sovereign Danish infrastructure**, using F-Droid's open-source tooling (fdroidserver) as a foundation but hosting all components on Danish-controlled servers.

We will NOT link to or depend on F-Droid's infrastructure for app distribution.

---

## Rationale

### 1. Sovereignty Requirements

The Danish government must retain full operational control over critical national infrastructure.

| Concern | F-Droid Linking | Sovereign Infrastructure |
|---------|-----------------|--------------------------|
| **Server location** | Germany (Hetzner) | Danish data centers |
| **Operational control** | F-Droid e.V. (German nonprofit) | Digitaliseringsstyrelsen |
| **Unilateral removal** | Possible by F-Droid | Only by Danish authorities |
| **Data jurisdiction** | German/EU law | Danish law |
| **Availability guarantee** | Best effort, volunteer-run | Contractual SLA (99.99%) |

**Critical scenario:** In a geopolitical crisis, international sanctions, or policy dispute, F-Droid could be pressured or compelled to remove Danish government applications. With sovereign infrastructure, only Danish authorities can make such decisions.

### 2. Security Requirements Incompatibility

DK-AppStore requires security controls that F-Droid's architecture cannot provide:

| Requirement | F-Droid Capability | DK-AppStore Requirement |
|-------------|-------------------|------------------------|
| **Signing keys** | F-Droid-managed | Sovereign HSM (FIPS 140-3 Level 3) |
| **Quorum signing** | No | 2-of-3 multi-party authorization |
| **Build verification** | Single builder | 3 independent builders |
| **Security review** | Automated only | Static + Dynamic + Human review |
| **Incident response** | Community volunteers | 24/7 professional SOC |
| **Audit trail** | Limited | Complete signing/build transparency logs |
| **Penetration testing** | Sporadic | Annual mandatory by external auditors |

For MitID (national digital identity), a security breach could enable identity theft affecting up to 6 million citizens. F-Droid's community-driven security model, while appropriate for general FOSS distribution, does not meet the requirements for national critical infrastructure.

### 3. Privacy Guarantees

DK-AppStore makes specific privacy commitments that F-Droid cannot contractually guarantee:

| Privacy Aspect | F-Droid | DK-AppStore |
|----------------|---------|-------------|
| **Download tracking** | Minimal but present | None |
| **IP retention** | Unclear policy | Anonymized within 7 days |
| **Third-party analytics** | None known | Prohibited |
| **GDPR controller** | F-Droid e.V. | DIGST (Danish authority) |
| **Data subject requests** | Via F-Droid | Direct to Danish authority |

Danish citizens have a right to expect that their government can directly answer questions about data handling. Linking to F-Droid would introduce a foreign intermediary.

### 4. Critical App Support Tiers

MitID, Sundhed.dk, and Borger.dk require enhanced security tiers:

| Feature | Standard F-Droid | DK-AppStore Critical Tier |
|---------|------------------|---------------------------|
| **Certification** | None | Common Criteria EAL4+ |
| **Formal verification** | No | Required for cryptographic code |
| **Uptime SLA** | None | 99.99% (52 min/year max downtime) |
| **Incident response** | Community | 15-minute critical response |
| **App suspension** | No capability | Immediate nationwide capability |

F-Droid has no mechanism for differentiated security tiers. All apps receive the same treatment regardless of criticality.

### 5. Legal and Compliance Requirements

| Regulation | Linking to F-Droid | Sovereign Infrastructure |
|------------|-------------------|--------------------------|
| **NSIS** (National Security Standard) | Non-compliant | Compliant |
| **eIDAS** (Electronic ID) | Unclear | Full compliance |
| **NIS2** (Critical Infrastructure) | Non-compliant | Compliant |
| **Danish public sector IT policy** | Non-compliant | Compliant |

Danish government IT policy requires critical systems to be operated by or under contract to Danish authorities. F-Droid, as a foreign nonprofit with no contractual relationship to DIGST, cannot satisfy these requirements.

---

## What We DO Use From F-Droid

This decision does not reject F-Droid's contributions. We leverage:

| Component | Usage |
|-----------|-------|
| **fdroidserver** | Python tooling for repository management, metadata handling |
| **Repository format** | index-v2.json structure, APK organization |
| **Client codebase** | Fork of F-Droid Android client as starting point |
| **Build recipes** | Metadata format for reproducible build instructions |
| **Community knowledge** | 13+ years of operational experience |

DK-AppStore is effectively a **sovereign fork** that inherits F-Droid's proven foundations while adding Danish-specific security, governance, and compliance layers.

---

## Alternatives Considered

### Alternative 1: Pure F-Droid Linking

**Description:** Danish government apps hosted on f-droid.org with a Danish landing page linking there.

**Rejection reasons:**
- No sovereignty (see above)
- No security SLA
- No incident response capability
- Non-compliant with Danish public sector IT policy
- Citizens must trust foreign nonprofit

### Alternative 2: F-Droid Custom Repository

**Description:** Operate a custom F-Droid repository (separate from main f-droid.org) but use F-Droid client without modification.

**Rejection reasons:**
- Still depends on F-Droid client updates
- No control over client security patches
- Limited branding and UX customization
- Cannot enforce certificate pinning to Danish servers
- F-Droid client could theoretically be modified to block custom repos

### Alternative 3: Contractual Agreement with F-Droid e.V.

**Description:** Negotiate a contract with F-Droid e.V. for enhanced SLA, Danish data handling, etc.

**Rejection reasons:**
- F-Droid e.V. is a small volunteer nonprofit; cannot provide enterprise SLAs
- Would require F-Droid to fundamentally change their infrastructure
- Cost of customization likely exceeds sovereign build
- Still leaves signing keys outside Danish control
- Creates single-vendor dependency on foreign entity

### Alternative 4: Full Custom Build (No F-Droid Components)

**Description:** Build everything from scratch without using fdroidserver or F-Droid repository format.

**Rejection reasons:**
- Unnecessary reinvention of proven technology
- Higher development cost and timeline
- fdroidserver is mature, audited, and fits purpose
- Repository format is well-documented and client-compatible
- Would delay Phase 1 by 6-12 months

**Verdict:** The chosen approach (sovereign infrastructure using F-Droid tooling) balances sovereignty requirements with pragmatic reuse of proven open-source components.

---

## Consequences

### Positive

1. **Full sovereignty** - Danish government controls all infrastructure and decisions
2. **Security compliance** - Can implement HSM signing, quorum authorization, and audit requirements
3. **Privacy guarantees** - Direct accountability to Danish citizens under Danish law
4. **Critical app support** - Differentiated security tiers for MitID-class applications
5. **Incident response** - Immediate suspension capability for compromised apps
6. **Regulatory compliance** - Meets NSIS, eIDAS, NIS2, and Danish IT policy requirements

### Negative

1. **Operational cost** - Must operate infrastructure rather than relying on volunteers
2. **Development effort** - Must build and maintain Danish-specific enhancements
3. **Ongoing maintenance** - Security patches, updates, and monitoring responsibility
4. **Staffing requirements** - Need 24/7 operations capability (via vendor contract)

### Cost-Benefit Analysis

| Factor | Linking to F-Droid | Sovereign Infrastructure |
|--------|-------------------|--------------------------|
| **Initial development** | Low | Medium |
| **Ongoing operations** | Very low | Medium |
| **Risk of MitID compromise** | Unacceptable | Managed |
| **Regulatory compliance** | Non-compliant | Compliant |
| **Citizen trust** | Questionable | High |

The operational cost is justified by the criticality of the applications being distributed. A single successful attack on MitID could cost billions in fraud and remediation. The sovereign infrastructure investment is proportionate to this risk.

---

## References

- [F-Droid Infrastructure Documentation](https://f-droid.org/docs/)
- [Danish Public Sector IT Policy](https://digst.dk/)
- [NSIS - National Standard for Identity Security](https://digst.dk/it-loesninger/nemid/nsis/)
- [eIDAS Regulation](https://eur-lex.europa.eu/legal-content/EN/TXT/?uri=uriserv:OJ.L_.2014.257.01.0073.01.ENG)
- [NIS2 Directive](https://eur-lex.europa.eu/eli/dir/2022/2555)

---

## Decision Record

| Date | Action | Author |
|------|--------|--------|
| 2026-02-16 | Initial decision documented | DIGST Technical Team |
