# WDEPC

Western Digital EPC(Extended Power Condition) control tools for Linux.

This tool is only tested on Western Digital `HC320` disk, but may work in other Western Digital disk supported `EPC`.

All query function is 99% safe, unless your disk translate those command into some wrong `WRITE` command.

**USE AT YOUR RISK.**


## What is EPC (Extended Power Condition)

### APM - Advanced Power Management
There was a power management mechanism called `APM` - `Advanced Power Management` in the late 90s.

It is supported by almost all hard drives.

APM defines a `APM Levels` from 0 - 255.

| level | description |
| --- | --- |
| 0 | Reserved |
| 1 | Minimum power consumption with Standby |
| 2 - 127 | Intermediate power management levels with Standby |
| 128 | Minimum power consumption without Standby |
| 129 - 254 | Intermediate power management levels without Standby |
| 254 | Maximum performance |
| 255 | Reserved |

where `standby` means spin down.


### EPC - Extended Power Condition
This is the latest power management standard in hard drives, it's usually supported on enterprise-grade hard drives (some newer hard drives don't support APM, EPC is used exclusively).

EPC defines two main state:

1. PM1: Idle state
   1. **Idle_a**: drive ready, not performing I/O; drive may power down some eletronics to reduce power without increasing response time.
   2. **Idle_b**: spindle rotation at 7200 with heads unloaded.
   3. **Idle_c**: spindle rotation at low RPM with heads unloaded.
2. PM2: Standby state
   1. **Standby_y**: same as Idle_c in Seagate and WD
   2. **Standby_Z**: Actuator is unloaded and spindle motor is stopped. Commands can be received immediately.

a SATA state `sleep`, same as Standby_z but require soft reset or hard reset to return to mode Standby_Z.

Current tools like `hdparm` can not update EPC settings like timer, enable or disable, so this tool came up.

## Usage

### Check Power Mode
get current power mode

```wdepc -d /dev/sda check```

Output:
```
idle a
````

### Enable EPC
Enable EPC and disable APM.

**The APM is disabled automatically and can not be controlled.**
```shell
wdepc -d /dev/sda enable
```

### Disable EPC
Disable EPC, but **doesn't re-enable APM**.

You must enable APM **MANUALLY** on demand.
```shell
wdepc -d /dev/sda disable
```

### Show EPC settings
Show EPC settings, include timer, state

```shell
wdepc -d /dev/sda info
```

Output:
```shell
* = enabled
All times are in 100 milliseconds

