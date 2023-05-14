const i = `${require('fs').readFileSync(0)}`.trim().split(/\s+/g);
const ans = [];
for (let j = 1; j <= +i[0] * 2; j += 2) {
  let cnt = 0;
  for (let k = 0; k < i[j].length; k++) if (i[j][k] !== i[j + 1][k]) cnt++;
  ans.push(`Hamming distance is ${cnt}.`);
}
console.log(ans.join('\n'));