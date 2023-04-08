const I = `${require('fs').readFileSync('/dev/stdin')}`.trim().split(' ');
const N = I[0];
const B = +I[1];
let res = 0;
for (let i = 0; i < N.length; i++) {
  const digit = Number.isNaN(+N[i]) ? +N[i].charCodeAt(0) - 55 : +N[i];
  res += digit * B ** (N.length - i - 1);
}
console.log(res);