console.log(
  ` @@@   @@@ 
@   @ @   @
@    @    @
@         @
 @       @ 
  @     @  
   @   @   
    @ @    
     @     
`
    .repeat(Number(require('fs').readFileSync('/dev/stdin', 'utf8').trim()))
    .replace(/\n$/, ''),
);