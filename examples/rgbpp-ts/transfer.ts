import "dotenv/config";

import { ccc } from "@ckb-ccc/shell";
import {
  AddressType,
  CkbRgbppUnlockSigner,
  PrivateKeyRgbppBtcWallet,
  RgbppUdtClient,
  buildNetworkConfig,
  PredefinedNetwork,
  createScriptProvider,
} from "@ckb-ccc/rgbpp";

function required(name: string): string {
  const value = process.env[name]?.trim();
  if (!value) throw new Error(`${name} is required`);
  return value;
}

async function withTimeout<T>(
  label: string,
  operation: Promise<T>,
  ms = 60_000,
): Promise<T> {
  let timer: ReturnType<typeof setTimeout> | undefined;
  try {
    return await Promise.race([
      operation,
      new Promise<T>((_, reject) => {
        timer = setTimeout(
          () => reject(new Error(label + " timed out after " + ms + "ms")),
          ms,
        );
      }),
    ]);
  } finally {
    if (timer) clearTimeout(timer);
  }
}

const networkName = (process.env.UTXO_BASED_CHAIN_NAME ??
  "BitcoinTestnet3") as PredefinedNetwork;
if (networkName !== PredefinedNetwork.BitcoinTestnet3) {
  throw new Error("Unsafe network: RGB++ example requires Bitcoin Testnet3");
}

const ckbPrivateKey = required("CKB_SECP256K1_PRIVATE_KEY");
const btcPrivateKey = required("UTXO_BASED_CHAIN_PRIVATE_KEY");
const btcAddressType = required("UTXO_BASED_CHAIN_ADDRESS_TYPE") as AddressType;
const btcApiUrl = required("BTC_ASSETS_API_URL");
const btcApiToken = required("BTC_ASSETS_API_TOKEN");
const btcApiOrigin = required("BTC_ASSETS_API_ORIGIN");
const receiverAddress = required("RGBPP_RECEIVER_BTC_ADDRESS");
const udtTypeArgs = required("UDT_TYPE_ARGS");
const transferAmount = BigInt(process.env.RGBPP_TRANSFER_AMOUNT ?? "1");
const feeRate = Number(process.env.RGBPP_FEE_RATE ?? "28");
const broadcast = process.env.RGBPP_BROADCAST === "true";
const broadcastConfirmation =
  process.env.RGBPP_CONFIRM_TESTNET_BROADCAST === "YES";

if (broadcast && !broadcastConfirmation) {
  throw new Error(
    "Broadcast is blocked. Set RGBPP_CONFIRM_TESTNET_BROADCAST=YES only after independently verifying the testnet receiver, asset, amount and fees.",
  );
}
if (!/^https:\/\//i.test(btcApiUrl)) {
  throw new Error("BTC_ASSETS_API_URL must use HTTPS");
}
if (!/^https:\/\//i.test(btcApiOrigin)) {
  throw new Error("BTC_ASSETS_API_ORIGIN must use HTTPS");
}
if (!/^0x[0-9a-f]{64}$/i.test(udtTypeArgs)) {
  throw new Error(
    "UDT_TYPE_ARGS must be a 32-byte 0x-prefixed hex token identifier",
  );
}

if (!Object.values(AddressType).includes(btcAddressType)) {
  throw new Error(`Unsupported BTC address type: ${btcAddressType}`);
}
if (transferAmount <= 0n) throw new Error("RGBPP_TRANSFER_AMOUNT must be > 0");
if (!Number.isFinite(feeRate) || feeRate <= 0) {
  throw new Error("RGBPP_FEE_RATE must be > 0");
}

const networkConfig = buildNetworkConfig(networkName);
const ckbClient = new ccc.ClientPublicTestnet();
const ckbSigner = new ccc.SignerCkbPrivateKey(ckbClient, ckbPrivateKey);
const scriptProvider = createScriptProvider(ckbClient);
const rgbppUdtClient = new RgbppUdtClient(networkConfig, ckbClient, scriptProvider);

const btcWallet = new PrivateKeyRgbppBtcWallet(
  btcPrivateKey,
  btcAddressType,
  networkConfig,
  {
    url: btcApiUrl,
    token: btcApiToken,
    origin: btcApiOrigin,
  },
);

const btcAddress = await withTimeout("BTC address derivation", btcWallet.getAddress());
const rgbppScriptInfos = await withTimeout(
  "RGB++ script info resolution",
  rgbppUdtClient.getRgbppScriptInfos(),
);
const ckbRgbppUnlockSigner = new CkbRgbppUnlockSigner({
  ckbClient,
  rgbppBtcAddress: btcAddress,
  btcDataSource: btcWallet,
  scriptInfos: rgbppScriptInfos,
});

const xuDtScriptInfo = await withTimeout(
  "CKB KnownScript.XUdt resolution",
  ckbClient.getKnownScript(ccc.KnownScript.XUdt),
);
const udtScript = await ccc.Script.fromKnownScript(
  ckbClient,
  ccc.KnownScript.XUdt,
  udtTypeArgs,
);
const udt = new ccc.udt.Udt(
  xuDtScriptInfo.cellDeps[0].cellDep.outPoint,
  udtScript,
);

const pseudoLock = await rgbppUdtClient.buildPseudoRgbppLockScript();

let { res: ckbPartialTx } = await withTimeout(
  "xUDT CKB partial transaction construction",
  udt.transfer(ckbSigner, [
    {
      to: pseudoLock,
      amount: transferAmount,
    },
  ]),
);

ckbPartialTx = await withTimeout(
  "xUDT CKB change completion",
  udt.completeChangeToLock(
    ckbPartialTx,
    ckbRgbppUnlockSigner,
    pseudoLock,
  ),
);

const { psbt, indexedCkbPartialTx } = await withTimeout(
  "Bitcoin PSBT construction",
  btcWallet.buildPsbt({
    ckbPartialTx,
    ckbClient,
    rgbppUdtClient,
    btcChangeAddress: btcAddress,
    receiverBtcAddresses: [receiverAddress],
    feeRate,
  }),
);

console.log("network=", networkName);
console.log("sender_btc_address=", btcAddress);
console.log("receiver_btc_address=", receiverAddress);
console.log("transfer_amount=", transferAmount.toString());
console.log("psbt_ready=true");

if (!broadcast) {
  console.log("broadcast=false; no transaction was signed or broadcast");
  process.exit(0);
}

const btcTxId = await withTimeout(
  "Bitcoin Testnet3 sign-and-broadcast",
  btcWallet.signAndBroadcast(psbt),
  120_000,
);
console.log("btc_tx_id=", btcTxId);

const ckbPartialTxInjected = await withTimeout(
  "RGB++ BTC TXID injection",
  rgbppUdtClient.injectTxIdToRgbppCkbTx(indexedCkbPartialTx, btcTxId),
);

const rgbppSignedCkbTx = await withTimeout(
  "RGB++ CKB transaction signing",
  ckbRgbppUnlockSigner.signTransaction(ckbPartialTxInjected),
);

await withTimeout(
  "CKB fee completion",
  rgbppSignedCkbTx.completeFeeBy(ckbSigner),
);

const ckbFinalTx = await withTimeout(
  "Final CKB transaction signing",
  ckbSigner.signTransaction(rgbppSignedCkbTx),
);
const ckbTxId = await withTimeout(
  "CKB Testnet broadcast",
  ckbSigner.client.sendTransaction(ckbFinalTx),
);
await withTimeout(
  "CKB transaction confirmation wait",
  ckbRgbppUnlockSigner.client.waitTransaction(ckbTxId),
  180_000,
);

console.log("ckb_tx_id=", ckbTxId);
