const i=`${require('fs').readFileSync(0)}`.trim()
console.log(i==='1 2 3 4 5 6 7 8'?'ascending':i==='8 7 6 5 4 3 2 1'?'descending':'mixed')