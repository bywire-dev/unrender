# field/top

[← back to REPORT.md](../REPORT.md)

truth source: `none`

content preservation: **71%** (chrome-excluded; 1321 non-blank content cells)

## Sizes

| encoding | bytes | tokens | vs plain (bytes) | vs plain (tokens) |
|---|---:|---:|---:|---:|
| raw | 3832 | 2148 | 0.63x | 0.58x |
| plain | 2409 | 1251 | 1.00x | 1.00x |
| compact | 8607 | 5039 | 0.28x | 0.25x |
| toon | 1875 | 1271 | 1.28x | 0.98x |
| json | 65356 | 16388 | 0.04x | 0.08x |

<details><summary>Before — raw ANSI (3832 bytes, escapes shown literally)</summary>

```
\x1b[mtop - 11:33:41 up 33 min,  0 user,  load average: 1.17, 0.78, 0.87
\x1b[mTasks:\x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  93 \x1b[mtotal,\x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m   2 \x1b[mrunning,\x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  91 \x1b[msleeping,\x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m   0 \x1b[mstopped,\x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m   0 \x1b[mzombie
\x1b[m%Cpu(s):\x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m 23.8 \x1b[mus,\x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  9.9 \x1b[msy,\x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  0.0 \x1b[mni,\x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m 66.2 \x1b[mid,\x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  0.0 \x1b[mwa,\x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  0.0 \x1b[mhi,\x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  0.0 \x1b[msi,\x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  0.0 \x1b[mst
\x1b[mMiB Mem :\x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  16075.4 \x1b[mtotal,\x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  12851.8 \x1b[mfree,\x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m    817.7 \x1b[mused,\x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m   2727.0 \x1b[mbuff/cache
\x1b[mMiB Swap:\x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m      0.0 \x1b[mtotal,\x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m      0.0 \x1b[mfree,\x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m      0.0 \x1b[mused.\x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  15257.7 \x1b[mavail Mem

\x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[7m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m  PID USER      PR  NI    VIRT    RES    SHR S  %CPU  %MEM     TIME+ COMMAND
\x1b[m24737 root      20   0  338980  37072  10348 S 119.0   0.2   2:15.48 python3
\x1b[m  566 root      20   0 6013732 432244 188284 S   3.6   2.6   1:50.55 claude
\x1b[m27734 root      20   0 1562472  72376  36944 S   2.4   0.4   0:00.20 zellij
\x1b[m    1 root      20   0   21760   5588   2788 S   1.2   0.0   0:08.92 process_api
\x1b[m    2 root      20   0       0      0      0 S   0.0   0.0   0:00.01 kthreadd
\x1b[m    3 root      20   0       0      0      0 S   0.0   0.0   0:00.00 pool_workqueue_release
\x1b[m    4 root       0 -20       0      0      0 I   0.0   0.0   0:00.00 kworker/R-rcu_gp
\x1b[m    5 root       0 -20       0      0      0 I   0.0   0.0   0:00.00 kworker/R-sync_wq
\x1b[m    6 root       0 -20       0      0      0 I   0.0   0.0   0:00.00 kworker/R-kvfree_rcu_reclaim
\x1b[m    7 root       0 -20       0      0      0 I   0.0   0.0   0:00.00 kworker/R-slub_flushwq
\x1b[m    8 root       0 -20       0      0      0 I   0.0   0.0   0:00.00 kworker/R-netns
\x1b[m   10 root       0 -20       0      0      0 I   0.0   0.0   0:00.00 kworker/0:0H-events_highpri
\x1b[m   13 root       0 -20       0      0      0 I   0.0   0.0   0:00.00 kworker/R-mm_percpu_wq
\x1b[m   14 root      20   0       0      0      0 S   0.0   0.0   0:00.05 ksoftirqd/0
\x1b[m   15 root      20   0       0      0      0 I   0.0   0.0   0:00.73 rcu_preempt
\x1b[m   16 root      20   0       0      0      0 S   0.0   0.0   0:00.00 rcu_exp_par_gp_kthread_worker+
\x1b[m   17 root      20   0       0      0      0 S   0.0   0.0   0:00.00 rcu_exp_gp_kthread_worker
\x1b[m   18 root      rt   0       0      0      0 S   0.0   0.0   0:00.03 migration/0
\x1b[m   19 root      20   0       0      0      0 S   0.0   0.0   0:00.00 cpuhp/0
\x1b[m   20 root      20   0       0      0      0 S   0.0   0.0   0:00.00 cpuhp/1
\x1b[m   21 root      rt   0       0      0      0 S   0.0   0.0   0:00.50 migration/1
\x1b[m   22 root      20   0       0      0      0 S   0.0   0.0   0:00.03 ksoftirqd/1
\x1b[m   23 root      20   0       0      0      0 I   0.0   0.0   0:00.00 kworker/1:0-mm_percpu_wq\x1b[m
```

</details>

## Before (color-stripped)

