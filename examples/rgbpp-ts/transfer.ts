import 'dotenv/config';
import { buildRgbppLockArgs, Collector } from 'rgbpp/ckb';
import { buildRgbppTransferTx } from 'rgbpp';
import { AddressType, BtcAssetsApi, DataSource, NetworkType, bitcoin, ECPair } from 'rgbpp/btc';
import { remove0x } from 'rgbpp/btc';

const required = (name: string): string => {
  const value = process.env[name];
  if (!value) throw new Error(`Missing environment variable: ${name}`);
  return value;
};

const isMainnet = process.env.IS_MAINNET === 'true';
const networkType = isMainnet ? NetworkType.MAINNET : NetworkType.TESTNET;
const testnetType = required('BTC_TESTNET_TYPE');

const btcPrivateKey = Buffer.from(remove0x(required('BTC_PRIVATE_KEY')), 'hex');
const addressType = process.env.BTC_ADDRESS_TYPE === 'P2TR' ? AddressType.P2TR : AddressType.P2WPKH;
const btcNetwork = networkType === NetworkType.MAINNET ? bitcoin.networks.bitcoin : bitcoin.networks.testnet;
const keyPair = ECPair.fromPrivateKey(btcPrivateKey, { network: btcNetwork });

const payment = addressType === AddressType.P2TR
  ? bitcoin.payments.p2tr({ internalPubkey: keyPair.publicKey.slice(1, 33), network: btcNetwork })
  : bitcoin.payments.p2wpkh({ pubkey: keyPair.publicKey, network: btcNetwork });

const fromAddress = payment.address;
if (!fromAddress) throw new Error('Could not derive BTC sender address');

const collector = new Collector({
  ckbNodeUrl: required('CKB_NODE_URL'),
  ckbIndexerUrl: required('CKB_INDEXER_URL'),
});

const service = BtcAssetsApi.fromToken(
  required('BTC_SERVICE_URL'),
  required('BTC_SERVICE_TOKEN'),
  required('BTC_SERVICE_ORIGIN'),
);
const dataSource = new DataSource(service, networkType);

const rgbppLockArgsList = required('RGBPP_LOCK_ARGS')
  .split(',')
  .map((value) => value.trim())
  .filter(Boolean);

const xudtTypeArgs = required('XUDT_TYPE_ARGS');
const toBtcAddress = required('TO_BTC_ADDRESS');
const transferAmount = BigInt(required('TRANSFER_AMOUNT'));

const result = await buildRgbppTransferTx({
  ckb: {
    collector,
    xudtTypeArgs,
    rgbppLockArgsList,
    transferAmount,
  },
  btc: {
    fromAddress,
    toAddress: toBtcAddress,
    fromPubkey: addressType === AddressType.P2TR ? keyPair.publicKey.toString('hex') : undefined,
    dataSource,
    testnetType,
  },
  isMainnet,
});

console.log(JSON.stringify({
  fromAddress,
  toBtcAddress,
  btcPsbtHex: result.btcPsbtHex,
  commitment: result.ckbVirtualTxResult.commitment,
}, null, 2));

// Deliberately stop before signing/broadcasting.
// The PSBT must be independently reviewed before a private key is used.
