const n = `${require('fs').readFileSync(0)}`.trim().split('\n');
let a = 0;
for (let i = 0; i < 10 && a !== 100; i++)
  if (Math.abs(a + +n[i] - 100) <= Math.abs(a - 100)) a += +n[i];
  else break;
console.log(a);