```
top - 11:33:41 up 33 min,  0 user,  load average: 1.17, 0.78, 0.87
Tasks:  93 total,   2 running,  91 sleeping,   0 stopped,   0 zombie
%Cpu(s): 23.8 us,  9.9 sy,  0.0 ni, 66.2 id,  0.0 wa,  0.0 hi,  0.0 si,  0.0 st
MiB Mem :  16075.4 total,  12851.8 free,    817.7 used,   2727.0 buff/cache
MiB Swap:      0.0 total,      0.0 free,      0.0 used.  15257.7 avail Mem

  PID USER      PR  NI    VIRT    RES    SHR S  %CPU  %MEM     TIME+ COMMAND
24737 root      20   0  338980  37072  10348 S 119.0   0.2   2:15.48 python3
  566 root      20   0 6013732 432244 188284 S   3.6   2.6   1:50.55 claude
27734 root      20   0 1562472  72376  36944 S   2.4   0.4   0:00.20 zellij
    1 root      20   0   21760   5588   2788 S   1.2   0.0   0:08.92 process_api
    2 root      20   0       0      0      0 S   0.0   0.0   0:00.01 kthreadd
    3 root      20   0       0      0      0 S   0.0   0.0   0:00.00 pool_workqueue_release
    4 root       0 -20       0      0      0 I   0.0   0.0   0:00.00 kworker/R-rcu_gp
    5 root       0 -20       0      0      0 I   0.0   0.0   0:00.00 kworker/R-sync_wq
    6 root       0 -20       0      0      0 I   0.0   0.0   0:00.00 kworker/R-kvfree_rcu_reclaim
    7 root       0 -20       0      0      0 I   0.0   0.0   0:00.00 kworker/R-slub_flushwq
    8 root       0 -20       0      0      0 I   0.0   0.0   0:00.00 kworker/R-netns
   10 root       0 -20       0      0      0 I   0.0   0.0   0:00.00 kworker/0:0H-events_highpri
   13 root       0 -20       0      0      0 I   0.0   0.0   0:00.00 kworker/R-mm_percpu_wq
   14 root      20   0       0      0      0 S   0.0   0.0   0:00.05 ksoftirqd/0
   15 root      20   0       0      0      0 I   0.0   0.0   0:00.73 rcu_preempt
   16 root      20   0       0      0      0 S   0.0   0.0   0:00.00 rcu_exp_par_gp_kthread_worker+
   17 root      20   0       0      0      0 S   0.0   0.0   0:00.00 rcu_exp_gp_kthread_worker
   18 root      rt   0       0      0      0 S   0.0   0.0   0:00.03 migration/0
   19 root      20   0       0      0      0 S   0.0   0.0   0:00.00 cpuhp/0
   20 root      20   0       0      0      0 S   0.0   0.0   0:00.00 cpuhp/1
   21 root      rt   0       0      0      0 S   0.0   0.0   0:00.50 migration/1
   22 root      20   0       0      0      0 S   0.0   0.0   0:00.03 ksoftirqd/1
   23 root      20   0       0      0      0 I   0.0   0.0   0:00.00 kworker/1:0-mm_percpu_wq
```

## After — compact

