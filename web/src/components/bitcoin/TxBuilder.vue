<script setup lang="ts">
import Frame from "./Frame.vue";
import TxInCard from "./TxInCard.vue";
import {computed, Ref, ref, watch} from "vue";
import {
  CHECK_DIGITS,
  defaultTxIn,
  defaultTxOut,
  GLOBAL_NETWORK,
  NetworkType,
  Transaction,
  TxIn,
  TxOut,
  updateNetwork
} from "../../bitcoin.ts";
import {ArrowForward as Arrow} from '@vicons/ionicons5';
import TxOutCard from "./TxOutCard.vue";
import {safeParseInt, useWasm} from "../../lib.ts";
import {useRoute, useRouter} from "vue-router";
import ImportTxModal from "./ImportTxModal.vue";

let wasm = useWasm();
let router = useRouter();
let route = useRoute();

let networkOptions: { label: string, value: NetworkType }[] = [
  {value: 'bitcoin', label: 'Bitcoin'},
  {value: 'testnet', label: 'Testnet3'},
  {value: 'testnet4', label: 'Testnet4'},
  {value: 'sigtest', label: 'Sigtest'},
  {value: 'regtest', label: 'Regtest'},
];
let selectedNetwork = ref<NetworkType>('bitcoin');

let version = ref(1);
let lockTime = ref(0);

let txIns: Ref<TxIn[]> = ref([]);
let txOuts: Ref<TxOut[]> = ref([]);

let transaction = computed<Transaction>(() => {
  return {
    version: version.value,
    lockTime: lockTime.value,
    in: txIns.value,
    out: txOuts.value,
  };
});

let transactionHex = computed(() => {
  try {
    return wasm.TxBuilder.json_to_tx_hex(JSON.stringify(transaction.value));
  } catch (e: any) {
    return e.toString();
  }
});

watch([transaction], () => {
  console.log(transaction.value);
  router.replace({
    ...router.currentRoute.value,
    query: {
      tx: JSON.stringify(transaction.value),
      network: GLOBAL_NETWORK,
    }
  });
}, {deep: true});

function updateTransaction(json: string) {
  console.log(json);
  let tx = JSON.parse(json) as Transaction;
  console.log(tx);
  version.value = tx.version;
  lockTime.value = tx.lockTime;
  txIns.value = tx.in;
  txOuts.value = tx.out;
}

let txQuery = route.query['tx'] as string | undefined;
let networkQuery = route.query['network'] as string | undefined;
if (txQuery) {
  updateTransaction(txQuery);
  updateNetwork(networkQuery as NetworkType);
}

let showModal = ref({
  importTx: false,
});

type OptionsButtonKey = 'import' | 'miscellaneous';
let optionsButtonDropdown: { key: OptionsButtonKey, label: string }[] = [
  {key: 'import', label: 'Import'},
  {key: 'miscellaneous', label: 'Miscellaneous'},
]

function optionsButtonOnSelect(key: OptionsButtonKey) {
  switch (key) {
    case "import":
      showModal.value.importTx = true;
      break;
    case "miscellaneous":
      router.push('/misc');
      break;
  }
}
</script>

<template>
  <div id="page-options">
    <n-dropdown trigger="click" @select="optionsButtonOnSelect" :options="optionsButtonDropdown">
      <n-button style="margin: 10px;" tag="a" text>Options</n-button>
    </n-dropdown>
  </div>

  <ImportTxModal v-model:show="showModal.importTx" @result="x => updateTransaction(x)"/>
  <div id="root-TxBuilder">
    <Frame title="Transaction" title-adjust="center" title-size="large">
      <n-form label-placement="top" inline>
        <n-form-item label="Version" style="margin: 0; padding: 0">
          <n-input placeholder="" size="small" style="min-width: 10em" autosize
                   :allow-input="CHECK_DIGITS"
                   :value="`${version}`"
                   @update:value="x => version = safeParseInt(x)"/>
        </n-form-item>
        <n-form-item label="LockTime" style="margin: 0; padding: 0">
          <n-input placeholder="" size="small" style="min-width: 10em" autosize
                   :allow-input="CHECK_DIGITS"
                   :value="`${lockTime}`"
                   @update:value="x => lockTime = safeParseInt(x)"/>
        </n-form-item>
        <n-form-item label="Network" style="margin: 0; padding: 0">
          <n-select :options="networkOptions" v-model:value="selectedNetwork"
                    size="small" style="min-width: 10em"
                    @update:value="x => updateNetwork(x)"
          />
        </n-form-item>
      </n-form>
      <div id="txs-div">
        <div>
          <TxInCard v-for="(_, index) in txIns" v-model:value="txIns[index]"
                    @close="txIns.splice(index, 1)" :index="index"
                    :tx="transaction"
          />
          <n-button type="primary" @click="txIns.push(defaultTxIn())" secondary>Add
          </n-button>
        </div>
        <div>
          <Arrow v-if="txIns.length !== 0 && txOuts.length !== 0"/>
        </div>
        <div>
          <TxOutCard v-for="(_, index) in txOuts" v-model:value="txOuts[index]"
                     @close="txOuts.splice(index, 1)" :index="index"
          />
          <n-button type="primary" @click="txOuts.push(defaultTxOut())" secondary>Add</n-button>
        </div>
      </div>
    </Frame>
    <div style="margin-top: 1em">Transaction:</div>
    <pre id="tx-output">{{ JSON.stringify(transaction, null, 2) }}</pre>
    <span>Consensus Encoded</span>
    <n-input type="textarea" rows="10" :value="transactionHex"/>
  </div>
</template>

<style scoped>
#root-TxBuilder {
  padding: 3px;
}

#txs-div {
  display: flex;
  justify-content: space-evenly;

  > *:nth-child(2) {
    width: 8em;
    //border-left: 1px solid black;
    //border-right: 1px solid black;
    display: inline-flex;
    align-items: center;
    text-align: center;
    padding: 0 .25em;
  }

  > *:nth-child(1), > *:nth-child(3) {
    width: 100%;
  }
}

#tx-output {
  margin: 0;
}

.center {
  display: flex;
  justify-content: center;
}

#page-options {
  position: fixed;
  left: 100%;
  transform: translateX(-100%);
  margin: 0;
  padding: 0;
}
</style>
