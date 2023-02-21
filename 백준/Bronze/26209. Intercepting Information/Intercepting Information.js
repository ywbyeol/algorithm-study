process.stdin.on('data', i => {
  console.log(i.toString().trim().includes('9') ? 'F' : 'S');
});