# Security Policy

## Supported versions

`ardosia-raknet` is pre-release software. Security fixes are made against the current `main` branch and, when practical, the exact revisions actively consumed by Ardosia. Older commits, unofficial forks, and modified builds are not generally supported.

## Reporting a vulnerability

Do **not** report suspected vulnerabilities in a public issue, pull request, discussion, benchmark report, or log paste.

Use GitHub private vulnerability reporting for this repository:

https://github.com/ardosia/ardosia-raknet/security/advisories/new

Include, when available:

- the affected commit SHA;
- the affected subsystem or API;
- the security impact and required conditions;
- a minimal reproduction or proof of concept;
- sanitized logs or packet captures;
- any known mitigation or suggested fix.

Do not include credentials, tokens, private infrastructure details, unrelated user data, or unnecessary third-party data.

If private vulnerability reporting is temporarily unavailable, contact the maintainers without publishing exploit details and request a private reporting channel.

## In-scope examples

Security-relevant reports may include:

- crafted RakNet traffic causing panics, memory corruption, uncontrolled allocation, or persistent resource exhaustion;
- denial-of-service conditions caused by handshake, session, ACK/NACK, retransmission, fragmentation, or queue handling;
- bypasses of transport-level rate limits, IP blocks, abuse controls, or processing budgets;
- malformed-frame or malformed-datagram behavior that crosses documented validation boundaries;
- session mix-ups, peer-routing errors, or reliability/ordering defects with a concrete security impact;
- unsafe temporary-file, socket, proxy-routing, or telemetry behavior where applicable;
- vulnerabilities in shipped dependencies that materially affect the hardfork.

Performance differences or benchmark ceilings without a security impact are not vulnerabilities by themselves.

## Out of scope

The following are generally outside this repository's security scope unless this hardfork causes or materially worsens the issue:

- Minecraft/MCPE gameplay or packet semantics implemented by consuming applications;
- authentication or authorization policy owned by consuming applications;
- vulnerabilities solely in private Ardosia application layers;
- defects that exist only in unsupported third-party modifications;
- generic volumetric network attacks that cannot be mitigated meaningfully at this library layer.

## Disclosure

Maintainers will assess reports, coordinate remediation and disclosure when appropriate, and keep reporters informed when practical. No fixed response or remediation SLA is guaranteed.

Please allow a reasonable remediation period before public disclosure. Do not test against infrastructure you do not own or have permission to assess.