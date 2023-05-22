const N = +`${require('fs').readFileSync(0)}`.trim();
const arr = Array(N * 2).fill('* '.repeat(Math.round(N / 2)).trim());
for (let i = 0; i < arr.length; i++) {
  if (i % 2 !== 0) {
    let str = ` ${arr[i]}`;
    if (str.length > N) str = str.slice(0, -2);
    arr[i] = str;
  }
}
console.log(arr.join('\n'));