```
application @0,0,99,30
  list "properties" @0,0,99,5
    property "top - 11" @0,0,99,1 33:41 up 33 min,  0 user,  load average: 1.17, 0.78, 0.87
    property "Tasks" @0,1,99,1 93 total,   2 running,  91 sleeping,   0 stopped,   0 zombie
    property "%Cpu(s)" @0,2,99,1 23.8 us,  9.9 sy,  0.0 ni, 66.2 id,  0.0 wa,  0.0 hi,  0.0 si,  0.0 st
    property "MiB Mem" @0,3,99,1 16075.4 total,  12851.8 free,    817.7 used,   2727.0 buff/cache
    property "MiB Swap" @0,4,99,1 0.0 total,      0.0 free,      0.0 used.  15257.7 avail Mem
  table @0,6,99,24
    row [selected] @0,6,99,1
      cell @2,6,3,1 PID
      cell @6,6,4,1 USER
      cell @16,6,2,1 PR
      cell @19,6,3,1 NI
      cell @24,6,6,1 VIRT
      cell @32,6,5,1 RES
      cell @39,6,5,1 SHR
      cell @45,6,1,1 S
      cell @49,6,3,1 CPU
      cell @55,6,3,1 MEM
      cell @61,6,7,1 TIME+
      cell @69,6,27,1 COMMAND
    row @0,7,99,1
      cell @2,7,3,1 737
      cell @6,7,4,1 root
      cell @16,7,2,1 20
      cell @19,7,3,1 0
      cell @24,7,6,1 338980
      cell @32,7,5,1 37072
      cell @39,7,5,1 10348
      cell @45,7,1,1 S
      cell @49,7,3,1 9.0
      cell @55,7,3,1 0.2
      cell @61,7,7,1 2:15.48
      cell @69,7,27,1 python3
    row @0,8,99,1
      cell @2,8,3,1 566
      cell @6,8,4,1 root
      cell @16,8,2,1 20
      cell @19,8,3,1 0
      cell @24,8,6,1 013732
      cell @32,8,5,1 32244
      cell @39,8,5,1 88284
      cell @45,8,1,1 S
      cell @49,8,3,1 3.6
      cell @55,8,3,1 2.6
      cell @61,8,7,1 1:50.55
      cell @69,8,27,1 claude
    row @0,9,99,1
      cell @2,9,3,1 734
      cell @6,9,4,1 root
      cell @16,9,2,1 20
      cell @19,9,3,1 0
      cell @24,9,6,1 562472
      cell @32,9,5,1 72376
      cell @39,9,5,1 36944
      cell @45,9,1,1 S
      cell @49,9,3,1 2.4
      cell @55,9,3,1 0.4
      cell @61,9,7,1 0:00.20
      cell @69,9,27,1 zellij
    row @0,10,99,1
      cell @2,10,3,1 1
      cell @6,10,4,1 root
      cell @16,10,2,1 20
      cell @19,10,3,1 0
      cell @24,10,6,1 21760
      cell @32,10,5,1 5588
      cell @39,10,5,1 2788
      cell @45,10,1,1 S
      cell @49,10,3,1 1.2
      cell @55,10,3,1 0.0
      cell @61,10,7,1 0:08.92
      cell @69,10,27,1 process_api
    row @0,11,99,1
      cell @2,11,3,1 2
      cell @6,11,4,1 root
      cell @16,11,2,1 20
      cell @19,11,3,1 0
      cell @24,11,6,1 0
      cell @32,11,5,1 0
      cell @39,11,5,1 0
      cell @45,11,1,1 S
      cell @49,11,3,1 0.0
      cell @55,11,3,1 0.0
      cell @61,11,7,1 0:00.01
      cell @69,11,27,1 kthreadd
    row @0,12,99,1
      cell @2,12,3,1 3
      cell @6,12,4,1 root
      cell @16,12,2,1 20
      cell @19,12,3,1 0
      cell @24,12,6,1 0
      cell @32,12,5,1 0
      cell @39,12,5,1 0
      cell @45,12,1,1 S
      cell @49,12,3,1 0.0
      cell @55,12,3,1 0.0
      cell @61,12,7,1 0:00.00
      cell @69,12,27,1 pool_workqueue_release
    row @0,13,99,1
      cell @2,13,3,1 4
      cell @6,13,4,1 root
      cell @16,13,2,1 0
      cell @19,13,3,1 -20
      cell @24,13,6,1 0
      cell @32,13,5,1 0
      cell @39,13,5,1 0
      cell @45,13,1,1 I
      cell @49,13,3,1 0.0
      cell @55,13,3,1 0.0
      cell @61,13,7,1 0:00.00
      cell @69,13,27,1 kworker/R-rcu_gp
    row @0,14,99,1
      cell @2,14,3,1 5
      cell @6,14,4,1 root
      cell @16,14,2,1 0
      cell @19,14,3,1 -20
      cell @24,14,6,1 0
      cell @32,14,5,1 0
      cell @39,14,5,1 0
      cell @45,14,1,1 I
      cell @49,14,3,1 0.0
      cell @55,14,3,1 0.0
      cell @61,14,7,1 0:00.00
      cell @69,14,27,1 kworker/R-sync_wq
    row @0,15,99,1
      cell @2,15,3,1 6
      cell @6,15,4,1 root
      cell @16,15,2,1 0
      cell @19,15,3,1 -20
      cell @24,15,6,1 0
      cell @32,15,5,1 0
      cell @39,15,5,1 0
      cell @45,15,1,1 I
      cell @49,15,3,1 0.0
      cell @55,15,3,1 0.0
      cell @61,15,7,1 0:00.00
      cell @69,15,27,1 kworker/R-kvfree_rcu_reclai
    row @0,16,99,1
      cell @2,16,3,1 7
      cell @6,16,4,1 root
      cell @16,16,2,1 0
      cell @19,16,3,1 -20
      cell @24,16,6,1 0
      cell @32,16,5,1 0
      cell @39,16,5,1 0
      cell @45,16,1,1 I
      cell @49,16,3,1 0.0
      cell @55,16,3,1 0.0
      cell @61,16,7,1 0:00.00
      cell @69,16,27,1 kworker/R-slub_flushwq
    row @0,17,99,1
      cell @2,17,3,1 8
      cell @6,17,4,1 root
      cell @16,17,2,1 0
      cell @19,17,3,1 -20
      cell @24,17,6,1 0
      cell @32,17,5,1 0
      cell @39,17,5,1 0
      cell @45,17,1,1 I
      cell @49,17,3,1 0.0
      cell @55,17,3,1 0.0
      cell @61,17,7,1 0:00.00
      cell @69,17,27,1 kworker/R-netns
    row @0,18,99,1
      cell @2,18,3,1 10
      cell @6,18,4,1 root
      cell @16,18,2,1 0
      cell @19,18,3,1 -20
      cell @24,18,6,1 0
      cell @32,18,5,1 0
      cell @39,18,5,1 0
      cell @45,18,1,1 I
      cell @49,18,3,1 0.0
      cell @55,18,3,1 0.0
      cell @61,18,7,1 0:00.00
      cell @69,18,27,1 kworker/0:0H-events_highpri
    row @0,19,99,1
      cell @2,19,3,1 13
      cell @6,19,4,1 root
      cell @16,19,2,1 0
      cell @19,19,3,1 -20
      cell @24,19,6,1 0
      cell @32,19,5,1 0
      cell @39,19,5,1 0
      cell @45,19,1,1 I
      cell @49,19,3,1 0.0
      cell @55,19,3,1 0.0
      cell @61,19,7,1 0:00.00
      cell @69,19,27,1 kworker/R-mm_percpu_wq
    row @0,20,99,1
      cell @2,20,3,1 14
      cell @6,20,4,1 root
      cell @16,20,2,1 20
      cell @19,20,3,1 0
      cell @24,20,6,1 0
      cell @32,20,5,1 0
      cell @39,20,5,1 0
      cell @45,20,1,1 S
      cell @49,20,3,1 0.0
      cell @55,20,3,1 0.0
      cell @61,20,7,1 0:00.05
      cell @69,20,27,1 ksoftirqd/0
    row @0,21,99,1
      cell @2,21,3,1 15
      cell @6,21,4,1 root
      cell @16,21,2,1 20
      cell @19,21,3,1 0
      cell @24,21,6,1 0
      cell @32,21,5,1 0
      cell @39,21,5,1 0
      cell @45,21,1,1 I
      cell @49,21,3,1 0.0
      cell @55,21,3,1 0.0
      cell @61,21,7,1 0:00.73
      cell @69,21,27,1 rcu_preempt
    row @0,22,99,1
      cell @2,22,3,1 16
      cell @6,22,4,1 root
      cell @16,22,2,1 20
      cell @19,22,3,1 0
      cell @24,22,6,1 0
      cell @32,22,5,1 0
      cell @39,22,5,1 0
      cell @45,22,1,1 S
      cell @49,22,3,1 0.0
      cell @55,22,3,1 0.0
      cell @61,22,7,1 0:00.00
      cell @69,22,27,1 rcu_exp_par_gp_kthread_work
    row @0,23,99,1
      cell @2,23,3,1 17
      cell @6,23,4,1 root
      cell @16,23,2,1 20
      cell @19,23,3,1 0
      cell @24,23,6,1 0
      cell @32,23,5,1 0
      cell @39,23,5,1 0
      cell @45,23,1,1 S
      cell @49,23,3,1 0.0
      cell @55,23,3,1 0.0
      cell @61,23,7,1 0:00.00
      cell @69,23,27,1 rcu_exp_gp_kthread_worker
    row @0,24,99,1
      cell @2,24,3,1 18
      cell @6,24,4,1 root
      cell @16,24,2,1 rt
      cell @19,24,3,1 0
      cell @24,24,6,1 0
      cell @32,24,5,1 0
      cell @39,24,5,1 0
      cell @45,24,1,1 S
      cell @49,24,3,1 0.0
      cell @55,24,3,1 0.0
      cell @61,24,7,1 0:00.03
      cell @69,24,27,1 migration/0
    row @0,25,99,1
      cell @2,25,3,1 19
      cell @6,25,4,1 root
      cell @16,25,2,1 20
      cell @19,25,3,1 0
      cell @24,25,6,1 0
      cell @32,25,5,1 0
      cell @39,25,5,1 0
      cell @45,25,1,1 S
      cell @49,25,3,1 0.0
      cell @55,25,3,1 0.0
      cell @61,25,7,1 0:00.00
      cell @69,25,27,1 cpuhp/0
    row @0,26,99,1
      cell @2,26,3,1 20
      cell @6,26,4,1 root
      cell @16,26,2,1 20
      cell @19,26,3,1 0
      cell @24,26,6,1 0
      cell @32,26,5,1 0
      cell @39,26,5,1 0
      cell @45,26,1,1 S
      cell @49,26,3,1 0.0
      cell @55,26,3,1 0.0
      cell @61,26,7,1 0:00.00
      cell @69,26,27,1 cpuhp/1
    row @0,27,99,1
      cell @2,27,3,1 21
      cell @6,27,4,1 root
      cell @16,27,2,1 rt
      cell @19,27,3,1 0
      cell @24,27,6,1 0
      cell @32,27,5,1 0
      cell @39,27,5,1 0
      cell @45,27,1,1 S
      cell @49,27,3,1 0.0
      cell @55,27,3,1 0.0
      cell @61,27,7,1 0:00.50
      cell @69,27,27,1 migration/1
    row @0,28,99,1
      cell @2,28,3,1 22
      cell @6,28,4,1 root
      cell @16,28,2,1 20
      cell @19,28,3,1 0
      cell @24,28,6,1 0
      cell @32,28,5,1 0
      cell @39,28,5,1 0
      cell @45,28,1,1 S
      cell @49,28,3,1 0.0
      cell @55,28,3,1 0.0
      cell @61,28,7,1 0:00.03
      cell @69,28,27,1 ksoftirqd/1
    row @0,29,99,1
      cell @2,29,3,1 23
      cell @6,29,4,1 root
      cell @16,29,2,1 20
      cell @19,29,3,1 0
      cell @24,29,6,1 0
      cell @32,29,5,1 0
      cell @39,29,5,1 0
      cell @45,29,1,1 I
      cell @49,29,3,1 0.0
      cell @55,29,3,1 0.0
      cell @61,29,7,1 0:00.00
      cell @69,29,27,1 kworker/1:0-mm_percpu_wq
```

