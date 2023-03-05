const fs = require('fs');
console.log(
  String(BigInt(fs.readFileSync('/dev/stdin').toString().trim()) % 20000303n),
);