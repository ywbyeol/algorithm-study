process.stdin.on('data', i => {
  const [a, b, c] = i.toString().trim().split(/[+=]/).map(Number);
  console.log(a + b === c ? 'YES' : 'NO');
});