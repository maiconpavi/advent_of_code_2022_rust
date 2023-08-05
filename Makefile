
test-all:
	cargo insta test  --review --release -- --nocapture 

test-last:
	cargo insta test  --review --release -- tests::test_last_day --nocapture --exact 

