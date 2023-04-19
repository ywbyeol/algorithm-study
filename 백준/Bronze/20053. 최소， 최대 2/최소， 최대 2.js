const [N, ...i] = `${require('fs').readFileSync(0)}`.trim().split('\n');
const ans = [];
for (let j = 0; j < +N * 2; j += 2) {
  const arr = i[j + 1].split(' ').map(Number);
  ans.push(`${Math.min(...arr)} ${Math.max(...arr)}`);
}
console.log(ans.join('\n'));