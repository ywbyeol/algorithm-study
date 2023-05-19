let [A, B] = `${require('fs').readFileSync(0)}`.trim().split(' ');
A = A.replace(/6/g, '5');
B = B.replace(/6/g, '5');
const min = +A + +B;
A = A.replace(/5/g, '6');
B = B.replace(/5/g, '6');
const max = +A + +B;
console.log(min, max);