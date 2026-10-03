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

const networkName = (process.env.UTXO_BASED_CHAIN_NAME ??
  "BitcoinTestnet3") as PredefinedNetwork;
if (
  networkName !== PredefinedNetwork.BitcoinTestnet3 &&
  false
) {
  throw new Error("This example only permits Bitcoin Testnet3 or Signet");
}

const ckbPrivateKey = required("CKB_SECP256K1_PRIVATE_KEY");
const btcPrivateKey = required("UTXO_BASED_CHAIN_PRIVATE_KEY");
const btcAddressType = required("UTXO_BASED_CHAIN_ADDRESS_TYPE") as AddressType;
const btcApiUrl = required("BTC_ASSETS_API_URL");
const btcApiToken = required("BTC_ASSETS_API_TOKEN");
const btcApiOrigin = required("BTC_ASSETS_API_ORIGIN");
const receiverAddress = required("RGBPP_RECEIVER_BTC_ADDRESS");
const udtCodeHash = required("UDT_CODE_HASH");
const udtCellDepTxHash = required("UDT_CELL_DEP_TX_HASH");
const udtCellDepIndex = Number(process.env.UDT_CELL_DEP_INDEX ?? "0");
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
if (!/^https:\\/\\//i.test(btcApiUrl)) {
  throw new Error("BTC_ASSETS_API_URL must use HTTPS");
}

if (!Object.values(AddressType).includes(btcAddressType)) {
  throw new Error(`Unsupported BTC address type: ${btcAddressType}`);
}
if (!Number.isInteger(udtCellDepIndex) || udtCellDepIndex < 0) {
  throw new Error("UDT_CELL_DEP_INDEX must be a non-negative integer");
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

const btcAddress = await btcWallet.getAddress();
const ckbRgbppUnlockSigner = new CkbRgbppUnlockSigner({
  ckbClient,
  rgbppBtcAddress: btcAddress,
  btcDataSource: btcWallet,
  scriptInfos: await rgbppUdtClient.getRgbppScriptInfos(),
});

const udt = new ccc.udt.Udt(
  {
    txHash: udtCellDepTxHash,
    index: udtCellDepIndex,
  },
  await ccc.Script.from({
    codeHash: udtCodeHash,
    hashType: process.env.UDT_HASH_TYPE ?? "type",
    args: required("UDT_TYPE_ARGS"),
  }),
);

const pseudoLock = await rgbppUdtClient.buildPseudoRgbppLockScript();

let { res: ckbPartialTx } = await udt.transfer(ckbSigner, [
  {
    to: pseudoLock,
    amount: transferAmount,
  },
]);

ckbPartialTx = await udt.completeChangeToLock(
  ckbPartialTx,
  ckbRgbppUnlockSigner,
  pseudoLock,
);

const { psbt, indexedCkbPartialTx } = await btcWallet.buildPsbt({
  ckbPartialTx,
  ckbClient,
  rgbppUdtClient,
  btcChangeAddress: btcAddress,
  receiverBtcAddresses: [receiverAddress],
  feeRate,
});

console.log("network=", networkName);
console.log("sender_btc_address=", btcAddress);
console.log("receiver_btc_address=", receiverAddress);
console.log("transfer_amount=", transferAmount.toString());
console.log("psbt_ready=true");

if (!broadcast) {
  console.log("broadcast=false; no transaction was signed or broadcast");
  process.exit(0);
}

const btcTxId = await btcWallet.signAndBroadcast(psbt);
console.log("btc_tx_id=", btcTxId);

const ckbPartialTxInjected = await rgbppUdtClient.injectTxIdToRgbppCkbTx(
  indexedCkbPartialTx,
  btcTxId,
);

const rgbppSignedCkbTx =
  await ckbRgbppUnlockSigner.signTransaction(ckbPartialTxInjected);

await rgbppSignedCkbTx.completeFeeBy(ckbSigner);
const ckbFinalTx = await ckbSigner.signTransaction(rgbppSignedCkbTx);
const ckbTxId = await ckbSigner.client.sendTransaction(ckbFinalTx);
await ckbRgbppUnlockSigner.client.waitTransaction(ckbTxId);

console.log("ckb_tx_id=", ckbTxId);
