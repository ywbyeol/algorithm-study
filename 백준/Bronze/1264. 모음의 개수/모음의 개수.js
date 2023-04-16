const input = `${require('fs').readFileSync(0)}`
  .trim()
  .split('\n')
  .slice(0, -1);
const arr = ['a', 'e', 'i', 'o', 'u'];
for (let i = 0; i < input.length; i++) {
  let ans = 0;
  const str = input[i].toLowerCase();
  for (let j = 0; j < str.length; j++) {
    if (arr.includes(str[j])) {
      ans++;
    }
  }
  console.log(ans);
}