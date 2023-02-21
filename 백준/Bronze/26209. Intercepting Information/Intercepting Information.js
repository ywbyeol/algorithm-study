process.stdin.on('data', i => {
  console.log(
    i.toString().trim().split(' ').map(Number).includes(9) ? 'F' : 'S',
  );
});