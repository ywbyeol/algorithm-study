const [n, ...arr] = `${require('fs').readFileSync(0)}`
  .trim()
  .split(/\s+/g)
  .map(Number);
let ans = 0;
let max = 0;
while (arr.length > 0) {
  const item = arr.pop();
  if (item > max) {
    max = item;
    ans++;
  }
}
console.log(ans);