.PHONY: challenge1 challenge2 challenge3 challenge4 challenge5 challenge6 challenge7 challenge8 challenge9 challenge10 test10 challenge11 challenge12 challenge13 challenge14 challenge15 challenge16 challenge17 challenge18 challenge19 challenge20 challenge21 challenge22 challenge23 challenge24 challenge25single challenge25 first25

# Variable so you can easily replace it with another path
bitcrack = ./target/release/bitcrack

challenge1:
	$(bitcrack) -o challenge1.txt --keyspace 1:1 1BgGZ9tcN4rm9KBzDn7KprQz87SZ26SAMH

challenge2:
	$(bitcrack) -o challenge2.txt --keyspace 2:3 1CUNEBjYrCn2y1SdiUMohaKUi4wpP326Lb

challenge3:
	$(bitcrack) -o challenge3.txt --keyspace 4:7 19ZewH8Kk1PDbSNdJ97FP4EiCjTRaZMZQA

challenge4:
	$(bitcrack) -o challenge4.txt --keyspace 8:f 1EhqbyUMvvs7BfL8goY6qcPbD6YKfPqb7e

challenge5:
	$(bitcrack) -o challenge5.txt --keyspace 10:1f 1E6NuFjCi27W5zoXg8TRdcSRq84zJeBW3k

challenge6:
	$(bitcrack) -o challenge6.txt --keyspace 20:3f 1PitScNLyp2HCygzadCh7FveTnfmpPbfp8

challenge7:
	$(bitcrack) -o challenge7.txt --keyspace 40:7f 1McVt1vMtCC7yn5b9wgX1833yCcLXzueeC

challenge8:
	$(bitcrack) -o challenge8.txt --keyspace 80:ff 1M92tSqNmQLYw33fuBvjmeadirh1ysMBxK

challenge9:
	$(bitcrack) -o challenge9.txt --keyspace 100:1ff 1CQFwcjw1dwhtkVWBttNLDtqL7ivBonGPV

challenge10:
	$(bitcrack) -o challenge10.txt --keyspace 200:3ff 1LeBZP5QCwwgXRtmVUvTVrraqPUokyLHqe

test10:
	$(bitcrack) -o challenge10.txt --keyspace 200:3ff -p 03a7a4c30291ac1db24b4ab00c442aa832f7794b5a0959bec6e8d7fee802289dcd

challenge11:
	$(bitcrack) -o challenge11.txt --keyspace 400:7ff 1PgQVLmst3Z314JrQn5TNiys8Hc38TcXJu

challenge12:
	$(bitcrack) -o challenge12.txt --keyspace 800:fff 1DBaumZxUkM4qMQRt2LVWyFJq5kDtSZQot

challenge13:
	$(bitcrack) -o challenge13.txt --keyspace 1000:1fff 1Pie8JkxBT6MGPz9Nvi3fsPkr2D8q3GBc1

challenge14:
	$(bitcrack) -o challenge14.txt --keyspace 2000:3fff 1ErZWg5cFCe4Vw5BzgfzB74VNLaXEiEkhk

challenge15:
	$(bitcrack) -o challenge15.txt --keyspace 4000:7fff 1QCbW9HWnwQWiQqVo5exhAnmfqKRrCRsvW

challenge16:
	$(bitcrack) -o challenge16.txt --keyspace 8000:ffff 1BDyrQ6WoF8VN3g9SAS1iKZcPzFfnDVieY

challenge17:
	$(bitcrack) -o challenge17.txt --keyspace 10000:1ffff 1HduPEXZRdG26SUT5Yk83mLkPyjnZuJ7Bm

challenge18:
	$(bitcrack) -o challenge18.txt --keyspace 20000:3ffff 1GnNTmTVLZiqQfLbAdp9DVdicEnB5GoERE

challenge19:
	$(bitcrack) -o challenge19.txt --keyspace 40000:7ffff 1NWmZRpHH4XSPwsW6dsS3nrNWfL1yrJj4w

challenge20:
	$(bitcrack) -o challenge20.txt --keyspace 80000:fffff 1HsMJxNiV7TLxmoF6uJNkydxPFDog4NQum

challenge21:
	$(bitcrack) -o challenge21.txt --keyspace 100000:1fffff 14oFNXucftsHiUMY8uctg6N487riuyXs4h

challenge22:
	$(bitcrack) -o challenge22.txt --keyspace 200000:3fffff 1CfZWK1QTQE3eS9qn61dQjV89KDjZzfNcv

challenge23:
	$(bitcrack) -o challenge23.txt --keyspace 400000:7fffff 1L2GM8eE7mJWLdo3HZS6su1832NX2txaac

challenge24:
	$(bitcrack) -o challenge24.txt --keyspace 800000:ffffff 1rSnXMr63jdCuegJFuidJqWxUPV7AtUf7

challenge25single:
	$(bitcrack) -o challenge25.txt --keyspace 1000000:1ffffff -t 1 15JhYXn6Mx3oF4Y7PcTAv2wVVAuCFFQNiP

challenge25: # Takes my laptop 38 seconds; 430.000 attempts per second
	$(bitcrack) -o challenge25.txt --keyspace 1000000:1ffffff -t 10 15JhYXn6Mx3oF4Y7PcTAv2wVVAuCFFQNiP

test25: # Takes my laptop 35 seconds; 480.000 attempts per second
	$(bitcrack) -o challenge25.txt --keyspace 1000000:1ffffff -t 10 -p 03057fbea3a2623382628dde556b2a0698e32428d3cd225f3bd034dca82dd7455a 

# Done instantly on my laptop
first10: challenge1 challenge2 challenge3 challenge4 challenge5 challenge6 challenge7 challenge8 challenge9 challenge10

# First25 takes my laptop 70 seconds
first25: first10 challenge11 challenge12 challenge13 challenge14 challenge15 challenge16 challenge17 challenge18 challenge19 challenge20 challenge21 challenge22 challenge23 challenge24 challenge25

challenge26: # Takes my laptop 54 seconds
	$(bitcrack) -o challenge26.txt --keyspace 2000000:3ffffff 1JVnST957hGztonaWK6FougdtjxzHzRMMg
	
challenge27: # Takes my laptop 119 seconds
	$(bitcrack) -o challenge27.txt --keyspace 4000000:7ffffff 128z5d7nN7PkCuX5qoA4Ys6pmxUYnEy86k

challenge28: # Takes my laptop 270 seconds
	$(bitcrack) -o challenge28.txt --keyspace 8000000:fffffff 12jbtzBb54r97TCwW3G1gCFoumpckRAPdY

challenge29:
	$(bitcrack) -o challenge29.txt --keyspace 10000000:1fffffff 19EEC52krRUK1RkUAEZmQdjTyHT7Gp1TYT

challenge30:
	$(bitcrack) -o challenge30.txt --keyspace 20000000:3fffffff 1LHtnpd8nU5VHEMkG2TMYYNUjjLc992bps

challenge31:
	$(bitcrack) -o challenge31.txt --keyspace 40000000:7fffffff 1LhE6sCTuGae42Axu1L1ZB7L96yi9irEBE

challenge65:
	$(bitcrack) -o challenge65.txt --keyspace 1a838b04ab606c000:1a838b1f835306000 18ZMbwUFLMHoZBbfpCjUJQTCMCbktshgpe

challenge66:
	$(bitcrack) -o challenge66.txt --keyspace 20000000000000000:3ffffffffffffffff 13zb1hQbWVsc2S7ZTZnP2G4undNNpdh5so