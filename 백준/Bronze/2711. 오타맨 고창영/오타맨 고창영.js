const ans = [];
const [T, ...input] = `${require('fs').readFileSync(0)}`.trim().split('\n');
for (let i = 0; i < +T; i++) {
  const [I, W] = input[i].trim().split(' ');
  ans.push(W.slice(0, +I - 1) + W.slice(+I));
}
console.log(ans.join('\n'));