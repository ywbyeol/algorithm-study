const i = `${require('fs').readFileSync(0)}`.trim().split('\n');
const [N, W, H] = i[0].split(' ');
const ans = [];
for (let j = 1; j <= +N; j++)
  ans.push(+i[j] <= ((+W) ** 2 + (+H) ** 2) ** 0.5 ? 'DA' : 'NE');
console.log(ans.join('\n'));