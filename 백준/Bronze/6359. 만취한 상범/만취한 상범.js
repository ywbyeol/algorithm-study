const [T, ...n] = `${require('fs').readFileSync(0)}`.trim().split('\n');
const ans = [];
for (let i = 0; i < +T; i++) {
  const map = new Map();
  for (let j = 1; j <= +n[i]; j++) map.set(j, j % 2 === 0);
  for (let j = 3; j <= +n[i]; j++)
    for (let k = 1; k <= +n[i]; k++)
      if (k % j === 0) map.set(k, map.get(k) !== true);
  let cnt = 0;
  for (const val of map.values()) if (val === false) cnt++;
  ans.push(cnt);
}
console.log(ans.join('\n'));