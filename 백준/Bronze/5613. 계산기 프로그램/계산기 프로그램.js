const i = `${require('fs').readFileSync(0)}`.trim().split(/\s+/g).slice(0, -1);
let ans = i.shift();
do {
  ans = Math.floor(eval(`${ans} ${i.splice(0, 2).join(' ')}`));
} while (i.length > 0);
console.log(ans);