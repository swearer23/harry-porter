<script setup lang="ts">
import { ref } from 'vue'
import axios, {AxiosResponse}  from 'axios'
import sha256 from 'crypto-js/sha256';
import hmacSHA512 from 'crypto-js/hmac-sha512';
import Base64 from 'crypto-js/enc-base64';
import { v4 as uuidv4 } from 'uuid';

type EncryptedParams = {
  name: string;
  age: number;
  breed: string;
  ts: number;
  sign?: string;
}

type SignatureInfo = {
  cnonce: string;
  flag: string;
  sign: string;
}

defineProps<{ msg: string }>()

const count = ref(0)

const randomNumber = () => {
  const privateKey = 91
  const salt = Math.floor(Math.random() * 100)
  return {
    salt,
    pubkey: (salt * privateKey).toString(16)
  }
}

const encryptedSign = (message: string): SignatureInfo => {
  const cnonce = uuidv4()
  const {salt, pubkey} = randomNumber()
  console.log("salt is ", salt)
  const hashDigest = sha256(cnonce + '123456');
  const hmacDigest = Base64.stringify(hmacSHA512(hashDigest, salt.toString()));
  return {
    cnonce: cnonce,
    sign: hmacDigest,
    flag: pubkey
  }
}

const onClick = () => {
  const params: EncryptedParams = {
    name: 'John',
    age: 23,
    breed: 'dog',
    ts: Date.now()
  }
  const signatureInfo = encryptedSign(JSON.stringify(params))
  console.log(signatureInfo)
  axios.post('http://localhost:3000/test', {...params, ...signatureInfo}).then((res: AxiosResponse) => {
    console.log(res.data)
    count.value++
  })
}
</script>

<template>
  <h1>{{ msg }}</h1>

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

  <button type="button" @click="onClick">count is: {{ count }}</button>
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
