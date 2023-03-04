process.stdin.on('data', i => {
  console.log(
    [...Array(i.toString().trim().split('\n').length - 1).keys()]
      .map(j => `Case ${j + 1}: Sorting... done!`)
      .join('\n'),
  );
});