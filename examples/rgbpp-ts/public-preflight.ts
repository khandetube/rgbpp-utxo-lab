import { ccc } from "@ckb-ccc/shell";
import {
  RgbppUdtClient,
  buildNetworkConfig,
  PredefinedNetwork,
  createScriptProvider,
} from "@ckb-ccc/rgbpp";

const network = PredefinedNetwork.BitcoinTestnet3;
const ckbClient = new ccc.ClientPublicTestnet();

async function withTimeout<T>(
  label: string,
  operation: Promise<T>,
  ms = 45_000,
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

const tip = await withTimeout("CKB getTip", ckbClient.getTip());
if (tip < 0n) throw new Error("Invalid CKB testnet tip");

const xuDtScriptInfo = await withTimeout(
  "CKB KnownScript.XUdt resolution",
  ckbClient.getKnownScript(ccc.KnownScript.XUdt),
);
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
const rgbppScriptInfos = await withTimeout(
  "RGB++ script info resolution",
  rgbppUdtClient.getRgbppScriptInfos(),
);

const rgbppScriptNames = Object.keys(rgbppScriptInfos);
if (rgbppScriptNames.length === 0) {
  throw new Error("RGB++ script info resolution returned no scripts");
}

console.log("public_preflight=ok");
console.log("network=", network);
console.log("ckb_testnet_tip=", tip.toString());
console.log("xu_dt_cell_deps=", xuDtScriptInfo.cellDeps.length);
console.log("rgbpp_script_infos=", rgbppScriptNames.join(","));
