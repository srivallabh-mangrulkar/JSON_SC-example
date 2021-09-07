# JSON_SC-example

To Deploy the base contract:
  cd contract/json-template
  cargo build --release
  casper-client put-deploy --chain-name=casper-test --node-address <NODE ADDRESS> 
  --secret-key=<PATH TO SECRET KEY>
  --session-path=<PATH TO .wasm FILE>   --payment-amount 3000000000
  
To Deploy caller contract:
  cd contract/json-call/
  cargo build --release
  casper-client put-deploy --chain-name=casper-test --node-address <NODE ADDRESS>
  --secret-key=<PATH TO SECRET KEY>
  --session-path=<PATH TO .wasm FILE>  
  --payment-amount 3000000000 
  --session-arg="data:string='<ARGUMENTS TO PASS>'"
