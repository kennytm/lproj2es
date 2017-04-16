lproj2es
========

**lproj2es** is a utility to extract all localized strings `*.lproj/*.strings` into an ElasticSearch index for easy
look-up. This allows translators to produce localizations using terms from endorsed by Apple.

## Loading the iOS root filesystem

1. Download an IPSW from <https://ipsw.me>. IPSWs nowadays are typically over 2 GB in size, so make sure you have got
    enough time to download the file, and enough harddisk space to store and unzip it.

2. Unzip the IPSW to extract the root filesystem image (the filenames vary between iOS versions).

    ```sh
    $ unzip iPhone_7Plus_10.3.1_14E304_Restore.ipsw 058-55433-171.dmg
    ```

3. Attach the image.

    ```sh
    $ open 058-55433-171.dmg
    ```

## Building the index

1. Start the ElasticSearch cluster. We assume ElasticSearch is accessible via http://127.0.0.1:9200.

2. Run the program. It typically takes less than 3 minutes to finish with ElasticSearch on the local machine.

    ```sh
    $ ./lproj2es /Volumes/Erie14E304.D11D111OS/
    ```

## Document structure

The indexed translations are stored in the `localizations` index with `ios` type, using 1 replica and 1 shard. Each
document looks like:

```json
{
    "BUNDLE": "/Volumes/Erie14E304.D11D111OS/System/Library/CoreServices/SpringBoard.app",
    "FILE": "SpringBoard.strings",
    "KEY": "AWAY_LOCK_LABEL",
    "en_US": "slide to unlock",
    "fr_FR": "Déverrouiller",
    "es_ES": "Deslizar para desbloquear",
    "de_DE": "Entsperren",
    ...
}
```

The **BUNDLE** field specifies the bundle the translation belongs to. The **FILE** field gives the `*.strings` file that
contains the translation. The document encodes a single key-value entry in the `*.strings` file, with the key in the
**KEY** field, and values in their respective locale-id field (**en\_US**, **fr\_FR**, etc.)

## Options

```
lproj2es 0.1.0
kennytm <kennytm@gmail.com>
Dump iOS localization strings into ElasticSearch

USAGE:
    lproj2es [OPTIONS] <root>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -u, --url <base>             Acesss point of the ElasticSearch cluster [default: http://127.0.0.1:9200]
    -i, --index <index>          Name of the index [default: localizations]
    -t, --type <type_>           Name of the type [default: ios]
        --shards <shards>        Number of shards of the new index [default: 1]
        --replicas <replicas>    Number of replicas of the new index [default: 1]

ARGS:
    <root>    Root directory to scan for localized bundles
```
