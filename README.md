# compio-signal
IPC bench using compio-signal to wait for signal asynchronously

Usage:
```
./run [COUNT]
```

## Unix signal

```
============ RESULTS ================
Message count:      1
Total duration:     0.000 ms
Average duration:   672.000 us
Minimum duration:   672.000 us
Maximum duration:   672.000 us

============ RESULTS ================
Message count:      10
Total duration:     1.000 ms
Average duration:   153.000 us
Minimum duration:   16.000 us
Maximum duration:   639.000 us

============ RESULTS ================
Message count:      100
Total duration:     5.000 ms
Average duration:   55.000 us
Minimum duration:   25.000 us
Maximum duration:   443.000 us

============ RESULTS ================
Message size:       1
Message count:      1000
Total duration:     26.000 ms
Average duration:   26.000 us
Minimum duration:   20.000 us
Maximum duration:   1083.000 us
```

## Linux signal with IO uring
```
============ RESULTS ================
Message size:       1
Message count:      1
Total duration:     0.000 ms
Average duration:   340.000 us
Minimum duration:   340.000 us
Maximum duration:   340.000 us

============ RESULTS ================
Message size:       1
Message count:      10
Total duration:     3.000 ms
Average duration:   315.000 us
Minimum duration:   142.000 us
Maximum duration:   560.000 us

============ RESULTS ================
Message size:       1
Message count:      100
Total duration:     4.000 ms
Average duration:   49.000 us
Minimum duration:   19.000 us
Maximum duration:   502.000 us

============ RESULTS ================
Message size:       1
Message count:      1000
Total duration:     110.000 ms
Average duration:   110.000 us
Minimum duration:   63.000 us
Maximum duration:   478.000 us
```