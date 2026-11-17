---
title: Permissions Method Group
description: TrUAPI method group for requesting and enforcing Product permissions, including remote-operation and device permissions, denial handling, and Host requirements.
categories: Apps, Reference
---

# Permissions

## Introduction

The Permissions method group is the gate every other method group implicitly passes through. Before a Product can sign a transaction, make an outbound request, or access the microphone, the Host checks whether the user granted the required permission. This method group is how a Product asks for a permission, and it defines what the Host must enforce when a permission is missing.

Most Products do not call these methods for every capability. The Product SDK requests the common ones for you, and several business calls trigger their permission prompt implicitly the first time they run. The method group is still the canonical place the permission set is defined.

## Permission Types

The [`@parity/truapi`](https://www.npmjs.com/package/@parity/truapi) client splits permissions into two families, each requested through its own call on the `permissions` domain.

### Remote-Operation Permissions

Requested with `requestRemotePermission`. Each value is a tagged `RemotePermission`:

| Permission        | What it authorizes                                                                                         |
|:-----------------:|:----------------------------------------------------------------------------------------------------------:|
| `Remote`          | Outbound HTTP and WebSocket access to a listed set of domains (`{ domains: string[] }`).                   |
| `WebRtc`          | WebRTC peer connections. Camera and microphone capture are gated separately by device permissions.         |
| `ChainSubmit`     | Requesting that the Host submit transactions on the user's behalf. See [Signing](/apps/reference/protocol/truapi/signing.md). |
| `PreimageSubmit`  | Submitting preimages on the user's behalf. See [Preimage](/apps/reference/protocol/truapi/preimage.md).     |
| `StatementSubmit` | Submitting statements on the user's behalf. See [Statement Store](/apps/reference/protocol/truapi/statement-store.md). |

`ChainSubmit`, `PreimageSubmit`, and `StatementSubmit` are also triggered implicitly by the corresponding business calls when they have not been granted yet.

### Device Permissions

Requested with `requestDevicePermission`. The value is one of `Notifications`, `Camera`, `Microphone`, `Bluetooth`, `NFC`, `Location`, `Clipboard`, `OpenUrl`, or `Biometrics`. `OpenUrl` covers external navigation, which is not part of the `Remote` permission.

Both calls return a `granted` flag. Resource allocations such as `AutoSigning` and the storage allowances are a separate axis, requested through resource allocation rather than this group. See [Allowances and Permissions](/apps/concepts/allowances.md).

## Requesting Permissions From a Product

The [`host`](/apps/product-sdk/host.md) package wraps both calls as `requestPermission` and `requestDevicePermission`. Each returns a `Result<boolean>`: `ok(true)` when granted, `ok(false)` when denied, and an error when the Host is unavailable or the call fails.

```typescript
import { requestPermission } from '@parity/product-sdk-host';

const result = await requestPermission({ tag: 'ChainSubmit', value: undefined });
if (!result.ok || !result.value) {
  // Explain why the feature is unavailable and how to enable it.
}
```

The [`signer`](/apps/product-sdk/signer.md) package requests `ChainSubmit` for you after a successful `connect()`, so most Products never request it by hand.

Treat a denied permission as normal control flow. A capability called without its permission returns `PermissionDenied`; show the user why the feature cannot proceed and how to enable it, rather than failing hard.

!!! warning "Provisional"
    Earlier Host documentation describes declaring permissions statically in a Product manifest and prompting for them on first open. The SDK requests permissions at run time, and this reference does not yet document a manifest format. Treat the runtime request calls above as the confirmed surface.

## Host Requirements

A Host implementing this method group must:

- **Enforce at the boundary**: Check the grant before dispatching any gated call, and return `PermissionDenied` when the permission is missing. A Product must not be able to reach a gated capability through any other path.
- **Scope `Remote` to its domains**: Block outbound requests to any domain outside the granted list, even when the user granted the permission. The domain list is the boundary, not the user's consent alone.
- **Keep `ChainSubmit` narrow**: Granting `ChainSubmit` lets a Product _ask_ for a signature. It does not authorize the Host to sign. Each signing request still needs its own user confirmation unless an `AutoSigning` allocation covers it.
- **Store decisions per Product**: A decision applies to the Product that asked. Later requests for a stored grant must not prompt again.
- **Respect the operating system grant**: A device permission resolves only while both the stored grant and the underlying OS or browser grant allow it. A stored grant whose OS grant was revoked answers `granted: false` without a prompt.
- **Offer a way back**: When a permission is denied, show the user where they can re-enable it in the Host's settings.

Hosts that run Products in a browser have extra constraints. See [Host Implementation Notes](/apps/reference/protocol/truapi/index.md#host-implementation-notes).

!!! note "Host-specific aliases"
    Some Host documentation uses friendly names for spec values. Polkadot Desktop calls the `Remote` permission `ExternalRequest` and describes its scope as a URL pattern. Older SDK releases used `TransactionSubmit` for `ChainSubmit`; the `signer` option `requestTransactionSubmitPermission` remains as a deprecated alias of `requestChainSubmitPermission`.

## Where to Go Next

<div class="grid cards" markdown>

- <span class="badge learn">Learn</span> **Signing**

    ---

    The signing model that `ChainSubmit` gates: what the permission grants and what it does not.

    [:octicons-arrow-right-24: Reference](/apps/reference/protocol/truapi/signing.md)

- <span class="badge learn">Learn</span> **Allowances and Permissions**

    ---

    The resource allocations, including `AutoSigning`, that sit alongside permissions.

    [:octicons-arrow-right-24: Learn](/apps/concepts/allowances.md)

</div>
