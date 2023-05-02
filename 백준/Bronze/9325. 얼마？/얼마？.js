const I = `${require('fs').readFileSync(0)}`.trim().split('\n');
let J = 1;
const ans = [];
for (let i = 0; i < +I[0]; i++) {
  let sum = +I[J++];
  const n = +I[J++];
  for (let j = 0; j < n; j++) {
    const [q, p] = I[J++].split(' ');
    sum += +q * +p;
  }
  ans.push(sum);
}
console.log(ans.join('\n'));