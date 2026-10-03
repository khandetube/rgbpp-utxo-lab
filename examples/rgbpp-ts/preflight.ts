import "dotenv/config";

import { ccc } from "@ckb-ccc/shell";
import {
  AddressType,
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

async function withTimeout<T>(label: string, operation: Promise<T>, ms = 60_000): Promise<T> {
  let timer: ReturnType<typeof setTimeout> | undefined;
  try {
    return await Promise.race([operation, new Promise<T>((_, reject) => {
      timer = setTimeout(() => reject(new Error(label + " timed out after " + ms + "ms")), ms);
    })]);
  } finally { if (timer) clearTimeout(timer); }
}

function httpsUrl(name: string): string {
  const value = required(name);
  if (!/^https:\/\//i.test(value)) {
    throw new Error(`${name} must use HTTPS`);
  }
  return value;
}

const networkName = (process.env.UTXO_BASED_CHAIN_NAME ??
  "BitcoinTestnet3") as PredefinedNetwork;

if (networkName !== PredefinedNetwork.BitcoinTestnet3) {
  throw new Error("Preflight only permits Bitcoin Testnet3");
}

const btcPrivateKey = required("UTXO_BASED_CHAIN_PRIVATE_KEY");
const btcAddressType = required("UTXO_BASED_CHAIN_ADDRESS_TYPE") as AddressType;
const btcApiUrl = httpsUrl("BTC_ASSETS_API_URL");
const btcApiToken = required("BTC_ASSETS_API_TOKEN");
const btcApiOrigin = required("BTC_ASSETS_API_ORIGIN");
const ckbPrivateKey = required("CKB_SECP256K1_PRIVATE_KEY");
const receiver = required("RGBPP_RECEIVER_BTC_ADDRESS");
const udtTypeArgs = required("UDT_TYPE_ARGS");

if (!Object.values(AddressType).includes(btcAddressType)) {
  throw new Error(`Unsupported BTC address type: ${btcAddressType}`);
}

if (!/^0x[0-9a-f]{64}$/i.test(udtTypeArgs)) {
  throw new Error("UDT_TYPE_ARGS must be a 32-byte 0x-prefixed hex token identifier");
}

if (!/^https:\/\//i.test(btcApiOrigin)) {
  throw new Error("BTC_ASSETS_API_ORIGIN must use HTTPS");
}

const networkConfig = buildNetworkConfig(networkName);
const ckbClient = new ccc.ClientPublicTestnet();

const tip = await withTimeout("CKB testnet tip query", ckbClient.getTip(), 45_000);
if (tip < 0n) throw new Error("Invalid CKB testnet tip");

const scriptProvider = createScriptProvider(ckbClient);
const rgbppUdtClient = new RgbppUdtClient(
  networkConfig,
  ckbClient,
  scriptProvider,
);

const xuDtScriptInfo = await withTimeout("CKB KnownScript.XUdt resolution", ckbClient.getKnownScript(ccc.KnownScript.XUdt), 45_000);
if (!xuDtScriptInfo.cellDeps.length) {
  throw new Error("CKB KnownScript.XUdt returned no cell dependency");
}

const btcWallet = new PrivateKeyRgbppBtcWallet(
  btcPrivateKey,
  btcAddressType,
  networkConfig,
  { url: btcApiUrl, token: btcApiToken, origin: btcApiOrigin },
);

const btcAddress = await withTimeout("BTC address derivation", btcWallet.getAddress(), 45_000);
const rgbppScriptInfos = await withTimeout("RGB++ script info resolution", rgbppUdtClient.getRgbppScriptInfos(), 45_000);

console.log("preflight=ok");
console.log("network=", networkName);
console.log("ckb_testnet_tip=", tip.toString());
console.log("btc_sender_address=", btcAddress);
console.log("receiver_btc_address=", receiver);
console.log("udt_type_args_bytes=", ((udtTypeArgs.length - 2) / 2).toString());
console.log("xu_dt_cell_dep=ok");
console.log("rgbpp_script_infos=ok");
console.log("api_url_https=ok");
console.log("secrets=loaded");
console.log("broadcast=not_performed");
