# dilithium-benchmakr

## Bench mark for dilithium2 signatures

`cargo build --release`

## to run

`./target/release/dillith-test.exe <n>` where n is the number of signatures

## For reference, windows test i5/ 5 gen
testing 500 signatures<br/>
Keygen in: 53.05ms<br/>
signed in: 238.42ms<br/>
verified in: 53.30ms<br/>
Elapsed: 345.75ms<br/>

## AWS

testing 500 signatures<br/>
Keygen in: 90.84ms<br/>
signed in: 406.88ms<br/>
verified in: 86.03ms<br/>
Elapsed: 583.80ms<br/>



