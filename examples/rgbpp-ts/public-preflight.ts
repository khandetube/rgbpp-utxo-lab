import { ccc } from "@ckb-ccc/shell";
import {
  RgbppUdtClient,
  buildNetworkConfig,
  PredefinedNetwork,
  createScriptProvider,
} from "@ckb-ccc/rgbpp";

const network = PredefinedNetwork.BitcoinTestnet3;
const ckbClient = new ccc.ClientPublicTestnet();

const tip = await ckbClient.getTip();
if (tip < 0n) throw new Error("Invalid CKB testnet tip");

const xuDtScriptInfo = await ckbClient.getKnownScript(ccc.KnownScript.XUdt);
if (xuDtScriptInfo.cellDeps.length === 0) {
  throw new Error("CKB KnownScript.XUdt returned no cell dependency");
}

const networkConfig = buildNetworkConfig(network);
const scriptProvider = createScriptProvider(ckbClient);
const rgbppUdtClient = new RgbppUdtClient(
  networkConfig,
  ckbClient,
  scriptProvider,
);
const rgbppScriptInfos = await rgbppUdtClient.getRgbppScriptInfos();

if (rgbppScriptInfos.length === 0) {
  throw new Error("RGB++ script info resolution returned no scripts");
}

console.log("public_preflight=ok");
console.log("network=", network);
console.log("ckb_testnet_tip=", tip.toString());
console.log("xu_dt_cell_deps=", xuDtScriptInfo.cellDeps.length);
console.log("rgbpp_script_infos=", rgbppScriptInfos.length);
