<script setup lang="ts">
import { ref, onMounted } from 'vue'
import axios, {AxiosResponse}  from 'axios'
import sha256 from 'crypto-js/sha256';
import hmacSHA512 from 'crypto-js/hmac-sha512';
import Base64 from 'crypto-js/enc-base64';
import { v4 as uuidv4 } from 'uuid';

import init, {ron_weasley_sign} from "./pkg/ron_weasley.js";

type EncryptedParams = {
  name: string;
  age: number;
  breed: string;
  ts: number;
  sign?: string;
}

defineProps<{ msg: string }>()

const signatureInfo = ref(<String[]>[]);

const encryptedSign = (message: string, cnonce: string): string => {
  const secret = '123456' // replace this with your own secret paired with wasm file
  const hashDigest = sha256(`${cnonce}|${message}`)
  const hmacDigest = Base64.stringify(hmacSHA512(hashDigest.toString().toUpperCase(), secret))
  return hmacDigest.toString().toUpperCase()
}

onMounted(async () => {
  await init()
})

const onClick = () => {
  signatureInfo.value = []
  const params: EncryptedParams = {
    name: 'John',
    age: 23,
    breed: 'dog',
    ts: Date.now()
  }
  const cnonce = uuidv4()
  signatureInfo.value.push(`cnonce: ${cnonce}`)
  const cSignature= encryptedSign("hello world", cnonce)
  signatureInfo.value.push(`cSignature: ${cSignature}`)

  async function runWasm () {
    const wasmSignature = ron_weasley_sign("hello world", cnonce)
    signatureInfo.value.push(`wasmSignature: ${wasmSignature}`)
    if (wasmSignature != cSignature) {
      signatureInfo.value.push(`wasm signature is not equal to c signature`)
    } else {
      signatureInfo.value.push(`wasm signature is equal to c signature`) 
    }
  }
  runWasm()
}
</script>

<template>
  <h1>{{ msg }}</h1>

  <template v-for="item in signatureInfo">
    <p>{{ item }}</p>
  </template>
  <p>
    Recommended IDE setup:
    <a href="https://code.visualstudio.com/" target="_blank">VSCode</a>
    +
    <a href="https://github.com/johnsoncodehk/volar" target="_blank">Volar</a>
  </p>

  <p>See <code>README.md</code> for more information.</p>

  <p>
    <a href="https://vitejs.dev/guide/features.html" target="_blank">
      Vite Docs
    </a>
    |
    <a href="https://v3.vuejs.org/" target="_blank">Vue 3 Docs</a>
  </p>

  <button type="button" @click="onClick">click me for signature</button>
  <p>
    Edit
    <code>components/HelloWorld.vue</code> to test hot module replacement.
  </p>
</template>

<style scoped>
a {
  color: #42b983;
}

label {
  margin: 0 0.5em;
  font-weight: bold;
}

code {
  background-color: #eee;
  padding: 2px 4px;
  border-radius: 4px;
  color: #304455;
}
</style>