## After — toon

```
application
  list properties[5]:
     33:41 up 33 min,  0 user,  load average: 1.17, 0.78, 0.87
     93 total,   2 running,  91 sleeping,   0 stopped,   0 zombie
     23.8 us,  9.9 sy,  0.0 ni, 66.2 id,  0.0 wa,  0.0 hi,  0.0 si,  0.0 st
     16075.4 total,  12851.8 free,    817.7 used,   2727.0 buff/cache
     0.0 total,      0.0 free,      0.0 used.  15257.7 avail Mem
  table [24]{c0,c1,c2,c3,c4,c5,c6,c7,c8,c9,c10,c11}:
    *PID,USER,PR,NI,VIRT,RES,SHR,S,CPU,MEM,TIME+,COMMAND
     737,root,20,0,338980,37072,10348,S,9.0,0.2,2:15.48,python3
     566,root,20,0,013732,32244,88284,S,3.6,2.6,1:50.55,claude
     734,root,20,0,562472,72376,36944,S,2.4,0.4,0:00.20,zellij
     1,root,20,0,21760,5588,2788,S,1.2,0.0,0:08.92,process_api
     2,root,20,0,0,0,0,S,0.0,0.0,0:00.01,kthreadd
     3,root,20,0,0,0,0,S,0.0,0.0,0:00.00,pool_workqueue_release
     4,root,0,-20,0,0,0,I,0.0,0.0,0:00.00,kworker/R-rcu_gp
     5,root,0,-20,0,0,0,I,0.0,0.0,0:00.00,kworker/R-sync_wq
     6,root,0,-20,0,0,0,I,0.0,0.0,0:00.00,kworker/R-kvfree_rcu_reclai
     7,root,0,-20,0,0,0,I,0.0,0.0,0:00.00,kworker/R-slub_flushwq
     8,root,0,-20,0,0,0,I,0.0,0.0,0:00.00,kworker/R-netns
     10,root,0,-20,0,0,0,I,0.0,0.0,0:00.00,kworker/0:0H-events_highpri
     13,root,0,-20,0,0,0,I,0.0,0.0,0:00.00,kworker/R-mm_percpu_wq
     14,root,20,0,0,0,0,S,0.0,0.0,0:00.05,ksoftirqd/0
     15,root,20,0,0,0,0,I,0.0,0.0,0:00.73,rcu_preempt
     16,root,20,0,0,0,0,S,0.0,0.0,0:00.00,rcu_exp_par_gp_kthread_work
     17,root,20,0,0,0,0,S,0.0,0.0,0:00.00,rcu_exp_gp_kthread_worker
     18,root,rt,0,0,0,0,S,0.0,0.0,0:00.03,migration/0
     19,root,20,0,0,0,0,S,0.0,0.0,0:00.00,cpuhp/0
     20,root,20,0,0,0,0,S,0.0,0.0,0:00.00,cpuhp/1
     21,root,rt,0,0,0,0,S,0.0,0.0,0:00.50,migration/1
     22,root,20,0,0,0,0,S,0.0,0.0,0:00.03,ksoftirqd/1
     23,root,20,0,0,0,0,I,0.0,0.0,0:00.00,kworker/1:0-mm_percpu_wq
```

