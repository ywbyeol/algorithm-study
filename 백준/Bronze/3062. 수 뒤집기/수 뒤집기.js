const [T, ...inp] = `${require('fs').readFileSync(0)}`.trim().split('\n');
const ans = [];
for (let i = 0; i < +T; i++) {
  const sum = `${+inp[i] + +inp[i].split('').reverse().join('')}`;
  ans.push(sum === [...sum].reverse().join('') ? 'YES' : 'NO');
}
console.log(ans.join('\n'));