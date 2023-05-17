const I = `${require('fs').readFileSync(0)}`.trim().replace(/\s+/g, '');
const map = new Map();
let maxCnt = 0;
for (let i = 0; i < I.length; i++) {
  const char = I[i];
  if (map.has(char)) map.set(char, map.get(char) + 1);
  else map.set(char, 1);
  maxCnt = Math.max(maxCnt, map.get(char));
}
const ans = [];
map.forEach((cnt, char) => {
  if (cnt === maxCnt) ans.push(char);
});
console.log(ans.sort().join(''));