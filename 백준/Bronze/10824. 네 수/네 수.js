const i = `${require('fs').readFileSync(0)}`.trim().split(' ');
console.log(+(i[0] + i[1]) + +(i[2] + i[3]));