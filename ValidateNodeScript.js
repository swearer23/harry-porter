const createHash = require('crypto').createHash;
const createHmac = require('crypto').createHmac;
const salt = 'b6bc6637-98ea-4851-88df-556a3e0871b8'
const hashDigest = createHash('sha256')
  .update('94035458-c07d-4f6f-8a56-ee533b1f06c0')
  .digest()
  .toString('hex')
  .toUpperCase();
console.log('sha256: ', hashDigest);

const hmacDigest = createHmac('sha256', salt)
  .update(hashDigest)
  .digest()
  .toString('base64');

console.log('hmacDigest: ', hmacDigest);