Name       Current Timer Default Timer Saved Timer Recovery Time Changeable Savable
Idle A     *20           *20           *20         1             true       true
Idle B     *6000         *6000         *6000       10            true       true
Idle C     0             0             0           40            true       true
Standby Y  0             0             0           40            true       true
Standby Z  0             0             0           150           true       true
```

### Force device goto a state
```shell
wdepc -d /dev/sda set <idle_a | idle_b | idle_c | standby_y | standby_z >
```

### Set timer
Set specific mode timer.

```shell
wdepc -d /dev/sda set-timer <mode> <timer> --save --enable true
```

If `--save` present, save the timer setting even after reboot.

`--enable` controls if the timer is enabled.

### Set state
Enable or disable a specific mode.

```shell
wdepc -d /dev/sda set-state <mode> --save --enable true
```

If `--save` present, save the state setting even after reboot.

`--enable` controls if the power state is enabled.

### Restore settings
Restore a specific power mode setting.

```shell
wdepc -d /dev/sda restore -d -s <mode>
```

If `--default` present, set current setting to default, else set current setting to saved setting.

If `--save` present, save current setting.

# Reference
1. [HC320 SATA spec](https://documents.westerndigital.com/content/dam/doc-library/en_us/assets/public/western-digital/product/data-center-drives/ultrastar-dc-hc300-series/product-manual-ultrastar-dc-hc320-sata-oem-spec.pdf)
2. https://serverfault.com/a/1047332
3. [Seagate SCSI Reference](https://www.seagate.com/files/staticfiles/support/docs/manual/Interface%20manuals/100293068k.pdf)

# Audit (2025-06-01, Schuwi)

I have audited the repository to ensure it does not contain any malicious code.

## Audit Result

I have found no malicious code in the repository. There might be some bugs, but they do not pose a security risk
(the `set-state` subcommand might always disable the power mode timer even with the `--enable` flag set).

For the low-level SCSI-to-ATA commands and unix SCSI interface, I have checked
multiple independent sources to ensure correctness.

For the high-level ATA commands I have only relied on the HC320 SATA spec [6.1] linked by the author,
which is published by Western Digital and should be trustworthy.

There is one minor concern. According to one source [5.1], the `SAT_ATA_PASS_THROUGH12` SCSI command
that is used for almost all ATA commands executed by this tool, "clashes with MMC BLANK command" because
it uses the same SCSI command code `0xA1`. It's also listed in [1.1] as "BLANK" but I could not determine
what this command does exactly.\
From the name "MMC BLANK" it COULD be related to eMMC memory cards - so maybe just make sure you don't
accidentally use this tool on such devices.

## References
- [1.1] https://www.t10.org/lists/op-num.htm ([accessed 2025-06-01](https://web.archive.org/web/20250601094958/https://www.t10.org/lists/op-num.htm "Wayback Machine"))
- [1.2] INCITS/T10: SCSI / ATA Translation (SAT), Draft 9 (13 September 2006, https://web.archive.org/web/20070221091003/http://www.t10.org/ftp/t10/drafts/sat/sat-r09.pdf)
- [2.1] https://android.googlesource.com/platform/system/sepolicy/+/ae46511bfa62b56938b3df824bb2ee737dceaa7a/ioctl_defines ([accessed 2025-06-01](https://web.archive.org/web/20250601095826/https://android.googlesource.com/platform/system/sepolicy/+/ae46511bfa62b56938b3df824bb2ee737dceaa7a/ioctl_defines "Wayback Machine"))
- [2.2] https://android.googlesource.com/platform/system/sepolicy/+/refs/heads/android15-qpr1-release/public/ioctl_defines ([accessed 2025-06-01](https://web.archive.org/web/20250601100201/https://android.googlesource.com/platform/system/sepolicy/+/refs/heads/android15-qpr1-release/public/ioctl_defines "Wayback Machine"))
- 3: Douglas Gilbert
- [3.1] https://sg.danny.cz/sg/sg_io.html ([accessed 2025-06-01](https://web.archive.org/web/20250601100457/https://sg.danny.cz/sg/sg_io.html "Wayback Machine"))
- [3.2] https://tldp.org/HOWTO/SCSI-Generic-HOWTO/sg_io.html ([accessed 2025-06-01](https://web.archive.org/web/20250601100709/https://tldp.org/HOWTO/SCSI-Generic-HOWTO/sg_io.html "Wayback Machine"))
- [3.3] https://tldp.org/HOWTO/SCSI-Generic-HOWTO/sg_io_hdr_t.html ([accessed 2025-06-01](https://web.archive.org/web/20250601101057/https://tldp.org/HOWTO/SCSI-Generic-HOWTO/sg_io_hdr_t.html "Wayback Machine"))
- [4.1] https://github.com/torvalds/linux/blob/v6.15/include/scsi/sg.h ([accessed 2025-06-01](https://web.archive.org/web/20250601101854/https://github.com/torvalds/linux/blob/v6.15/include/scsi/sg.h "Wayback Machine"))
- 5: Douglas Gilbert (sg3-utils)
- [5.1] https://git.launchpad.net/ubuntu/+source/sg3-utils/tree/src/sg_sat_identify.c?h=ubuntu%2Fjammy ([accessed 2025-06-01](https://web.archive.org/web/20250601104102/https://git.launchpad.net/ubuntu/+source/sg3-utils/tree/src/sg_sat_identify.c?h=ubuntu%2Fjammy "Wayback Machine"))
- [6.1] HC320 SATA Spec, https://documents.westerndigital.com/content/dam/doc-library/en_us/assets/public/western-digital/product/data-center-drives/ultrastar-dc-hc300-series/product-manual-ultrastar-dc-hc320-sata-oem-spec.pdf ([accessed 2025-06-01](https://web.archive.org/web/20250601120755/https://documents.westerndigital.com/content/dam/doc-library/en_us/assets/public/western-digital/product/data-center-drives/ultrastar-dc-hc300-series/product-manual-ultrastar-dc-hc320-sata-oem-spec.pdf "Wayback Machine"))