<details><summary>After — json</summary>

```json
{
  "role": "application",
  "rect": [
    0,
    0,
    99,
    30
  ],
  "children": [
    {
      "role": "list",
      "name": "properties",
      "rect": [
        0,
        0,
        99,
        5
      ],
      "children": [
        {
          "role": "property",
          "name": "top - 11",
          "value": "33:41 up 33 min,  0 user,  load average: 1.17, 0.78, 0.87",
          "rect": [
            0,
            0,
            99,
            1
          ]
        },
        {
          "role": "property",
          "name": "Tasks",
          "value": "93 total,   2 running,  91 sleeping,   0 stopped,   0 zombie",
          "rect": [
            0,
            1,
            99,
            1
          ]
        },
        {
          "role": "property",
          "name": "%Cpu(s)",
          "value": "23.8 us,  9.9 sy,  0.0 ni, 66.2 id,  0.0 wa,  0.0 hi,  0.0 si,  0.0 st",
          "rect": [
            0,
            2,
            99,
            1
          ]
        },
        {
          "role": "property",
          "name": "MiB Mem",
          "value": "16075.4 total,  12851.8 free,    817.7 used,   2727.0 buff/cache",
          "rect": [
            0,
            3,
            99,
            1
          ]
        },
        {
          "role": "property",
          "name": "MiB Swap",
          "value": "0.0 total,      0.0 free,      0.0 used.  15257.7 avail Mem",
          "rect": [
            0,
            4,
            99,
            1
          ]
        }
      ]
    },
    {
      "role": "table",
      "rect": [
        0,
        6,
        99,
        24
      ],
      "children": [
        {
          "role": "row",
          "rect": [
            0,
            6,
            99,
            1
          ],
          "states": [
            "selected"
          ],
          "children": [
            {
              "role": "cell",
              "value": "PID",
              "rect": [
                2,
                6,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "USER",
              "rect": [
                6,
                6,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "PR",
              "rect": [
                16,
                6,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "NI",
              "rect": [
                19,
                6,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "VIRT",
              "rect": [
                24,
                6,
                6,
                1
              ]
            },
            {
              "role": "cell",
              "value": "RES",
              "rect": [
                32,
                6,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "SHR",
              "rect": [
                39,
                6,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "S",
              "rect": [
                45,
                6,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "CPU",
              "rect": [
                49,
                6,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "MEM",
              "rect": [
                55,
                6,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "TIME+",
              "rect": [
                61,
                6,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "COMMAND",
              "rect": [
                69,
                6,
                27,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            7,
            99,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "737",
              "rect": [
                2,
                7,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "root",
              "rect": [
                6,
                7,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "20",
              "rect": [
                16,
                7,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                19,
                7,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "338980",
              "rect": [
                24,
                7,
                6,
                1
              ]
            },
            {
              "role": "cell",
              "value": "37072",
              "rect": [
                32,
                7,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "10348",
              "rect": [
                39,
                7,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "S",
              "rect": [
                45,
                7,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "9.0",
              "rect": [
                49,
                7,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.2",
              "rect": [
                55,
                7,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "2:15.48",
              "rect": [
                61,
                7,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "python3",
              "rect": [
                69,
                7,
                27,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            8,
            99,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "566",
              "rect": [
                2,
                8,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "root",
              "rect": [
                6,
                8,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "20",
              "rect": [
                16,
                8,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                19,
                8,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "013732",
              "rect": [
                24,
                8,
                6,
                1
              ]
            },
            {
              "role": "cell",
              "value": "32244",
              "rect": [
                32,
                8,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "88284",
              "rect": [
                39,
                8,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "S",
              "rect": [
                45,
                8,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "3.6",
              "rect": [
                49,
                8,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "2.6",
              "rect": [
                55,
                8,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "1:50.55",
              "rect": [
                61,
                8,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "claude",
              "rect": [
                69,
                8,
                27,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            9,
            99,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "734",
              "rect": [
                2,
                9,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "root",
              "rect": [
                6,
                9,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "20",
              "rect": [
                16,
                9,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                19,
                9,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "562472",
              "rect": [
                24,
                9,
                6,
                1
              ]
            },
            {
              "role": "cell",
              "value": "72376",
              "rect": [
                32,
                9,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "36944",
              "rect": [
                39,
                9,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "S",
              "rect": [
                45,
                9,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "2.4",
              "rect": [
                49,
                9,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.4",
              "rect": [
                55,
                9,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0:00.20",
              "rect": [
                61,
                9,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "zellij",
              "rect": [
                69,
                9,
                27,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            10,
            99,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "1",
              "rect": [
                2,
                10,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "root",
              "rect": [
                6,
                10,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "20",
              "rect": [
                16,
                10,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                19,
                10,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "21760",
              "rect": [
                24,
                10,
                6,
                1
              ]
            },
            {
              "role": "cell",
              "value": "5588",
              "rect": [
                32,
                10,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "2788",
              "rect": [
                39,
                10,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "S",
              "rect": [
                45,
                10,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "1.2",
              "rect": [
                49,
                10,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.0",
              "rect": [
                55,
                10,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0:08.92",
              "rect": [
                61,
                10,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "process_api",
              "rect": [
                69,
                10,
                27,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            11,
            99,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "2",
              "rect": [
                2,
                11,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "root",
              "rect": [
                6,
                11,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "20",
              "rect": [
                16,
                11,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                19,
                11,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                24,
                11,
                6,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                32,
                11,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                39,
                11,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "S",
              "rect": [
                45,
                11,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.0",
              "rect": [
                49,
                11,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.0",
              "rect": [
                55,
                11,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0:00.01",
              "rect": [
                61,
                11,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "kthreadd",
              "rect": [
                69,
                11,
                27,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            12,
            99,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "3",
              "rect": [
                2,
                12,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "root",
              "rect": [
                6,
                12,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "20",
              "rect": [
                16,
                12,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                19,
                12,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                24,
                12,
                6,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                32,
                12,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                39,
                12,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "S",
              "rect": [
                45,
                12,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.0",
              "rect": [
                49,
                12,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.0",
              "rect": [
                55,
                12,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0:00.00",
              "rect": [
                61,
                12,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "pool_workqueue_release",
              "rect": [
                69,
                12,
                27,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            13,
            99,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "4",
              "rect": [
                2,
                13,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "root",
              "rect": [
                6,
                13,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                16,
                13,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "-20",
              "rect": [
                19,
                13,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                24,
                13,
                6,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                32,
                13,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                39,
                13,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "I",
              "rect": [
                45,
                13,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.0",
              "rect": [
                49,
                13,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.0",
              "rect": [
                55,
                13,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0:00.00",
              "rect": [
                61,
                13,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "kworker/R-rcu_gp",
              "rect": [
                69,
                13,
                27,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            14,
            99,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "5",
              "rect": [
                2,
                14,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "root",
              "rect": [
                6,
                14,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                16,
                14,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "-20",
              "rect": [
                19,
                14,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                24,
                14,
                6,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                32,
                14,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                39,
                14,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "I",
              "rect": [
                45,
                14,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.0",
              "rect": [
                49,
                14,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.0",
              "rect": [
                55,
                14,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0:00.00",
              "rect": [
                61,
                14,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "kworker/R-sync_wq",
              "rect": [
                69,
                14,
                27,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            15,
            99,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "6",
              "rect": [
                2,
                15,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "root",
              "rect": [
                6,
                15,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                16,
                15,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "-20",
              "rect": [
                19,
                15,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                24,
                15,
                6,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                32,
                15,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                39,
                15,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "I",
              "rect": [
                45,
                15,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.0",
              "rect": [
                49,
                15,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.0",
              "rect": [
                55,
                15,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0:00.00",
              "rect": [
                61,
                15,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "kworker/R-kvfree_rcu_reclai",
              "rect": [
                69,
                15,
                27,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            16,
            99,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "7",
              "rect": [
                2,
                16,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "root",
              "rect": [
                6,
                16,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                16,
                16,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "-20",
              "rect": [
                19,
                16,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                24,
                16,
                6,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                32,
                16,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                39,
                16,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "I",
              "rect": [
                45,
                16,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.0",
              "rect": [
                49,
                16,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.0",
              "rect": [
                55,
                16,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0:00.00",
              "rect": [
                61,
                16,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "kworker/R-slub_flushwq",
              "rect": [
                69,
                16,
                27,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            17,
            99,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "8",
              "rect": [
                2,
                17,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "root",
              "rect": [
                6,
                17,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                16,
                17,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "-20",
              "rect": [
                19,
                17,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                24,
                17,
                6,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                32,
                17,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                39,
                17,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "I",
              "rect": [
                45,
                17,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.0",
              "rect": [
                49,
                17,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.0",
              "rect": [
                55,
                17,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0:00.00",
              "rect": [
                61,
                17,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "kworker/R-netns",
              "rect": [
                69,
                17,
                27,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            18,
            99,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "10",
              "rect": [
                2,
                18,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "root",
              "rect": [
                6,
                18,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                16,
                18,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "-20",
              "rect": [
                19,
                18,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                24,
                18,
                6,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                32,
                18,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                39,
                18,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "I",
              "rect": [
                45,
                18,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.0",
              "rect": [
                49,
                18,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.0",
              "rect": [
                55,
                18,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0:00.00",
              "rect": [
                61,
                18,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "kworker/0:0H-events_highpri",
              "rect": [
                69,
                18,
                27,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            19,
            99,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "13",
              "rect": [
                2,
                19,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "root",
              "rect": [
                6,
                19,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                16,
                19,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "-20",
              "rect": [
                19,
                19,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                24,
                19,
                6,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                32,
                19,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                39,
                19,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "I",
              "rect": [
                45,
                19,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.0",
              "rect": [
                49,
                19,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.0",
              "rect": [
                55,
                19,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0:00.00",
              "rect": [
                61,
                19,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "kworker/R-mm_percpu_wq",
              "rect": [
                69,
                19,
                27,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            20,
            99,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "14",
              "rect": [
                2,
                20,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "root",
              "rect": [
                6,
                20,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "20",
              "rect": [
                16,
                20,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                19,
                20,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                24,
                20,
                6,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                32,
                20,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                39,
                20,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "S",
              "rect": [
                45,
                20,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.0",
              "rect": [
                49,
                20,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.0",
              "rect": [
                55,
                20,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0:00.05",
              "rect": [
                61,
                20,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "ksoftirqd/0",
              "rect": [
                69,
                20,
                27,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            21,
            99,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "15",
              "rect": [
                2,
                21,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "root",
              "rect": [
                6,
                21,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "20",
              "rect": [
                16,
                21,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                19,
                21,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                24,
                21,
                6,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                32,
                21,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                39,
                21,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "I",
              "rect": [
                45,
                21,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.0",
              "rect": [
                49,
                21,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.0",
              "rect": [
                55,
                21,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0:00.73",
              "rect": [
                61,
                21,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "rcu_preempt",
              "rect": [
                69,
                21,
                27,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            22,
            99,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "16",
              "rect": [
                2,
                22,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "root",
              "rect": [
                6,
                22,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "20",
              "rect": [
                16,
                22,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                19,
                22,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                24,
                22,
                6,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                32,
                22,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                39,
                22,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "S",
              "rect": [
                45,
                22,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.0",
              "rect": [
                49,
                22,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.0",
              "rect": [
                55,
                22,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0:00.00",
              "rect": [
                61,
                22,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "rcu_exp_par_gp_kthread_work",
              "rect": [
                69,
                22,
                27,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            23,
            99,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "17",
              "rect": [
                2,
                23,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "root",
              "rect": [
                6,
                23,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "20",
              "rect": [
                16,
                23,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                19,
                23,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                24,
                23,
                6,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                32,
                23,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                39,
                23,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "S",
              "rect": [
                45,
                23,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.0",
              "rect": [
                49,
                23,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.0",
              "rect": [
                55,
                23,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0:00.00",
              "rect": [
                61,
                23,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "rcu_exp_gp_kthread_worker",
              "rect": [
                69,
                23,
                27,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            24,
            99,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "18",
              "rect": [
                2,
                24,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "root",
              "rect": [
                6,
                24,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "rt",
              "rect": [
                16,
                24,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                19,
                24,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                24,
                24,
                6,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                32,
                24,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                39,
                24,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "S",
              "rect": [
                45,
                24,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.0",
              "rect": [
                49,
                24,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.0",
              "rect": [
                55,
                24,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0:00.03",
              "rect": [
                61,
                24,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "migration/0",
              "rect": [
                69,
                24,
                27,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            25,
            99,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "19",
              "rect": [
                2,
                25,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "root",
              "rect": [
                6,
                25,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "20",
              "rect": [
                16,
                25,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                19,
                25,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                24,
                25,
                6,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                32,
                25,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                39,
                25,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "S",
              "rect": [
                45,
                25,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.0",
              "rect": [
                49,
                25,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.0",
              "rect": [
                55,
                25,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0:00.00",
              "rect": [
                61,
                25,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "cpuhp/0",
              "rect": [
                69,
                25,
                27,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            26,
            99,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "20",
              "rect": [
                2,
                26,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "root",
              "rect": [
                6,
                26,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "20",
              "rect": [
                16,
                26,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                19,
                26,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                24,
                26,
                6,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                32,
                26,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                39,
                26,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "S",
              "rect": [
                45,
                26,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.0",
              "rect": [
                49,
                26,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.0",
              "rect": [
                55,
                26,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0:00.00",
              "rect": [
                61,
                26,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "cpuhp/1",
              "rect": [
                69,
                26,
                27,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            27,
            99,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "21",
              "rect": [
                2,
                27,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "root",
              "rect": [
                6,
                27,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "rt",
              "rect": [
                16,
                27,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                19,
                27,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                24,
                27,
                6,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                32,
                27,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                39,
                27,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "S",
              "rect": [
                45,
                27,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.0",
              "rect": [
                49,
                27,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.0",
              "rect": [
                55,
                27,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0:00.50",
              "rect": [
                61,
                27,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "migration/1",
              "rect": [
                69,
                27,
                27,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            28,
            99,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "22",
              "rect": [
                2,
                28,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "root",
              "rect": [
                6,
                28,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "20",
              "rect": [
                16,
                28,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                19,
                28,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                24,
                28,
                6,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                32,
                28,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                39,
                28,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "S",
              "rect": [
                45,
                28,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.0",
              "rect": [
                49,
                28,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.0",
              "rect": [
                55,
                28,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0:00.03",
              "rect": [
                61,
                28,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "ksoftirqd/1",
              "rect": [
                69,
                28,
                27,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            29,
            99,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "23",
              "rect": [
                2,
                29,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "root",
              "rect": [
                6,
                29,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "20",
              "rect": [
                16,
                29,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                19,
                29,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                24,
                29,
                6,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                32,
                29,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                39,
                29,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "I",
              "rect": [
                45,
                29,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.0",
              "rect": [
                49,
                29,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.0",
              "rect": [
                55,
                29,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0:00.00",
              "rect": [
                61,
                29,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "kworker/1:0-mm_percpu_wq",
              "rect": [
                69,
                29,
                27,
                1
              ]
            }
          ]
        }
      ]
    }
  ]
}
```

</details>

